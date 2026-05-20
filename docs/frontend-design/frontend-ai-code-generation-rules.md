# 前端 AI 代码生成规则

本文是 AI 生成或修改 STDAS React + TypeScript 前端代码时必须执行的专项规则。它补充 [前端代码质量规则](frontend-code-quality-rules.md)，并与 [AI 代码生成治理机制](../architecture-design/ai-code-generation-governance.md) 共同约束偏航提醒、替代方案和待优化标记。

目标是让前端代码保持可读、可测试、状态边界清楚，并符合 STDAS 高密度分析工作台的交互要求。

## 一手依据

生成前端方案时优先参考：

- React 官方文档：Thinking in React、Rules of Hooks、状态和副作用边界。
- TypeScript 官方 Handbook：类型收窄、对象类型、联合类型、泛型和类型安全。
- Vite 官方文档：本地开发、构建、环境变量和前端工具链边界。
- STDAS 前端技术架构、前端代码质量规则、UI/UX 约束、页面层级和前后端同步设计。

## 编码前设计

写代码前必须先回答：

- **数据来源**：数据来自 URL、server/API、workspace state、local UI state 还是权限/脱敏状态。
- **API 边界**：是否通过 `shared/api` 的 typed client；DTO 是否需要转换成 view model。
- **组件边界**：页面、widget、feature、entity、shared component 是否符合依赖方向。
- **状态边界**：是否存在重复 server state、派生 state、隐式全局状态或可由 URL 表达却放在本地的状态。
- **副作用边界**：`useEffect` 是否只用于外部同步；依赖是否完整；是否处理 abort、race、cleanup。
- **交互状态**：loading、empty、error、permission denied、redacted、stale、refreshing、partial data 是否完整。
- **性能边界**：表格、图表、筛选、搜索、导出和高频交互是否有预算、虚拟化或节流策略。
- **可访问性**：键盘、焦点、label、语义元素、错误提示和颜色对比是否可用。

这些问题回答不清楚时，必须先补设计或提醒风险，不能直接堆组件。

## 推荐模式

- 按 Feature-Sliced Analytical Workbench Architecture 放置代码：`app`、`pages`、`widgets`、`features`、`entities`、`shared` 依赖方向不可反转。
- 服务端数据通过 typed API client 获取；前端组件不直接 `fetch` 后端。
- DTO 与 view model 分离；转换逻辑放在靠近 API 或 entity/feature 边界的位置。
- UI 状态使用明确有限状态，而不是多个布尔值互相推断。
- 派生值通过计算得到，不复制到 state；真正需要缓存时说明失效策略。
- 表单、筛选、查询参数和 URL 状态保持明确同步规则。
- 页面组件负责组织，复杂交互下沉到 feature/widget，基础展示下沉到 shared/entity。
- 图表、表格和 wafer/map 类组件必须有稳定尺寸、空态、错误态、加载态和权限态。
- 用户可见文案描述业务状态和操作结果，不展示实现说明、代码规则或快捷键教程。

## 需要提醒用户的偏航模式

遇到下列模式时，AI 必须先提醒风险并提出替代方案：

- 使用 `any`、`as any`、非空断言或宽泛 `Record<string, unknown>` 绕开类型问题。
- 在组件内直接 `fetch`，绕开 `shared/api`、统一错误、request id、权限和脱敏处理。
- 把 server state 同时放进多个 store 或 local state，造成刷新和权限状态不一致。
- 用 `useEffect` 计算派生状态，或故意省略 Hook 依赖。
- 页面组件同时处理 API、权限、表格、图表、表单、导出和复杂交互。
- 为简单交互引入新状态管理库、UI 库或工具库。
- 为了赶页面跳过 loading、empty、error、permission denied、redacted、stale 状态。
- 图表和表格没有数据规模预算、虚拟化或取消策略。
- 前端字段、枚举或默认值与后端 API 契约不一致。
- 用 UI 便利性反推后端 API 或领域模型。

若用户不采纳替代方案，只能在不破坏正确性、权限和契约的范围内实现，并按项目治理规则添加 `TODO(AI-OPTIMIZE, area=frontend, ...)`。

## 禁止兜底

以下问题不能用 `TODO(AI-OPTIMIZE, ...)` 延后：

- 权限、脱敏、客户隔离或敏感字段处理缺失。
- API 错误被吞掉，用户无法区分无权限、无数据、加载失败和过期数据。
- 类型断言导致运行时字段缺失、枚举漂移或错误渲染。
- Hook 规则或依赖错误导致数据不同步或无限请求。
- 表单可能提交非法或越权数据。
- 缺少验证导致核心页面无法证明可用。

## 输出要求

完成前端变更后，最终回复必须说明：

- 关键设计选择，例如数据来源、状态归属、API 边界、组件拆分。
- 是否新增 `TODO(AI-OPTIMIZE, ...)`，以及为什么存在。
- 已执行的验证命令；若未执行，说明具体原因。

