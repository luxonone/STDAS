# 项目目录结构

本文件定义 STDAS 当前 monorepo 的项目管理边界。架构能力域、服务职责和数据边界仍以 [系统架构](architecture-design/system-architecture.md) 与 [Rust Workspace 与服务边界](backend-design/workspace-and-crates.md) 为准。

## 顶层结构

```text
STDAS/
├── backend/
│   ├── services/
│   │   └── stdas-gateway/
│   ├── shared/          # 未来真实共享基础能力 crate 出现时再创建
│   ├── libs/            # 未来真实基础设施库出现时再创建
│   ├── tools/           # 未来真实后端工具 crate 出现时再创建
│   └── proto/           # 未来 gRPC 和事件 contract 出现时再创建
├── frontend/
│   └── web/
├── docs/
│   └── specs/
├── scripts/
├── deploy/
├── .cargo/
├── Cargo.toml
├── Cargo.lock
├── package.json
├── pnpm-workspace.yaml
└── pnpm-lock.yaml
```

## 目录职责

| 路径 | 职责 |
|------|------|
| `backend/services/stdas-gateway` | Rust + Axum 后端服务。package name 仍为 `stdas-gateway`，因为它是系统架构中的唯一外部 API 入口。当前阶段它也是唯一 backend runtime service 和 modular monolith 容器；内部按未来服务边界组织 `modules/`，并保留 `system/`、`audit/`、`telemetry/`、`shared/` 等横切边界。 |
| `backend/services` | 后续内部 Rust 服务进程的放置位置，例如 `identity-service`、`customer-service`、`data-pipeline-service`、`analytics-service`。当前不创建空服务目录；服务拆分必须按 ADR-0014 的触发条件审查。 |
| `backend/shared` | 后续共享基础能力 crate，例如 config、telemetry、auth、error、service-client。当前没有真实共享库时不创建空目录。 |
| `backend/libs` | 后续稳定基础设施库，例如 event-bus、object-store、outbox、inbox、profile-contracts。当前没有真实基础设施库时不创建空目录。 |
| `backend/tools` | 后续运维 CLI 和本地数据工具。当前没有真实工具 crate 时不创建空目录。 |
| `backend/proto` | 后续 gRPC 和事件 proto 契约。当前没有真实服务 contract 时不创建空目录。 |
| `frontend/web` | React + TypeScript 前端工作台。包含 Vite、ESLint、TypeScript 配置和前端源码。 |
| `docs` | 项目事实来源。代码、目录和验证命令变更必须同步更新对应文档。 |
| `docs/specs` | 项目 SPEC 专区。保存铁律级规则，例如 AI Agent 启动上下文、编码规范、安全 gate。普通设计文档只能引用 SPEC，不能覆盖 SPEC。 |
| `docs/specs/vendor` | SPEC 引用的外部规范快照，例如 Rust Coding Guidelines 中文版。vendor 内容用于完整性和可追溯性，不自动覆盖 STDAS 自有 SPEC。 |
| `scripts` | 后续跨平台启动、停止、验证和维护脚本。 |
| `deploy` | 后续 Windows/Linux 部署资源。 |
| `.cargo` | workspace 级 Cargo alias。当前 `cargo gateway` 和 `cargo gateway-routes` 映射到 `stdas-gateway`。 |
| `Cargo.toml` | Rust workspace manifest。必须保留在仓库根目录，用于统一管理 crates 和 workspace dependency 口径。 |
| `Cargo.lock` | Rust workspace lockfile。STDAS 当前包含可执行服务，lockfile 必须保留在仓库根目录并随代码提交。 |
| `package.json` | repo-level pnpm 命令入口。只放跨项目 scripts，不放前端应用源码。 |
| `pnpm-workspace.yaml` | pnpm workspace 成员声明。当前包含 `frontend/*`。 |
| `pnpm-lock.yaml` | 前端 workspace lockfile。必须保留在仓库根目录并随代码提交。 |

## 本地生成目录

以下目录可能出现在仓库一级目录，但不是源码。AI Agent 和开发者不能只因为它们“看起来乱”就移动或删除，必须先判断是否可再生成、是否被 Git 跟踪、是否属于当前工具链的推荐位置。

| 路径 | 是否应存在于一级目录 | 管理方式 |
|------|----------------------|----------|
| `node_modules/` | 可以存在。pnpm workspace 从仓库根目录安装依赖时，根目录 `node_modules/` 是正常本地依赖目录。 | Git ignore；可用 `pnpm install` 再生成；不要移动到源码目录。 |
| `target/` | 可以存在。Cargo workspace 默认在仓库根目录生成 `target/`。 | Git ignore；可删除以释放磁盘，但不要提交、不要移动、不要把它当成源码。 |
| `frontend/web/dist/` | 可以存在。它是当前前端应用的 Vite build output。 | Git ignore；由 `pnpm build` 生成。 |
| `dist/` | 不应作为当前结构的长期目录。根目录 `dist/` 是旧前端根目录布局留下的构建产物。 | 如确认只是构建产物，应清理；新的前端产物只保留在 `frontend/web/dist/`。 |
| `tmp/` | 不应作为项目源码目录。仅允许短期本地 scratch。 | Git ignore；任务结束应清空或删除。 |
| `.swarm/` | 可以存在。它是本地 agent 状态，不属于项目源码。 | Git ignore；不参与架构、构建或交付。 |
| `reference-project/` | 可以存在。它是本地参考材料，不属于 STDAS 源码。 | Git ignore；不得把外部参考项目结构直接复制为 STDAS 架构。 |

## 命令入口

后端 Gateway：

```bash
cargo gateway
cargo gateway-routes
cargo run -p stdas-gateway
cargo run -p stdas-gateway -- routes
```

前端工作台：

```bash
pnpm dev
pnpm lint
pnpm typecheck
pnpm test
pnpm build
```

这些命令从仓库根目录执行。根目录 `package.json` 只保留 repo-level scripts；前端依赖和 Vite 配置在 `frontend/web/package.json`。

## 管理规则

- 不再把前端源码和 Vite 配置直接放在仓库根目录。
- `backend/` 只放 Rust 后端服务、后端共享 crate、后端基础设施库、后端工具和后端 contract。
- `frontend/` 只放前端应用和前端相关源码。
- 服务私有配置只在服务真正需要配置文件时放入服务目录；当前最小 `stdas-gateway` 使用环境变量 `STDAS_GATEWAY_ADDR` 配置监听地址，并使用 `STDAS_DATABASE_URL` 配置 PostgreSQL。
- Axum 服务使用 `src/app.rs` 作为 application assembly 边界；不得沿用 Loco/MVC 的 `src/controllers`、`src/http/*` 或 service-local `.cargo/config.toml` 作为默认结构；除非已有多个 binary，否则单服务 crate 使用 Cargo 默认 `src/main.rs`。
- 当前 Gateway 是 single runtime modular monolith：`routes/` 管 API version router，业务能力放入 `modules/`，运维端点放入 `system/`，横切能力放入 `audit/`、`telemetry/`、`middleware/`、`errors/`、`shared/`。身份会话已经是第一个真实数据库用例，因此 `stdas-gateway` 现在包含 `src/db/`、SQLx dependency 和 service-local `migrations/`。
- 根目录只保留 workspace、文档、锁文件、repo-level scripts 和跨项目工具配置。
- `docs/specs/` 是文档体系中的强约束专区，不按前端/后端实现目录拆分。
- `docs/specs/vendor/` 只保存外部规范快照和来源说明，不放本项目实现源码，不把外部项目结构复制为 STDAS 架构。
- 根目录 `Cargo.toml`、`Cargo.lock`、`package.json`、`pnpm-workspace.yaml`、`pnpm-lock.yaml` 是项目级管理文件，不得为了“目录更少”移动到子目录。
- 清理一级目录前，必须先用 Git 状态和 ignore 规则区分源码、锁文件、配置、本地生成物和外部参考材料。
- 新增后端服务优先放在 `backend/services/<service-name>`；新增前端应用优先放在 `frontend/<app-name>`；新增后端共享能力优先放在 `backend/shared/` 或 `backend/libs/`。
- 新增目录必须同步更新本文件、相关 section README 和验证命令。

## Gateway 服务内部结构

`stdas-gateway` 当前采用以下结构：

```text
backend/services/stdas-gateway/
└── src/
    ├── main.rs
    ├── lib.rs
    ├── app.rs
    ├── audit/
    ├── config/
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
    ├── server.rs
    ├── state.rs
    └── telemetry/
```

采用该结构的原因是让 AI Agent 和开发者都能根据目录名判断职责：

- `app.rs` 只负责组装 Axum `Router`、route tree、middleware 和 shared state。
- `routes/` 只负责 route catalog 和 API version router。
- `system/` 放 health、preflight 等运维端点，不属于业务服务边界。
- `modules/` 放未来可能升级为 crate 或 runtime service 的业务边界；每个 module 出现真实代码后，再在 module 内部建立 handler、service/usecase、repository、DTO、model。
- `audit/` 是横切审计边界，记录谁或哪个系统对业务对象做了什么。
- `telemetry/` 是运行观测边界，负责 tracing、metrics、request id、correlation id。
- `shared/` 只放稳定、低业务含义、跨模块确实共用的基础类型，不能成为垃圾桶。
- `errors/` 是 typed error 和 API error mapping 边界。

暂不创建 root-level `config/*.toml`、`migrations/`、`src/db/`、`src/cache/`、`src/extractors/` 或 `src/tasks/`。数据库连接和 migration 是 `stdas-gateway` 的真实功能依赖，因此放在 `backend/services/stdas-gateway/src/db/` 和 `backend/services/stdas-gateway/migrations/`，不放到仓库根目录。
