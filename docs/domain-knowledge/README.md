# 领域知识

> **定位**：半导体测试、OSAT、FT 分析、MES 上下文和数据语义的背景知识。不替代架构、前端或后端设计，而是帮助理解这些设计为什么存在。
> **稳定性**：高稳定 — 业务事实和行业语义不随实现变化。
> **事实来源范围**：业务术语、命名口径、FT/CP/BI/SLT 边界、数据语义。

## 文档索引

| 文档 | 说明 |
|------|------|
| [stdas-domain-primer.md](stdas-domain-primer.md) | STDAS 背景知识总览：业务场景、用户、核心对象、数据流、分析问题和领域不变量 |

## 任务必读

| 任务类型 | 必读文档 |
|----------|----------|
| 理解领域术语、命名口径、FT/CP/BI/SLT 边界 | [stdas-domain-primer.md](stdas-domain-primer.md) |
| 架构变更涉及业务概念时 | 本目录全部文档 |

## 变更原则

- 领域知识描述业务事实、行业语义和工程分析上下文。
- 架构边界、服务拆分、技术选型和 ADR 仍以 [architecture-design](../architecture-design/README.md) 为准。
- UI/UX 和前端实现仍以 [frontend-design](../frontend-design/README.md) 为准。
- API、数据平台、缓存、任务和后端实现仍以 [backend-design](../backend-design/README.md) 为准。