# ADR-0011：Gateway 直接采用 Axum，持久化层采用 SQLx

## 状态

Accepted

## 背景

STDAS 当前仍处于最小应用和架构基线阶段，没有必须兼容的业务历史包袱。后端技术选择应优先服务长期目标：清晰服务边界、稳定 API 契约、可测试的 Rust 代码、显式 SQL、可控数据库迁移，以及未来可能出现的数据库迁移、项目迁移甚至语言迁移。

Rust skills 三层分析给出的约束如下：

- Layer 3：STDAS 是内部工程分析工作台和数据平台，不是通用 CRUD 后台；Gateway 必须保持薄 HTTP 边界，不能把数据解析、权限、SQL、业务流程、缓存和事件混进 handler。
- Layer 2：架构应以领域模型、service/usecase、repository 和稳定 API contract 分层，而不是依赖框架 scaffold 或 ORM model 生成业务结构。
- Layer 1：Rust 实现应使用 Axum extractor、Router、Tower middleware、typed error、typed DTO 和 SQLx query/repository pattern，避免 `unwrap`、fat handler、primitive obsession 和不必要的共享可变状态。

Loco 基于 Axum，能提供统一 scaffold 和 CLI，但其默认生态更偏向框架组织和 ORM 集成。STDAS 更看重显式边界、SQL 可审查性和迁移弹性，因此不把 Loco 作为后端基线。

本 ADR 的口径不是“从 Loco 最小迁移”，而是把当前最小应用作为新起点，重新建立 Axum service crate 结构。项目结构参考 Melrose《Rust + Axum 后端架构设计文档》（https://melrose1994.com/index.php/2026/04/29/rust-axum-%e5%90%8e%e7%ab%af%e6%9e%b6%e6%9e%84%e8%ae%be%e8%ae%a1%e6%96%87%e6%a1%a3/）中展示的 Axum application assembly、配置、错误、中间件、route、state 和 telemetry 边界，但后续由 [ADR-0014](0014-gateway-modular-monolith.md) 调整为 business module boundary 优先，而不是全局 `handlers/services/repositories/dto/models` 目录长期扩张。

## 决策

`stdas-gateway` 直接采用 Axum 作为 HTTP 应用框架。当前 crate path 为 `backend/services/stdas-gateway`；目录落点决策见 [ADR-0013](0013-backend-frontend-physical-partition.md)。

持久化层采用 SQLx + PostgreSQL，不采用 ORM / Active Record。SQLx 只在实际需要数据库访问的服务或 repository 中引入；当前最小 Gateway 没有数据库边界，因此不为了“预留”而加入 unused SQLx dependency。

`backend/services/stdas-gateway` 当前采用以下服务内部结构：

```text
src/
├── main.rs
├── lib.rs
├── app.rs
├── config/
├── audit/
├── errors/
├── middleware/
├── modules/
│   ├── identity/
│   ├── customer/
│   ├── data_pipeline/
│   ├── analytics/
│   ├── evidence/
│   ├── workflow/
│   └── integration/
├── routes/
├── shared/
├── system/
├── state.rs
├── server.rs
└── telemetry/
```

结构含义：

- `src/main.rs` 是单 binary crate 的 Cargo 默认入口，只解析进程级命令并调用 server。
- `src/app.rs` 是 Axum application assembly 边界，只组装 `Router`、route tree、middleware 和 shared state。
- `src/server.rs` 负责监听地址、`TcpListener` 和 `axum::serve`。
- `src/config/` 负责环境变量和服务配置；当前只包含 `STDAS_GATEWAY_ADDR`。
- `src/state.rs` 是 Axum shared state 边界；当前挂载最小 `SystemService`，未来可挂载 module usecase、service client 或 SQLx pool。
- `src/routes/` 负责 route catalog 和 API version router。
- `src/middleware/` 放 Tower middleware；当前 CORS 使用本地前端 origin 白名单，不使用 wildcard 作为默认基线。
- `src/modules/` 放未来可能升级为 crate 或 runtime service 的业务边界；每个 module 内部再按真实需要放 handler、service/usecase、repository、DTO 和 model。
- `src/system/` 放 health、preflight 等运维端点，不属于业务服务边界。
- `src/shared/` 放稳定、低业务含义、跨模块确实共用的基础类型；不得成为垃圾桶。
- `src/audit/` 放跨模块 audit 边界，记录谁或哪个系统对业务对象做了什么。
- `src/errors/` 放 typed error 和 API error mapping。
- `src/telemetry/` 放 tracing、metrics、request id 等观测性边界；当前只保留归属边界。

当前不创建空的 `db/`、`cache/`、`extractors/`、`tasks/`、service-local `config/*.toml` 或 `migrations/`。这些目录只有在出现真实 SQLx pool、Redis、认证 extractor、后台任务、配置文件或数据库迁移需求时才创建。

其中 SQLx pool、transaction、migration 调用和 repository 实现位于拥有该数据的 module 内部 repository/data access 边界，handler 不直接写 SQL。

## 设计规则

- `stdas-gateway` 是唯一外部 HTTP API 入口，前端只访问 Gateway。
- Gateway handler 只做协议适配：extractor、request validation、auth context、调用 command/query service、response mapping。
- Gateway 不直接读写内部服务数据库；需要数据时通过内部 service client、gRPC、事件构建的 projection 或明确的 gateway-owned 查询边界实现。
- 业务服务的 repository 使用 SQLx，SQL 必须显式、可审查、可测试。
- 数据库迁移使用 SQLx migration，不使用 ORM migration。
- API contract 先于数据库表结构，前端不得依赖表结构。
- 错误统一映射为稳定 API error code；内部错误使用 typed error。
- 对高成本查询、导出、摄入和后台任务，优先使用 job、query snapshot、budget 和 event/outbox 设计，不把复杂流程塞进 HTTP handler。
- 单服务 crate 使用 Cargo 默认 `src/main.rs`；只有确实存在多个 binary 时才使用 `src/bin/*`。
- 使用 `src/app.rs` 作为 Axum application assembly；不使用 Loco/MVC 的 `src/controllers`、`src/http/*` 或 service-local Cargo alias 作为 Axum Gateway 默认结构。

## 影响

- `stdas-gateway` 移除 Loco dependency、Loco YAML 配置和 `cargo loco` 命令。
- `stdas-gateway` 移除 Loco/MVC 风格目录，采用 `main/app/config/server/state/routes/middleware/errors/telemetry/audit/shared/system/modules` 的 Axum modular monolith 结构。
- Gateway 启动命令改为 `cargo run -p stdas-gateway` 或 `cargo gateway`。
- 路由检查命令改为 `cargo run -p stdas-gateway -- routes` 或 `cargo gateway-routes`。
- ADR-0009 只作为历史记录保留，不再作为活跃架构决策。
- Loco error handling 调查日志只保留为早期技术评估证据。

## 验证

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo run -p stdas-gateway -- routes`
- 前端验证命令按变更范围执行：`pnpm lint`、`pnpm typecheck`、`pnpm test`、`pnpm build`
