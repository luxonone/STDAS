# AI Agent 运行时规则

本文件定义 AI Agent 在 STDAS 项目中修改文档、设计或代码前的工作方式。它只规定读取顺序、边界判断和交付检查，不重复前端、后端或架构文档中的具体规则。

## Codex 启动必读

Codex 或其他 AI Agent 每次进入 STDAS 工作区并准备修改文档、设计或代码前，必须按顺序读取：

1. [docs/README.md](../README.md)
2. [AI Agent 运行时规则](ai-agent-runtime-rules.md)
3. 与任务相关的分区 README。
4. 与任务相关的专题文档。

涉及代码生成、实现路径评审、偏航提醒或待优化标记时，还必须读取 [AI 代码生成治理机制](ai-code-generation-governance.md)。

最低读取要求：

| 任务范围 | 额外必读 |
|----------|----------|
| 领域或业务语义 | [领域知识 README](../domain-knowledge/README.md)、[STDAS 背景知识总览](../domain-knowledge/stdas-domain-primer.md) |
| 架构、服务边界、ADR | [架构设计 README](README.md)、[系统架构](system-architecture.md)、[ADR 目录](adr/README.md) |
| 前端页面或代码 | [前端设计 README](../frontend-design/README.md)、[前端技术架构](../frontend-design/frontend-tech-architecture.md)、[前端代码质量规则](../frontend-design/frontend-code-quality-rules.md)、[前端 AI 代码生成规则](../frontend-design/frontend-ai-code-generation-rules.md) |
| UI/UX、表格、图表、工作台体验 | [UI/UX 约束](../frontend-design/ui-ux-constraints.md)、[前端工作台设计](../frontend-design/workbench-design.md) |
| 后端 Rust 代码 | [后端设计 README](../backend-design/README.md)、[Rust Workspace 与服务边界](../backend-design/workspace-and-crates.md)、[Rust 代码质量规则](../backend-design/rust-code-quality-rules.md)、[Rust AI 代码生成规则](../backend-design/rust-ai-code-generation-rules.md)、[Rust 高质量项目参考与模式](../backend-design/rust-reference-projects-and-patterns.md) |
| API、数据、查询、缓存、事件、安全 | [后端设计 README](../backend-design/README.md) 以及对应专题文档 |
| 功能切片端到端交付 | [前后端同步设计](frontend-backend-sync-design.md)、[首批功能切片 V1](feature-slices-v1.md) |

如果当前任务只询问事实或做轻量说明，也必须至少以 [docs/README.md](../README.md) 和本文件作为上下文；如果涉及实现或修改，则必须继续读取任务相关专题文档。

## 单一事实来源

AI Agent 必须按任务类型读取对应权威文档，不能在本文件重新解释或改写这些规则。

| 任务类型 | 必读文档 | 说明 |
|----------|----------|------|
| 架构边界、能力域、长期约束 | [architecture-design/README.md](README.md) | 架构目录是系统级边界和 ADR 的事实来源 |
| AI 代码生成偏航提醒、替代方案、待优化标记 | [AI 代码生成治理机制](ai-code-generation-governance.md) | 只定义跨技术栈执行流程，不复制 Rust 或前端具体规则 |
| 页面、交互、状态、可访问性 | [frontend-design/README.md](../frontend-design/README.md)、[UI/UX 约束](../frontend-design/ui-ux-constraints.md) | UI/UX 规则只在前端设计中维护 |
| 前端技术架构和 React/TypeScript 代码生成 | [前端技术架构](../frontend-design/frontend-tech-architecture.md)、[前端代码质量规则](../frontend-design/frontend-code-quality-rules.md)、[前端 AI 代码生成规则](../frontend-design/frontend-ai-code-generation-rules.md) | 前端实现规则只在前端设计中维护 |
| API 字段、枚举、错误、分页、版本兼容 | [backend-design/README.md](../backend-design/README.md)、[API 契约规则](../backend-design/api-contract-rules.md) | API 契约只在后端设计中维护 |
| Rust 代码生成、所有权、错误处理、异步和测试 | [Rust 代码质量规则](../backend-design/rust-code-quality-rules.md)、[Rust AI 代码生成规则](../backend-design/rust-ai-code-generation-rules.md)、[Rust 高质量项目参考与模式](../backend-design/rust-reference-projects-and-patterns.md) | Rust 实现规则只在后端设计中维护 |
| QuerySnapshot、查询预算、Evidence | [分析引擎](../backend-design/analytics-engine.md) | 分析语义和执行边界以后端设计为准 |
| 权限、脱敏、任务状态机、可靠性 | [安全与可靠性](../backend-design/security-reliability.md) | 安全和任务生命周期以后端设计为准 |
| 前后端同步推进 | [前后端同步设计](frontend-backend-sync-design.md) | 只定义跨目录协作方式，不承载具体规则副本 |

如果发现两个文档对同一规则给出不同定义，必须先指出冲突并收敛到对应事实来源，再继续修改。

## 工作原则

- 先判断任务属于架构、前端、后端、代码实现、验证修复中的哪一类。
- 架构设计高稳定，除非用户明确要求，不主动改变服务边界、数据边界和 ADR。
- 前端和后端按功能切片同步推进；同一切片必须能追溯到页面、API、数据、权限、状态和验收。
- 新增或修改规则时，必须放入对应事实来源文档，不在多个文档中复制规则正文。
- 涉及代码生成、实现路径选择、偏航提醒或待优化标记时，必须读取 [AI 代码生成治理机制](ai-code-generation-governance.md)。
- 生成或修改 Rust 代码前，必须读取 [Rust 代码质量规则](../backend-design/rust-code-quality-rules.md) 和 [Rust AI 代码生成规则](../backend-design/rust-ai-code-generation-rules.md)；非平凡后端实现还必须读取 [Rust 高质量项目参考与模式](../backend-design/rust-reference-projects-and-patterns.md)。
- 生成或修改前端代码前，必须读取 [前端技术架构](../frontend-design/frontend-tech-architecture.md)、[前端代码质量规则](../frontend-design/frontend-code-quality-rules.md) 和 [前端 AI 代码生成规则](../frontend-design/frontend-ai-code-generation-rules.md)。
- 项目开发优先采用对应技术的官方或主流推荐架构，不自创难维护的非常规结构。
- 任务必须拆小，排查范围必须足够小、足够可验证。

## 修改前检查

修改前必须回答：

1. 本次修改属于哪个目录的事实来源？
2. 是否需要同步更新前端、后端或架构的引用关系？
3. 是否改变了已确认的架构边界或 ADR？
4. 是否引入了新的 UI/API/数据/权限规则？如果是，是否写入了对应权威文档？
5. 是否存在与现有规则重复或冲突的描述？

## 文档修改规则

- 修改前先读取当前目录的 `README.md`。
- 新增文档必须从对应目录的 `README.md` 链接。
- `docs/README.md` 只作为入口，不承载详细设计。
- 架构层不记录临时实现细节。
- 前端/后端文档可以高频演进，但不得反向破坏架构原则。
- 跨域规则应写为引用、契约和交付检查，不复制前端或后端规则正文。
- 删除或迁移文档时，必须全局检查旧路径引用。

## 前后端同步规则

功能切片的同步方式以 [前后端同步设计](frontend-backend-sync-design.md) 为准。AI Agent 只需要确认切片是否同时覆盖以下交付面：

- 前端入口和页面状态。
- 后端 API、请求、响应和错误。
- 数据对象、版本、快照和审计。
- 权限、CustomerScope、脱敏和数据隐藏。
- 同步/异步策略。
- 验收方式。

具体 UI/UX 规则必须链接到 [UI/UX 约束](../frontend-design/ui-ux-constraints.md)，具体 API 规则必须链接到 [API 契约规则](../backend-design/api-contract-rules.md)。

## 验证 Gate

进入代码实现前，必须确认本地验证闭环可运行：

```text
写代码 -> 编译/检查 -> 读取错误 -> 修复 -> 再验证
```

最低检查项：

- Rust toolchain 可用。
- 前端包管理器和构建命令可用。
- 后端 cargo check/test 可运行。
- 前端 lint/typecheck/build 可运行。
- 本地配置样例可加载。
- 测试失败时错误输出可被 AI Agent 捕获。

如果当前任务只修改文档，可以明确记录“未执行代码验证，原因：文档阶段不涉及构建”。

## 小步交付

任务默认拆成：

1. 骨架。
2. 主链路。
3. 编辑流。
4. 优化体验。
5. 回归验证。

每个 milestone 必须可以独立验收；具体切片拆分以 [前后端同步设计](frontend-backend-sync-design.md) 为准。
