# STDAS API Gateway

`backend/services/stdas-gateway` 是 STDAS 当前的 Rust + Axum backend service。Cargo package name 保持为 `stdas-gateway`，因为它是系统架构中的唯一外部 HTTP API 入口。

当前阶段 `stdas-gateway` 也是唯一 backend runtime service 和 modular monolith 容器；未来只有满足服务拆分触发条件时，内部 module 才升级为 crate 或 runtime service。

## 当前结构

```text
backend/services/stdas-gateway/
├── Cargo.toml
├── migrations/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── app.rs
│   ├── audit/
│   ├── config/
│   ├── db/
│   ├── errors/
│   ├── middleware/
│   ├── modules/
│   │   ├── identity/
│   │   ├── customer/
│   │   ├── data_pipeline/
│   │   ├── analytics/
│   │   ├── evidence/
│   │   ├── workflow/
│   │   └── integration/
│   ├── routes/
│   ├── shared/
│   ├── system/
│   ├── server.rs
│   ├── state.rs
│   └── telemetry/
└── tests/
```

## 分层规则

- `main.rs` 只处理进程入口和命令分发。
- `app.rs` 只组装 Axum `Router`、routes、middleware 和 `AppState`。
- `server.rs` 只负责监听地址、`TcpListener` 和 `axum::serve`。
- `routes/` 管 API 路由注册和 route catalog。
- `middleware/` 管 Tower/Axum middleware。
- `system/` 管 health、preflight 等运维端点，不属于业务服务边界。
- `modules/` 管未来可能升级为 crate 或 runtime service 的业务边界。
- `audit/` 管横切审计边界。
- `shared/` 只放稳定、低业务含义、跨模块确实共用的基础类型。
- `errors/` 管 typed error 和 API error mapping。
- `config/` 管应用配置结构；当前读取 `STDAS_GATEWAY_ADDR` 和 `STDAS_DATABASE_URL`。
- `db/` 管 PostgreSQL 连接池和 SQLx migration 入口。
- `state.rs` 管 Axum shared state。
- `telemetry/` 管 tracing、metrics、request id 等观测性边界。

## Module 边界

```text
modules/
├── identity/
├── customer/
├── data_pipeline/
├── analytics/
├── evidence/
├── workflow/
└── integration/
```

这些目录现在只表达边界，不代表已经实现对应业务。每个 module 出现真实功能后，再按需要创建 `routes.rs`、`handlers.rs`、`dto.rs`、`service.rs`、`repository.rs`、`models.rs` 或更细目录。

`customer` 不是普通客户 CRUD；它承载 CustomerConfig、DataProfile、ProfileResolutionKey、rule binding、feature flags 和 customer extension registry。

## 当前持久化边界

参考 Melrose《Rust + Axum 后端架构设计文档》时，只采用当前真实需要的结构。现在已经出现真实身份会话持久化，因此 `migrations/`、`src/db/` 和 SQLx dependency 已经启用。

当前只落地 STDAS 测试部门内部需要的最小身份模型：

- `c_users`：用户账号主表，字段名参考 MES `c_users` 语义。
- `c_roles`：角色主表，只保留本系统需要的最小角色信息。
- `c_user_rl`：用户与角色关联。
- `r_user_session`：当前 access session，只保存 token hash。

以下目录仍等真实功能出现后再创建：

- `config/*.toml`
- `src/cache/`
- `src/extractors/`
- `src/tasks/`
- fake handler / fake repository / fake service

这样做是为了避免空目录和 unused dependency 让项目看起来“完整”，但实际降低可维护性。

## 常用命令

从仓库根目录执行：

```bash
cargo gateway
cargo gateway-routes
cargo gateway-seed-dev-admin
cargo run -p stdas-gateway
cargo run -p stdas-gateway -- routes
cargo run -p stdas-gateway -- seed-dev-admin
```

## 本地启动依赖

`stdas-gateway` 当前依赖本机 PostgreSQL。默认配置来自环境变量，未设置时使用：

| 配置 | 默认值 |
|------|--------|
| `STDAS_GATEWAY_ADDR` | `127.0.0.1:8080` |
| `STDAS_DATABASE_URL` | `postgres://stdas:stdas@localhost:5432/stdas` |

Windows + Scoop 本地启动 PostgreSQL：

```powershell
New-Item -ItemType Directory -Force tmp | Out-Null
pg_ctl start `
  -D C:\Users\UW00133\scoop\persist\postgresql\data `
  -l D:\Code\Project\temp\STDAS\tmp\postgresql.log
pg_isready -h localhost -p 5432
```

若 `stdas` role 或 database 尚不存在，先用 PostgreSQL 管理账号创建：

```powershell
psql -h localhost -p 5432 -U postgres -d postgres -c "CREATE ROLE stdas LOGIN PASSWORD 'stdas';"
createdb -h localhost -p 5432 -U postgres -O stdas stdas
```

启动 Gateway 前必须至少执行一次管理员初始化。该命令会先执行 SQLx migrations，再 upsert `c_users`、`c_roles` 和 `c_user_rl`：

```powershell
cargo gateway-seed-dev-admin
```

`seed-dev-admin` 用于创建或更新本地/部署数据库中的初始管理员。默认本地开发流程是交互式输入密码；服务端只把 Argon2id hash 写入 `c_users.passwd`，不会把明文密码写入代码、migration、日志或本地文件：

```powershell
cargo gateway-seed-dev-admin
```

自动化或 CI 场景仍可用一次性环境变量注入密码，但不要把该变量写入提交文件：

```powershell
$env:STDAS_BOOTSTRAP_ADMIN_PASSWORD = "<password>"
cargo gateway-seed-dev-admin
```

可选环境变量：`STDAS_BOOTSTRAP_ADMIN_USERNAME`、`STDAS_BOOTSTRAP_ADMIN_DISPLAY_NAME`、`STDAS_BOOTSTRAP_ADMIN_PERSON_CODE`、`STDAS_BOOTSTRAP_ADMIN_SITE_ID`。
