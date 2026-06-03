# STDAS API Gateway

`backend/services/stdas-gateway` 是 STDAS 当前的 Rust + Axum backend service。Cargo package name 保持为 `stdas-gateway`，因为它是系统架构中的唯一外部 HTTP API 入口。

当前阶段 `stdas-gateway` 也是唯一 backend runtime service 和 modular monolith 容器；未来只有满足服务拆分触发条件时，内部 module 才升级为 crate 或 runtime service。

## 当前结构

```text
backend/services/stdas-gateway/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── app.rs
│   ├── audit/
│   ├── config/
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
- `config/` 管应用配置结构；当前只读取环境变量 `STDAS_GATEWAY_ADDR`。
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

## 暂不创建的目录

参考 Melrose《Rust + Axum 后端架构设计文档》时，只采用当前真实需要的结构。以下目录等真实功能出现后再创建：

- `config/*.toml`
- `migrations/`
- `src/db/`
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
cargo run -p stdas-gateway
cargo run -p stdas-gateway -- routes
```
