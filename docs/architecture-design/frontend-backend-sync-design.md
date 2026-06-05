# 前后端同步设计

本文定义 STDAS 前端、后端、数据、权限和验收如何按功能切片同步推进。它不定义固定页面、固定 route、固定导航或固定 mockup 顺序。

## 定位

本文件只回答：

1. 一个功能切片需要同时牵动哪些前端、后端、数据和权限设计。
2. 修改某一侧时，需要检查哪些事实来源。
3. 切片完成时，需要提交哪些可验收材料。

具体页面视觉和交互应进入 [AI Mockup Prompt Workflow](../frontend-design/mockup-prompt-workflow.md)；具体代码架构应进入 [前端技术架构](../frontend-design/frontend-tech-architecture.md)。

## 事实来源

| 规则类型 | 事实来源 |
|----------|----------|
| 前端技术栈、目录边界、状态模型、API client、adapter | [前端技术架构](../frontend-design/frontend-tech-architecture.md) |
| 前端 TypeScript、React、API、状态、测试规则 | [前端代码质量规则](../frontend-design/frontend-code-quality-rules.md) |
| AI 前端代码生成规则 | [前端 AI 代码生成规则](../frontend-design/frontend-ai-code-generation-rules.md) |
| 通用 UI/UX 护栏 | [UI/UX 通用护栏](../frontend-design/ui-ux-constraints.md) |
| AI 页面视觉和 mockup 协作 | [AI Mockup Prompt Workflow](../frontend-design/mockup-prompt-workflow.md) |
| API 字段、枚举、错误码、分页、版本兼容 | [API 契约规则](../backend-design/api-contract-rules.md) |
| API 组织原则 | [API 契约原则](../backend-design/api-principles.md) |
| DataVersion、QuerySnapshot、查询预算、Evidence | [分析引擎](../backend-design/analytics-engine.md) + [数据架构](../backend-design/data-architecture.md) |
| 权限、脱敏、任务状态机、导出安全 | [安全与可靠性](../backend-design/security-reliability.md) |
| 首批能力切片 | [首批功能切片 V1](feature-slices-v1.md) |

如果本文件和事实来源冲突，以事实来源为准，并应删除本文件中的重复描述。

## 同步原则

- 功能切片先定义用户任务、后端能力、数据对象、权限边界和验收方式。
- 前端页面、route、导航和布局不在切片前预设；它们由 AI 设计候选、用户确认和实现约束共同决定。
- API 契约是前后端共同边界，但不是前端体验的替代品。
- 后端不能机械照搬前端组件，前端也不能直接依赖数据库结构。
- 前端不得为了视觉方便绕开 CustomerScope、权限、QuerySnapshot、DataVersion、Job、Evidence 或导出安全。
- 每个 milestone 必须小步、可验证、可回归。
- 跨域文档只写“检查什么”和“引用哪里”，不复制具体规则正文。

## 功能切片交付卡

每个功能切片必须提交一张交付卡。交付卡用于串联事实来源，而不是把所有规则重写一遍。

| 项 | 必填内容 | 事实来源 |
|----|----------|----------|
| 切片名称 | 稳定、短、面向用户任务 | 本文件 |
| 用户目标 | 用户为什么需要这个能力 | 领域知识、后端能力、用户输入 |
| 后端能力域 | identity / customer / data_pipeline / analytics / evidence / workflow / integration | [系统架构](system-architecture.md) |
| 前端设计入口 | 是否需要 AI mockup；是否已有用户确认候选图；是否允许新增 route | [mockup-prompt-workflow.md](../frontend-design/mockup-prompt-workflow.md) |
| 前端技术约束 | React + TypeScript、feature slice、adapter、状态分层 | [frontend-tech-architecture.md](../frontend-design/frontend-tech-architecture.md) |
| UI/UX 护栏 | 数据可信、权限、URL、表格、图表、表单、异步、可访问性 | [ui-ux-constraints.md](../frontend-design/ui-ux-constraints.md) |
| 前端状态 | URL State、Session State、Forbidden URL State、本地状态、server state | [ui-ux-constraints.md](../frontend-design/ui-ux-constraints.md) |
| API 契约 | 方法、路径、字段、类型、默认值、取值范围、错误码 | [api-principles.md](../backend-design/api-principles.md) + [api-contract-rules.md](../backend-design/api-contract-rules.md) |
| 数据语义 | DataProfile、DataVersion、QuerySnapshot、Evidence、审计 | [data-architecture.md](../backend-design/data-architecture.md) + [analytics-engine.md](../backend-design/analytics-engine.md) |
| 权限策略 | Principal、CustomerScope、脱敏、隐藏、重新校验 | [security-reliability.md](../backend-design/security-reliability.md) |
| 同步/异步 | 同步响应、异步 job、取消、重试、过期 | [security-reliability.md](../backend-design/security-reliability.md) |
| 验收方式 | 手动检查、契约测试、单元测试、集成测试、截图审阅 | 对应事实来源 |

## Milestone 拆分

### M0：骨架

- 建立最小 route 或入口，不要求最终页面形态。
- 建立 typed API client 调用边界。
- 建立 loading、empty、error、permission denied、data trust 占位。
- 后端可以返回 mock 或最小真实结构，但响应信封、错误和权限语义必须稳定。

### M1：主链路

- 打通用户主任务，例如登录、查看授权上下文、查询 Lot、运行分析、查看任务。
- 前端展示核心结果和关键状态。
- 后端返回真实数据或可验证样例数据。

### M2：编辑流

- 支持配置、模板、规则、筛选、表单等编辑能力。
- 后端校验返回可定位错误。
- 前端展示字段级、区域级、页面级和权限级错误。

### M3：异步与大数据

- 高成本查询受查询预算控制。
- 大结果、导出和重算转为异步任务。
- 前端展示 job id、阶段、进度、取消、重试、过期和失败诊断。

### M4：回归验证

- 回归前端页面、API 契约、后端状态机、错误码和权限。
- 验证 AI Agent 开发闭环。
- 用户确认的视觉/交互规则必须能被截图或手动检查验证。

## 首批能力切片

端点级交付卡见 [首批功能切片 V1](feature-slices-v1.md)。本节只保留能力顺序，不定义前端页面名称或 route。

| 顺序 | 能力切片 | 后端能力域 | 前端设计要求 |
|------|----------|------------|--------------|
| 1 | 身份、会话与授权上下文 | identity、customer | 允许 AI 重新设计登录后入口；必须显示会话、权限和可见范围状态。 |
| 2 | 系统健康与本地 preflight | system、telemetry | 只服务开发/运维验证，不作为产品页面固定入口。 |
| 3 | 受控选项与上下文解析 | customer、data_pipeline | 前端通过 options API 获取客户、产品、测试类型、测试站点等受控选项。 |
| 4 | Lot / Run / File 查询与追溯 | data_pipeline | 页面形态由 AI 设计；必须保留权限、LotEndTime、lineage 和数据可信状态。 |
| 5 | 分析查询与结果可信 | analytics | 必须表达查询预算、partial、stale、QuerySnapshot 和 DataVersion set。 |
| 6 | Evidence / Investigation 证据沉淀 | evidence、analytics、workflow | 必须保留 Evidence version、QuerySnapshot、权限变化状态。 |
| 7 | Job / Export 生命周期 | workflow、analytics、data_pipeline | 必须表达异步状态、失败诊断、过期、下载权限重校验。 |
| 8 | Customer / Profile / Rule 治理 | customer、integration、analytics | 治理 UI 可以显式展示 DataProfile、规则版本、diff、影响范围和审计。 |

## 同步修改规则

| 修改内容 | 必须同步检查 |
|----------|--------------|
| 页面、route、导航、视觉布局 | [mockup-prompt-workflow.md](../frontend-design/mockup-prompt-workflow.md)、[ui-ux-constraints.md](../frontend-design/ui-ux-constraints.md)、相关 API 契约 |
| 前端目录、组件、状态、API client | [frontend-tech-architecture.md](../frontend-design/frontend-tech-architecture.md)、[frontend-code-quality-rules.md](../frontend-design/frontend-code-quality-rules.md) |
| 表单、筛选、表格、图表、弹窗 | [ui-ux-constraints.md](../frontend-design/ui-ux-constraints.md)、Options/API 契约 |
| API 字段、默认值、枚举、错误码 | [api-contract-rules.md](../backend-design/api-contract-rules.md)、前端展示和错误处理 |
| DataVersion、latest committed、QuerySnapshot | [analytics-engine.md](../backend-design/analytics-engine.md)、历史 workspace/case/evidence/export 展示 |
| 权限、CustomerScope、脱敏、隐藏 | [security-reliability.md](../backend-design/security-reliability.md)、前端权限状态 |
| 任务状态机、导出、取消、重试、过期 | [security-reliability.md](../backend-design/security-reliability.md)、前端 job/export 状态 |
| DataProfile、规则、模板版本 | [data-architecture.md](../backend-design/data-architecture.md)、治理能力切片 |

## 完成定义

```text
设计完成 =
  功能切片交付卡完整
  + 后端能力域清楚
  + 前端设计入口和创意空间清楚
  + UI/UX 护栏已引用
  + API 契约事实来源已更新
  + 数据、权限、异步语义已落到后端事实来源
  + 验收方式清楚
  + 无重复规则副本
  + 无固定旧页面假设
```
