# ADR-0009：`stdas-gateway` 采用 Loco 作为 HTTP 应用框架

## 状态

Accepted

## 背景

Phase 0 preflight 最小后端最初直接使用 Axum 手写 `main`、router、tracing 和服务监听逻辑。该方式足够小，但后续由 AI 持续生成 Gateway controller、配置、任务和测试时，容易让启动、配置、middleware、路由发现和测试组织分散在多个临时位置。

STDAS 仍然要求 Gateway handler 只做协议适配，业务流程进入 service/usecase，且不得改变 `stdas-gateway` 作为唯一外部 HTTP API 入口的架构约束。

## 决策

`stdas-gateway` 采用 Loco 作为 HTTP 应用框架。Loco 基于 Axum，因此 Gateway 仍可使用 Axum extractor、response、middleware、router/layer 扩展点，但默认代码组织应优先采用：

- `src/app.rs` 中的 Loco `Hooks` 作为应用启动、路由注册、worker/task 注册入口。
- `src/controllers/*` 中的 controller `routes()` 和 handler。
- `config/{development,test,production}.yaml` 中的 server、logger 和 middleware 配置。
- Loco CLI 暴露的 `start`、`routes`、`middleware`、`doctor` 等命令。

## 影响

- Gateway 启动命令从直接运行 Axum server 改为 `cargo loco start`。
- 路由可通过 `cargo loco routes` 检查。
- 现有 `/api/v1/system/health` 和 `/api/v1/system/preflight` 契约保持不变。
- Loco 是 Gateway HTTP 层的组织约定，不改变 STDAS 微服务边界、数据边界、事件架构或前端只访问 Gateway 的规则。

## 约束

- 不把 Loco controller 写成业务服务；controller 只做协议适配。
- 不为迁移框架而提前引入数据库模型、认证、后台任务或 scaffold 生成的大量模板。
- 需要底层能力时可以使用原生 Axum，但应优先通过 Loco `Hooks`、controller routes 或明确的 Axum 扩展点接入。
- Loco 配置不得替代架构、API、权限或数据契约事实来源。

## 验证

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets -- -D warnings`
- `cargo loco routes`
