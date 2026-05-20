# 前后端同步设计

本文件定义 STDAS 前端设计和后端设计如何按功能切片同步推进。它不是 UI/UX 规则、API 契约、数据模型或权限模型的替代文档；这些具体规则必须回到各自事实来源维护。

## 定位

本文件只回答三个问题：

1. 一个功能切片需要同时牵动哪些前端、后端、数据和权限设计？
2. 修改某一侧时，需要检查哪些对应文档？
3. 切片完成时，需要提交哪些可验收材料？

本文件不重复以下内容：

| 规则类型 | 事实来源 |
|----------|----------|
| 前端技术栈、分层、状态模型、适配层 | [前端技术架构](../frontend-design/frontend-tech-architecture.md) |
| 前端代码生成、TypeScript、React、API、状态和测试规则 | [前端代码质量规则](../frontend-design/frontend-code-quality-rules.md) |
| 页面族、布局、交互、表格、图表、弹窗、可访问性 | [UI/UX 约束](../frontend-design/ui-ux-constraints.md) |
| 页面层级、路由、页面族归属 | [页面层级设计](../frontend-design/page-hierarchy-design.md) |
| 前端工作台结构和组件分层 | [工作台设计](../frontend-design/workbench-design.md) |
| API 字段、枚举、错误码、分页、版本兼容 | [API 契约规则](../backend-design/api-contract-rules.md) |
| QuerySnapshot、查询预算、DataVersion 冻结、Evidence | [分析引擎](../backend-design/analytics-engine.md) |
| 数据分层、DataVersion、存储演进 | [数据架构](../backend-design/data-architecture.md) |
| 权限、脱敏、任务状态机、可靠性 | [安全与可靠性](../backend-design/security-reliability.md) |
| 首批功能切片交付卡 | [首批功能切片 V1](feature-slices-v1.md) |

如果本文件和事实来源出现冲突，以事实来源为准，并应删除本文件中的重复描述。

## 同步设计原则

- 一个功能切片同时包含前端、后端、数据、权限、错误和验收。
- API 契约是前后端共同边界，但不是前端体验的替代品。
- 页面族先由前端设计确认，API 和数据契约再按该用户任务补齐。
- 后端不能机械照搬前端组件，前端也不能直接依赖数据库结构。
- 每个 milestone 必须小步、可验证、可回归。
- 跨域文档只写“检查什么”和“引用哪里”，不复制具体规则正文。

## 功能切片交付卡

每个功能切片必须提交一张交付卡。交付卡用于串联事实来源，而不是把所有规则重写一遍。

| 项 | 必填内容 | 事实来源 |
|----|----------|----------|
| 切片名称 | 稳定、短、面向用户任务 | 本文件 |
| 用户目标 | 用户为什么需要这个能力 | 前端设计、产品语境 |
| 页面归属 | 页面族、route、主入口、返回路径 | [页面层级设计](../frontend-design/page-hierarchy-design.md) |
| 前端技术约束 | React + TypeScript、feature slice、adapter、状态分层 | [前端技术架构](../frontend-design/frontend-tech-architecture.md)、[前端代码质量规则](../frontend-design/frontend-code-quality-rules.md) |
| UI/UX 引用 | 适用页面族、P0/P1、例外豁免 | [UI/UX 约束](../frontend-design/ui-ux-constraints.md) |
| 前端状态 | Required URL State、Session State、Forbidden URL State、本地状态、服务端状态 | [UI/UX 约束](../frontend-design/ui-ux-constraints.md) |
| 后端入口 | REST API、gRPC、event、job | [后端设计](../backend-design/README.md) |
| API 契约 | 字段、类型、默认值、取值范围、错误码 | [API 契约规则](../backend-design/api-contract-rules.md) |
| 数据语义 | DataProfile、DataVersion、QuerySnapshot、Evidence、审计 | [数据架构](../backend-design/data-architecture.md)、[分析引擎](../backend-design/analytics-engine.md) |
| 权限策略 | Principal、CustomerScope、脱敏、隐藏、重新校验 | [安全与可靠性](../backend-design/security-reliability.md) |
| 同步/异步 | 同步响应、异步 job、取消、重试、过期 | [安全与可靠性](../backend-design/security-reliability.md) |
| 验收方式 | 手动检查、契约测试、单元测试、集成测试 | 对应事实来源 |

## Milestone 拆分

### M0：骨架

目标：

- 建立 route、API client、服务入口、契约占位。
- 页面能加载空状态。
- API 能返回 mock 或最小真实结构。
- URL 状态、权限状态、数据可信状态有最小占位。

验收依据：

- 前端按 [UI/UX 约束](../frontend-design/ui-ux-constraints.md) 检查页面状态占位。
- 后端按 [API 契约规则](../backend-design/api-contract-rules.md) 检查字段和错误结构。

### M1：主链路

目标：

- 打通用户主任务，例如上传文件、查看 Lot、执行分析查询。
- 后端返回真实数据或可验证样例数据。
- 前端展示核心结果。

验收依据：

- 页面主任务按前端页面族约束验收。
- API 请求、响应、权限和错误按后端契约验收。

### M2：编辑流

目标：

- 支持配置、模板、规则、筛选、表单等编辑能力。
- 后端校验返回可定位错误。
- 前端能展示字段级、页面级和权限级错误。

验收依据：

- 表单、Options API、级联字段和退出保护按 [UI/UX 约束](../frontend-design/ui-ux-constraints.md) 验收。
- 字段校验、错误码和 Options 响应按 [API 契约规则](../backend-design/api-contract-rules.md) 验收。

### M3：异步与大数据

目标：

- 优化筛选、表格、图表、缓存、异步任务和导出体验。
- 高成本查询受查询预算控制。
- 大结果、导出和重算可转为异步任务。

验收依据：

- 图表、表格、partial data、over budget、任务入口按 [UI/UX 约束](../frontend-design/ui-ux-constraints.md) 验收。
- 查询预算、QuerySnapshot、任务状态机和导出权限按 [分析引擎](../backend-design/analytics-engine.md) 与 [安全与可靠性](../backend-design/security-reliability.md) 验收。

### M4：回归验证

目标：

- 回归前端页面、API 契约、后端状态机、错误码和权限。
- 验证 AI Agent 开发闭环。

验收依据：

- build/test 可跑。
- 功能切片交付卡中的事实来源检查全部通过。

## 首批建议功能切片

端点级交付卡见 [首批功能切片 V1](feature-slices-v1.md)。本节只保留切片顺序和主验收来源。

| 顺序 | 切片 | 前端入口 | 后端入口 | 主验收来源 |
|------|------|----------|----------|------------|
| 1 | 登录与当前上下文 | Global Shell、`/login`、`/app/overview` | identity、gateway auth、customer APIs | UI/UX 约束、API 契约、安全与可靠性 |
| 2 | Overview summary | `/app/overview` | analytics、data pipeline | UI/UX 约束、分析引擎 |
| 3 | Data Explorer：Lot 列表与详情 | `/app/data/lots`、`/app/data/lots/:lot_id` | data query、analytics summary | UI/UX 约束、API 契约、数据架构 |
| 4 | Data Explorer：文件和数据版本上下文 | `/app/data/files/:file_id`、`/app/data/versions/:data_version` | data pipeline、object metadata | 数据架构、API 契约 |
| 5 | Analysis Workspace 基础查询 | `/app/analysis`、`/app/analysis/:workspace_id` | analytics APIs | UI/UX 约束、分析引擎 |
| 6 | Data Explorer -> Analysis Workspace：多 Lot 分析 | Lot List 多选、Analysis Workspace | analytics、workspace APIs | UI/UX 约束、分析引擎、API 契约 |
| 7 | Analysis Workspace 模板应用与保存 | Analysis Workspace 模板入口 | workspace/template APIs | UI/UX 约束、API 契约 |
| 8 | Alerts & Investigation 上下文跳转 | `/app/alerts`、`/app/alerts/:alert_id`、`/app/cases/:case_id` | analytics、workflow events | UI/UX 约束、分析引擎、安全与可靠性 |
| 9 | Jobs & Exports 状态管理 | `/app/jobs`、`/app/exports` | workflow、job APIs | UI/UX 约束、安全与可靠性 |
| 10 | Governance：Customer 与 DataProfile | `/app/governance/customers`、`/app/governance/profiles` | customer/profile APIs | UI/UX 约束、API 契约、数据架构 |
| 11 | Governance：规则与模板版本 | `/app/governance/rules`、`/app/governance/templates` | analytics、profile APIs | UI/UX 约束、API 契约、分析引擎 |

## 同步修改规则

| 修改内容 | 必须同步检查 |
|----------|--------------|
| 页面、路由、页面族 | [页面层级设计](../frontend-design/page-hierarchy-design.md)、[UI/UX 约束](../frontend-design/ui-ux-constraints.md)、相关 API 契约 |
| 表单、筛选、表格、图表、弹窗 | [UI/UX 约束](../frontend-design/ui-ux-constraints.md)、Options/API 契约 |
| API 字段、默认值、枚举、错误码 | [API 契约规则](../backend-design/api-contract-rules.md)、前端展示和错误处理 |
| DataVersion policy、latest committed、QuerySnapshot | [分析引擎](../backend-design/analytics-engine.md)、历史 workspace/case/export 展示 |
| 权限、CustomerScope、脱敏、隐藏 | [安全与可靠性](../backend-design/security-reliability.md)、前端权限状态 |
| 任务状态机、导出、取消、重试、过期 | [安全与可靠性](../backend-design/security-reliability.md)、Jobs & Exports 页面 |
| DataProfile、规则、模板版本 | [数据架构](../backend-design/data-architecture.md)、System Governance 页面 |

## 完成定义

```text
设计完成 =
  功能切片交付卡完整
  + 页面归属清楚
  + UI/UX 事实来源已引用
  + API 契约事实来源已更新
  + 数据、权限、异步语义已落到后端事实来源
  + 验收方式清楚
  + 无重复规则副本
```
