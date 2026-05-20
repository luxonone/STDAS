# AI 代码生成治理机制

本文定义 STDAS 使用 AI 生成或修改代码时的项目级治理流程，重点解决：

- Rust 后端代码容易被生成成“能编译但不优雅、不符合所有权/错误/异步边界”的实现。
- React + TypeScript 前端代码容易把状态、API、组件和副作用混在一起。
- 当用户提出的实现路径偏离项目最佳实践时，AI 必须先提醒风险、提出替代方案；若用户仍要求原路径，必须在安全和正确性允许的范围内收敛实现，并显式标记待优化项。

本文不替代语言和框架细则：

- Rust 代码细则见 [Rust 代码质量规则](../backend-design/rust-code-quality-rules.md)、[Rust AI 代码生成规则](../backend-design/rust-ai-code-generation-rules.md) 和 [Rust 高质量项目参考与模式](../backend-design/rust-reference-projects-and-patterns.md)。
- 前端代码细则见 [前端代码质量规则](../frontend-design/frontend-code-quality-rules.md) 和 [前端 AI 代码生成规则](../frontend-design/frontend-ai-code-generation-rules.md)。

## 依据优先级

AI 生成代码或提出方案时，依据优先级从高到低为：

1. STDAS 项目文档、现有源码、API 契约、功能切片和 ADR。
2. 官方或一手技术文档，例如 Rust Book、Rust API Guidelines、rustfmt、Clippy、Tokio、axum、SQLx、thiserror、tracing、React、TypeScript、Vite。
3. 成熟开源 Rust 项目和官方示例的工程组织方式，例如 tokio-rs、Meilisearch、Qdrant；具体抽取方式见 [Rust 高质量项目参考与模式](../backend-design/rust-reference-projects-and-patterns.md)。它们只能作为参考，不得直接覆盖 STDAS 架构边界。
4. 本地验证结果，包括 `cargo check`、`cargo fmt`、`cargo clippy`、`cargo test`、`pnpm lint`、`pnpm typecheck`、`pnpm test`、`pnpm build`。

博客、问答、论坛和二手总结可以帮助理解问题，但不能单独成为项目规则来源。若要写入规则，必须能被项目文档或一手资料支撑。

## 触发条件

以下情况必须执行本文流程：

- 新增或修改非平凡 Rust 后端代码。
- 新增或修改 React + TypeScript 页面、组件、状态、API client、表格、图表、表单或前端测试。
- 调整服务边界、API 契约、数据模型、缓存、事件、权限、观测性或部署方式。
- 引入新依赖、新抽象、新 trait、新宏、新全局状态、新后台任务或新异步并发模型。
- 用户提出的实现路径可能绕开项目规则、官方最佳实践或长期架构边界。
- AI 自身因为 Rust、异步、生命周期、所有权、前端状态或 API 契约不确定而无法可靠判断实现质量。

## 强制流程

每次触发时，AI 必须按以下顺序执行：

1. **定位事实源**：读取 `docs/README.md`、[AI Agent 运行规则](ai-agent-runtime-rules.md)、相关 section README 和任务相关 topic 文档。
2. **识别偏航点**：判断用户要求是否会造成所有权设计混乱、错误类型不稳定、异步边界不清、API 契约漂移、状态重复、组件职责过大、依赖膨胀、测试缺失或安全/权限风险。
3. **先提醒再编码**：如果存在偏航风险，编码前必须说明风险、对应项目规则、推荐替代方案和取舍。
4. **采用最佳路径**：如果用户接受建议，按项目规则和语言/框架最佳实践实现。
5. **受限执行用户路径**：如果用户仍要求原路径，只有在不破坏安全、权限、数据正确性、API 兼容性和可验证性的前提下才能实现。
6. **标记待优化项**：对已经明确偏离最佳实践但暂时被接受的代码，必须添加 `TODO(AI-OPTIMIZE, ...)` 标记，并在最终回复中列出。
7. **验证和汇报**：执行与变更风险匹配的检查；若是文档阶段或环境缺失导致未执行代码验证，必须说明原因。

如果用户要求会直接破坏安全、权限隔离、数据正确性、审计要求或可恢复性，AI 不得用 `TODO(AI-OPTIMIZE, ...)` 掩盖问题，必须要求改方案或缩小范围。

## 偏航提醒模板

当需要提醒用户时，使用简短、具体的结构：

```text
这里有一个实现风险：<具体风险>。
项目规则/一手实践要求：<对应规则或来源>。
建议改为：<推荐方案>。
取舍：<成本和收益>。
如果仍采用原方案，我会把实现限制在 <边界>，并用 TODO(AI-OPTIMIZE, ...) 标记后续优化点。
```

提醒必须聚焦当前任务，不得泛泛说“更优雅”或“最佳实践”。必须说明具体风险，例如“在 async handler 中持有互斥锁跨 `.await` 会造成阻塞和死锁风险”“把服务端查询状态复制到多个前端 store 会引入不一致”。

## 待优化标记

待优化标记格式固定为：

```rust
// TODO(AI-OPTIMIZE, area=rust, date=2026-05-20): <当前方案为什么不是最佳实践>; preferred: <目标方案>; revisit when: <触发条件>.
```

前端使用同样格式：

```ts
// TODO(AI-OPTIMIZE, area=frontend, date=2026-05-20): <当前方案为什么不是最佳实践>; preferred: <目标方案>; revisit when: <触发条件>.
```

规则：

- `area` 只能使用 `rust`、`frontend`、`api`、`data`、`security`、`observability`、`test`。
- 标记必须靠近产生技术债的代码。
- 标记必须说明当前方案的非理想点、推荐目标和重新处理条件。
- 标记不能替代测试、错误处理、权限校验、数据校验或 API 兼容性。
- 标记不能用于长期架构分歧。长期分歧必须进入 ADR 或对应设计文档。
- 最终回复必须列出新增的 `TODO(AI-OPTIMIZE, ...)` 及其原因。

## AI 自检清单

编码前必须至少确认：

- 变更是否属于架构、后端、前端、领域、API 或跨端功能切片。
- 是否已经读取对应源文档。
- 是否需要先更新契约或设计文档。
- 是否有更简单的官方或项目内既有模式可复用。
- 是否引入了用户难以审查的 Rust 复杂度，例如无必要的 trait/generic/macro/lifetime gymnastics。
- 是否引入了用户难以审查的前端复杂度，例如重复状态、隐式副作用、过大组件或临时 `any`。

编码后必须至少确认：

- Rust 代码没有生产路径 `unwrap`/`expect`、临时 `String` 错误、无意义 clone、跨 `.await` 持锁、无边界后台任务、无解释的新依赖。
- 前端代码没有绕开 `shared/api`、没有 `any`/`as any`/非空断言兜底、没有错误的 Hook 依赖、没有重复服务端状态、没有缺失 loading/empty/error/permission 状态。
- API 契约、错误码、权限、数据版本和 query snapshot 与文档一致。
- 需要的验证命令已经执行或明确说明未执行原因。

## 验证要求

代码变更的验证以相关 section 文档为准。默认基线：

- Rust 后端：`cargo fmt --check`、`cargo check`、`cargo clippy --all-targets --all-features -- -D warnings`、相关 `cargo test`。
- 前端：`pnpm lint`、`pnpm typecheck`、相关 `pnpm test`、必要时 `pnpm build`。
- 跨端契约：同时检查后端 API 文档、前端 API client、页面状态、权限/脱敏和测试。
- 文档-only 变更：允许记录“未执行代码验证，原因：文档阶段不涉及构建”，但必须检查入口文档和交叉链接一致。
