# 后端设计

> **定位**：STDAS Rust 后端设计。后端设计会随实现、性能验证、数据规模和部署方式持续调整，因此允许高频迭代。
> **稳定性**：高频演进 — 后端实现细节可以频繁更新，但不得破坏架构设计中定义的能力域和数据边界。
> **事实来源范围**：Rust workspace、数据平台、API 契约、缓存策略、摄入流水线、分析引擎、安全可靠性、部署与可观测性。

当前 Phase 0 / 0.5 本机环境基线记录在 [phase-0-5-environment-validation.md](phase-0-5-environment-validation.md)：Windows + Scoop 原生工具链，`stdas-gateway` HTTP 应用层直接采用 Axum，持久化层采用 SQLx + PostgreSQL，不采用 ORM；Windows 本地开发不安装、不使用 Docker。

当前后端 runtime 采用 [ADR-0014](../architecture-design/adr/0014-gateway-modular-monolith.md)：`stdas-gateway` 是唯一 backend runtime service，内部按未来服务边界组织 `modules/`。Redis、NATS、MinIO、gRPC、Outbox/Inbox 只在真实功能和拆分触发条件出现后引入。

## 相关 SPEC

| SPEC | 说明 |
|------|------|
| [Rust Coding Guidelines SPEC](../specs/rust-coding-guidelines-spec.md) | STDAS 采用 Rust 编码规范中文站作为 Rust coding baseline 的铁律 |

## 文档索引

| 文档 | 说明 |
|------|------|
| [workspace-and-crates.md](workspace-and-crates.md) | Rust workspace、crate 边界、依赖方向、技术选型 |
| [rust-code-quality-rules.md](rust-code-quality-rules.md) | Rust 代码生成、所有权、错误处理、异步、测试和代码修改约束 |
| [rust-ai-code-generation-rules.md](rust-ai-code-generation-rules.md) | AI 生成或修改 Rust 代码时的设计检查、偏航提醒和待优化标记规则 |
| [rust-reference-projects-and-patterns.md](rust-reference-projects-and-patterns.md) | 高质量 Rust 项目参考、可借鉴模式和反照搬规则 |
| [data-architecture.md](data-architecture.md) | 数据分层、PostgreSQL 定位、数据版本、参数数据演进 |
| [cache-strategy.md](cache-strategy.md) | 缓存、Redis 使用策略、缓存接口、key 和失效规则 |
| [ingestion-pipeline.md](ingestion-pipeline.md) | 摄入流水线、Parser 边界、幂等、文件安全 |
| [analytics-engine.md](analytics-engine.md) | 分析查询类型、查询预算、缓存、OLAP backend |
| [api-principles.md](api-principles.md) | API 契约、响应格式、端点组织、错误码 |
| [api-contract-rules.md](api-contract-rules.md) | 字段取值范围、默认值、编码约束、版本兼容和契约验收 |
| [event-contract-rules.md](event-contract-rules.md) | NATS/Outbox 事件信封、事件清单、幂等和重放规则 |
| [security-reliability.md](security-reliability.md) | 认证、授权、数据保护、Job 状态机、降级 |
| [deployment-observability.md](deployment-observability.md) | 部署、配置、日志、指标、健康检查、运维工具 |
| [implementation-roadmap.md](implementation-roadmap.md) | 后端实施路线图 |
| [phase-0-5-environment-validation.md](phase-0-5-environment-validation.md) | 当前本机 Phase 0.5 环境验证结果 |
| [loco-error-handling-investigation.md](loco-error-handling-investigation.md) | Loco 默认错误处理开发体验调查日志；当前仅作历史记录，活跃 HTTP 基线以 Axum ADR 为准 |

## 任务必读

| 任务类型 | 必读文档 |
|----------|----------|
| Rust 代码生成或修改 | [Rust Coding Guidelines SPEC](../specs/rust-coding-guidelines-spec.md) + [rust-code-quality-rules.md](rust-code-quality-rules.md) + [rust-ai-code-generation-rules.md](rust-ai-code-generation-rules.md)；非平凡实现还需 [rust-reference-projects-and-patterns.md](rust-reference-projects-and-patterns.md) |
| API 契约、字段、枚举、错误、分页 | [api-principles.md](api-principles.md) + [api-contract-rules.md](api-contract-rules.md) |
| 数据版本、QuerySnapshot、Evidence | [data-architecture.md](data-architecture.md) + [analytics-engine.md](analytics-engine.md) |
| 摄入、Parser、文件安全 | [ingestion-pipeline.md](ingestion-pipeline.md) |
| 缓存、Redis、查询预算 | [cache-strategy.md](cache-strategy.md) + [analytics-engine.md](analytics-engine.md) |
| 事件、Outbox/Inbox、重放 | [event-contract-rules.md](event-contract-rules.md) |
| 权限、脱敏、任务状态机、可靠性 | [security-reliability.md](security-reliability.md) |
| 部署、配置、日志、指标、健康检查 | [deployment-observability.md](deployment-observability.md) |
| Workspace、crate 边界、技术选型 | [workspace-and-crates.md](workspace-and-crates.md) |

## 变更原则

- 后端实现细节可以频繁更新。
- Rust 代码生成和修改必须遵守 [Rust Coding Guidelines SPEC](../specs/rust-coding-guidelines-spec.md) 和 [Rust 代码质量规则](rust-code-quality-rules.md)；AI 生成或修改 Rust 代码还必须遵守 [Rust AI 代码生成规则](rust-ai-code-generation-rules.md)，非平凡后端实现还必须按 [Rust 高质量项目参考与模式](rust-reference-projects-and-patterns.md) 做参考项目校准。
- 查询策略、缓存策略、作业策略应基于验证结果迭代。
- 不得破坏架构设计中定义的能力域、数据范围和长期演进方向。
- 重大方向变化需要先更新架构 ADR。
- API 分组和契约必须对齐前端功能切片和用户任务，但不能机械照搬前端组件或 AI 视觉稿。
- 后端契约必须支撑前端 [UI/UX 通用护栏](../frontend-design/ui-ux-constraints.md) 中的 P0 要求，包括 query snapshot、DataVersion 冻结、Options API、权限脱敏、任务生命周期和 evidence 版本。
- 后端 API 调整必须同步更新前端设计和 [前后端同步设计](../architecture-design/frontend-backend-sync-design.md)。
