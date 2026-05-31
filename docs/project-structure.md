# 项目目录结构

本文件定义 STDAS 当前 monorepo 的项目管理边界。架构能力域、服务职责和数据边界仍以 [系统架构](architecture-design/system-architecture.md) 与 [Rust Workspace 与服务边界](backend-design/workspace-and-crates.md) 为准。

## 顶层结构

```text
STDAS/
├── apps/
│   └── web/
├── crates/
│   └── services/
│       └── stdas-gateway/
├── docs/
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
| `apps/web` | React + TypeScript 前端工作台。包含 Vite、ESLint、TypeScript 配置和前端源码。 |
| `crates/services/stdas-gateway` | Loco Gateway 服务。包含 Loco `src/app.rs`、controllers、service-local `config/` 和 request tests。 |
| `crates/shared` | 后续共享基础能力 crate，例如 config、telemetry、auth、error、service-client。 |
| `crates/libs` | 后续稳定基础设施库，例如 event-bus、object-store、outbox、inbox、profile-contracts。 |
| `crates/tools` | 后续运维 CLI 和本地数据工具。 |
| `docs` | 项目事实来源。代码、目录和验证命令变更必须同步更新对应文档。 |
| `proto` | 后续 gRPC 和事件 proto 契约。 |
| `scripts` | 后续跨平台启动、停止、验证和维护脚本。 |
| `deploy` | 后续 Windows/Linux 部署资源。 |
| `.cargo` | workspace 级 Cargo alias。当前 `cargo loco ...` 映射到 `stdas-gateway`。 |
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
cargo loco start
cargo loco routes
cargo loco middleware
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
- Loco 服务的 `config/{development,test,production}.yaml` 放在服务目录内，避免多个服务共享同一个配置目录。
- 根目录只保留 workspace、文档、锁文件、repo-level scripts 和跨项目工具配置。
- 根目录 `Cargo.toml`、`Cargo.lock`、`package.json`、`pnpm-workspace.yaml`、`pnpm-lock.yaml` 是项目级管理文件，不得为了“目录更少”移动到子目录。
- 清理一级目录前，必须先用 Git 状态和 ignore 规则区分源码、锁文件、配置、本地生成物和外部参考材料。
- 新增服务优先放在 `crates/services/<service-name>`，新增前端应用优先放在 `apps/<app-name>`。
- 新增目录必须同步更新本文件、相关 section README 和验证命令。
