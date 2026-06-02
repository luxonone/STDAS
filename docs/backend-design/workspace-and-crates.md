# Rust Workspace 与服务边界

STDAS 后端采用 Rust workspace 管理所有微服务、共享契约、基础设施库和运维工具。每个业务服务都是可独立编译、独立运行、独立配置的进程。

服务划分采用粗粒度平台能力边界，避免把每个处理步骤都拆成一个服务。

## 推荐结构

```text
stdas/
├── Cargo.toml
├── apps/
│   ├── api/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   └── tests/
│   └── web/
│       ├── package.json
│       ├── vite.config.ts
│       └── src/
├── proto/
│   ├── identity.proto
│   ├── profile.proto
│   ├── data_pipeline.proto
│   ├── analytics.proto
│   ├── workflow.proto
│   ├── integration.proto
│   └── events.proto
├── crates/
│   ├── shared/
│   │   ├── config/
│   │   ├── telemetry/
│   │   ├── auth/
│   │   ├── error/
│   │   └── service-client/
│   ├── libs/
│   │   ├── event-bus/
│   │   ├── object-store/
│   │   ├── outbox/
│   │   ├── inbox/
│   │   └── profile-contracts/
│   ├── services/
│   │   ├── identity-service/
│   │   ├── customer-service/
│   │   ├── data-pipeline-service/
│   │   ├── analytics-service/
│   │   ├── workflow-service/
│   │   └── integration-service/
│   └── tools/
│       └── stdas-cli/
├── config/ 或 apps/api/config/ 或 crates/services/<service>/config/
│   └── 服务级配置；只有服务真正需要配置文件时才创建
├── scripts/
│   ├── start-all.ps1
│   ├── stop-all.ps1
│   ├── start-all.sh
│   └── stop-all.sh
└── deploy/
    ├── windows/
    └── linux/
```

## 服务职责

| Crate | 职责 |
|------|------|
| `stdas-gateway` | 外部 HTTPS API、BFF、认证上下文、请求聚合、限流。当前 crate path 为 `apps/api`，因为它是本项目的对外 API application。 |
| `identity-service` | 用户、角色、token、会话、权限、CustomerScope |
| `customer-service` | 客户专属服务：CustomerConfig、DataProfile、ProfileResolutionKey、规则复用/分叉、扩展注册、客户专属能力隔离 |
| `data-pipeline-service` | 文件登记、raw metadata、parser 选择、staging、mapping、canonical TestData、DataVersion、lineage |
| `analytics-service` | 分析执行框架、聚合、OLAP adapter、算法 registry、告警规则评估、分析会话、模板、导出、客户专属分析扩展 |
| `workflow-service` | Saga / Process Manager、作业状态、重试、补偿、Dead Letter |
| `integration-service` | MES、客户接口、外部文件交换、设备数据同步 |
| `stdas-cli` | 运维、修复、回放、数据验证工具 |

## 服务内部模块

服务内部可以按模块组织，但模块不是独立进程。

| 服务 | 内部模块 |
|------|------|
| `data-pipeline-service` | ingestion、parser、normalization、canonical writer、data version、lineage |
| `analytics-service` | query、aggregation、analysis registry、execution engine、result materialization、alerting、workspace、export |
| `customer-service` | customer config、data profile、parser/mapping/spec rule binding、rule fork、feature flags、extension registry |

## 共享库边界

共享库只能放稳定基础能力，不能沉淀跨服务业务流程。当前没有真实共享库时，不创建或提交空的 `crates/` 子目录。

| 共享库 | 可包含 | 禁止包含 |
|------|------|------|
| `shared/config` | TOML 加载、环境变量、平台路径 | 服务业务默认值 |
| `shared/telemetry` | tracing、metrics、health helper | 业务日志解释 |
| `shared/auth` | token 验签、claims、scope helper | 用户管理流程 |
| `shared/error` | 错误码、错误映射 | 服务私有错误细节 |
| `libs/event-bus` | NATS publish/subscribe、JetStream helper | 具体事件编排 |
| `libs/outbox` | outbox 表模型、publisher 框架 | 某服务业务事件含义 |
| `libs/inbox` | 幂等消费、consumer checkpoint | 具体补偿逻辑 |
| `libs/profile-contracts` | ProfileResolutionKey、DataProfile DTO、RuleBinding DTO | 客户私有 parser |

## 依赖规则

```text
gateway          -> service clients + shared
service          -> own domain + own repository + shared + libs
service clients  -> proto generated clients only
libs             -> shared only
shared           -> no service dependency
```

规则：

- 服务不能直接依赖另一个服务的 domain crate。
- 服务间只通过 gRPC contract、event contract 或对象存储引用协作。
- 跨服务长流程必须进入 `workflow-service` 或事件流水线。
- Parser 不能直接写 analytics 或 integration 数据。
- Analytics 不依赖客户专用 parser。

## 技术选型

| 能力 | 推荐 |
|------|------|
| Gateway HTTP | `axum` + `tower` / `tower-http`，Gateway 直接使用 Axum router、extractor、state、middleware 和 response pattern |
| gRPC | `tonic` |
| async runtime | `tokio` |
| event bus | `async-nats` + NATS JetStream |
| DB | `sqlx` + PostgreSQL |
| object storage | S3 compatible client + MinIO |
| serialization | `serde` + `prost` |
| error | `thiserror` + typed `AppError` |
| tracing | `tracing` + `tracing-subscriber` |
| auth | `jsonwebtoken` + `argon2` |
| OpenAPI | `utoipa`，由 gateway 暴露 |

## 编码规则

Rust 通用代码质量约束见 [rust-code-quality-rules.md](rust-code-quality-rules.md)。本节只记录 STDAS workspace 内的分层规则。

- Handler 只做协议适配，不写复杂 SQL。
- `apps/api` 参考 Melrose《Rust + Axum 后端架构设计文档》的 Axum API app 目录化分层：`src/main.rs` 为进程入口，`src/app.rs` 组装 Axum application，`src/routes/` 管路由配置，`src/middleware/` 管中间件，`src/handlers/` 管请求处理器，`src/dto/` 管 API 数据传输对象，`src/models/` 管内部模型，`src/services/` 管业务用例，`src/repositories/` 管数据访问边界，`src/errors/` 管错误边界，`src/config/` 管配置，`src/state.rs` 管 Axum shared state，`src/telemetry.rs` 管观测性边界，`src/utils/` 管领域无关工具。
- `src/server.rs` 负责监听地址、`TcpListener` 和 `axum::serve`；不得承载业务规则或 SQL。
- 当前最小服务没有真实数据库用例时，`repositories/` 只作为数据访问边界说明；出现真实持久化需求后，repository 使用 SQLx，并在该目录下按 use case 或 aggregate 拆分。
- 暂不创建 root-level `config/*.toml`、`migrations/`、`src/db/`、`src/cache/`、`src/extractors/` 或 `src/tasks/`；只有出现真实 SQLx pool、Redis、认证 extractor、后台任务、配置文件或数据库迁移需求时才创建。
- 持久化服务采用 SQLx repository，不采用 ORM / Active Record；handler 不直接写 SQL。
- Service use case 拥有本地事务边界。
- Repository/query 只实现本服务数据访问，不承载跨服务流程。
- 对外错误统一映射为稳定错误码。
- 大数组和图表结果要考虑内存上限和响应大小。
- 所有事件 consumer 必须实现幂等。

