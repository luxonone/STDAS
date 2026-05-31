# ADR-0010：采用清晰的 `apps/` + `crates/` monorepo 目录结构

## 状态

Accepted

## 背景

Phase 0 preflight 期间，前端 Vite 文件、React 源码、Rust workspace、Loco 配置和全局文档混在仓库根目录。该结构在最小验证阶段可以接受，但随着 STDAS 进入多服务、多前端工作区和 AI 代码生成阶段，根目录混放会降低可读性，并增加工具命令、配置文件和文档事实来源互相覆盖的风险。

STDAS 后端已经按 Rust workspace 和粗粒度服务边界设计；前端也已经按工作台应用与 Feature-Sliced 结构设计。因此仓库布局应让前端应用、后端服务、共享库、文档和工具边界在文件系统上直接可见。

## 决策

STDAS 采用以下 monorepo 管理结构：

```text
STDAS/
├── apps/
│   └── web/
├── crates/
│   ├── services/
│   ├── shared/
│   ├── libs/
│   └── tools/
├── docs/
├── proto/
├── scripts/
└── deploy/
```

当前实现阶段：

- React + TypeScript 前端移动到 `apps/web`。
- `stdas-gateway` 保持在 `crates/services/stdas-gateway`。
- Loco Gateway 的 `config/` 保持在服务目录内。
- 仓库根目录保留 `Cargo.toml`、`package.json`、`pnpm-workspace.yaml`、lockfile、文档和跨项目工具配置。

## 影响

- 前端根路径从仓库根目录变为 `apps/web`。
- 根目录 `package.json` 作为 repo-level command surface，代理到 `apps/web`。
- 根目录 `.cargo/config.toml` 提供 `cargo loco ...` alias，映射到 `stdas-gateway`。
- 新增应用或服务时按 `apps/<app>`、`crates/services/<service>` 扩展，而不是继续在根目录堆叠配置。

## 约束

- 根目录不承载前端源码、Vite 配置或服务私有配置。
- 服务私有配置不得放在全局根 `config/`，除非该配置明确服务多个进程。
- 新增顶层目录必须更新 [项目目录结构](../../project-structure.md)。
- 目录调整不得改变 `stdas-gateway` 作为唯一外部 HTTP API 入口的架构约束。

## 验证

- `cargo loco routes`
- `cargo test`
- `pnpm lint`
- `pnpm typecheck`
- `pnpm test`
- `pnpm build`
