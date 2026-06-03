# ADR-0013：采用 `backend/` + `frontend/` 物理分区

## 状态

Accepted

## 背景

STDAS 是一个会演进为多后端服务的数据分析平台。此前 `apps/api` 与 `apps/web` 并列，符合部分 fullstack monorepo 对 “application” 的命名习惯，但对当前项目会造成两个问题：

- `stdas-gateway` 是后端入口服务，不是前端旁边的轻量 API 子项目。
- 用户需要清晰学习 Rust、前端和 Git/GitHub 项目管理，目录语义应直接降低理解成本，避免 AI Agent 把前后端边界混淆。

Axum 不强制顶层目录名称；Rust Cargo workspace 也只要求 workspace member 是有效 package。因此顶层目录应优先服务 STDAS 的长期服务边界和可维护性。

## 决策

STDAS 采用前后端物理分区：

```text
STDAS/
├── backend/
│   ├── services/
│   │   ├── stdas-gateway/
│   │   ├── identity-service/
│   │   ├── customer-service/
│   │   ├── data-pipeline-service/
│   │   ├── analytics-service/
│   │   ├── workflow-service/
│   │   └── integration-service/
│   ├── shared/
│   ├── libs/
│   ├── tools/
│   └── proto/
├── frontend/
│   └── web/
├── docs/
├── scripts/
└── deploy/
```

当前实现阶段只创建真实存在的项目：

- `backend/services/stdas-gateway`：当前唯一外部 HTTP API Gateway，package name 保持 `stdas-gateway`。
- `frontend/web`：React + TypeScript 前端工作台。

未来服务按业务和数据所有权放入 `backend/services/<service-name>`。`stdas-gateway` 是系统入口基线服务，不作为业务拆分节点。当前阶段采用 [ADR-0014](0014-gateway-modular-monolith.md) 定义的 single runtime modular monolith；任何业务服务拆分都必须由触发条件驱动，而不是按固定顺序或预设时间表执行。

## 约束

- 前端源码不得放入 `backend/`。
- 后端服务不得放入 `frontend/`。
- 后端服务统一作为 Rust workspace member 管理。
- `backend/shared/`、`backend/libs/`、`backend/tools/` 和 `backend/proto/` 只在出现真实 crate、工具或 contract 时创建；不提交空目录。
- 不使用含义模糊的 `admin-service`。后台管理能力按数据所有权进入 `identity-service`、`customer-service` 等稳定边界。
- `stdas-gateway` 仍是唯一外部 HTTP API 入口，前端不得绕过 Gateway 访问内部服务。

## 影响

- Rust workspace member 从 `apps/api` 改为 `backend/services/stdas-gateway`。
- Frontend workspace member 从 `apps/web` 改为 `frontend/web`。
- 根目录 `package.json` 继续作为 repo-level command surface，但脚本代理到 `frontend/web`。
- `cargo run -p stdas-gateway`、`cargo gateway`、`cargo gateway-routes` 命令保持不变。
- ADR-0010 和 ADR-0012 标记为 Superseded。

## 验证

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo run -p stdas-gateway -- routes`
- `pnpm lint`
- `pnpm typecheck`
- `pnpm test`
- `pnpm build`
