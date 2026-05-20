# 前端代码质量规则

本文件是 STDAS 前端代码生成和修改的事实来源。AI Agent、开发者和代码审查都应以本文件约束 React + TypeScript 实现；UI/UX 规则以 [UI/UX 约束](ui-ux-constraints.md) 为准，技术架构以 [前端技术架构](frontend-tech-architecture.md) 为准。

> AI 生成或修改前端代码还必须读取 [前端 AI 代码生成规则](frontend-ai-code-generation-rules.md)；偏航提醒、替代方案和 `TODO(AI-OPTIMIZE, ...)` 机制见 [AI 代码生成治理机制](../architecture-design/ai-code-generation-governance.md)。

## 目标

生成或修改的代码必须符合 React + TypeScript 工程最佳实践，并能支撑 STDAS 半导体测试数据分析工作台。代码应正确、清晰、类型安全、可维护、可访问、安全，并遵循当前项目已有风格。

优先级：

1. 正确性。
2. 用户体验。
3. 可访问性。
4. 安全性。
5. 类型安全。
6. 可维护性。
7. 性能。
8. 可测试性。
9. 简洁性。

不要为了炫技写复杂代码。不要为了小需求引入大抽象。不要为了“看起来高级”牺牲可读性。

核心原则：

- 用 TypeScript 约束数据。
- 用 React 组件表达界面。
- 用 hooks 管理副作用和复用逻辑。
- 用明确状态表达 UI 流程。
- 用语义化 HTML 保证可访问性。
- 用测试覆盖关键用户行为。
- 用 STDAS 语义组件表达 LotNo、TestType、TestStation、LotEndTime、QuerySnapshot、lot_scope、权限和数据可信状态。

## 项目一致性

- 必须遵循当前项目已有 React 版本、目录结构、组件模式、状态管理方式、样式方案和测试工具。
- STDAS 前端包管理器统一使用 pnpm；依赖安装和脚本执行使用 `pnpm install`、`pnpm lint`、`pnpm typecheck`、`pnpm test`、`pnpm build`。
- 不随意引入新的 UI 库、状态管理库、请求库、表单库、路由库或构建工具。
- 不随意改变公共组件 API、路由结构、数据结构、API client 或状态模型。
- 不为了小需求做大规模重构。
- 不修改与任务无关的文件。
- 不删除已有测试，除非测试明显错误并替换为更正确的测试。
- 修改应保持最小、聚焦、可回滚。
- 如果项目已有封装好的组件、hook、API client、store、样式 token，应优先复用。
- 不假设不存在的组件、hook、store、API 或工具函数。

## TypeScript

- 优先使用 TypeScript 明确表达数据结构。
- 避免使用 `any`。
- 无法确定类型时，优先使用 `unknown`，再进行类型收窄。
- 不使用 `as any`。
- 不使用 `as unknown as Xxx` 掩盖类型问题。
- 不滥用类型断言。
- 不滥用非空断言 `!`。
- 组件 props、hook 参数、API 返回值、状态结构、事件处理函数都应有明确类型。
- 不为了省事把所有字段都写成 optional。
- 不让类型定义和真实数据结构脱节。
- 有限状态必须使用 union type，不使用随意字符串。
- 复杂状态优先使用 discriminated union。
- 外部 API DTO 和前端内部 view model / domain model 必须区分。
- URL 参数、localStorage、sessionStorage、表单输入、后端响应都应视为不可信数据。

优先：

```ts
type UserStatus = "active" | "disabled" | "pending";

interface User {
  id: string;
  name: string;
  status: UserStatus;
}
```

避免：

```ts
interface User {
  id: any;
  name: any;
  status: string;
}
```

STDAS 状态优先使用 discriminated union：

```ts
type ResultState =
  | { kind: "loading"; scope: string }
  | { kind: "ready"; querySnapshotId: string }
  | { kind: "partial"; querySnapshotId: string; reason: string }
  | { kind: "permission-denied"; reason: string }
  | { kind: "error"; code: number; message: string };
```

## React 组件

- 优先使用函数组件。
- 不新增 class component，除非项目已有强约束。
- 组件职责单一。
- 不让一个组件同时负责页面布局、数据请求、复杂业务逻辑、表单校验、弹窗控制和状态管理。
- 大组件应拆分为容器组件、展示组件、业务 hook 或工具函数。
- 展示组件应尽量通过 props 接收数据。
- 组件 props 应清楚、稳定、语义明确。
- 不设计过于庞大的 props。
- 不把大量无关 props 传给组件。
- 不滥用 boolean props 组合复杂状态。
- 多状态 UI 优先使用明确状态类型，而不是多个布尔值。
- 不在 render 阶段触发副作用。
- 不在 render 阶段调用 `setState`。
- 不在组件顶层写复杂业务计算；复杂计算应提取为函数或 hook。
- 不在 JSX 中写过长、难读的条件表达式。
- 条件渲染必须清楚表达 loading、error、empty、success、permission、partial 等状态。
- route/page 组件只做页面组合、URL 状态接入和错误边界。
- 不把后端 DTO 原样传遍整个组件树；需要时转换为 view model。

优先：

```ts
type ButtonVariant = "primary" | "secondary" | "danger";

interface ButtonProps {
  variant?: ButtonVariant;
  loading?: boolean;
  disabled?: boolean;
  onClick?: () => void;
}
```

避免：

```ts
interface ButtonProps {
  isPrimary?: boolean;
  isSecondary?: boolean;
  isDanger?: boolean;
  isLoading?: boolean;
}
```

## Hooks

- 必须遵守 Rules of Hooks。
- 不在条件语句、循环、嵌套函数中调用 hook。
- 自定义 hook 必须以 `use` 开头。
- hook 应表达清楚的复用逻辑。
- 不为了“抽象”创建无意义 hook。
- `useEffect` 只能用于副作用或与外部系统同步，不能用于简单派生状态。
- `useEffect` 必须声明正确依赖。
- 不为了消除 lint warning 随意删除依赖。
- 不使用空依赖数组隐藏真实依赖。
- `useEffect` 中的订阅、计时器、事件监听、请求等必须清理。
- 异步请求应处理竞态条件。
- 组件卸载后不应继续更新状态。
- 不滥用 `useMemo`、`useCallback`、`React.memo`。
- 只有存在明确性能收益、引用稳定性需求、依赖数组需求或 memoized child 需求时，才使用 memoization。

不推荐：

```tsx
useEffect(() => {
  setFullName(`${firstName} ${lastName}`);
}, [firstName, lastName]);
```

推荐：

```tsx
const fullName = `${firstName} ${lastName}`;
```

## 状态管理

- 状态应尽量靠近使用它的地方。
- 能用局部 state 解决的，不放进全局 store。
- 能从已有 state 计算出来的，不重复存储。
- 不复制服务端数据到本地 state，除非确实需要编辑草稿、乐观更新或本地快照。
- URL 相关状态优先放在 URL 中，例如搜索条件、筛选项、分页、排序、query snapshot 引用。
- 表单临时状态可以放在组件或表单库中。
- 跨多个远距离组件共享的状态，才考虑全局状态。
- 不让全局 store 变成业务逻辑垃圾桶。
- 不直接修改 state。
- 更新数组和对象时必须使用不可变更新。
- 状态命名应表达业务含义。
- 多个互斥状态不要用多个 boolean 表达，优先使用 union。

STDAS 状态必须分层：

| 状态类型 | 示例 | 规则 |
|----------|------|------|
| URL State | customer、product、test_type、test_station、lot_end_time_range、page、sort、filters、query_snapshot_id | 可刷新、分享、back/forward 恢复 |
| Server State | Lot、DataVersion、QuerySnapshot、Job、Options、Permission State | 由 API client / query layer 管理 |
| Workspace State | lot_scope、DataVersion policy、layout、unsaved changes、chart selections、draft notes | 表达退出保护和保存目标 |
| Local UI State | drawer open、tab、hover、temporary menu | 留在组件局部 |
| Permission / Redaction State | hidden、masked、unauthorized、not_found、disabled reason | 显式展示，不能只隐藏 |

推荐：

```ts
type DialogState =
  | { type: "closed" }
  | { type: "confirm-delete"; userId: string }
  | { type: "edit-user"; userId: string };
```

避免：

```ts
const [isDeleteOpen, setIsDeleteOpen] = useState(false);
const [isEditOpen, setIsEditOpen] = useState(false);
const [selectedUserId, setSelectedUserId] = useState<string | null>(null);
```

## 异步数据与 API

- 必须处理 loading、error、empty、success 状态。
- 不只实现 happy path。
- 不假设请求一定成功。
- 不吞掉错误。
- 不只在 console 中打印错误。
- 用户可见错误信息应清楚，但不暴露内部实现细节。
- 请求逻辑优先放在已有 API client、service、query function 或 custom hook 中。
- 不在多个组件里散落重复 fetch 逻辑。
- 不在循环中无控制地发起大量请求。
- 不在前端硬编码 token、密钥、私钥或数据库连接信息。
- 请求参数应进行必要编码。
- 用户输入、URL 参数、localStorage 数据不能直接信任。
- 对重复提交、竞态请求、快速切换页面要有处理。
- 如果项目已有 TanStack Query、SWR、Relay、Apollo 或其他数据请求方案，必须沿用已有方案。
- 不为了一个简单请求新增请求库。

STDAS API 规则：

- 页面、widget、feature 不得散写 raw `fetch`。
- 所有请求必须走 `shared/api` typed client。
- API client 必须统一处理响应信封、错误码、权限状态、redaction state、request id、query id、query hash、query snapshot id 和 abort。
- 前端只访问 `stdas-gateway` 的 `/api/v1/*`，不得调用内部服务地址。
- 高成本查询必须显式触发，不隐式随每个输入变化运行。
- 修改筛选、DataVersion policy、lot_scope 或时间范围后，旧响应不得覆盖新状态。
- Options API 必须处理 loading、empty、error、permission denied、deprecated、hidden、unauthorized、not_found。

推荐 async state：

```ts
type AsyncState<T> =
  | { status: "idle" }
  | { status: "loading" }
  | { status: "success"; data: T }
  | { status: "error"; error: string };
```

## 表单和 Options

- 表单必须处理输入、校验、提交、loading、error、success 状态。
- 提交中应禁用提交按钮，防止重复提交。
- 表单字段必须有 label。
- 错误信息应靠近对应字段。
- 错误信息应可被屏幕阅读器感知。
- 必填字段应明确标识。
- 不只依赖前端校验；前端校验是用户体验，不是安全边界。
- 提交前应 trim 或 normalize 输入，除非业务不允许。
- 密码、token、验证码、密钥等敏感字段不能打印到日志。
- 文件上传必须处理类型、大小、失败状态和进度状态。
- 不在用户输入过程中频繁打断。
- 不在每个 key stroke 都发昂贵请求，必要时使用 debounce。
- 表单库应遵循项目已有选择，不随意新增。

STDAS 结构化输入规则：

- 日期、时间、枚举、单位、范围、客户、产品、Lot、Bin、参数等字段默认使用结构化控件。
- 表单默认值必须来自 API 契约或页面契约，前端不能猜。
- 字段级错误、页面级错误和权限错误必须区分。
- 长表单必须分组并保证提交/取消操作可达。
- 上游字段变化后，下游 options 必须重新加载并校验已选值。
- deprecated、hidden、unauthorized、not_found 的已选值不得静默清空。

## JSX 与渲染

- JSX 应保持清晰。
- 不在 JSX 中写复杂业务逻辑。
- 不在 JSX 中写多层嵌套三元表达式。
- 列表渲染必须使用稳定 key。
- 不使用数组 index 作为 key，除非列表永远不会重排、插入或删除。
- 条件渲染应覆盖 loading、error、empty 和正常状态。
- 不返回空白页面。
- 不让用户在异常情况下不知道下一步做什么。
- 重复 JSX 应提取为小组件或渲染函数，但不要过度拆分。
- 事件处理函数应命名清楚。
- props 回调命名使用 `onXxx`。
- 组件内部事件处理函数命名使用 `handleXxx`。

## 可访问性和 HTML 语义

- 优先使用语义化 HTML。
- 操作按钮必须使用 `<button>`。
- 页面跳转必须使用 `<a>` 或项目路由 Link 组件。
- 不用 `<div>` 或 `<span>` 模拟按钮。
- 所有交互元素必须支持键盘操作。
- 表单控件必须有 label。
- 图片必须有合适的 alt；装饰性图片使用空 alt。
- 不移除 focus outline，除非提供同等清晰的 focus 样式。
- 不只依赖颜色表达状态。
- loading 状态应有可理解文本或 aria 提示。
- 图标按钮必须有 `aria-label` 或 accessible name。
- 模态框应管理焦点，关闭后恢复焦点。
- 下拉框、菜单、弹窗、toast 应考虑键盘和屏幕阅读器体验。
- 动画应尊重 reduced motion 偏好。
- 颜色对比度应足够。
- 页面主体应使用合理的 `main`、`section`、`article`、`header`、`footer`、`nav`。
- 表单提交按钮必须设置 `type="submit"`。
- 普通按钮必须设置 `type="button"`，避免在表单中误触发提交。
- 外部链接新窗口打开时必须使用 `rel="noopener noreferrer"`。
- 表格数据应使用 table 或具备等价语义的数据网格，不用无语义 div 模拟复杂表格。
- 文本层级应使用合理标题结构，不为样式乱用 heading。

推荐：

```tsx
<button type="button" aria-label="Close dialog" onClick={onClose}>
  <CloseIcon aria-hidden="true" />
</button>
```

## 样式

- 必须遵循项目已有样式方案。
- 不混用多个样式体系。
- 不随意引入新的 UI 组件库。
- 颜色、字号、间距、圆角、阴影应优先使用设计系统 token。
- 不写大量魔法尺寸。
- 避免全局样式污染。
- 避免滥用 `!important`。
- 避免过深选择器。
- 样式命名应清楚。
- 页面必须考虑响应式。
- 不只适配单一屏幕尺寸。
- 长文本、长单词、长 URL 应考虑换行。
- 弹窗、菜单、tooltip、dropdown 应考虑视口边界。
- 不让重要操作在小屏不可见或不可点击。
- STDAS 不使用通用 Admin Template 作为产品结构。

## 安全

- 不使用 `dangerouslySetInnerHTML`，除非内容已可信且经过严格清洗。
- 不直接插入未信任 HTML。
- 不绕过 React 默认 XSS 防护。
- 不在前端硬编码密钥、私钥、token、cookie、数据库连接串。
- 不把敏感信息打印到 console。
- 不把敏感信息放进 URL query。
- 不信任用户输入、URL 参数、localStorage/sessionStorage、第三方脚本返回值。
- URL、路径、query 参数应进行必要编码。
- 权限控制不能只依赖前端隐藏按钮，后端必须校验。
- 不在前端暴露不必要的内部字段。
- 外链打开新窗口时必须使用 `rel="noopener noreferrer"`。
- 错误信息不暴露内部栈、SQL、token、服务端路径或敏感配置。

STDAS 权限与脱敏规则：

- 前端不得用隐藏 UI 作为安全边界。
- API 返回 hidden、masked、unauthorized、not_found 时，前端必须按状态展示。
- 分享链接打开时，不能在无权限状态下展示敏感查询摘要。
- 导出下载、workspace、case、QuerySnapshot 页面必须处理权限变化。
- 禁用操作必须解释原因，例如无权限、状态不允许、缺少必填条件、任务运行中。

## 表格

- 大表格必须使用服务端分页、服务端排序、服务端筛选或虚拟滚动。
- 表格行必须有稳定业务 ID。
- Lot、Run、File、DataVersion、QuerySnapshot 等核心对象不得只用显示文本识别。
- 当前页选择、当前筛选结果选择、手动选择必须区分。
- 跨页批量操作必须显示影响范围和数量。
- permission、hidden、masked、not_found、partial、stale 状态必须可展示。
- 列配置、排序、分页和主筛选应可恢复。
- 表格组件必须通过 grid adapter 接入，不在页面内直接绑定第三方 grid API。

## 图表和位置分析

- 图表组件只负责渲染和交互，不负责 yield、bin、spec、DataVersion policy 或客户规则计算。
- 图表输入必须包含指标名称、单位、数据口径、时间范围、聚合粒度、DataVersion 或 QuerySnapshot 引用。
- 图表必须支持 loading、empty、error、partial data、stale、over budget 状态。
- brush、zoom、legend toggle、drilldown 必须有 reset 或返回入口。
- 图表和表格联动时，当前选择范围必须可见。
- WaferLot / WaferNo / X / Y 等位置分析能力不是 FT 第一阶段默认主视图；需要时必须通过 adapter 层封装，不能把坐标、颜色、权限和 drilldown 逻辑散落在页面里。
- 不得仅依赖颜色表达良品、异常、告警或超规格状态。
- 前端不得计算后端拥有的 yield、bin、spec、DataVersion policy、profile resolution 等业务语义。

## 性能

- 不过早优化。
- 先保证正确性和可读性。
- 避免明显不必要的重复渲染。
- 避免在 render 中执行昂贵计算。
- 大列表应分页、懒加载或虚拟列表。
- 大图片应设置合适尺寸，避免直接加载超大资源。
- 非首屏组件可以懒加载，但不要过度拆包。
- 不为小功能引入大型依赖。
- 不重复引入功能相似的工具库。
- 不滥用 `useMemo`、`useCallback`、`React.memo`。
- 性能优化必须有明确原因。
- 输入搜索、自动保存、远程校验等场景应考虑 debounce 或 throttle。
- 避免不必要的全局状态更新。
- 避免父组件频繁重渲染导致大面积子组件更新。
- 热路径中避免频繁创建大对象、大数组或复杂正则。
- STDAS 大表、大图、大导出、大查询必须走分页、聚合、采样、partial data 或后端异步任务。

## 错误、Loading、Empty、Disabled

- UI 必须有错误状态。
- 不静默失败。
- 不只写 `console.error`。
- 用户操作失败后应给出下一步行动，例如重试、返回、重新登录、联系支持。
- 请求错误、权限错误、资源不存在、网络错误应尽量区分。
- 关键页面应有 fallback UI。
- 必要时使用 Error Boundary。
- Error Boundary 应展示用户可理解的错误界面。
- 不把内部错误栈直接展示给用户。
- 错误上报应包含必要上下文，但不能包含敏感信息。
- 异步页面必须有 loading 状态。
- 数据为空必须有 empty state。
- 操作中必须有 disabled 或 loading feedback。
- loading 不应造成严重布局抖动。
- 不让用户面对空白页面。
- 不让用户重复点击危险操作。
- empty state 应说明当前没有数据，并尽量提供下一步操作。
- disabled 状态应有明确原因，必要时显示提示。
- success 状态应给出反馈，例如 toast、状态更新或页面变化。

STDAS 必须显式处理：

- loading。
- empty。
- error。
- permission denied。
- hidden / masked / unauthorized / not_found。
- partial data。
- stale。
- snapshot。
- over budget。
- async running。

## 路由

- 遵循项目已有路由方案。
- 路由参数必须校验。
- 页面不存在时显示 404 或合理 fallback。
- 权限不足时显示明确状态或跳转。
- 不在多个地方重复拼接复杂 URL。
- URL 中的状态应有清楚的编码、解析和默认值处理。
- 不把敏感数据放进 URL。
- 页面切换时应考虑 loading、error、scroll 和 focus。
- 对深链接刷新后的状态要可恢复。
- 搜索、筛选、分页、排序等页面状态优先放进 URL。
- STDAS 的 query_snapshot_id、workspace_id、job_id、export_id 等 URL 引用打开时必须重新做权限校验。

## 测试

- 新增核心逻辑时应添加测试。
- 修复 bug 时应添加回归测试。
- 测试关注用户可观察行为，而不是组件内部实现细节。
- 优先使用项目已有测试工具。
- React 组件测试应模拟真实用户行为。
- 表单、权限、错误状态、loading 状态、empty 状态应测试。
- 工具函数和数据转换函数应单元测试。
- 不只测试 happy path。
- 不过度依赖 snapshot。
- mock 应贴近真实场景。
- 测试名应描述行为。
- 测试不应依赖执行顺序。
- 异步测试应正确等待 UI 更新。
- 不为了测试暴露不必要的实现细节。

STDAS 必测点：

- URL state parse / serialize。
- API client 错误解析、权限状态和 abort / 过期响应。
- Options 级联和 deprecated / hidden / unauthorized 状态。
- 表格选择范围、跨页选择和批量操作。
- Analysis Workspace 的 QuerySnapshot、DataVersion policy、退出保护。
- permission、partial、stale、snapshot、over budget、async running。

## 依赖管理

- 不随意新增依赖。
- 新增依赖前，先检查项目中是否已有同类工具。
- 不为简单功能引入大型库。
- 不同时引入多个功能重复的库。
- 不引入不活跃、不可信、不必要的包。
- 不修改 package manager 类型。
- 不随意修改 lockfile，除非依赖确实变化。
- 新依赖必须有明确用途。
- 新依赖不应显著增加 bundle 体积，除非收益明确。
- 日期、表单、请求、状态管理、动画、图表等依赖必须遵循项目已有选择。
- 表格、图表、wafer map 依赖必须通过 adapter layer 封装。

## 命名

- 组件名使用 `PascalCase`。
- 类型和接口使用 `PascalCase`。
- 函数、变量、hook 使用 `camelCase`。
- React hook 必须以 `use` 开头。
- 常量使用项目已有风格，常见为 `UPPER_SNAKE_CASE`。
- 布尔值使用 `is`、`has`、`can`、`should` 等语义前缀。
- 事件处理函数使用 `handleXxx`。
- props 回调使用 `onXxx`。
- 组件文件名遵循项目已有风格。
- 名字应表达业务含义，不要过度缩写。
- 避免无意义命名，例如 `data`、`item`、`value`，除非上下文非常清楚。

## 注释

- 注释应解释“为什么”，不要重复“做了什么”。
- 复杂业务规则应有注释。
- 临时 workaround 必须说明原因。
- 不保留无意义或过时注释。
- 不用 TODO 逃避实现。
- TODO 应说明后续行动或阻塞原因。
- 安全相关、兼容性相关、复杂状态转换相关逻辑应适当注释。

避免：

```ts
// Set loading to true
setLoading(true);
```

推荐：

```ts
// Keep the button disabled until the request finishes to prevent duplicate submissions.
setSubmitting(true);
```

## 禁止行为

禁止生成以下代码：

- 无理由使用 `any`、`as any`、非空断言 `!`。
- 无理由新增依赖。
- 无理由改变项目架构、公共组件 API、路由或数据结构。
- 无理由创建全局状态。
- 无理由使用 `dangerouslySetInnerHTML`。
- 直接信任用户输入、URL 参数或 localStorage 数据。
- 硬编码密钥、token、私钥、凭证。
- 把敏感信息打印到 console。
- 静默吞掉错误。
- 只实现 happy path。
- 用 `<div>` 模拟按钮。
- 交互元素不可键盘访问。
- 表单字段没有 label。
- 图片缺少合理 alt。
- 列表使用不稳定 key。
- 在 React render 阶段触发副作用或调用 setState。
- 滥用 `useEffect` 存储派生状态。
- 忽略 hook 依赖。
- 为了消除 lint warning 随意删依赖。
- 在组件中堆砌大量业务逻辑。
- 过度抽象、过度封装、过度组件化。
- 为小功能引入大型库。
- 留下生产环境无意义 console。
- 写无法响应式适配的固定布局。
- 使用魔法数字和魔法字符串表达业务规则。
- 不处理 loading、error、empty、disabled 状态。
- 不处理请求失败。
- 不处理重复提交。
- 客户硬编码分支，例如 `if customer === "X"`。
- 前端实现权限安全边界。
- 直接把第三方 grid/chart API 泄漏到页面层。

## 推荐行为

生成 React + TypeScript 代码时优先：

- 使用明确 TypeScript 类型。
- 使用 union type 表达有限状态。
- 使用 discriminated union 表达复杂 UI 状态。
- 使用语义化 HTML。
- 使用可访问交互元素。
- 使用稳定 key。
- 使用不可变状态更新。
- 使用局部 state，而不是过早使用全局 state。
- 使用派生计算，而不是重复存储派生状态。
- 使用项目已有 API client。
- 使用项目已有样式系统。
- 使用项目已有组件库。
- 使用清楚的小组件。
- 使用 custom hook 抽离复杂副作用或复用逻辑。
- 使用工具函数抽离纯业务计算。
- 使用测试覆盖关键用户行为。
- 使用 lint、typecheck、test 和 build 验证代码。

## 生成代码前自检

输出 React + TypeScript 代码前必须自检：

1. 是否符合当前项目技术栈？
2. 是否符合当前项目目录结构？
3. 是否引入了不必要依赖？
4. 是否修改了无关代码？
5. 是否改变了公共 API？
6. TypeScript 类型是否明确？
7. 是否存在 `any`、`as any`、不必要类型断言或非空断言？
8. 是否处理 loading、error、empty、disabled、success 状态？
9. 是否只实现了 happy path？
10. 是否存在 XSS 或敏感信息泄露风险？
11. 是否使用语义化 HTML？
12. 是否支持键盘操作？
13. 表单字段是否有 label？
14. 图片是否有合理 alt？
15. 列表 key 是否稳定？
16. 状态是否放在合理位置？
17. 是否重复存储派生状态？
18. 是否存在不必要全局状态？
19. 是否存在 render 阶段副作用？
20. hook 依赖是否正确？
21. effect 是否需要 cleanup？
22. 是否存在请求竞态？
23. 是否存在重复提交问题？
24. 是否存在过度抽象？
25. 是否有魔法数字或魔法字符串？
26. 是否有客户硬编码？
27. 是否错误地在前端实现业务分析语义？
28. 表格、图表、wafer map 是否通过 adapter 接入？
29. 是否需要测试，是否已补？
30. 是否能通过 lint、typecheck、test、build？
31. 用户在异常情况下是否知道下一步该做什么？

## 输出要求

生成 React + TypeScript 代码时：

- 优先输出完整、可运行、类型明确的代码。
- 修改现有代码时，只输出必要修改。
- 新增文件时说明文件路径。
- 新增依赖时说明原因。
- 不输出伪代码，除非用户明确要求。
- 不假设不存在的组件、hook、API、store 或工具函数。
- 对关键设计选择给出简短说明。
- 对状态处理、错误处理、可访问性和测试给出必要说明。
- 无法确认项目上下文时，采用最保守、最少侵入的实现方式。
