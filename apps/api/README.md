# STDAS API Gateway

`apps/api` 是 STDAS 当前的 Rust + Axum backend application。Cargo package name 保持为 `stdas-gateway`，因为它是系统架构中的唯一外部 HTTP API 入口。

## 当前结构

```text
apps/api/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── app.rs
│   ├── config/
│   ├── dto/
│   ├── errors/
│   ├── handlers/
│   ├── middleware/
│   ├── models/
│   ├── repositories/
│   ├── routes/
│   ├── services/
│   ├── server.rs
│   ├── state.rs
│   ├── telemetry.rs
│   └── utils/
└── tests/
```

## 分层规则

- `main.rs` 只处理进程入口和命令分发。
- `app.rs` 只组装 Axum `Router`、routes、middleware 和 `AppState`。
- `server.rs` 只负责监听地址、`TcpListener` 和 `axum::serve`。
- `routes/` 管 API 路由注册和 route catalog。
- `middleware/` 管 Tower/Axum middleware。
- `handlers/` 只做 extractor、DTO mapping、调用 service 和 response mapping。
- `services/` 管业务 use case。
- `repositories/` 管数据访问边界；未来 SQLx 查询放这里或明确的 data access 子模块。
- `dto/` 放外部 API request/response 数据传输对象。
- `models/` 放服务内部模型，不混入 HTTP envelope。
- `errors/` 管 typed error 和 API error mapping。
- `config/` 管应用配置结构；当前只读取环境变量 `STDAS_GATEWAY_ADDR`。
- `state.rs` 管 Axum shared state。
- `telemetry.rs` 管 tracing、metrics、request id 等观测性边界。

## 暂不创建的目录

参考 Melrose《Rust + Axum 后端架构设计文档》时，只采用当前真实需要的结构。以下目录等真实功能出现后再创建：

- `config/*.toml`
- `migrations/`
- `src/db/`
- `src/cache/`
- `src/extractors/`
- `src/tasks/`

这样做是为了避免空目录和 unused dependency 让项目看起来“完整”，但实际降低可维护性。

## 常用命令

从仓库根目录执行：

```bash
cargo gateway
cargo gateway-routes
cargo run -p stdas-gateway
cargo run -p stdas-gateway -- routes
```
