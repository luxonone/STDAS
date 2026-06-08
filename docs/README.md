# STDAS 文档中心

> **定位**：`docs` 是项目事实来源。所有设计、代码和验证工作都必须先从这里建立上下文。
> **稳定性分级**：见下方文档分区表，不同分区有不同的稳定性等级和变更节奏。
> **标准结构**：每个分区 README 都遵循 **定位 → 文档索引 → 任务必读 → 变更原则**。

`specs/` 是 SPEC 专区。SPEC 是项目铁律，优先级高于普通设计文档；普通文档负责背景、设计解释、索引和辅助规则。后续从普通文档升级出来的强制规则，必须进入 SPEC 专区。

---

## AI Agent 上下文读取

AI Agent 的文档读取遵循 [AI Agent Startup Context SPEC](specs/agent-startup-context-spec.md) 定义的分层策略：冷启动完整读取；热延续在确认上下文新鲜时按需增量读取；不确定时必须按冷启动策略或任务路由表补读关键文档。

### 冷启动（新会话、领域切换、context compaction 后）

按顺序读取：

1. [SPEC 中心](specs/README.md) — 项目铁律、SPEC 和普通文档分区。
2. [AI Agent Startup Context SPEC](specs/agent-startup-context-spec.md) — 上下文感知分层读取策略。
3. 本文件 — 文档分区、当前状态和任务路由。
4. [STDAS 背景知识总览](domain-knowledge/stdas-domain-primer.md) — 业务场景和领域不变量。

### 热延续（同一会话、同一任务领域）

在能够明确确认相关文档已在当前上下文窗口中时，可以跳过重复读取。只按下方任务路由表读取本次请求新增领域或新增风险点对应的专题文档。

### 微调（对上一次输出的延续修改）

在能够明确确认上下文新鲜、任务范围完全一致且不涉及 SPEC、架构、Git/GitHub 或跨域规则时，可以不新增读取，直接基于现有上下文执行。

### 冲突处理

如果任务描述和 `docs` 中的规则冲突，必须先指出冲突，按对应事实来源收敛后再继续。

---

## 任务路由表

**按任务类型直接定位必读文档。热延续时只读本次任务新增或上下文不确定的行。**

| 任务类型 | 必读文档 |
|----------|----------|
| **领域术语、命名口径** | [领域知识 README](domain-knowledge/README.md) → [stdas-domain-primer.md](domain-knowledge/stdas-domain-primer.md) |
| **架构边界、服务拆分、ADR** | [架构设计 README](architecture-design/README.md) → [system-architecture.md](architecture-design/system-architecture.md) + [adr/README.md](architecture-design/adr/README.md) |
| **多客户、DataProfile、客制化** | [osat-multi-customer-architecture.md](architecture-design/osat-multi-customer-architecture.md) + [domain-model.md](architecture-design/domain-model.md) |
| **功能切片规划或验收** | [frontend-backend-sync-design.md](architecture-design/frontend-backend-sync-design.md) + [feature-slices-v1.md](architecture-design/feature-slices-v1.md) |
| **AI Agent 运行规则** | [ai-agent-runtime-rules.md](architecture-design/ai-agent-runtime-rules.md) |
| **AI 代码生成治理** | [ai-code-generation-governance.md](architecture-design/ai-code-generation-governance.md) |
| **Git/GitHub/GitLab 提交、推送、PR/MR、合并、回退** | [Git Commit and Collaboration SPEC](specs/git-commit-collaboration-spec.md) + [git-github-sop.md](architecture-design/git-github-sop.md) |
| **前端代码架构、路由实现、状态、组件** | [前端设计 README](frontend-design/README.md) → [frontend-tech-architecture.md](frontend-design/frontend-tech-architecture.md) + [frontend-code-quality-rules.md](frontend-design/frontend-code-quality-rules.md) + [frontend-ai-code-generation-rules.md](frontend-design/frontend-ai-code-generation-rules.md) |
| **UI/UX 通用护栏、响应式、表格、图表、表单** | [前端设计 README](frontend-design/README.md) → [ui-ux-constraints.md](frontend-design/ui-ux-constraints.md) |
| **AI 页面视觉、prompt、mockup 生成** | [前端设计 README](frontend-design/README.md) → [mockup-prompt-workflow.md](frontend-design/mockup-prompt-workflow.md) + [ui-ux-constraints.md](frontend-design/ui-ux-constraints.md) |
| **Rust 后端代码** | [后端设计 README](backend-design/README.md) → [Rust Coding Guidelines SPEC](specs/rust-coding-guidelines-spec.md) + [rust-code-quality-rules.md](backend-design/rust-code-quality-rules.md) + [rust-ai-code-generation-rules.md](backend-design/rust-ai-code-generation-rules.md)；非平凡实现还需 [rust-reference-projects-and-patterns.md](backend-design/rust-reference-projects-and-patterns.md) |
| **API 字段、枚举、错误、分页** | [api-principles.md](backend-design/api-principles.md) + [api-contract-rules.md](backend-design/api-contract-rules.md) |
| **数据版本、QuerySnapshot、Evidence** | [data-architecture.md](backend-design/data-architecture.md) + [analytics-engine.md](backend-design/analytics-engine.md) |
| **摄入、Parser、文件安全** | [ingestion-pipeline.md](backend-design/ingestion-pipeline.md) |
| **缓存、Redis、查询预算** | [cache-strategy.md](backend-design/cache-strategy.md) + [analytics-engine.md](backend-design/analytics-engine.md) |
| **事件、Outbox/Inbox、重放** | [event-contract-rules.md](backend-design/event-contract-rules.md) |
| **权限、脱敏、任务状态机** | [security-reliability.md](backend-design/security-reliability.md) |
| **部署、配置、日志、健康检查** | [deployment-observability.md](backend-design/deployment-observability.md) |

→ 每行中的 **README** 是该分区的导航入口，包含文档索引、任务必读和变更原则。先读 README 再读具体文档。

---

## 文档分区

| 分区 | 位置 | 定位 | 稳定性 | 入口 |
|------|------|------|--------|------|
| SPEC | `specs/` | 项目铁律、启动规则、编码基线 | 最高稳定 | [specs/README.md](specs/README.md) |
| 架构设计 | `architecture-design/` | 产品愿景、领域模型、系统架构、ADR | 高稳定 | [architecture-design/README.md](architecture-design/README.md) |
| 领域知识 | `domain-knowledge/` | STDAS 背景、OSAT 场景、测试数据语义 | 高稳定 | [domain-knowledge/README.md](domain-knowledge/README.md) |
| 前端设计 | `frontend-design/` | 前端代码架构、通用体验护栏、AI 设计协作流程 | 高频演进 | [frontend-design/README.md](frontend-design/README.md) |
| 后端设计 | `backend-design/` | Rust workspace、数据平台、API | 高频演进 | [backend-design/README.md](backend-design/README.md) |
| 验证记录 | `verification/` | Phase 0/0.5 环境验证截图和记录 | 辅助 | [phase-0-preflight.md](verification/phase-0-preflight.md) |

每个分区 README 都遵循标准结构：**定位 → 文档索引 → 任务必读 → 变更原则**。AI Agent 只需读取对应分区的 README 即可获得完整的导航信息。

---

## 跨域规则

| 文档 | 说明 |
|------|------|
| [SPEC 中心](specs/README.md) | 项目铁律、SPEC 和普通文档分区、SPEC 升级规则 |
| [AI Agent Startup Context SPEC](specs/agent-startup-context-spec.md) | 上下文感知分层读取策略：冷启动/热延续/微调三级读取 gate |
| [Git Commit and Collaboration SPEC](specs/git-commit-collaboration-spec.md) | Git 提交、推送、PR/MR、主分支合并和回退铁律 |
| [AI Agent 运行时规则](architecture-design/ai-agent-runtime-rules.md) | AI Agent 修改文档或代码前的工作方式、上下文读取策略和交付检查 |
| [AI 代码生成治理机制](architecture-design/ai-code-generation-governance.md) | AI 生成代码时的偏航提醒、替代方案、用户坚持原方案后的待优化标记机制 |
| [Git / GitHub 安全 SOP](architecture-design/git-github-sop.md) | Git remote 绑定、提交、推送、PR/MR、AI 生成代码和安全回退流程 |
| [前后端同步设计](architecture-design/frontend-backend-sync-design.md) | 前端和后端按功能切片同步设计、同步修改、同步验收 |
| [首批功能切片 V1](architecture-design/feature-slices-v1.md) | 第一批端到端功能切片的页面、API、权限、数据语义和验收基线 |
| [项目目录结构](project-structure.md) | monorepo 顶层目录、前端应用、后端服务、工具、文档边界和本地生成目录管理规则 |

---

## 当前状态

当前文档定为 V1 架构和设计基线。后续停止泛化扩写，优先进入首批功能切片规格、Phase 0 代码骨架和 Phase 0.5 环境验证闭环。

当前前端/产品设计的事实来源只恢复到已确认的登录页和“身份、会话与授权上下文”最小切片。登录成功后的正式工程入口仍等待下一张页面设计稿确认；当前实现只保留空白工作区占位，不定义固定 Overview、Dashboard 或 Data Explorer 默认路由。

当前活跃后端 runtime 口径见 [ADR-0014](architecture-design/adr/0014-gateway-modular-monolith.md)：`stdas-gateway` 是第一阶段唯一 backend runtime service，内部采用 strong module boundary；多服务、NATS、Outbox/Inbox、MinIO 等能力为触发条件满足后的 future expansion，不作为 Phase 0 必需项。

当前本机环境记录见 [Phase 0.5 环境验证记录](backend-design/phase-0-5-environment-validation.md)。Phase 0 / 0.5 基线为 Windows + Scoop 本机工具链，前端统一使用 pnpm；Gateway HTTP 应用层直接采用 Axum；持久化层采用 SQLx + PostgreSQL，不采用 ORM；Redis 已安装并可按缓存策略启用；Windows 本地开发不安装、不使用 Docker。

---

## 维护规则

- 架构设计只记录长期稳定的系统边界和原则，重大变化必须新增 ADR。
- 领域知识记录业务事实和行业语义，不承载具体实现方案。
- 前端和后端按功能切片同步推进；同一切片必须能追溯到页面、API、数据、权限、状态和验收。
- 前后端文档不得反向修改架构原则；需要改变架构时先提交 ADR。
- `docs/README.md` 只作为入口和路由中心，不承载详细设计。
- 强制性规则优先放入 `docs/specs/`；普通文档只能引用 SPEC，不能复制或改写铁律正文。Git 提交、推送、PR/MR、主分支合并和回退必须遵守 [Git Commit and Collaboration SPEC](specs/git-commit-collaboration-spec.md)。
- 新增或移动文档时，必须同步更新本入口、对应分区 README 和相关交叉引用。
- 进入代码实现前必须完成环境验证 gate，确认 build/test 和 AI Agent 修复闭环可跑。
- V1 baseline 之后新增规则必须服务于具体功能切片或代码实现，不再扩写泛化原则。
- 每个分区 README 必须遵循标准结构：**定位 → 文档索引 → 任务必读 → 变更原则**。
