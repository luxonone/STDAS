# 架构设计

> **定位**：STDAS 的长期稳定设计。架构设计一旦确认，不应随具体实现细节频繁改动。
> **稳定性**：高稳定 — 影响系统边界、能力域、数据平台方向、部署形态或关键约束的变化，必须通过 ADR 记录。
> **事实来源范围**：产品愿景、领域模型、系统架构、服务边界、关键 ADR、功能切片基线。

## 相关 SPEC

| SPEC | 说明 |
|------|------|
| [SPEC 中心](../specs/README.md) | 项目铁律、SPEC 和普通文档分区、SPEC 升级规则 |
| [AI Agent Startup Context SPEC](../specs/agent-startup-context-spec.md) | 上下文感知分层读取策略：冷启动/热延续/微调三级读取 gate |
| [Rust Coding Guidelines SPEC](../specs/rust-coding-guidelines-spec.md) | Rust coding baseline |
| [Git Commit and Collaboration SPEC](../specs/git-commit-collaboration-spec.md) | Git 提交、推送、PR/MR、主分支合并和回退铁律 |

## 文档索引

| 文档 | 说明 |
|------|------|
| [product-vision.md](product-vision.md) | 产品愿景、系统目标、用户角色、产品边界 |
| [domain-model.md](domain-model.md) | 能力域、核心实体、关键值对象、数据范围 |
| [system-architecture.md](system-architecture.md) | 总体架构、进程边界、Command/Query 分离、系统约束 |
| [osat-multi-customer-architecture.md](osat-multi-customer-architecture.md) | OSAT 多客户客制化架构 |
| [ai-agent-runtime-rules.md](ai-agent-runtime-rules.md) | AI Agent 修改文档或代码前的工作方式、上下文读取策略和交付检查 |
| [ai-code-generation-governance.md](ai-code-generation-governance.md) | AI 代码生成、偏航提醒、替代方案和待优化标记的治理机制 |
| [git-github-sop.md](git-github-sop.md) | Git remote 绑定、提交、推送、PR/MR、AI 生成代码和安全回退 SOP |
| [frontend-backend-sync-design.md](frontend-backend-sync-design.md) | 前端、后端、数据、权限和验收按功能切片同步推进的设计规则 |
| [feature-slices-v1.md](feature-slices-v1.md) | 首批功能切片交付卡和端点级契约索引 |
| [architecture-review.md](architecture-review.md) | 架构合理性审查和六问检查结论 |
| [architecture-diagram.png](architecture-diagram.png) | 总体架构图 PNG |
| [architecture-diagram.drawio](architecture-diagram.drawio) | 可编辑 draw.io 架构图 |
| [architecture-overview.html](architecture-overview.html) | 架构图介绍页 |
| [adr/README.md](adr/README.md) | 架构决策记录 |

## 任务必读

| 任务类型 | 必读文档 |
|----------|----------|
| 架构边界、服务拆分、长期约束 | 本 README + [system-architecture.md](system-architecture.md) + [adr/README.md](adr/README.md) |
| 多客户、DataProfile、客制化 | [osat-multi-customer-architecture.md](osat-multi-customer-architecture.md) |
| 功能切片规划或验收 | [frontend-backend-sync-design.md](frontend-backend-sync-design.md) + [feature-slices-v1.md](feature-slices-v1.md) |
| AI Agent 运行规则和上下文读取 | [ai-agent-runtime-rules.md](ai-agent-runtime-rules.md) |
| AI 代码生成偏航提醒和治理 | [ai-code-generation-governance.md](ai-code-generation-governance.md) |
| Git/GitHub/GitLab 操作、提交、推送、合并、回退 | [Git Commit and Collaboration SPEC](../specs/git-commit-collaboration-spec.md) + [git-github-sop.md](git-github-sop.md) |

## 变更原则

- 只接受影响长期方向的修改。
- 不记录临时实现细节。
- 不跟随页面布局、crate 文件结构或接口字段的日常调整。
- 架构变化必须说明原因、影响和验证方式。

## 架构图一致性检查

架构图必须与文字架构保持一致。当前文字架构以 [ADR-0014](adr/0014-gateway-modular-monolith.md) 为准：`stdas-gateway` 是当前唯一 backend runtime service，内部通过 module boundary 表达未来服务边界。

更新 `architecture-diagram.drawio` 或 `architecture-diagram.png` 时必须检查：

- 必须包含 frontend workbench 和 `stdas-gateway`。
- 当前阶段图必须表达 `stdas-gateway` 内部 module boundary：`identity`、`customer`、`data_pipeline`、`analytics`、`evidence`、`workflow`、`integration`。
- NATS JetStream、service schemas/databases、MinIO/S3、Parquet/DuckDB/ClickHouse 只能作为 future expansion 或未来服务化视图出现，不得表达为 Phase 0 必需 runtime。
- 不得出现 per-customer service、frontend 直连内部服务、service 直接写其他服务 DB、PostgreSQL 作为无限 OLAP。
