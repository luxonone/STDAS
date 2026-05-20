# 前端工作台设计

STDAS 前端不是传统后台管理系统，而是面向测试工程师的高密度分析工作台。

前端技术基线见 [前端技术架构](frontend-tech-architecture.md)。STDAS 采用 React + TypeScript 和 Feature-Sliced Analytical Workbench Architecture；本文件只描述工作台体验、状态和组件策略。

## 设计目标

- 高信息密度，减少装饰。
- 快速筛选、对比、下钻和导出。
- 图表与表格并列组织，避免长页面堆叠。
- 支持从 Overview、Data Explorer、Alerts、Template 快速进入分析上下文。
- 对大数据结果保持响应性，避免一次性渲染无限明细。
- 遵守 [页面层级设计](page-hierarchy-design.md)，避免按后端服务、CRUD 或临时 route 清单组织页面。
- 遵守 [UI/UX 约束](ui-ux-constraints.md)，保证 P0 规则、上下文连续、数据可信、结构化输入和主任务首屏优先级。

## 关注点优先

前端导航和页面信息架构围绕分析关注点组织，不围绕复杂岗位角色组织。初始只有工程师和管理员两类角色，但工程师视角内可以覆盖多个关注点：

- Lot 详情。
- 多 Lot 对比。
- 良率趋势。
- Bin 分布。
- 参数能力。
- 告警追溯。
- 模板复用。
- 导出与案例沉淀。

管理员视角只承载系统治理能力：

- 用户和权限。
- 客户和产品配置。
- 采集任务。
- 告警规则。
- 数据保留和系统配置。

## FT 第一阶段用户语义

当前第一阶段先开发 FT 测试系统。CP 晶圆测试系统暂不考虑；Wafer 相关字段只在 FT 数据实际提供且具体分析需要时作为可选字段出现，不作为 Overview、Lot List 的默认主维度。

工程师日常页面应优先展示和筛选：

- 客户、产品。
- LotNo、客户 LotNo。
- 测试类型，例如 FT、BI、BIT、SLT 等客户/产品专属命名。
- 测试站点，例如 FT1、FT2、FT3、FTA、BI1 等客户/产品专属命名。
- 测试次数，例如第 1 次、第 2 次、第 3 次。
- LotEndTime，表示 LotNo 在某个测试类型-测试站点维度下的实际作业结束时间。
- Yield、Bin、Retest、Tester、Handler、Site、Test Program、Program Version。

DataProfile、DataVersion policy、UserRole 选择等属于账号、权限、后台解析或治理概念，不应作为普通工程页面的主筛选或显性控件。需要追溯时，可以在详情、Evidence、Export 元数据或治理页面中暴露内部引用。

## 客户差异支持

前端必须支持 OSAT 多客户客制化：

- 客户、产品、测试类型、测试站点变化后，后台按当前数据上下文加载对应解析规则和客户专属配置。
- 导航、Tab、筛选项、图表、模板由 feature flags 和 profile 控制，但普通用户不直接选择 DataProfile。
- CP、FT、BI、SLT 等测试类型可以拥有不同信息结构。当前 FT 系统中 BI、SLT 归入 FT 部门范围；CP 暂不进入第一版页面。
- 设备类型相关字段和筛选项由 equipment profile 决定。
- Lot Detail 页面不为客户复制代码，通过 profile 控制可见 section。
- 报表和导出格式按客户模板展示。
- 告警解释必须显示触发时使用的规则版本。
- 前端不写 `if customer == "X"` 的分支，客户差异通过 profile 驱动。

## 页面层级

前端页面层级以 [页面层级设计](page-hierarchy-design.md) 为准。工作台不再使用旧的 `/dashboard`、`/lots`、`/ingestion`、`/analysis`、`/alerts`、`/workspaces`、`/templates`、`/admin` 平级清单作为主信息架构。

顶层分为：

- Global Shell：登录、当前用户、权限、当前业务上下文、通知和任务队列。
- Engineering Workbench：Overview、Data Explorer、Analysis Workspace、Alerts & Investigation、Jobs & Exports。
- System Governance：Users & Access、Customer & Profile、Ingestion Config、Rules & Templates、Data Governance。

## 核心页面族

| 页面族 | 职责 |
|------|------|
| Overview | FT KPI、近期趋势、risky lots、open alerts、快捷入口 |
| Data Explorer | 高密度 LotNo 列表、多 Lot 选择、Lot 详情、文件、测试运行和 LotEndTime 上下文 |
| Analysis Workspace | 多 Lot 对比、跨批次、多参数、多图联动、模板分析、案例沉淀和导出 |
| Alerts & Investigation | 告警列表、确认、上下文跳转、调查案例 |
| Jobs & Exports | 文件摄入、分析、导出等异步任务状态、失败诊断和下载 |
| System Governance | 用户、客户、权限、DataProfile、采集配置、规则、模板和数据治理 |

Analysis Workspace 是分析能力的核心容器。良率趋势、Bin 分布、参数能力、重测收益等能力作为工作区内的分析视图和模板，不作为一级导航孤立存在。

## 多 Lot 分析

多 Lot 查看和分析是一等工作流：

- Data Explorer 的 Lot List 支持手动多选和基于当前筛选结果加入 Analysis Workspace。
- 加入工作区前必须展示所选 LotNo 的 customer、product、test type、test station、test attempt、LotEndTime range 和数据完整性摘要。
- Analysis Workspace 使用 `lot_scope` 表达单 Lot、多 Lot 或查询结果集合。
- 多 Lot 分析必须保留每个 LotNo 对应的测试类型、测试站点、测试次数、LotEndTime 和内部稳定数据引用；普通用户界面不使用 DataVersion policy 作为主控件。
- 跨批次分析至少覆盖良率趋势、Bin 对比、参数分布、Lot-to-lot delta、重测收益和异常 Lot 识别。
- 多 Lot 查询必须受查询预算限制，超预算时转异步任务或返回 partial data。

## 前端分层

```text
src/
├── app/
│   ├── routes/
│   ├── providers/
│   └── shell/
├── pages/
├── widgets/
├── features/
├── entities/
└── shared/
    ├── api/
    ├── auth/
    ├── permissions/
    ├── url-state/
    ├── options/
    ├── query/
    ├── telemetry/
    ├── ui/
    └── utils/
```

依赖方向和具体边界以 [前端技术架构](frontend-tech-architecture.md) 为准。页面不按后端服务、数据库表或 CRUD 模板组织。

## 状态策略

- 服务端数据通过 API client 获取，不在页面中散写 fetch。
- 大型图表数据使用浅层状态或不可变引用，避免深代理开销。
- 每个页面必须声明 URL 状态契约，区分 Required URL State、Session State 和 Forbidden URL State。
- 筛选条件应按页面契约序列化到 URL，便于分享和恢复；超长选择集合进入 workspace/session。
- 登录态、当前用户、权限范围集中管理。
- 分析工作区状态与普通页面状态分离。
- 分析工作区必须保存 `lot_scope`、test dimension、LotEndTime range、query snapshot、分析视图、图表布局、导出设置和未保存状态。
- 用户修改筛选、test dimension、lot_scope 或 LotEndTime 范围后，前端必须取消旧请求或忽略过期响应。
- 从历史 workspace、case、export 打开结果时，默认使用当时固化的内部数据引用，不静默切到新的解析结果。

## 组件和交互策略

- 页面级操作、区域级操作、表格批量操作、行级操作必须位置明确，不混用。
- Toast、banner、inline error、modal 的使用场景必须区分，错误和风险优先使用 inline 或 banner 表达。
- 抽屉用于辅助查看、轻量编辑、drilldown 和配置预览；复杂流程进入独立页面或工作区。
- 表格、图表、表单、Options 控件必须支持 loading、empty、error、permission denied 状态。
- 禁用按钮必须说明原因，不能只显示灰色。

## 图表策略

- 图表组件只负责渲染和交互事件。
- 数据聚合和统计计算由后端完成。
- 图表输入必须有最大点数约束。
- 大图表支持 loading、empty、error、partial data 状态。
- 图表必须显示指标名称、单位、LotEndTime 范围、测试类型、测试站点、数据口径和聚合粒度。DataVersion 等内部引用默认不作为普通图表主标签。
- 图表选中状态与表格联动时，当前选中范围必须可见，并能 reset。
- 导出大数据走后端异步任务，小图可在前端导出图片。

## Options 和结构化输入

- 客户、产品、测试类型、测试站点、测试次数、设备类型、程序名、参数名、Bin、规则版本等字段来自后端 options API 或受控配置。
- 上游字段变化后，下游 options 必须重新加载并校验已选值。
- 已选值变为 deprecated、hidden、unauthorized 或不存在时，必须保留可解释状态，不静默清空。
- 搜索型 options 必须 debounce，并避免旧搜索响应覆盖新搜索结果。

## API 协作

前端依赖后端提供面向页面任务的 API：

- Overview summary。
- Data Explorer lot/file/run/lineage query。
- Analysis Workspace datasets and session。
- Multi-lot analysis query and workspace context。
- Query snapshot and internal data reference metadata。
- Options API with permission-aware states。
- Alerts investigation context。
- Jobs and exports status。
- Governance profile/rule/template management。

前端不直接理解数据库表结构。

前端新增或修改页面时，必须同步更新：

- 后端 API 契约。
- 字段取值范围、默认值和编码约束。
- URL 状态契约、query snapshot、权限脱敏、查询预算和降级策略。
- 错误、空状态、loading、permission denied、partial data、stale、over budget、async running 状态。
- [前后端同步设计](../architecture-design/frontend-backend-sync-design.md) 中对应功能切片。
