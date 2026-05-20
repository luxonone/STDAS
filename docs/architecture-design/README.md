# 架构设计

本目录保存 STDAS 的长期稳定设计。架构设计一旦确认，不应随具体实现细节频繁改动。影响系统边界、能力域、数据平台方向、部署形态或关键约束的变化，必须通过 ADR 记录。

## 文档

| 文档 | 说明 |
|------|------|
| [product-vision.md](product-vision.md) | 产品愿景、系统目标、用户角色、产品边界 |
| [domain-model.md](domain-model.md) | 能力域、核心实体、关键值对象、数据范围 |
| [system-architecture.md](system-architecture.md) | 总体架构、进程边界、Command/Query 分离、系统约束 |
| [osat-multi-customer-architecture.md](osat-multi-customer-architecture.md) | OSAT 多客户客制化架构 |
| [ai-agent-runtime-rules.md](ai-agent-runtime-rules.md) | AI Agent 修改文档或代码前必须读取的项目级运行规则 |
| [ai-code-generation-governance.md](ai-code-generation-governance.md) | AI 代码生成、偏航提醒、替代方案和待优化标记的治理机制 |
| [frontend-backend-sync-design.md](frontend-backend-sync-design.md) | 前端、后端、数据、权限和验收按功能切片同步推进的设计规则 |
| [feature-slices-v1.md](feature-slices-v1.md) | 首批功能切片交付卡和端点级契约索引 |
| [architecture-review.md](architecture-review.md) | 架构合理性审查和六问检查结论 |
| [architecture-diagram.png](architecture-diagram.png) | 总体架构图 PNG |
| [architecture-diagram.drawio](architecture-diagram.drawio) | 可编辑 draw.io 架构图 |
| [architecture-overview.html](architecture-overview.html) | 架构图介绍页 |
| [adr/README.md](adr/README.md) | 架构决策记录 |

## 变更原则

- 只接受影响长期方向的修改。
- 不记录临时实现细节。
- 不跟随页面布局、crate 文件结构或接口字段的日常调整。
- 架构变化必须说明原因、影响和验证方式。

## 架构图一致性检查

架构图必须与文字架构保持一致。更新 `architecture-diagram.drawio` 或 `architecture-diagram.png` 时必须检查：

- 必须包含 React Analytical Workbench、`stdas-gateway`、`identity-service`、`customer-service`、`data-pipeline-service`、`analytics-service`、`workflow-service`、`integration-service`。
- 必须包含 NATS JetStream、PostgreSQL service schemas/databases、MinIO/S3。
- 必须表达 Parquet/DuckDB/ClickHouse 的可演进分析路径。
- 不得出现 per-customer service、frontend 直连内部服务、service 直接写其他服务 DB、PostgreSQL 作为无限 OLAP。
