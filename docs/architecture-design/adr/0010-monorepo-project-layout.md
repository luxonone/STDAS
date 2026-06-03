# ADR-0010：采用清晰的 `apps/` + `crates/` monorepo 目录结构

## 状态

Superseded

本 ADR 已被 [ADR-0013](0013-backend-frontend-physical-partition.md) 取代。历史上的 monorepo 分区意图仍有参考价值，但当前 STDAS 目录事实来源以 ADR-0013 和 [项目目录结构](../../project-structure.md) 为准。

## 背景

Phase 0 preflight 期间，前端 Vite 文件、React 源码、Rust workspace、服务配置和全局文档混在仓库根目录。该结构在最小验证阶段可以接受，但随着 STDAS 进入多服务、多前端工作区和 AI 代码生成阶段，根目录混放会降低可读性，并增加工具命令、配置文件和文档事实来源互相覆盖的风险。

STDAS 后端已经按 Rust workspace 和粗粒度服务边界设计；前端也已经按工作台应用与 Feature-Sliced 结构设计。因此仓库布局应让前端应用、后端服务、共享库、文档和工具边界在文件系统上直接可见。

## 决策

以下内容是本 ADR 被取代前的历史决策记录，不代表当前目录事实来源。当前目录结构以 ADR-0013、ADR-0014 和 [项目目录结构](../../project-structure.md) 为准。

STDAS 采用以下 monorepo 管理结构：

```text
STDAS/
├── apps/
│   ├── api/
│   └── web/
├── crates/
│   ├── shared/
│   ├── libs/
│   ├── tools/
│   └── services/
├── docs/
├── proto/
├── scripts/
└── deploy/
```

当前实现阶段：

- React + TypeScript 前端移动到 `apps/web`。
- `stdas-gateway` 作为当前唯一 backend API application，放在 `apps/api`。
- `crates/` 只在出现真实共享库、基础设施库、工具 crate 或内部服务 crate 时创建；不提交空目录。
- Gateway 服务私有配置只在服务真正需要配置文件时放入 `apps/api/config/`；当前最小 Axum Gateway 使用环境变量配置监听地址。
- 仓库根目录保留 `Cargo.toml`、`package.json`、`pnpm-workspace.yaml`、lockfile、文档和跨项目工具配置。

## 影响

以下影响描述的是历史 `apps/` + `crates/` 阶段，不代表当前 `backend/` + `frontend/` 阶段。

- 前端根路径从仓库根目录变为 `apps/web`。
- 根目录 `package.json` 作为 repo-level command surface，代理到 `apps/web`。
- 根目录 `.cargo/config.toml` 提供 `cargo gateway` 和 `cargo gateway-routes` alias，映射到 `stdas-gateway`。
- 新增对外应用时按 `apps/<app>` 扩展，新增内部服务时按 `crates/services/<service>` 扩展，而不是继续在根目录堆叠配置。

## 约束

以下约束描述的是历史阶段的约束。当前约束见 ADR-0013、ADR-0014 和 [项目目录结构](../../project-structure.md)。

- 根目录不承载前端源码、Vite 配置或服务私有配置。
- 服务私有配置不得放在全局根 `config/`，除非该配置明确服务多个进程。
- `apps/api` 内部结构必须继续按 Axum API 分层组织，具体见 ADR-0012。
- 新增顶层目录必须更新 [项目目录结构](../../project-structure.md)。
- 目录调整不得改变 `stdas-gateway` 作为唯一外部 HTTP API 入口的架构约束。

## 验证

- `cargo gateway-routes`
- `cargo run -p stdas-gateway -- routes`
- `cargo test`
- `pnpm lint`
- `pnpm typecheck`
- `pnpm test`
- `pnpm build`
