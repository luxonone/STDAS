# 项目目录结构

本文件定义 STDAS 当前 monorepo 的项目管理边界。架构能力域、服务职责和数据边界仍以 [系统架构](architecture-design/system-architecture.md) 与 [Rust Workspace 与服务边界](backend-design/workspace-and-crates.md) 为准。

## 顶层结构

```text
STDAS/
├── apps/
│   ├── api/
│   └── web/
├── crates/              # 未来真实共享库、基础设施库、工具或内部服务出现时再创建
├── docs/
│   └── specs/
├── proto/
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
| `apps/api` | Rust + Axum API Gateway 应用。package name 仍为 `stdas-gateway`，因为它是系统架构中的唯一外部 API 入口。内部结构参考 Melrose《Rust + Axum 后端架构设计文档》：`app.rs` 应用组装，`routes/` 路由，`middleware/` 中间件，`handlers/` 请求处理器，`services/` 业务用例，`repositories/` 数据访问边界，`models/` 内部模型，`dto/` API 数据传输对象，`errors/` 错误边界，`config/` 配置，`state.rs` shared state，`telemetry.rs` 观测性边界。 |
| `apps/web` | React + TypeScript 前端工作台。包含 Vite、ESLint、TypeScript 配置和前端源码。 |
| `crates/shared` | 后续共享基础能力 crate，例如 config、telemetry、auth、error、service-client。当前没有真实共享库时不创建空目录。 |
| `crates/libs` | 后续稳定基础设施库，例如 event-bus、object-store、outbox、inbox、profile-contracts。当前没有真实基础设施库时不创建空目录。 |
| `crates/tools` | 后续运维 CLI 和本地数据工具。当前没有真实工具 crate 时不创建空目录。 |
| `crates/services` | 后续内部 Rust 服务进程，例如 identity-service、data-pipeline-service、analytics-service。当前唯一外部 API Gateway 放在 `apps/api`，不放在 `crates/services`。 |
| `docs` | 项目事实来源。代码、目录和验证命令变更必须同步更新对应文档。 |
| `docs/specs` | 项目 SPEC 专区。保存铁律级规则，例如 AI Agent 启动上下文、编码规范、安全 gate。普通设计文档只能引用 SPEC，不能覆盖 SPEC。 |
| `docs/specs/vendor` | SPEC 引用的外部规范快照，例如 Rust Coding Guidelines 中文版。vendor 内容用于完整性和可追溯性，不自动覆盖 STDAS 自有 SPEC。 |
| `proto` | 后续 gRPC 和事件 proto 契约。 |
| `scripts` | 后续跨平台启动、停止、验证和维护脚本。 |
| `deploy` | 后续 Windows/Linux 部署资源。 |
| `.cargo` | workspace 级 Cargo alias。当前 `cargo gateway` 和 `cargo gateway-routes` 映射到 `stdas-gateway`。 |
| `Cargo.toml` | Rust workspace manifest。必须保留在仓库根目录，用于统一管理 crates 和 workspace dependency 口径。 |
| `Cargo.lock` | Rust workspace lockfile。STDAS 当前包含可执行服务，lockfile 必须保留在仓库根目录并随代码提交。 |
| `package.json` | repo-level pnpm 命令入口。只放跨项目 scripts，不放前端应用源码。 |
| `pnpm-workspace.yaml` | pnpm workspace 成员声明。当前包含 `apps/*`。 |
| `pnpm-lock.yaml` | 前端 workspace lockfile。必须保留在仓库根目录并随代码提交。 |

## 本地生成目录

以下目录可能出现在仓库一级目录，但不是源码。AI Agent 和开发者不能只因为它们“看起来乱”就移动或删除，必须先判断是否可再生成、是否被 Git 跟踪、是否属于当前工具链的推荐位置。

| 路径 | 是否应存在于一级目录 | 管理方式 |
|------|----------------------|----------|
| `node_modules/` | 可以存在。pnpm workspace 从仓库根目录安装依赖时，根目录 `node_modules/` 是正常本地依赖目录。 | Git ignore；可用 `pnpm install` 再生成；不要移动到源码目录。 |
| `target/` | 可以存在。Cargo workspace 默认在仓库根目录生成 `target/`。 | Git ignore；可删除以释放磁盘，但不要提交、不要移动、不要把它当成源码。 |
| `apps/web/dist/` | 可以存在。它是当前前端应用的 Vite build output。 | Git ignore；由 `pnpm build` 生成。 |
| `dist/` | 不应作为当前结构的长期目录。根目录 `dist/` 是旧前端根目录布局留下的构建产物。 | 如确认只是构建产物，应清理；新的前端产物只保留在 `apps/web/dist/`。 |
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

这些命令从仓库根目录执行。根目录 `package.json` 只保留 repo-level scripts；前端依赖和 Vite 配置在 `apps/web/package.json`。

## 管理规则

- 不再把前端源码和 Vite 配置直接放在仓库根目录。
- 服务私有配置只在服务真正需要配置文件时放入服务目录；当前最小 `stdas-gateway` 只使用环境变量 `STDAS_GATEWAY_ADDR` 配置监听地址。
- Axum 服务使用 `src/app.rs` 作为 application assembly 边界；不得沿用 Loco/MVC 的 `src/controllers`、`src/http/*` 或 service-local `.cargo/config.toml` 作为默认结构；除非已有多个 binary，否则单服务 crate 使用 Cargo 默认 `src/main.rs`。
- 当前最小 Gateway 按 `routes -> middleware -> handlers -> services -> repositories` 的方向组织代码；没有真实数据库用例时，`repositories/` 只作为数据访问边界，不引入 unused SQLx dependency。
- 根目录只保留 workspace、文档、锁文件、repo-level scripts 和跨项目工具配置。
- `docs/specs/` 是文档体系中的强约束专区，不按前端/后端实现目录拆分。
- `docs/specs/vendor/` 只保存外部规范快照和来源说明，不放本项目实现源码，不把外部项目结构复制为 STDAS 架构。
- 根目录 `Cargo.toml`、`Cargo.lock`、`package.json`、`pnpm-workspace.yaml`、`pnpm-lock.yaml` 是项目级管理文件，不得为了“目录更少”移动到子目录。
- 清理一级目录前，必须先用 Git 状态和 ignore 规则区分源码、锁文件、配置、本地生成物和外部参考材料。
- 新增对外应用优先放在 `apps/<app-name>`；新增内部服务优先放在 `crates/services/<service-name>`；新增前端应用优先放在 `apps/<app-name>`。
- 新增目录必须同步更新本文件、相关 section README 和验证命令。

## Gateway 服务内部结构

`stdas-gateway` 当前采用以下结构：

```text
apps/api/
└── src/
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
    ├── server.rs
    ├── state.rs
    ├── telemetry.rs
    └── utils/
```

采用该结构的原因是让 AI Agent 和开发者都能根据目录名判断职责：

- `app.rs` 只负责组装 Axum `Router`、route tree、middleware 和 shared state。
- `routes/` 只负责 route catalog 和 API version router。
- `handlers/` 只负责 Axum extractor、request/response mapping 和调用 service。
- `services/` 负责业务用例；handler 不直接承载业务流程。
- `repositories/` 是 SQLx 数据访问边界；当前没有数据库用例，所以只保留边界说明。
- `dto/` 是外部 API 数据传输对象；`models/` 是服务内部模型，二者不得混用。
- `errors/`、`telemetry.rs`、`utils/` 当前可以保持最小，但作为错误、观测性和通用工具的固定归属边界。

暂不创建 root-level `config/*.toml`、`migrations/`、`src/db/`、`src/cache/`、`src/extractors/` 或 `src/tasks/`。这些目录只有在出现真实 SQLx pool、Redis、认证 extractor、后台任务、配置文件或数据库迁移需求时才创建，避免为了“看起来完整”引入空目录和 unused dependency。
