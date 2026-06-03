# 前端设计

> **当前状态**：冻结待重审。用户已在 2026-06-03 明确要求“前端设计和产品设计部分的内容全部作废不做讨论”。因此，本目录中的页面、路由、mockup、UI/UX 和产品化页面设计材料暂时只作为历史材料保留，不作为当前开发事实来源。后续必须由用户重新发起前端/产品设计 review 后，才能恢复或重写本目录的设计契约。
> **定位**：STDAS 前端工作台设计。前端设计会随产品验证、用户反馈和组件实现持续调整，因此允许高频迭代。
> **稳定性**：高频演进 — 页面、组件、交互、状态和可视化策略可以快速迭代，但不得反向破坏架构原则。
> **事实来源范围**：工作台信息架构、页面、组件、交互、状态、可视化策略和前端代码规则。

## 文档索引

| 文档 | 说明 |
|------|------|
| [frontend-tech-architecture.md](frontend-tech-architecture.md) | React + TypeScript 技术基线、Feature-Sliced Analytical Workbench Architecture、状态和适配层 |
| [frontend-code-quality-rules.md](frontend-code-quality-rules.md) | 前端代码生成、TypeScript、React、状态、API、表格、图表、测试约束 |
| [frontend-ai-code-generation-rules.md](frontend-ai-code-generation-rules.md) | AI 生成或修改前端代码时的状态、API、组件检查、偏航提醒和待优化标记规则 |
| [mockup-prompt-workflow.md](mockup-prompt-workflow.md) | 前端开发前的页面 mockup 提示词、审阅流程、参考转译规则和页面级 prompt |
| [page-hierarchy-design.md](page-hierarchy-design.md) | 全局壳层、工程分析工作台、系统治理的页面层级和路由基线 |
| [frontend-page-design-v1.md](frontend-page-design-v1.md) | STDAS 第一阶段全量页面、次级页面、独立页面和页面级设计契约 |
| [workbench-design.md](workbench-design.md) | 前端工作台目标、页面结构、组件分层、状态和图表策略 |
| [ui-ux-constraints.md](ui-ux-constraints.md) | 按页面族定义的高密度工作台 UI/UX 硬约束和验收检查 |

## 任务必读

| 任务类型 | 必读文档 |
|----------|----------|
| 前端代码生成或修改 | [frontend-tech-architecture.md](frontend-tech-architecture.md) + [frontend-code-quality-rules.md](frontend-code-quality-rules.md) + [frontend-ai-code-generation-rules.md](frontend-ai-code-generation-rules.md) |
| 页面结构、路由、状态 | [page-hierarchy-design.md](page-hierarchy-design.md) + [workbench-design.md](workbench-design.md) |
| 第一阶段全量页面设计 | [frontend-page-design-v1.md](frontend-page-design-v1.md) + [page-hierarchy-design.md](page-hierarchy-design.md) + [ui-ux-constraints.md](ui-ux-constraints.md) |
| UI/UX、响应式、表格、图表 | [ui-ux-constraints.md](ui-ux-constraints.md) + [workbench-design.md](workbench-design.md) |
| 页面 mockup 生成 | [mockup-prompt-workflow.md](mockup-prompt-workflow.md) |

## 变更原则

- 前端可以快速迭代页面、组件、交互和图表表达。
- 前端技术基线为 React + TypeScript，架构采用 [Feature-Sliced Analytical Workbench Architecture](frontend-tech-architecture.md)。
- 前端包管理器统一使用 pnpm。
- 前端代码生成和修改必须遵守 [前端代码质量规则](frontend-code-quality-rules.md)；AI 生成或修改前端代码还必须遵守 [前端 AI 代码生成规则](frontend-ai-code-generation-rules.md)。
- 页面 mockup 生成必须先按 [mockup-prompt-workflow.md](mockup-prompt-workflow.md) 完成提示词审阅，确认后再生成图片；图片确认后再固化具体视觉风格。
- 前端 API 调整需要同步后端设计中的 API 契约。
- 不改变架构设计中定义的能力域和数据边界。
- 新页面或大改页面必须按 [ui-ux-constraints.md](ui-ux-constraints.md) 提交页面交付说明，明确 URL 状态契约、数据可信状态、权限脱敏、查询预算和例外豁免。
- 第一阶段页面设计、次级页面和独立页面以 [frontend-page-design-v1.md](frontend-page-design-v1.md) 为页面设计契约；实现时不得只按 route 名称或后端服务机械生成页面。
- 前端功能切片必须同步更新 [前后端同步设计](../architecture-design/frontend-backend-sync-design.md) 中的契约和验收。
