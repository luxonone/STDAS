# UI/UX 约束

本文件定义 STDAS 前端必须遵守的 UI/UX 约束。它不是视觉风格指南，而是高密度数据工作台的可验收规则，用来避免复杂数据、长表单、多 Lot 分析、图表联动、异步任务和治理配置在实现中失控。

页面族按 [页面层级设计](page-hierarchy-design.md) 划分。新增页面或大改页面必须先确认所属页面族，再套用对应约束。

## 规则分级

| 等级 | 含义 | 示例 |
|------|------|------|
| P0 | 违反即不允许上线 | 业务上下文丢失、LotEndTime/测试维度不可见、危险操作无确认、关键操作不可达、数据状态未标识、权限状态混淆 |
| P1 | 允许灰度，但必须在当前或下一迭代补齐 | 状态恢复不完整、辅助说明不足、部分键盘访问缺口、低频状态缺少细节 |
| P2 | 体验优化项 | 快捷入口优化、布局密度微调、低频说明优化、视觉层级细化 |

每条页面级规则必须标记适用范围：Global Shell、Overview、Data Explorer、Analysis Workspace、Alerts & Investigation、Jobs & Exports、System Governance。

## 术语与状态定义

| 术语 | 定义 | UI 必须展示 |
|------|------|-------------|
| CustomerScope | 当前用户被授权查看和操作的客户范围 | 当前客户、全部客户或受限客户范围；不能用空值表达全部 |
| DataProfile | 数据解析、映射、规格、规则和模板的组合配置 | 主要用于治理和诊断页面；普通工程页面不要求用户选择 |
| DataVersion | 某批数据参与查看、分析或导出时使用的内部稳定数据版本 | 主要用于追溯、Evidence、Export 和诊断；普通工程页面优先显示业务时间和快照 |
| DataVersion policy | 多 Lot 或工作区选择 DataVersion 的内部规则 | 不作为普通工程页面主控件；需要追溯时显示引用或状态 |
| latest committed | 已提交且可用于分析的最新稳定 DataVersion | 版本号、生成时间或解析完成时间 |
| LotNo | 工厂内部批次号 | 列表、详情、分析和告警中的主对象 |
| CustomerLotNo | 客户侧批次号 | 客户需要时展示或筛选 |
| TestType | 客户/产品定义的测试类型，例如 FT、BI、BIT、SLT | 作为 FT 系统核心筛选和上下文 |
| TestStation | 客户/产品定义的测试站点，例如 FT1、FT2、FT3、FTA、BI1 | 与 TestType 共同构成核心分析维度 |
| TestAttempt | 同一 LotNo 在同一 TestType-TestStation 下的测试次数 | 初测、重测、第三次测试等 |
| LotEndTime | LotNo 在 TestType-TestStation 维度下的作业结束时间 | 时间范围筛选、列表和图表优先使用 |
| lot_scope | 当前分析覆盖的 LotNo 范围 | 单 Lot、多 Lot、当前筛选结果集合，以及数量 |
| partial data | 查询结果只返回了部分数据 | 缺失范围、原因、是否可重查或转异步 |
| stale | 当前结果不是最新数据 | 最近更新时间、刷新入口 |
| snapshot | 最近一次成功结果快照 | 快照时间、原始查询条件 |
| query budget | 查询预算，包括点数、行数、时间范围、计算成本 | 当前消耗、超限项、降级方式 |
| query snapshot | 一次查询运行时固化的查询条件、lot_scope、DataVersion 集合和结果引用 | snapshot id、生成时间、查询摘要 |
| evidence version | Investigation Case 中一次证据保存或重算的版本 | evidence id、生成时间、来源 query snapshot |
| trigger context | 告警触发时的规则、数据和关联对象上下文 | rule version、trigger time、related lots、DataVersion |

## 核心原则

- 主任务优先：首屏必须让用户看到当前页面的核心任务、关键上下文和主要结果区。
- 上下文不断：customer、product、TestType、TestStation、TestAttempt、LotEndTime range、来源入口和查询条件不能在跳转中隐式丢失。
- 用户语义优先：普通工程页面不让用户选择 DataProfile、DataVersion policy 或 user role；这些属于账号权限、后台解析或治理诊断语义。
- 数据可信：实时、缓存、最近成功快照、partial data、采样、超预算和异步结果必须明确标识。
- 长内容可达：任何表单、表格、抽屉、弹窗中的长内容必须可滚动、可定位、可提交。
- 关键操作可达：保存、提交、取消、确认、导出、重试等关键操作不能被输入态、浮层、抽屉或临时面板遮挡。
- 结构化输入优先：日期、时间、枚举、范围、单位、客户、站点、设备、Lot、Bin、参数等字段默认使用专用控件。
- 次要操作收纳：高级筛选、批量操作、导出设置、列配置等默认收纳，但已生效状态必须可见。
- 高密度但可读：STDAS 优先桌面端工程工作台，不使用展示型官网布局、装饰性 hero、大面积说明或低密度卡片堆叠。

## 全局壳层

| 约束 | 等级 | 规则 |
|------|------|------|
| 当前用户 | P0 | 用户身份、角色、会话操作必须全局可达 |
| CustomerScope | P0 | 当前授权客户范围必须可见，不能用空值表达全部客户 |
| 客户切换 | P0 | 切换客户后必须重新解析 DataProfile，并刷新依赖上下文的数据 |
| 当前上下文 | P0 | 页面按场景显示 customer、product、TestType、TestStation、LotEndTime range |
| 全局任务 | P0 | 存在长耗时分析、摄入、导出或后台运行任务时，必须可从全局任务入口查看 |
| 权限状态 | P0 | 未登录、无页面权限、无客户数据权限、数据被规则隐藏必须区分展示 |

普通工程页面的顶部壳层不得出现 user role 选择器、DataProfile 选择器或 DataVersion policy 选择器。角色由账号配置决定；解析规则由系统按客户/产品/测试类型/测试站点解析；用户可见时间以 LotEndTime 和 Snapshot Time 为主。

## URL、导航与状态恢复

- 每个页面必须声明 URL 状态契约，区分 Required URL State、Optional URL State、Session State 和 Forbidden URL State。
- URL 不应包含敏感 token、临时权限、未保存表单内容、超长选择集合或安全策略禁止明文暴露的客户、产品、Lot、规则信息。
- 手动选择的少量 Lot 可以进入 URL；大批量选择必须进入 workspace/session，并在页面显示 selection source。
- 浏览器 back/forward 必须恢复对应筛选和结果状态，不得静默重置。
- 从详情页返回列表时，筛选、排序、分页、滚动位置和已选项必须恢复。
- URL 中指定的 DataVersion 不存在、过期或无权限时，必须明确提示并提供切换方案。
- 敏感客户、产品、Lot 或规则信息不允许明文进入 URL 时，必须使用 share token、workspace id 或 query snapshot id，并在打开时重新做权限校验。

页面族默认 URL 状态：

| 页面族 | Required URL State | Session State | Forbidden URL State |
|------|--------------------|---------------|---------------------|
| Overview | customer、product、test_type、test_station、lot_end_time_range | 卡片展开、临时排序 | token、临时权限 |
| Data Explorer | customer、product、test_type、test_station、lot_end_time_range、主筛选、分页、排序 | 大批量 Lot 选择、滚动位置 | 未授权客户/Lot 明文、未保存筛选草稿 |
| Analysis Workspace | workspace id 或 query snapshot id、核心查询参数 | 未保存布局、临时刷选、草稿注释 | 超长 lot_scope、未保存表单、敏感字段明文 |
| Alerts & Investigation | alert id、rule version、case id 或 trigger context 引用 | 临时筛选展开、局部排序 | 未授权告警摘要 |
| Jobs & Exports | job id、export id、任务筛选状态 | 临时下载状态 | 过期下载 token |
| System Governance | profile/rule/template id、version、tab | 未保存配置草稿 | secret、token、未发布敏感配置 |

## DataVersion 冻结规则

- Analysis Workspace 运行查询时，必须将每个 Lot 实际使用的 DataVersion 固化到 query snapshot。
- 已生成的图表、表格、导出和 investigation evidence 必须引用固化后的 DataVersion 集合。
- 用户点击刷新或重新运行时，可以重新解析 latest committed，但必须提示 DataVersion 可能变化。
- 打开历史 workspace、case、export 时，默认展示当时固化的 DataVersion，不得静默切换到新的 latest committed。
- 如果某个 Lot 的 latest committed 已变化，必须提示有新 DataVersion 可用，并提供重新运行入口。

## 数据可信与统一状态矩阵

| 状态 | 适用对象 | 必须展示 | 用户可执行动作 |
|------|----------|----------|----------------|
| loading | 页面、表格、图表、任务 | 加载范围、是否可取消 | 等待、取消 |
| empty | 表格、图表、列表 | 空的原因 | 修改筛选、清空筛选 |
| error | 查询、图表、任务、表单 | 错误类型、影响范围 | 重试、查看详情、联系管理员 |
| permission denied | 页面、操作、数据 | 权限缺失类型 | 申请权限、切换客户 |
| partial data | 图表、表格、导出 | 缺失范围、原因 | 缩小范围、转异步、重查 |
| stale | KPI、图表、缓存结果 | 最近更新时间 | 刷新 |
| snapshot | Overview、历史结果 | 快照时间、原始查询 | 使用快照、重新查询 |
| over budget | 查询、图表 | 超出预算项 | 聚合、采样、异步任务 |
| async running | 分析、导出、摄入 | 任务阶段、进度、发起人 | 查看任务、取消 |

## 权限与数据隐藏

- 无页面权限：显示权限不足页面，不渲染页面主体数据。
- 无客户数据权限：不得泄露客户数据内容；是否显示客户名称由权限策略决定。
- 操作无权限：按钮可隐藏或禁用，但同一操作在全系统内策略必须一致。
- 禁用按钮必须说明禁用原因，例如无权限、状态不允许、缺少必填条件、任务运行中。
- 数据被规则隐藏时，必须区分“无数据”和“有数据但不可见”。
- 权限变化导致导出文件不可下载时，必须显示权限变化状态，而不是死链接。
- 无客户数据权限时，是否展示客户名、Lot 数量、产品名、筛选命中数，必须由权限策略定义。
- 对受限数据，UI 必须区分 hidden、masked、unauthorized、not found。
- 不得通过空状态、错误文案、导出文件名、URL 参数泄露未授权客户、产品或 Lot 信息。
- 分享链接打开时必须重新校验权限；无权限时不得展示原始查询摘要中的敏感字段。
- 导出文件下载时必须重新校验权限。

## 页面族约束

### Overview

| 约束 | 等级 | 规则 |
|------|------|------|
| FT 第一阶段 | P0 | Overview 默认面向 FT 测试系统；不展示 Wafer KPI 或 CP 主维度 |
| 首屏内容 | P0 | 首屏必须展示 FT KPI、近期趋势、open alerts、risky lots 和快捷入口 |
| 快捷入口 | P0 | 从 Overview 进入 Data Explorer、Alerts 或 Analysis Workspace 时必须携带筛选上下文 |
| 业务时间 | P0 | KPI、趋势和列表必须优先显示 LotEndTime 范围、Snapshot Time 或缓存状态，不以 Updated 作为主时间 |
| 异常优先 | P1 | 告警、风险批次、明显偏移指标的视觉权重高于普通统计说明 |
| 降级状态 | P0 | Overview 可以展示最近成功快照，但必须标识 stale 或 snapshot |
| 用户语义 | P0 | Overview 不显示 DataProfile、DataVersion policy、UserRole 选择器；不出现 Wafer Count 默认 KPI |

### Data Explorer

Data Explorer 承担高密度数据对象浏览和多 Lot 分析入口。

| 约束 | 等级 | 规则 |
|------|------|------|
| Lot 列表 | P0 | 必须支持服务端分页、排序、筛选，不能一次性渲染无限数据 |
| 核心列 | P0 | LotNo、CustomerLotNo、customer、product、TestType、TestStation、TestAttempt、status、yield、LotEndTime 必须优先展示或可固定 |
| 多选 | P0 | Lot List 必须支持手动多选，并显示已选数量 |
| 选择范围 | P0 | 手动选择和当前筛选结果选择必须明确区分 |
| 上下文摘要 | P0 | 多 Lot 进入分析前必须显示 customer、product、TestType、TestStation、TestAttempt、LotEndTime range 摘要 |
| 差异提示 | P0 | 所选 Lot 的 customer、product、TestType、TestStation、TestAttempt、program 或口径不一致时必须标记差异 |
| 详情返回 | P1 | 从 Lot Detail 返回列表时，筛选、排序、分页、已选项应恢复 |
| 追溯关系 | P0 | LotNo、Run、File、解析记录、内部数据引用之间必须能互相定位 |
| 历史结果 | P0 | 从历史 workspace/case/export 进入分析时，必须保留当时的业务上下文和内部数据引用 |

### Analysis Workspace

Analysis Workspace 是多 Lot、多参数、多图联动分析的核心容器。

| 约束 | 等级 | 规则 |
|------|------|------|
| lot_scope | P0 | 工作区必须显示当前是单 Lot、多 Lot 还是查询结果集合 |
| 测试维度 | P0 | 必须显示每个 LotNo 参与分析的 TestType、TestStation、TestAttempt、LotEndTime；内部数据引用可在详情中展开 |
| 查询摘要 | P0 | 每个结果区必须能看到查询条件摘要、LotEndTime 范围、单位、聚合粒度和点数/预算状态 |
| 多图联动 | P0 | 图表、表格、刷选区和 drilldown 必须共享同一上下文 |
| 跨 Lot 对齐 | P0 | 跨批次图表和表格必须能按 Lot、stage、site、Bin、parameter、time bucket 对齐 |
| 预算反馈 | P0 | 超预算时必须说明是拒绝、partial data、采样还是转异步 job |
| 保存状态 | P0 | 保存 workspace、case、template 或 export 时必须显示保存目标和影响范围 |
| 模板应用 | P0 | 应用模板前必须显示模板适用的 customer/profile/test type/test station 条件 |
| 结果可信 | P0 | 图表和表格必须标识实时、缓存、快照、partial、采样或 stale 状态 |
| 退出保护 | P0 | 工作区有未保存布局、查询或注释时，离开前必须提示 |

### Alerts & Investigation

| 约束 | 等级 | 规则 |
|------|------|------|
| 告警列表 | P0 | 必须支持状态、严重度、customer、product、time range、rule version 筛选 |
| 告警详情 | P0 | 必须显示触发规则、规则版本、触发时间、关联 Lot/DataVersion 和解释上下文 |
| 跳转分析 | P0 | 从告警进入 Analysis Workspace 时必须携带 alert id、rule version、trigger context、related lots |
| 状态变更 | P0 | 确认、关闭、重开、分派必须有成功/失败反馈和审计上下文 |
| 调查案例 | P1 | Investigation Case 必须保留证据、图表、查询条件、结论和引用版本 |

### Investigation Evidence

- 加入 Investigation Case 的图表、表格、查询结果必须保存 query snapshot、DataVersion 集合和生成时间。
- Evidence 默认不可静默重算；重新计算必须生成新的 evidence version。
- 删除 evidence、修改结论、关闭 case 必须记录审计。
- 从 case 打开 evidence 时，必须显示这是历史快照还是当前重算结果。
- 如果 evidence 所引用的数据已过期、被隐藏或权限变化，必须显示状态。

### Jobs & Exports

| 约束 | 等级 | 规则 |
|------|------|------|
| 任务状态 | P0 | 异步任务必须显示状态、阶段、发起人、开始时间、更新时间、耗时 |
| 失败诊断 | P0 | 失败任务必须显示失败阶段、错误原因、是否可重试和下一步动作 |
| 重试/重放 | P0 | 重试、重放、取消必须显示影响范围并二次确认 |
| 导出结果 | P0 | 导出文件必须携带查询条件、DataVersion、生成时间、操作者和数据范围 |
| 下载状态 | P0 | 结果过期、权限变化、文件缺失必须有明确状态，不显示不可用死链接 |

### 异步任务生命周期

- 任务必须展示状态流转：queued、running、succeeded、failed、canceling、canceled、expired。
- 任务必须展示创建时间、更新时间、完成时间、过期时间。
- 取消任务必须显示是否取消成功；如果无法取消，必须说明原因。
- 重试任务必须说明复用原 query snapshot，还是按当前 latest committed 重新解析。
- 导出任务必须显示数据范围、行数估计、DataVersion 集合、是否脱敏、文件格式和过期时间。
- 任务完成、失败、过期时必须有可追踪入口，不能只依赖 toast。
- 任务详情必须保留 request id 或 job id，便于排查。

### System Governance

| 约束 | 等级 | 规则 |
|------|------|------|
| 配置编辑 | P0 | 长表单必须分组、可滚动、可定位错误，关键按钮固定或双位置可达 |
| 发布状态 | P0 | Profile、rule、template 必须显示 draft、published、deprecated 等状态 |
| 版本影响 | P0 | 发布、覆盖、删除、回放必须显示影响的 customer、product、test type、test station、DataProfile 或规则版本 |
| Profile 预览 | P0 | DataProfile resolution 必须支持输入解析键并预览命中的 parser/mapping/spec/rule/template |
| 审计 | P0 | 关键治理动作必须显示操作者、时间、变更摘要和可追溯版本 |
| 客户差异 | P0 | 前端不写客户硬编码分支，差异必须来自 profile、feature flag 或 options API |

## Governance 变更安全

- 发布前必须展示变更 diff、影响范围、受影响 customer/product/test type/test station/DataProfile/rule version。
- 发布前必须完成配置校验，并显示 blocking errors 与 warnings。
- 发布动作必须说明立即生效、定时生效或仅对新数据生效。
- 关键配置必须支持查看历史版本和回滚入口。
- 多人并发编辑同一配置时，必须提示版本冲突。
- 删除或 deprecated 前必须显示依赖对象和下游影响。

## 表单与结构化输入

| 约束 | 等级 | 规则 |
|------|------|------|
| 长表单 | P0 | 内容区域必须可滚动，不能让底部字段不可达 |
| 提交按钮 | P0 | 固定在可见操作区，或在长内容顶部和末尾同时提供明确操作 |
| 输入态遮挡 | P0 | 输入控件获得焦点后，当前字段、错误提示和关键操作不能被遮挡 |
| 错误定位 | P0 | 提交失败后必须能定位到第一个错误字段 |
| 必填说明 | P1 | 必填字段、默认值和格式要求必须在输入前可见 |
| 危险操作 | P0 | 删除、覆盖、重跑、回放、发布等必须二次确认并显示影响范围 |
| 日期时间 | P0 | 日期、时间、时间范围默认使用 picker，时间范围必须明确时区 |
| 枚举/单位 | P0 | 枚举使用受控选项；数值字段必须声明单位、范围、步进和默认值 |

## 筛选与查询

| 类型 | 等级 | 展示策略 |
|------|------|----------|
| 主筛选 | P0 | Data Explorer 和 Analysis Workspace 的客户、产品、Lot、时间范围、测试类型、测试站点等高频筛选必须放在主区域；其他页面按场景可降为 P1 |
| 高级筛选 | P1 | 设备类型、程序版本、参数名、Bin、规则版本等默认收纳 |
| 超长选项 | P0 | 必须支持搜索、分页或虚拟滚动 |
| 已选条件 | P0 | 必须以 chip/tag/summary 形式可见，并支持快速清除 |
| URL 恢复 | P0 | 关键筛选条件必须可序列化到 URL |
| 查询触发 | P0 | 高成本查询页面默认使用显式“应用筛选/运行查询”，避免每次输入自动触发 |
| 默认范围 | P0 | 默认时间范围必须由页面场景定义，不能隐式无限查询 |

## 查询竞态与过期响应

- 用户修改筛选、DataVersion policy、lot_scope 或时间范围后，旧查询结果不得被误认为新结果。
- 前端必须取消旧请求，或忽略过期响应。
- 每个结果区必须绑定 query id、query hash 或 snapshot id。
- 响应返回时必须校验当前页面状态与响应对应的查询状态一致。
- 如果展示缓存结果，必须明确标识缓存时间和对应查询条件。
- 查询运行中修改条件时，必须区分“已应用条件”和“待应用条件”。

## Options API 与级联字段

- 客户、产品、测试类型、测试站点、设备类型、程序名、参数名、Bin、规则版本、DataProfile 等字段必须来自后端 options API 或受控配置。
- options 必须支持 loading、empty、error、permission denied 状态。
- 上游字段变化后，下游字段必须重新加载并校验已选值是否仍然有效。
- 已选值变为 deprecated、hidden、unauthorized 或不存在时，必须保留可解释状态，不得静默清空。
- 超长 options 必须支持搜索、分页或虚拟滚动。
- 搜索型 options 必须有 debounce，并避免旧搜索响应覆盖新搜索结果。
- 除非页面明确允许 free text，否则不得提交 options API 中不存在的值。

## 表格细则

适用范围：Data Explorer、Analysis Workspace、Alerts & Investigation、Jobs & Exports，以及任何包含数据表格的页面。

- 表格必须有稳定行标识，Lot、Run、File、DataVersion 等关键对象必须可识别。
- 大表格必须支持分页、虚拟滚动或服务端查询。
- 大表格横向滚动时，主键列和行操作列必须可固定或可恢复。
- 当前页选择、手动选择、当前筛选结果选择必须明确区分。
- 批量操作必须显示影响对象数量、范围和是否跨页。
- 行级操作超过 3 个时，低频操作应收纳到更多菜单。
- 空状态必须区分：无数据、筛选无结果、无权限、查询失败、数据被规则隐藏。
- 列配置必须支持恢复默认。
- 排序、分页、列配置和主筛选应可恢复。
- 数值列默认右对齐，单位一致，空值表达一致。

## 图表细则

适用范围：Overview、Analysis Workspace、Alerts & Investigation，以及任何包含可视化结果的页面。

- 图表必须显示指标名称、单位、时间范围、数据口径、聚合粒度，并提供 QuerySnapshot 或 DataVersion 追溯引用；普通图表主标题优先显示业务维度，不把 DataVersion 当作主标签。
- 坐标轴、图例、tooltip 必须能解释业务含义。
- 采样、聚合、截断、异常值过滤必须在图表附近标识。
- 图表 brush、zoom、legend toggle、drilldown 必须有明确 reset 或返回入口。
- 图表选中状态与表格联动时，当前选中范围必须可见。
- 图表异常点必须能追溯到 Lot、site、Bin、parameter、DataVersion 或相关对象。
- 不得仅依赖颜色区分良品、异常、告警或超规格状态。
- 图表必须有 loading、empty、error、partial data 状态。
- 导出大数据必须走后端异步任务；小图可前端导出图片。

## 抽屉、弹窗与临时面板

适用范围：Global Shell、Data Explorer、Analysis Workspace、System Governance，以及任何使用 overlay 的页面。

- 抽屉用于辅助查看、轻量编辑、drilldown 和配置预览，不承载复杂主流程。
- 弹窗只用于确认、短表单、轻量设置和不可忽略的反馈。
- 超过 6 个字段、超过 2 步、需要预览/比较/批量确认的流程，应进入独立页面或工作区。
- 抽屉和弹窗内部内容必须可滚动，操作区必须 sticky。
- 打开抽屉或弹窗后，焦点必须进入当前上下文；关闭后焦点回到触发元素。
- 存在未保存输入、未应用筛选或未提交配置时，关闭前必须提示。
- 浮层、下拉菜单、tooltip 不得遮挡保存、提交、取消、确认等关键操作。
- 多层浮层不得超过 2 层；禁止弹窗中再打开复杂弹窗。

## 组件语义一致性

- 同一语义操作必须使用同一组件和交互模式。
- 页面级操作、区域级操作、表格批量操作、行级操作必须位置明确，不得混用。
- 每个操作上下文最多只有一个主操作。
- 危险操作必须使用统一危险确认模式。
- Toast、banner、inline error、modal 的使用场景必须区分：toast 用于轻量成功反馈；inline error 用于字段级错误；banner 用于页面级状态或风险提示；modal 用于必须阻断用户继续操作的确认。
- 空状态、错误状态、partial data、stale data、permission denied 必须使用统一模式。

## 响应式与工作区布局

- 主目标视口为桌面工作台；`1366x768`、`1440x900`、`1920x1080` 必须可用。
- 小屏可以降级为上下布局，但关键上下文、筛选状态、DataVersion 和操作入口不能丢。
- 复杂分析在过窄视口下可以提示不适合当前视口，不能挤坏布局。
- 固定格式元素如表格工具栏、图表容器、选择栏和操作区必须有稳定尺寸，不能因 loading、hover、动态文案导致布局跳动。

## 可访问性细则

- 所有表单控件必须有关联 label，不能只依赖 placeholder。
- 图标按钮必须有 tooltip 或 aria-label。
- Tab 顺序必须符合视觉顺序和任务顺序。
- 弹窗、抽屉打开后必须进行焦点管理，关闭后焦点回到触发元素。
- 错误信息必须能被屏幕阅读器读取。
- 表格排序、筛选、展开、选择、行操作必须支持键盘访问。
- 关键操作点击区域不得小于 44px。
- 禁用状态必须提供原因，不能只显示灰色。
- 颜色不能作为唯一状态表达，告警、错误、超限必须有文本或图标辅助。
- 文本与背景对比度必须满足基本可读性要求，错误、告警、禁用状态不得仅靠低对比度表达。
- 所有键盘可达元素必须有可见 focus ring。
- Toast、banner、任务状态变化、错误提示应能被辅助技术感知。
- 虚拟滚动表格不得破坏键盘焦点和当前行定位。
- 高密度表格中正文字号不得低于系统约定最小字号。

## 默认阈值与页面覆盖

除页面族另有说明，默认采用以下阈值。具体数值可以被 API 查询预算或页面设计覆盖，但覆盖必须写入对应功能切片。

| 项目 | 默认阈值 | 超过后的处理 |
|------|----------|--------------|
| 主筛选数量 | <= 5 个 | 其余进入高级筛选 |
| 表格单页行数 | <= 100 行 | 服务端分页 |
| 前端虚拟滚动行数 | > 200 行 | 必须启用虚拟滚动 |
| 图表点数 | 由查询预算声明 | 聚合、采样、partial data 或异步任务 |
| 查询时间范围 | 由页面默认范围声明 | 要求缩小范围或转异步 |
| 长表单 | > 8 个字段或超过 1 屏 | 分组 + sticky 操作区 |
| 弹窗表单 | > 6 个字段或超过 2 步 | 改为独立页面或抽屉流程 |
| 抽屉宽度 | 480-720px | 超过复杂度进入独立页面 |

## 页面交付说明模板

每个新页面或大改页面必须提交：

1. 页面族归属。
2. 主任务说明。
3. 核心用户路径。
4. 当前上下文字段。
5. 必须进入 URL 的状态。
6. 不进入 URL 的临时状态。
7. 数据可信状态展示方式。
8. 权限状态处理。
9. 表格/图表状态矩阵。
10. 异步任务与导出策略。
11. 危险操作与二次确认策略。
12. 空状态、错误状态、partial data、stale data 处理。
13. 响应式降级策略。
14. 未保存状态与退出保护。
15. 验收截图或验收说明。
16. 接口与数据契约：关键 API、options API、query snapshot、job/export id。
17. 查询预算与降级策略：预算来源、超限表现、是否支持异步。
18. 权限与脱敏策略：哪些字段可见、隐藏、脱敏或禁止进入 URL。
19. 例外与豁免：若不满足某条 P0/P1，必须记录原因、风险和补救计划。

## 验收检查

每个新页面或大改页面必须回答：

- 页面是否归属到明确页面族？
- 规则分级中所有 P0 是否满足？
- 首屏是否展示主任务、关键上下文和核心结果区？
- CustomerScope、时间范围和适用业务上下文是否可见？DataProfile、DataVersion 是否在治理、诊断、Evidence、Export 或详情展开区可追溯？
- 多 Lot 场景下 lot_scope、已选数量、上下文差异和固化版本状态是否可见或可展开？
- 跨页面跳转后上下文是否保持或明确重置？
- URL、back/forward、详情返回和分享链接是否恢复正确状态？
- 长内容是否仍然可达？
- 关键操作是否始终可见或可恢复？
- 输入态、浮层、抽屉或临时面板是否遮挡关键操作？
- 日期/时间/枚举/单位等结构化字段是否避免自由手输？
- 高级筛选和次要操作是否收纳且已生效状态可见？
- 数据结果是否说明实时、缓存、快照、partial、采样、stale 或超预算状态？
- 权限不足、无数据、数据被隐藏、查询失败是否区分展示？
- 错误状态是否可定位、可理解、可修复？
- 导出、保存、发布、重试等动作是否显示影响范围和最终状态？
- 是否声明接口与数据契约、查询预算、权限脱敏策略？
- 如存在例外或豁免，是否记录原因、风险和补救计划？
