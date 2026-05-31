# Rust 高质量项目参考与模式

本文是 AI 生成或评审 STDAS Rust 后端方案时的参考项目经验库。它不替代 STDAS 架构、API 契约或已有代码，而是从官方资料、官方示例和成熟 Rust 项目中抽取可检查的工程模式，帮助避免“能编译但不好维护”的 Rust 实现。

## 使用边界

- STDAS 项目文档、现有源码、API 契约和 ADR 优先；Rust 官方和框架一手资料其次；本文用于参考项目校准。
- 涉及非平凡 Rust 后端实现时必须使用本文，包括 API handler、service/usecase、repository、异步任务、查询、缓存、摄入、搜索、后台 job、错误和观测性。
- 如果参考项目模式和 STDAS 文档冲突，必须先指出冲突，并以 STDAS 文档为准。
- 参考项目用于回答“可借鉴什么模式、不能照搬什么复杂度”，不得为了贴近外部项目而改变 STDAS 服务边界。

## 参考来源与抽取规则

| 来源 | 参考价值 | STDAS 抽取规则 |
|------|----------|----------------|
| [Rust Book](https://doc.rust-lang.org/book/) | Rust 所有权、借用、错误、泛型、trait、生命周期、测试和并发基础 | 用作语言正确性和可读性底线；优先让所有权边界自然，而不是用 clone、生命周期技巧或全局状态绕过设计问题 |
| [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html) | 命名、类型转换、错误、trait、可预期性和未来兼容 | 公开 crate/API、domain type、错误类型和 builder/options 设计应能通过这些 checklist 解释 |
| [rustfmt](https://github.com/rust-lang/rustfmt) / [Clippy](https://doc.rust-lang.org/clippy/) | 格式和 lint 基线 | 格式交给工具；lint 警告优先推动代码结构调整，而不是局部 suppress |
| [Tokio tutorial](https://tokio.rs/tokio/tutorial) / [mini-redis](https://github.com/tokio-rs/mini-redis) | async runtime、connection、command、task、channel 和共享状态示例 | 异步任务要有所有权、取消、背压和错误路径；避免无界 task、跨 `.await` 持锁和隐藏阻塞 I/O |
| [Loco docs](https://loco.rs/docs/) / [loco-rs crate](https://docs.rs/loco-rs/latest/loco_rs/) | app hooks、controller、routes、配置、CLI、测试和基于 Axum 的扩展点 | `stdas-gateway` 优先按 Loco 的 app/controller/routes/config 约定组织；业务流程仍放入 service/usecase，不能把 controller 写成业务容器 |
| [axum docs](https://docs.rs/axum/latest/axum/) / [axum examples](https://github.com/tokio-rs/axum/tree/main/examples) | extractor、state、handler、middleware、response 和错误映射 | Loco 未覆盖或需要底层扩展时使用 Axum 模式；handler 只做协议适配，业务流程放入 service/usecase，错误通过统一映射返回 API response |
| [SQLx docs](https://docs.rs/sqlx/latest/sqlx/) / [SQLx repo](https://github.com/launchbadge/sqlx) | 类型化 SQL、连接池、事务、迁移和数据库边界 | repository 拥有 SQL；显式字段、分页、排序、事务和连接池；不得从 handler 直接拼 SQL |
| [thiserror](https://docs.rs/thiserror/latest/thiserror/) | 稳定、可组合的错误枚举 | 领域、基础设施、契约、权限和内部错误应有明确 enum；跨 API 边界不得用临时字符串错误 |
| [tracing](https://docs.rs/tracing/latest/tracing/) | 结构化 span、字段和异步上下文观测 | 后台 job、查询、摄入和 API 入口应带 request、customer、job、query、data_version 等可排障字段 |
| [Meilisearch](https://github.com/meilisearch/meilisearch) | 生产级 Rust 搜索服务、索引、任务和模块组织 | 借鉴服务/索引/任务边界、可观测性和可测试性；不得照搬其搜索引擎复杂度 |
| [Qdrant](https://github.com/qdrant/qdrant) | 生产级 Rust 向量数据库、存储、查询和后台任务组织 | 借鉴长任务、查询预算、存储边界和 backpressure 思路；不得把 STDAS 变成通用向量数据库架构 |

## STDAS 抽取模式

1. **HTTP controller/handler 是协议适配器**：接收 extractor、校验协议输入、调用 service/usecase、返回统一响应；不直接承载 SQL、权限、缓存、事件和业务流程。
2. **service/usecase 拥有业务流程**：事务、幂等、权限、缓存、事件、任务状态机和领域规则集中在 service/usecase 层组织。
3. **repository 拥有持久化细节**：SQL、row model、连接池、事务参与和数据库错误转换只在 repository 或 data access 层暴露。
4. **domain type 优先于裸字符串**：customer、engineering_lot、wafer、test、query、job、data_version、unit、status 等关键概念用 newtype、enum 或明确结构体表达。
5. **错误必须类型化并可映射**：领域错误、权限错误、契约错误、基础设施错误和内部错误应能稳定映射到 API error code 与日志字段。
6. **异步任务必须可治理**：每个 task 都要说明生命周期、取消、超时、并发上限、错误处理、tracing 字段和测试入口。
7. **查询型功能必须有预算**：大查询、导出、分析、摄入和后台 job 要有分页、过滤、snapshot、data_version、缓存、限流或异步化策略。
8. **测试按边界分层**：纯领域逻辑用单元测试；repository/API/任务状态机用集成或契约测试；错误路径和边界条件必须覆盖。

## 必须提醒用户的偏航

AI 遇到下列实现倾向时，必须在编码前提醒风险并提出替代方案：

- fat handler：Loco/Axum handler 同时做解析、权限、SQL、业务、缓存、事件和响应拼装。
- SQL 泄漏：从 handler 或前端便利性反推 SQL、schema 或后端 domain model。
- JSON 泄漏：用 `serde_json::Value`、`HashMap<String, Value>` 或字符串枚举替代稳定 domain type。
- async 共享状态滥用：用 `Arc<Mutex<_>>`、`RwLock` 或全局缓存解决普通业务所有权问题，尤其跨 `.await` 持锁。
- 无边界后台任务：`tokio::spawn` 后没有取消、JoinHandle、超时、错误上报、tracing 或并发上限。
- 无查询预算：大查询、导出、分析或摄入直接同步执行，没有分页、snapshot、job 化或缓存策略。
- 字符串错误：用 `String`、`Box<dyn Error>` 或临时 `anyhow::Error` 穿过领域/API 边界。
- 复制大型项目复杂度：把 Meilisearch/Qdrant 的搜索、索引、分片、调度或泛型架构直接搬入 STDAS。

## 反照搬规则

- 不因为参考项目使用复杂 trait、宏、泛型、runtime wrapper 或插件机制，就在 STDAS 中引入同等复杂度。
- 不在没有明确数据规模和功能切片需求时引入通用搜索引擎、向量索引、任务调度平台或自定义 actor 系统。
- 不为了减少短期代码量牺牲 API 契约、权限隔离、数据版本、错误类型和可验证性。
- 不为了“Rust 风格”写难以审查的生命周期技巧；优先清晰的所有权、简单结构体和明确函数边界。

## 编码前检查清单

写非平凡 Rust 后端代码前，必须回答：

- 当前问题最接近哪个参考来源：Loco app/controller、Tokio async、axum API、SQLx persistence、thiserror/tracing、Meilisearch 搜索/任务，还是 Qdrant 查询/存储/任务？
- 参考来源中可借鉴的具体模式是什么？
- 哪些模式因为 STDAS 规模、架构或当前切片不需要，必须明确不照搬？
- handler、service/usecase、repository、domain type、error、async task 和 test 边界是否已经分清？
- 如果用户坚持偏离最佳实践，是否仍满足安全、正确性、API 兼容和验证要求；是否需要 `TODO(AI-OPTIMIZE, area=rust, ...)`？

## 评审检查清单

评审 AI 生成的 Rust 后端代码时，必须确认：

- handler 足够薄，没有直接写 SQL 或混入业务流程。
- transaction、idempotency、permission、cache、event 和 job lifecycle 放在合适 service/usecase。
- repository 返回明确类型，不泄漏 HTTP/UI 概念。
- 错误是稳定 enum 或项目错误类型，能映射到 API error code。
- async task 有取消、背压、超时、错误上报和 tracing 字段。
- 大查询、导出、摄入和后台 job 有查询预算或异步 job 策略。
- 测试覆盖成功路径、错误路径和边界条件。

## 维护规则

- 新增后端依赖、框架、异步模型、任务系统、查询引擎或搜索/索引能力时，必须更新本文的参考来源和抽取规则。
- 如果外部参考项目升级导致模式发生变化，先用一手资料复核，再修改本文。
- 本文只记录可执行的参考项目模式和反照搬规则，不记录泛泛的“最佳实践”口号。
