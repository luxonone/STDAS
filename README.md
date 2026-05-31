# STDAS Phase 0 Preflight

当前仓库只包含 STDAS 技术基线的最小验证切片。该切片仅用于前置验证，尚未实现半导体测试数据业务工作流、Domain Model、ingestion、cache、authentication 或正式功能行为。

## 技术栈

- Backend：Rust workspace，`stdas-gateway` 使用 Loco（基于 Axum）和 Tokio。
- Frontend：`apps/web`，Vite、React、TypeScript、pnpm workspace。
- API prefix：`/api/v1`。
- Health endpoint：`GET /api/v1/system/health`。

## 目录结构

```text
STDAS/
├── apps/
│   └── web/                         # React + TypeScript workbench
├── crates/
│   └── services/
│       └── stdas-gateway/           # Loco gateway service
├── docs/                            # project source of truth
├── .cargo/config.toml               # workspace cargo aliases
├── Cargo.toml                       # Rust workspace
├── package.json                     # repo-level pnpm scripts
└── pnpm-workspace.yaml              # frontend workspace membership
```

根目录 `Cargo.toml` / `Cargo.lock` 和 `package.json` / `pnpm-lock.yaml` / `pnpm-workspace.yaml` 是项目级 workspace 管理文件，应保留在仓库根目录。`node_modules/`、`target/` 和 `apps/web/dist/` 是本地生成目录，已被 Git ignore；根目录 `dist/` 不再是当前结构的有效构建产物位置。更完整的目录规则见 [docs/project-structure.md](docs/project-structure.md)。

## 项目历史

项目可见变更和本地 commit 编号约定见 [CHANGELOG.md](CHANGELOG.md)。该约定用于区分 code/configuration commit 与 documentation-only commit，避免回档时混淆代码和文档。

## 运行

安装 Frontend dependencies：

```bash
pnpm install
```

运行 Gateway：

```bash
cargo loco start
```

查看 Gateway 路由：

```bash
cargo loco routes
```

运行 Frontend：

```bash
pnpm dev
```

打开 `http://127.0.0.1:5173`。

## 验证

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
pnpm lint
pnpm typecheck
pnpm test
pnpm build
```
