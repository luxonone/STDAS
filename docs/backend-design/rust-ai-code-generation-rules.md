# Rust AI 代码生成规则

本文是 AI 生成或修改 STDAS Rust 后端代码时必须执行的专项规则。它补充 [Rust 代码质量规则](rust-code-quality-rules.md)，并与 [Rust 高质量项目参考与模式](rust-reference-projects-and-patterns.md)、[AI 代码生成治理机制](../architecture-design/ai-code-generation-governance.md) 共同约束参考项目校准、偏航提醒、替代方案和待优化标记。

AI 生成或修改 Rust 后端代码前，还必须读取 [Rust Coding Guidelines SPEC](../specs/rust-coding-guidelines-spec.md)。该 SPEC 引用外部 Rust 编码规范中文站作为通用 coding baseline；STDAS 架构、API、错误、权限和数据边界仍以项目文档为准。

目标不是写“炫技 Rust”，而是写可维护、可审查、可验证的业务后端 Rust：所有权清楚、错误类型稳定、异步边界明确、服务分层符合 STDAS workspace 设计。

## 一手依据

生成 Rust 方案时优先参考：

- Rust Book：所有权、借用、错误处理、泛型、trait、生命周期、测试和并发基础。
- Rust Coding Guidelines 中文站：命名、格式、注释、数据类型、函数设计、async、Unsafe Rust 等通用 coding baseline。
- Rust API Guidelines：命名、类型转换、错误、trait、可预期性和未来兼容。
- rustfmt 和 Clippy：格式、lint 和常见错误模式。
- Tokio：异步 runtime、任务、同步原语、取消和背压。
- Axum：router、extractor、state、middleware、handler、response 和错误映射。
- SQLx：类型化 SQL、连接池、事务、迁移和查询边界。
- thiserror：稳定、可组合的错误枚举。
- tracing：结构化 span、字段和跨服务排障。
- tokio-rs、Meilisearch、Qdrant 等成熟 Rust 项目：按 [Rust 高质量项目参考与模式](rust-reference-projects-and-patterns.md) 做工程组织和可读性校准，不得覆盖 STDAS 架构规则。

## 编码前设计

写代码前必须先回答：

- **所有权**：数据由谁拥有，函数只读时是否使用 `&str`、`&[T]`、`&Path`、`&T`，是否存在为了哄编译器而 clone。
- **错误模型**：错误是否属于领域错误、基础设施错误、权限错误、契约错误或内部错误；是否能映射到 STDAS API error code。
- **异步边界**：哪些操作会 `.await`，是否持有锁、事务、借用或大对象跨 `.await`，是否需要取消、超时或背压。
- **分层边界**：handler 是否只做协议适配；service/usecase 是否持有业务流程和事务；repository 是否只做数据访问。
- **参考项目校准**：当前问题是否可从 Tokio、axum、SQLx、Meilisearch、Qdrant 等参考来源抽取模式；哪些模式不能照搬。
- **数据契约**：DTO、domain model、repository row model 是否分离；枚举、ID、状态、时间和单位是否有明确类型。
- **可观测性**：是否需要 `tracing` span 和字段，是否带 request/query/customer/job/test/data_version 等上下文。
- **测试策略**：单元测试、集成测试、契约测试或错误路径测试是否覆盖该变更风险。

这些问题回答不清楚时，必须先补设计或向用户说明风险，不能直接堆代码。

## 推荐模式

- Axum handler 接收 extractor、校验协议层输入、调用 command/query service、返回统一响应，不直接写 SQL 或承载业务流程。
- service/usecase 组织事务、幂等、权限、缓存、事件和领域规则。
- repository 只封装存储访问，返回明确 row/model/result，不泄漏 HTTP 或 UI 概念。
- 使用 `thiserror` 定义稳定错误枚举，通过项目错误映射转换为 API response。
- 对客户、工程、lot、wafer、test、query、job、data_version 等关键概念使用 newtype 或明确 domain type，避免裸 `String` 在多层流动。
- 对有限状态使用 enum；只有真正开放的扩展字段才使用 map 或 JSON value。
- 优先使用简单函数和结构体；只有当调用方边界稳定、重复真实存在时才引入 trait、generic 或宏。
- 异步任务必须有明确生命周期、取消策略、错误处理、日志字段和测试入口。
- SQL 使用类型化查询、显式字段、明确分页/排序/过滤；事务边界放在 usecase/service。
- 大查询、导出、摄入和后台任务必须遵守查询预算、Job 状态机、幂等和观测性规则。

## 需要提醒用户的偏航模式

遇到下列模式时，AI 必须先提醒风险并提出替代方案：

- 为了通过借用检查而大量 `.clone()`，但没有说明数据规模和所有权边界。
- 使用 `Arc<Mutex<_>>` 或 `RwLock` 处理普通业务共享状态，尤其是跨 `.await` 持锁。
- 在生产路径使用 `unwrap`、`expect`、`panic` 或丢弃错误上下文。
- 使用 `Box<dyn Error>`、`anyhow::Error`、`String` error 贯穿领域/API 边界。
- 在 Axum handler 中混合解析、权限、SQL、业务、缓存、事件和响应拼装。
- 用 `serde_json::Value`、`HashMap<String, Value>` 或字符串枚举替代明确 domain type。
- 为少量重复引入 trait/generic/macro，导致用户难以审查。
- 在 Tokio runtime 中执行阻塞 I/O、CPU 密集任务或无界并发。
- 新增 crate 只为少量可用标准库或既有依赖完成的逻辑。
- 从前端组件便利性反推后端 schema、API 或领域模型。
- 为赶进度跳过 API 契约、错误码、权限、数据版本或测试。

若用户不采纳替代方案，只能在不破坏正确性和安全性的范围内实现，并按项目治理规则添加 `TODO(AI-OPTIMIZE, area=rust, ...)`。

## 禁止兜底

以下问题不能用 `TODO(AI-OPTIMIZE, ...)` 延后：

- 权限、客户隔离、认证、脱敏或审计缺失。
- 数据写入可能损坏、重复、丢失或无法回滚。
- API 契约与前端/文档不兼容。
- 错误被吞掉，导致调用方无法判断失败。
- 后台任务不可取消、不可观测、不可恢复，且会影响核心数据。
- 测试或验证缺失导致无法证明关键路径正确。

## 输出要求

完成 Rust 变更后，最终回复必须说明：

- 关键设计选择，例如所有权、错误模型、异步边界、服务分层。
- 是否新增 `TODO(AI-OPTIMIZE, ...)`，以及为什么存在。
- 已执行的验证命令；若未执行，说明具体原因。
