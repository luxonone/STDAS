# Rust Workspace 与后端模块边界

STDAS 后端采用 Rust workspace 管理当前唯一运行服务 `stdas-gateway`，并为未来 crate / runtime service 拆分保留清晰边界。

当前阶段遵循 [ADR-0014](../architecture-design/adr/0014-gateway-modular-monolith.md)：**单一运行服务 + 强模块边界**。未来是否拆出 `identity-service`、`customer-service`、`data-pipeline-service`、`analytics-service`、`workflow-service` 或 `integration-service`，由触发条件驱动，不由预设时间表驱动。

## 当前结构

```text
stdas/
├── Cargo.toml
├── backend/
│   └── services/
│       └── stdas-gateway/
│           ├── Cargo.toml
│           ├── src/
│           │   ├── main.rs
│           │   ├── lib.rs
│           │   ├── app.rs
│           │   ├── config/
│           │   ├── errors/
│           │   ├── middleware/
│           │   ├── modules/
│           │   │   ├── identity/
│           │   │   ├── customer/
│           │   │   ├── data_pipeline/
│           │   │   ├── analytics/
│           │   │   ├── evidence/
│           │   │   ├── workflow/
│           │   │   └── integration/
│           │   ├── routes/
│           │   ├── shared/
│           │   ├── system/
│           │   ├── audit/
│           │   ├── telemetry/
│           │   ├── server.rs
│           │   └── state.rs
│           └── tests/
├── frontend/
│   └── web/
└── docs/
```

当前不创建空的 `backend/shared/`、`backend/libs/`、`backend/tools/` 或 `backend/proto/`。这些目录只有出现真实共享 crate、基础设施库、工具或 contract 时才创建。

## 当前运行服务

| Crate | 当前职责 |
|-------|----------|
| `stdas-gateway` | 唯一后端 runtime service、唯一外部 HTTP API 入口、Axum application assembly、business module container |

`stdas-gateway` 保留名称，因为它始终是唯一外部 API 入口。当前阶段它不是薄 Gateway，而是 modular monolith；未来如果服务拆分，它再逐步收敛为 Gateway / BFF。

## 业务模块边界

| Module | 职责 | 未来可能升级为 |
|--------|------|----------------|
| `identity` | 用户、角色、session、permission、CustomerScope | `identity-service` |
| `customer` | CustomerConfig、DataProfile、ProfileResolutionKey、parser/mapping/spec rule binding、rule fork、feature flags、extension registry；不是普通客户 CRUD | `customer-service` |
| `data_pipeline` | file registration、raw metadata、parser selection、normalization、canonical TestData、DataVersion、lineage | `data-pipeline-service` |
| `analytics` | query、aggregation、QuerySnapshot、analysis result、export | `analytics-service` |
| `evidence` | DataVersion、lineage、QuerySnapshot、analysis result、export/report 的证据链视图 | crate 或 service，按压力判断 |
| `workflow` | job state、retry、compensation、long-running process coordination | `workflow-service` |
| `integration` | MES schema reference、未来 MES runtime connector、客户接口、外部文件交换 | `integration-service` |

规则：

- `modules/` 只放未来可能升级为 crate 或 runtime service 的业务边界。
- 当前可以先建立 module shell 和边界说明，不为了“看起来完整”生成 fake handler、fake repository 或 fake service。
- 每个 module 内部出现真实代码后，再按需要创建 `routes.rs`、`handlers.rs`、`dto.rs`、`service.rs`、`repository.rs`、`models.rs` 或更细目录。
- `lineage` 归属 `data_pipeline`，因为它描述数据生命周期来源链。
- `evidence` 独立为 module，因为它连接 `data_pipeline`、`analytics` 和前端证据展示，但不拥有 raw data、parser 或 query engine。

## 横切边界

| 路径 | 职责 |
|------|------|
| `system/` | health、preflight、route catalog 相关运维端点；不是业务服务边界 |
| `telemetry/` | tracing、metrics、request id、correlation id |
| `audit/` | 谁或哪个系统对业务对象做了什么 |
| `middleware/` | Axum/Tower middleware |
| `errors/` | typed error、API error mapping |
| `shared/` | 稳定、低业务含义、跨模块确实共用的基础类型，例如 API envelope、ID newtype、pagination、time range、CustomerScope |
| `config/` | 环境变量和服务配置；当前读取 `STDAS_GATEWAY_ADDR` 和 `STDAS_DATABASE_URL` |
| `db/` | PostgreSQL connection pool 和 SQLx migration 入口；当前由身份会话切片使用 |

`shared/` 禁止承载具体业务 use case、parser logic、DataProfile 发布逻辑、analytics query engine、repository implementation 或“不知道放哪里”的代码。

判断标准：

```text
只有一个 module 使用 -> 放回拥有该概念的 module
强业务含义 -> 放进 owning module
稳定、通用、跨多个 module 使用 -> 才放 shared
```

## 未来 workspace 扩展

未来满足拆分条件时，后端目录可以扩展为：

```text
backend/
├── services/
│   ├── stdas-gateway/
│   ├── identity-service/
│   ├── customer-service/
│   ├── data-pipeline-service/
│   ├── analytics-service/
│   ├── workflow-service/
│   └── integration-service/
├── crates/
│   ├── stdas-ingestion-core/
│   ├── stdas-data-profile-core/
│   └── stdas-evidence-core/
├── shared/
├── libs/
├── tools/
└── proto/
```

推荐演进路径：

```text
module -> independent crate -> runtime service
```

不要直接因为“概念上像服务”就拆 runtime service。拆分必须先满足 ADR-0014 的黄色/红色触发条件。

## 未来服务职责

| 未来服务 | 职责 |
|----------|------|
| `identity-service` | 用户、角色、token、会话、权限、CustomerScope |
| `customer-service` | CustomerConfig、DataProfile、ProfileResolutionKey、规则复用/分叉、扩展注册、客户专属能力隔离 |
| `data-pipeline-service` | 文件登记、raw metadata、parser 选择、staging、mapping、canonical TestData、DataVersion、lineage |
| `analytics-service` | 分析执行框架、聚合、OLAP adapter、算法 registry、告警规则评估、分析会话、模板、导出、客户专属分析扩展 |
| `workflow-service` | Saga / Process Manager、作业状态、重试、补偿、Dead Letter |
| `integration-service` | MES runtime connector、客户接口、外部文件交换、设备数据同步 |
| `stdas-cli` | 运维、修复、回放、数据验证工具 |

## 依赖规则

当前 single runtime：

```text
routes -> owning module router/handler
handler -> owning module service/usecase
service/usecase -> owning module repository/data access
module -> shared only for stable foundations
module -> another module only through explicit query/usecase boundary
```

未来 multi-service：

```text
gateway          -> service clients + shared
service          -> own domain + own repository + shared + libs
service clients  -> proto generated clients only
libs             -> shared only
shared           -> no service dependency
```

规则：

- Handler 只做协议适配，不写 SQL，不承载业务流程。
- Service/usecase 拥有业务流程、事务、幂等、权限、缓存、事件和领域规则。
- Repository/data access 只实现 owning module 的数据访问。
- 持久化采用 SQLx repository，不采用 ORM / Active Record。
- Parser 不能直接写 analytics 或 integration 数据。
- Analytics 不依赖客户专用 parser。
- 跨模块长流程先进入 `modules/workflow`；只有 runtime service 拆分后才引入 `workflow-service`。

## 技术选型

| 能力 | 当前策略 |
|------|----------|
| Gateway HTTP | `axum` + `tower` / `tower-http` |
| async runtime | `tokio` |
| DB | `sqlx` + PostgreSQL；只在真实 repository 出现时引入 pool、migration 和 SQL |
| serialization | `serde` |
| error | `thiserror` + typed `AppError`；当前尚未有真实错误模型时只保留边界 |
| tracing | `tracing` + `tracing-subscriber`；当前只保留 telemetry boundary |
| auth | 后续按 `identity` module 真实功能引入 |
| OpenAPI | 后续由 Gateway 暴露 |
| gRPC / event bus / object storage | 未来服务拆分或真实数据流水线压力出现后再引入 |

## 编码规则

Rust 通用代码质量约束见 [rust-code-quality-rules.md](rust-code-quality-rules.md)。本节只记录 STDAS workspace 内的分层规则。

- `src/main.rs` 只处理进程入口和命令分发。
- `src/app.rs` 只组装 Axum `Router`、routes、middleware 和 `AppState`。
- `src/server.rs` 只负责监听地址、`TcpListener` 和 `axum::serve`。
- `src/routes/` 管 API version router 和 route catalog。
- 运维端点放 `src/system/`，业务端点放 owning module。
- 每个 business module 内部按真实需要建立 handler、service/usecase、repository、DTO、model。
- 身份会话已经是当前真实数据库用例；`stdas-gateway` 允许包含 `src/db/`、service-local `migrations/` 和 SQLx dependency。
- 当前没有 Redis、认证 extractor、后台任务或配置文件时，不创建 `cache/`、`extractors/`、`tasks/` 或 service-local `config/*.toml`。
- 字段命名不能凭空设计；用户和权限相关字段优先参考 MES 语义，但表范围按 STDAS 测试部门内部系统裁剪。
