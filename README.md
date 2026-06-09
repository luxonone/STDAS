# STDAS Phase 0 / Data Explorer Slice

当前仓库包含 STDAS 技术基线和首轮登录 + Data Explorer / Lot List 评审切片。该切片已经接入 PostgreSQL-backed 最小身份会话模型，并提供登录后 Data Explorer / Lot List 页面和样例查询 API；真实测试数据摄入、真实解析、分析工作台保存、Lot Detail 和完整权限矩阵仍等待后续功能切片。

## 技术栈

- Backend：`backend/services/stdas-gateway`，Rust workspace，`stdas-gateway` 直接使用 Axum 和 Tokio。
- Frontend：`frontend/web`，Vite、React、TypeScript、pnpm workspace。
- Database：本机 PostgreSQL，Gateway 默认连接 `postgres://stdas:stdas@localhost:5432/stdas`。
- API prefix：`/api/v1`。
- Health endpoint：`GET /api/v1/system/health`。
- Auth endpoints：`POST /api/v1/auth/login`、`GET /api/v1/auth/me`。
- Data Explorer endpoint：`GET /api/v1/data/lots`。

## 目录结构

```text
STDAS/
├── backend/
│   └── services/
│       └── stdas-gateway/           # Rust + Axum API gateway service
├── frontend/
│   └── web/                         # React + TypeScript workbench
├── docs/                            # project source of truth
├── .cargo/config.toml               # workspace cargo aliases
├── Cargo.toml                       # Rust workspace
├── package.json                     # repo-level pnpm scripts
└── pnpm-workspace.yaml              # frontend workspace membership
```

根目录 `Cargo.toml` / `Cargo.lock` 和 `package.json` / `pnpm-lock.yaml` / `pnpm-workspace.yaml` 是项目级 workspace 管理文件，应保留在仓库根目录。`node_modules/`、`target/` 和 `frontend/web/dist/` 是本地生成目录，已被 Git ignore；根目录 `dist/` 不再是当前结构的有效构建产物位置。更完整的目录规则见 [docs/project-structure.md](docs/project-structure.md)。

## 项目历史

项目可见变更和本地 commit 编号约定见 [CHANGELOG.md](CHANGELOG.md)。该约定用于区分 code/configuration commit 与 documentation-only commit，避免回档时混淆代码和文档。

## Windows 本地启动

安装 Frontend dependencies：

```powershell
pnpm install
```

当前本地开发需要启动三类进程：

| 启动项 | 默认地址 | 当前用途 |
|--------|----------|----------|
| PostgreSQL | `localhost:5432` | `c_users`、`c_roles`、`c_user_rl`、`r_user_session` 和 SQLx migrations |
| `stdas-gateway` | `http://127.0.0.1:8080` | `/api/v1` 后端 API、migration、auth、Data Explorer 样例查询 |
| Frontend Web | `http://127.0.0.1:5173` | React 工作台和登录页 |

启动 PostgreSQL：

```powershell
New-Item -ItemType Directory -Force tmp | Out-Null
pg_ctl start `
  -D C:\Users\UW00133\scoop\persist\postgresql\data `
  -l D:\Code\Project\temp\STDAS\tmp\postgresql.log
pg_isready -h localhost -p 5432
```

首次使用当前默认连接串时，确认本机 PostgreSQL 已存在 `stdas` role 和 `stdas` database。若不存在，用 PostgreSQL 管理账号执行：

```powershell
psql -h localhost -p 5432 -U postgres -d postgres -c "CREATE ROLE stdas LOGIN PASSWORD 'stdas';"
createdb -h localhost -p 5432 -U postgres -O stdas stdas
```

初始化或更新本地管理员。默认会交互式隐藏输入密码，只把 Argon2id hash 写入数据库：

```powershell
cargo gateway-seed-dev-admin
```

需要非交互自动化时，可以只在当前 PowerShell 进程中临时设置密码变量，用完立即移除，不能写入提交文件：

```powershell
$env:STDAS_BOOTSTRAP_ADMIN_PASSWORD = "<password>"
cargo gateway-seed-dev-admin
Remove-Item Env:\STDAS_BOOTSTRAP_ADMIN_PASSWORD
```

运行 Gateway：

```powershell
cargo gateway
```

查看 Gateway 路由：

```powershell
cargo gateway-routes
```

运行 Frontend：

```powershell
pnpm dev
```

打开 `http://127.0.0.1:5173`。本地验证账号取决于 `seed-dev-admin` 输入的密码；文档不得记录真实本机密码。

当前切片不需要启动 Redis、NATS、MinIO。它们已作为未来能力预留，只有缓存、事件、对象存储或摄入切片真实接入后才进入必需启动项。

## 验证

```powershell
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
pnpm lint
pnpm typecheck
pnpm test
pnpm build
```
