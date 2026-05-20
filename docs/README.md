# STDAS 文档中心

STDAS 是面向半导体测试数据的 Rust 数据分析平台。`docs` 是项目事实来源，所有设计、代码和验证工作都必须先从这里建立上下文。

## Codex 必读入口

每次 Codex 或其他 AI Agent 在本项目中开始工作时，必须先读取：

1. [docs/README.md](README.md)：确认文档分区、当前阶段和任务阅读路径。
2. [AI Agent 运行时规则](architecture-design/ai-agent-runtime-rules.md)：确认修改前检查、事实来源和验证 gate。
3. 与本次任务相关的分区 README 和专题文档。

涉及代码生成、实现路径评审、偏航提醒或待优化标记时，还必须读取 [AI 代码生成治理机制](architecture-design/ai-code-generation-governance.md)。

如果任务描述和 `docs` 中的规则冲突，必须先指出冲突，按对应事实来源收敛后再继续。

## 当前状态

当前文档定为 V1 架构和设计基线。后续停止泛化扩写，优先进入首批功能切片规格、Phase 0 代码骨架和 Phase 0.5 环境验证闭环。

当前本机环境记录见 [Phase 0.5 环境验证记录](backend-design/phase-0-5-environment-validation.md)。Phase 0 / 0.5 基线为 Windows + Scoop 本机工具链，前端统一使用 pnpm；Redis 已安装并可按缓存策略启用；Windows 本地开发不安装、不使用 Docker。

## 文档分区

| 分区 | 稳定性 | 事实来源范围 |
|------|--------|--------------|
| [领域知识](domain-knowledge/README.md) | 高稳定 | STDAS 背景、OSAT 场景、测试数据语义、用户工作流和领域不变量。 |
| [架构设计](architecture-design/README.md) | 高稳定 | 产品愿景、领域模型、总体架构、服务边界、关键 ADR、功能切片基线。 |
| [前端设计](frontend-design/README.md) | 高频演进 | 工作台信息架构、页面、组件、交互、状态、可视化策略和前端代码规则。 |
| [后端设计](backend-design/README.md) | 高频演进 | Rust workspace、数据平台、摄入、分析、API、安全可靠性、运维和路线图。 |

## 推荐阅读路径

### 了解项目

1. [领域知识总览](domain-knowledge/stdas-domain-primer.md)
2. [产品愿景](architecture-design/product-vision.md)
3. [领域模型](architecture-design/domain-model.md)
4. [系统架构](architecture-design/system-architecture.md)

### 开始开发

1. [AI Agent 运行时规则](architecture-design/ai-agent-runtime-rules.md)
2. [AI 代码生成治理机制](architecture-design/ai-code-generation-governance.md)
3. [前后端同步设计](architecture-design/frontend-backend-sync-design.md)
4. [首批功能切片 V1](architecture-design/feature-slices-v1.md)
5. [Phase 0.5 环境验证记录](backend-design/phase-0-5-environment-validation.md)

### 前端任务

1. [前端设计 README](frontend-design/README.md)
2. [前端技术架构](frontend-design/frontend-tech-architecture.md)
3. [前端代码质量规则](frontend-design/frontend-code-quality-rules.md)
4. [前端 AI 代码生成规则](frontend-design/frontend-ai-code-generation-rules.md)
5. [页面层级设计](frontend-design/page-hierarchy-design.md)
6. [UI/UX 约束](frontend-design/ui-ux-constraints.md)

页面 mockup、视觉稿或图片生成任务还必须读取 [Mockup Prompt Workflow](frontend-design/mockup-prompt-workflow.md)。

### 后端任务

1. [后端设计 README](backend-design/README.md)
2. [Rust Workspace 与服务边界](backend-design/workspace-and-crates.md)
3. [Rust 代码质量规则](backend-design/rust-code-quality-rules.md)
4. [Rust AI 代码生成规则](backend-design/rust-ai-code-generation-rules.md)
5. [Rust 高质量项目参考与模式](backend-design/rust-reference-projects-and-patterns.md)
6. [API 契约原则](backend-design/api-principles.md)
7. [API 契约规则](backend-design/api-contract-rules.md)

涉及数据、查询、缓存、事件或运维时，再读取对应专题文档。

## 任务到文档映射

| 任务 | 必读文档 |
|------|----------|
| 领域术语、命名口径、FT/CP/BI/SLT 边界 | [领域知识总览](domain-knowledge/stdas-domain-primer.md)、[领域模型](architecture-design/domain-model.md) |
| 架构边界、服务拆分、长期约束 | [架构设计 README](architecture-design/README.md)、[系统架构](architecture-design/system-architecture.md)、[ADR](architecture-design/adr/README.md) |
| 多客户、DataProfile、客制化 | [OSAT 多客户客制化架构](architecture-design/osat-multi-customer-architecture.md)、[领域模型](architecture-design/domain-model.md) |
| 功能切片规划或验收 | [前后端同步设计](architecture-design/frontend-backend-sync-design.md)、[首批功能切片 V1](architecture-design/feature-slices-v1.md) |
| 前端页面、路由、状态、组件 | [前端设计 README](frontend-design/README.md)、[前端技术架构](frontend-design/frontend-tech-architecture.md)、[前端代码质量规则](frontend-design/frontend-code-quality-rules.md)、[前端 AI 代码生成规则](frontend-design/frontend-ai-code-generation-rules.md)、[页面层级设计](frontend-design/page-hierarchy-design.md)、[前端工作台设计](frontend-design/workbench-design.md) |
| UI/UX、响应式、表格、图表 | [UI/UX 约束](frontend-design/ui-ux-constraints.md)、[前端代码质量规则](frontend-design/frontend-code-quality-rules.md) |
| Rust 后端代码 | [后端设计 README](backend-design/README.md)、[Rust Workspace 与服务边界](backend-design/workspace-and-crates.md)、[Rust 代码质量规则](backend-design/rust-code-quality-rules.md)、[Rust AI 代码生成规则](backend-design/rust-ai-code-generation-rules.md)、[Rust 高质量项目参考与模式](backend-design/rust-reference-projects-and-patterns.md) |
| API 字段、枚举、错误、分页、兼容 | [API 契约原则](backend-design/api-principles.md)、[API 契约规则](backend-design/api-contract-rules.md) |
| 数据版本、QuerySnapshot、Evidence | [数据平台架构](backend-design/data-architecture.md)、[分析引擎](backend-design/analytics-engine.md) |
| 摄入、Parser、文件安全 | [摄入流水线](backend-design/ingestion-pipeline.md)、[安全与可靠性](backend-design/security-reliability.md) |
| 缓存、Redis、查询预算 | [缓存与 Redis 使用策略](backend-design/cache-strategy.md)、[分析引擎](backend-design/analytics-engine.md) |
| 事件、Outbox/Inbox、重放 | [事件契约规则](backend-design/event-contract-rules.md)、[系统架构](architecture-design/system-architecture.md) |
| 部署、配置、日志、指标、健康检查 | [部署与可观测性](backend-design/deployment-observability.md)、[Phase 0.5 环境验证记录](backend-design/phase-0-5-environment-validation.md) |

## 跨域规则

| 文档 | 说明 |
|------|------|
| [AI Agent 运行时规则](architecture-design/ai-agent-runtime-rules.md) | AI Agent 修改文档或代码前必须读取的项目规则。 |
| [AI 代码生成治理机制](architecture-design/ai-code-generation-governance.md) | AI 生成代码时的偏航提醒、替代方案、用户坚持原方案后的待优化标记机制。 |
| [前后端同步设计](architecture-design/frontend-backend-sync-design.md) | 前端和后端按功能切片同步设计、同步修改、同步验收。 |
| [首批功能切片 V1](architecture-design/feature-slices-v1.md) | 第一批端到端功能切片的页面、API、权限、数据语义和验收基线。 |

## 维护规则

- 架构设计只记录长期稳定的系统边界和原则，重大变化必须新增 ADR。
- 领域知识记录业务事实和行业语义，不承载具体实现方案。
- 前端设计可以随产品体验、页面结构和组件实现持续迭代。
- 后端设计可以随 Rust 实现、数据规模、查询策略和部署方式持续迭代。
- 前后端文档不得反向修改架构原则；需要改变架构时先提交 ADR。
- `docs/README.md` 只作为总入口和阅读地图，不承载详细设计。
- 新增或移动文档时，必须同步更新本入口、对应分区 README 和相关交叉引用。
- 进入代码实现前必须完成环境验证 gate，确认 build/test 和 AI Agent 修复闭环可跑。
- V1 baseline 之后新增规则必须服务于具体功能切片或代码实现，不再扩写泛化原则。
