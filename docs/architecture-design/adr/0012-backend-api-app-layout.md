# ADR-0012：Backend API app 放在 `apps/api`

## 状态

Accepted

## 背景

STDAS 当前仍是最小应用阶段，只有一个对外 HTTP API Gateway 和一个前端工作台。此前把 `stdas-gateway` 放在 `crates/services/stdas-gateway`，虽然符合未来多服务 workspace 的想象，但对当前项目不够清晰：

- `crates/` 容易让新手误以为当前 backend 是可复用 library 或内部服务集合，而不是一个对外 API application。
- 当前项目没有多个内部服务进程，过早使用 `crates/services/` 会让目录结构显得比实际复杂。
- 用户明确要求参考 Melrose《Rust + Axum 后端架构设计文档》的 Axum 后端结构。该文章展示的是一个 API project 的 `Cargo.toml`、`src/`、`tests/`、配置和迁移边界，而不是 library crate 集合。

## 决策

STDAS 保留 monorepo，但当前 backend API application 放在：

```text
apps/api/
├── Cargo.toml
├── src/
└── tests/
```

`apps/api/Cargo.toml` 的 package name 继续使用 `stdas-gateway`，因为它仍是系统架构中的唯一外部 API 入口。命名和路径的含义分开：

- `apps/api`：文件系统上表示“这是当前对外 API application”。
- `stdas-gateway`：架构上表示“这是唯一外部 HTTP API Gateway”。

`apps/api/src` 继续参考 Melrose 文章的 Axum 分层：

```text
src/
├── main.rs
├── lib.rs
├── app.rs
├── config/
├── dto/
├── errors/
├── handlers/
├── middleware/
├── models/
├── repositories/
├── routes/
├── services/
├── state.rs
├── server.rs
├── telemetry.rs
└── utils/
```

`crates/` 只在出现真实共享库、基础设施库、工具 crate 或内部服务 crate 时创建，例如：

- `crates/shared/config`
- `crates/shared/error`
- `crates/libs/event-bus`
- `crates/services/data-pipeline-service`
- `crates/tools/stdas-cli`

当前没有这些真实 crate 时，不提交空 `crates/` 目录。

## 约束

- 不把参考文章的 standalone backend project 直接复制到仓库根目录；STDAS 仍然有 `apps/web`、`docs`、workspace lockfile 和 repo-level command surface。
- 不为了“看起来完整”创建空的 `apps/api/config/*.toml`、`apps/api/migrations/`、`src/db/`、`src/cache/`、`src/extractors/` 或 `src/tasks/`。
- 出现真实 PostgreSQL persistence 时，才创建 SQLx pool、migration 和 repository 实现。
- 出现真实 Redis cache 时，才创建 cache module。
- 出现真实认证或 request validation extractor 时，才创建 extractors module。
- Handler 不直接写 SQL；SQLx 代码必须进入 repository/data access 边界。
- API contract、DTO、model、service/usecase、repository、error mapping 和 telemetry 边界必须保持可审查。

## 影响

- Rust workspace member 从 `crates/services/stdas-gateway` 改为 `apps/api`。
- `cargo run -p stdas-gateway`、`cargo gateway`、`cargo gateway-routes` 命令保持不变。
- 文档中所有当前路径应使用 `apps/api`；历史 Loco 调查或已废弃路径必须标注为历史背景。
- ADR-0010 的 `apps/ + crates/` monorepo 原则仍然有效，但 Gateway app 位置由本 ADR 修订。

## 验证

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo run -p stdas-gateway -- routes`
