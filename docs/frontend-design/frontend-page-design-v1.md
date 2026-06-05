# 前端页面设计 V1

本文记录 STDAS 第一阶段前端页面设计候选。当前只有登录页和身份会话最小链路已恢复为实现基线；登录后的正式工程入口、route、导航和页面族仍需用户确认下一张设计稿后再固化。

本文是前端页面事实来源，不替代后端 API 契约、数据架构、权限模型或架构 ADR。涉及 API 字段、错误码、查询预算、DataVersion、QuerySnapshot、Evidence、权限和 Job 状态机时，仍以对应后端文档为准。

## 1. 设计定位

STDAS 前端是内部测试数据分析工作台，不是展示型官网、通用 Admin Template、BI 大屏、MES/ERP/WMS，也不是外部客户 Portal。

第一阶段面向 FT 测试系统，BI、BIT、SLT 等成品形态相关测试按客户/产品流程纳入 TestType / TestStation 语义；CP 晶圆测试系统暂不进入默认页面。WaferLot / WaferNo / X / Y 仅在 FT 数据实际提供且分析场景需要时作为可选分析字段出现，不作为 Lot List 或全局上下文主维度。

页面设计目标：

- 让工程师快速定位异常 LotNo、Bin、参数、机台、Handler、Site、Test Program、TestType、TestStation、TestAttempt 和 LotEndTime。
- 保持上下文连续：customer、product、TestType、TestStation、TestAttempt、LotEndTime range、lot_scope、source entry、applied filters 不隐式丢失。
- 明确数据可信状态：live、cached、snapshot、partial、stale、over budget、async running、permission denied、hidden、masked、unauthorized、not found。
- 支撑多 Lot、多参数、多图联动分析，并将结论沉淀为 Workspace、Evidence、Export 或 Investigation Case。
- 将工程页面和治理页面分离；普通工程页面不提供 DataProfile、DataVersion policy 或 user role 选择器。

## 2. 行业经验转译

公开行业产品只作为能力和信息架构参考，不复制品牌、界面、文案或受保护视觉。

| 来源 | 可借鉴能力 | STDAS 转译 | 禁止照搬 |
|------|------------|------------|----------|
| PDF Solutions Exensio Analytics Platform | 面向 Foundry、IDM、OSAT、Fabless 的端到端半导体数据分析，覆盖 manufacturing、test、assembly、in-field 数据连接和可视化分析 | STDAS 需要连接测试文件、MES 上下文、解析规则、DataVersion、QuerySnapshot、Evidence 和 Export，而不是单一报表系统 | 不出现 Exensio、Spotfire、PDF Solutions 品牌；不照搬页面布局 |
| PDF Solutions Manufacturing Analytics | yield ramp、root cause、guided analytics、online/offline rules、parametric trigger、bin/yield statistical rules | Analysis Workspace 要提供 guided investigation、查询预算、规则触发上下文、Evidence 版本和 root cause 路径 | 不引入无限自由分析；所有高成本查询必须有预算和异步路径 |
| Gubo OneData | 设计到量产的数据整合、NPI/量产爬坡、良率管理、Spec 到 ATE 测试数据闭环、工程数据分析、Fail Bin、相关性、异常批次管理 | STDAS 借鉴产品生命周期追溯、良率多角度分析、初复测/设备因素定位、模板化工程分析、跨阶段追溯思路 | 不把 STDAS 做成芯片设计公司外部协作平台；不默认引入晶圆主视图 |
| 本地 SAS 生产系统参考 | Lot 查询、Yield Summary、Bin Summary、Param DUT/Chip、Test Step、Tester、Handler、Test Program、Raw Data Export | STDAS 工程页面必须保留真实 FT 字段、导出入口、Lot Detail 追溯和可选位置分析入口 | 不复制旧技术栈、旧视觉或过时交互 |

落地原则：

- Data Explorer 不是 CRUD 表，而是 Lot / Run / File / DataVersion 的高密度对象浏览和多 Lot 选择入口；是否作为登录后的正式工程入口仍需后续设计确认。
- Analysis Workspace 是核心，不把 Yield、Bin、Parametric、Retest、SPC、Correlation 拆成孤立一级页面。
- Alerts & Investigation 必须把告警、规则版本、触发上下文、QuerySnapshot、Evidence 和 Case 串起来。
- Jobs & Exports 必须承载异步分析、大导出、摄入任务、失败诊断、重试/取消/过期，而不是简单任务列表。
- System Governance 必须面向版本、diff、影响范围、发布、回滚和审计，而不是普通 settings 表单。

参考链接：

- [PDF Solutions Exensio Analytics Platform](https://www.pdf.com/products/exensio-analytics-platform/overview/)
- [PDF Solutions Manufacturing Analytics](https://www.pdf.com/products/exensio-analytics-platform/modules/manufacturing-analytics/)
- [Gubo OneData English](https://www.guwave.com/en/onedata/)
- [Gubo OneData 中文](https://www.guwave.com/onedata/)

## 3. 信息架构总览

```text
STDAS Frontend
├── Session Entry
│   └── Login
├── Global Shell
│   ├── App Shell
│   ├── Context Bar
│   ├── Global Search
│   ├── Notification Center
│   ├── Job Queue Drawer
│   └── User / Session Menu
├── Engineering Workbench
│   ├── Data Explorer
│   │   ├── Lot List
│   │   ├── Lot Detail
│   │   ├── Run Detail
│   │   ├── File Detail
│   │   ├── DataVersion Trace
│   │   ├── Product / Program Browser
│   │   └── Saved Filters
│   ├── Analysis Workspace
│   │   ├── Workspace Home
│   │   ├── Workspace Detail
│   │   ├── Yield Analysis View
│   │   ├── Bin Analysis View
│   │   ├── Parametric Analysis View
│   │   ├── Retest Analysis View
│   │   ├── SPC / Capability View
│   │   ├── Correlation / Root Cause View
│   │   ├── Optional Position Analysis View
│   │   ├── Template Apply / Save Flow
│   │   └── Evidence Capture Flow
│   ├── Alerts & Investigation
│   │   ├── Alert List
│   │   ├── Alert Detail
│   │   ├── Investigation Case List
│   │   ├── Investigation Case Detail
│   │   ├── Evidence Detail
│   │   └── Rule Trigger Context
│   └── Jobs & Exports
│       ├── Job List
│       ├── Job Detail
│       ├── Export List
│       ├── Export Detail
│       ├── Export Wizard
│       └── Dead Letter / Replay Console
├── System Governance
│   ├── Users & Access
│   ├── Customers
│   ├── Products & Test Flow
│   ├── DataProfiles
│   ├── Profile Resolution
│   ├── Ingestion Sources
│   ├── Parser / Mapping / Spec Rules
│   ├── Alert Rules
│   ├── Analysis Templates
│   ├── Report Templates
│   ├── Feature Flags
│   ├── Data Retention / DataVersion Governance
│   ├── Audit Log
│   └── System Health
└── Independent Pages
    ├── 403 Permission Denied
    ├── 404 / Object Not Found
    ├── Expired Share / Export Link
    ├── Session Expired
    ├── Maintenance / Degraded Mode
    ├── Preflight / Environment Check
    └── Not Suitable Viewport
```

## 4. 全局布局与设计系统

### 4.1 Figma 文件结构

后续进入 Figma 时按以下结构建立稳定设计源：

| Figma Page | 内容 |
|------------|------|
| `00 Cover` | 设计目标、状态、使用文档、版本、待确认项 |
| `01 Foundations` | color、typography、spacing、radii、elevation、layout widths、status colors、density modes |
| `02 Components` | shell、navigation、buttons、inputs、filters、tables、charts、badges、alerts、dialogs、drawers、job widgets |
| `03 Patterns` | context bar、filter bar、dense grid、chart panel、query summary、permission state、empty/error/partial/stale patterns |
| `04 Screens` | Login、Data Explorer、Lot Detail、Analysis Workspace、Alerts、Jobs、Governance 和独立页面 |
| `05 Review Notes` | design delta、截图检查、Rejected variants、open questions |

### 4.2 视觉方向

- 主题：浅色、冷静、工程化、长时间工作友好。
- 密度：桌面优先，信息密度高但分区清晰；默认适配 `1366x768`、`1440x900`、`1920x1080`。
- 字体：正文优先使用系统可读字体；数据列、ID、code 使用等宽或 tabular number 风格。
- 配色：背景中性、表面轻层级、状态色克制；禁止紫蓝渐变、官网 hero、装饰性大面积插画。
- 圆角：工作台控件和卡片默认小圆角，避免营销式大圆角卡片。
- 图表：必须有标题、单位、坐标轴、图例、query/status metadata、工具栏；不能只有抽象色块。
- 表格：主键列和操作列可固定；列密度高；数值右对齐；状态 badge 和 tooltip 可解释。
- 动效：只用于反馈和状态变化，不做炫技动画；尊重 reduced motion。

### 4.3 全局组件

| 组件 | 位置 | 设计要求 |
|------|------|----------|
| `AppShell` | 登录后所有页面 | 左侧主导航 + 顶部状态栏 + 主内容区；不嵌套卡片式大容器 |
| `ContextBar` | 顶部或页面内 | 展示当前 customer、product、TestType、TestStation、TestAttempt、LotEndTime range、Snapshot/Freshness；可编辑项由页面 filter bar 承载 |
| `GlobalSearch` | 顶栏 | 搜索 LotNo、CustomerLotNo、File、Job、Alert、Case、Workspace；结果按权限裁剪 |
| `JobQueueDrawer` | 全局右侧抽屉 | 显示 running/failed/export ready；可进入 Jobs & Exports 详情 |
| `NotificationCenter` | 顶栏 | 告警、导出完成、权限变化、系统降级；不可替代可追踪 Job 页面 |
| `PermissionStateBanner` | 页面/区域 | 区分 no page permission、no customer data permission、hidden、masked、unauthorized、not_found |
| `ResultStateBanner` | 查询结果区 | 表达 loading、snapshot、partial、stale、over budget、cache hit、async running |
| `QuerySummary` | 图表/表格附近 | 展示 query id/hash/snapshot、LotEndTime range、lot_scope、数据口径、聚合粒度、预算结果 |
| `LotScopeSelector` | Data Explorer / Analysis | 区分单 Lot、多 Lot、当前筛选结果集合、历史 snapshot 集合 |
| `EvidencePanel` | Analysis / Investigation | 保存图表/表格证据，绑定 QuerySnapshot、DataVersion set、Evidence version |

### 4.4 全局状态规则

| 状态 | 页面表现 |
|------|----------|
| 未登录 | 跳转 `/login`，保留 intended route；不展示工作台壳层 |
| 无页面权限 | 渲染 403 独立页；不渲染主体数据 |
| 无客户数据权限 | 显示权限状态，按策略决定是否展示客户名或命中数 |
| 数据被隐藏 | 区分 hidden / masked / unauthorized / not_found，不用 empty state 掩盖 |
| stale / snapshot | 在 KPI、图表、表格和导出元数据附近显示生成时间和原查询摘要 |
| over budget | 提供缩小范围、聚合、采样、转异步 job 三类动作 |
| async running | 结果区和全局 JobQueue 同步展示 job id、阶段、进度和可取消状态 |
| 旧响应返回 | 前端忽略并保留当前 applied filters；必要时提示 pending changes |

## 5. 页面设计契约

### 5.1 Login / Session Entry

| 项 | 设计 |
|----|------|
| Route | `/login` |
| 页面族 | Session Entry，独立于 AppShell |
| 用户目标 | 工厂内部工程师或管理员使用账号密码进入系统 |
| 主布局 | 右侧紧凑登录面板；左侧抽象测试数据视觉，例如 signal trace、die grid、yield curve，不展示真实功能预览 |
| 表单 | Factory Account、Password、Remember Account、Sign In、Password Help |
| 禁止 | SSO、环境选择、客户选择、DataProfile、DataVersion、UserRole、工厂租户选择、底部营销链接 |
| 状态 | idle、submitting、invalid credentials、locked/limited、maintenance、session expired |
| API | 已实现：`POST /api/v1/auth/login`、`GET /api/v1/auth/me`；后续：`POST /api/v1/auth/refresh` |
| 验收 | 登录页不能泄露业务数据；不能像官网 hero；不能出现 post-login 表格或 KPI |

当前实现说明：登录成功后进入临时空白工作区，用于验证 auth 链路；该占位页不是 AppShell 或 Data Explorer 的正式视觉基线。

### 5.2 App Shell / Current Context

| 项 | 设计 |
|----|------|
| Route | 登录后所有 `/app/*` |
| 主导航 | Data Explorer、Analysis Workspace、Alerts & Investigation、Jobs & Exports、System Governance |
| 顶栏 | STDAS、当前用户、CustomerScope 摘要、当前业务上下文摘要、Global Search、Notification、JobQueue、Session Menu |
| ContextBar | 页面按场景展示 customer、product、TestType、TestStation、TestAttempt、LotEndTime range；可编辑上下文主要在页面 filter bar |
| 权限 | 当前用户和 CustomerScope 全局可见；不提供 user role selector |
| 任务 | 长耗时分析、摄入、导出、重放必须可从 JobQueue 进入详情 |
| 响应式 | 窄屏折叠左导航；复杂分析页允许提示不适合当前视口 |

### 5.4 Data Explorer / Lot List

| 项 | 设计 |
|----|------|
| Route | `/app/data/lots` |
| 用户目标 | 搜索、筛选、比较和选择 Lot，进入详情或分析 |
| 页面结构 | Filter bar + active filter chips + dense grid + selection summary + right detail drawer |
| 主筛选 | LotNo、CustomerLotNo、Product、TestType、TestStation、TestAttempt、LotEndTime、Yield range、Bin、Program、Tester、Handler、status |
| Grid | 服务端分页/排序/筛选；pinned LotNo 和 row actions；支持列配置 |
| 核心列 | LotNo、CustomerLotNo、Product、TestType、TestStation、TestAttempt、Yield、Bin1-Bin8 summary、Test Program、Tester、Handler、File Count、LotEndTime、Permission State |
| 多选 | 当前页选择、手动选择、当前筛选结果选择必须区分；跨页选择显示影响范围 |
| 批量动作 | Add to Analysis Workspace、Compare Lots、Export Yield Summary、Export Raw Data、Save Filter |
| 状态 | loading、empty、permission denied、hidden、partial、over budget、stale |
| API | `GET /api/v1/data/lots`、options APIs、`POST /api/v1/analysis/workspaces` 或 workspace draft API |
| 验收 | 不一次性渲染无限数据；不以 DataVersion 作为普通主列；不暴露 DataProfile 主控件 |

### 5.5 Lot Detail

| 项 | 设计 |
|----|------|
| Route | `/app/data/lots/:lot_id` |
| 用户目标 | 查看单 Lot 的测试结果、文件组合、Run、Bin、参数摘要和追溯状态 |
| Header | LotNo、CustomerLotNo、Product、Customer、TestType、TestStation、TestAttempt、LotEndTime、status、latest committed / historical badge |
| Summary | Yield、Retest Rate、Top Fail Bin、Device Count、Test Duration、File Count、Data Freshness |
| Tabs | Yield of Tests、Bin Summary、Parametric Summary、Run & Test Step、Files & Lineage、DataVersion History、Optional Position |
| 图表 | Yield by run/test step、Bin Pareto、Parametric distribution、SPC mini chart、optional position map |
| 动作 | Add to Workspace、Open Latest QuerySnapshot、Export Lot Report、Open File Detail、Open DataVersion Trace |
| 状态 | latest committed、historical、stale、partial file set、permission hidden/masked |
| API | `GET /api/v1/data/lots/{lot_id}`、runs/files/versions APIs |
| 验收 | 从列表返回时恢复筛选、排序、分页、滚动和已选项 |

### 5.6 Run Detail

| 项 | 设计 |
|----|------|
| Route | `/app/data/lots/:lot_id/runs/:run_id` 或由 Lot Detail drawer 承载 |
| 用户目标 | 查看一次测试运行、测试文件组合、机台、Handler、Site、Program Version 和结果口径 |
| 内容 | Run metadata、Tester、Handler、Site summary、Test Program、start/end time、test step、file list、yield/bin/parametric summary |
| 状态 | partial run、missing MES context、superseded run、permission restricted |
| API | `GET /api/v1/data/lots/{lot_id}/runs`、`GET /api/v1/data/runs/{run_id}` |

### 5.7 File Detail

| 项 | 设计 |
|----|------|
| Route | `/app/data/files/:file_id` |
| 用户目标 | 查看原始测试文件、解析状态、指纹、错误和 lineage |
| 内容 | file name、hash、source、size、format、ingestion job、parser version、mapping version、parse errors、related Lot/Run/DataVersion |
| 动作 | Open ingestion job、Open DataVersion、Download if permitted、Replay parse if admin |
| 状态 | raw、parsed、normalized、failed、quarantined、hidden/masked |
| API | `GET /api/v1/data/files/{file_id}` |

### 5.8 DataVersion Trace

| 项 | 设计 |
|----|------|
| Route | `/app/data/versions/:data_version` |
| 用户目标 | 追溯稳定数据版本如何由文件、规则和解析过程生成 |
| 内容 | DataVersion status、created/ready time、raw file hashes、parser/mapping/spec version、DataProfile version、lineage graph、consuming QuerySnapshots/Exports/Evidence |
| 动作 | Compare versions、Open lineage、Open consuming query、Request reparse if admin |
| 状态 | committed、superseded、deprecated、failed、permission denied |
| API | `GET /api/v1/data/versions/{data_version}`、`GET /api/v1/data/versions/{data_version}/lineage` |
| 验收 | DataVersion 只在追溯/诊断/治理上下文显性展示，不反向污染普通工程入口 |

### 5.9 Product / Program Browser

| 项 | 设计 |
|----|------|
| Route | `/app/data/products`、`/app/data/programs` 或 Data Explorer 次级 tab |
| 用户目标 | 浏览产品、Device、Test Program、Program Version 与 Lot/Run 的覆盖情况 |
| 内容 | product list、program list、version timeline、related lots、yield delta around program change |
| 动作 | Filter Lot List、Open Analysis Workspace with program grouping |
| 状态 | option deprecated、hidden product、no authorized lots |
| API | options APIs、`GET /api/v1/data/lots` with grouping |

### 5.10 Saved Filters

| 项 | 设计 |
|----|------|
| Route | `/app/data/saved-filters` 或 Data Explorer drawer |
| 用户目标 | 保存和复用常用 Lot 查询条件 |
| 内容 | filter name、owner、scope、query summary、last used、permission visibility |
| 动作 | Apply、Duplicate、Delete、Share within scope |
| 状态 | stale options、permission changed、invalid filter |

### 5.11 Analysis Workspace Home

| 项 | 设计 |
|----|------|
| Route | `/app/analysis` |
| 用户目标 | 从 Lot selection、模板、历史 workspace 或 query snapshot 进入分析 |
| 内容 | Recent Workspaces、Templates、Start from Lot Scope、Start from QuerySnapshot、Open jobs |
| 动作 | New Workspace、Open Workspace、Apply Template、Import Lot Selection |
| 状态 | no recent workspace、template permission denied、stale snapshot |

### 5.12 Analysis Workspace Detail

| 项 | 设计 |
|----|------|
| Route | `/app/analysis/:workspace_id` 或 `/app/analysis?query_snapshot_id=...` |
| 用户目标 | 执行跨 Lot、多参数、多图联动分析并保存结论 |
| 布局 | Top toolbar + left config panel + central resizable canvas + right evidence/inspector panel |
| Toolbar | Run Query、Save Workspace、Save as Template、Export、Add Evidence、Create Case、Reset、Compare DataVersion |
| Left Panel | Lot Scope、Analysis Type、Parameters、Group By、Filters、Advanced DataVersion metadata、Query Budget |
| Canvas | Query summary、result state banner、chart panels、result table、linked selection、drilldown |
| Right Panel | selected anomaly、related Lots、Evidence draft、Case link、permission/redaction notes |
| 状态 | applied filters vs pending changes、QuerySnapshot、DataVersion set、partial/stale/cache、over budget、async running、unsaved changes |
| API | `POST /api/v1/analysis/queries/run`、query snapshot、workspace、exports、evidence APIs |
| 验收 | 每个结果区都能追溯 QuerySnapshot；旧查询响应不能覆盖新状态；离开未保存工作区必须提示 |

### 5.13 Analysis Views

| View | 目标 | 必备组件 | 必备元数据 |
|------|------|----------|------------|
| Yield Analysis | 比较 Lot、TestStation、Tester、Handler、Program 的良率趋势 | Yield trend、lot table、delta panel、drilldown | QuerySnapshot、LotEndTime range、yield definition、aggregation |
| Bin Analysis | 定位 Fail Bin 贡献和变化 | Pareto、stacked trend、bin transition、top bin table | Hard/Soft Bin、count/%、pass/fail property |
| Parametric Analysis | 看参数分布、漂移、Cpk/SPC | histogram、box plot、CDF、SPC chart、outlier table | parameter、unit、spec limits、n、Cpk/Cp |
| Retest Analysis | 判断初测/复测收益和无效复测 | retest funnel、recovery matrix、INI/RT/MER table | TestAttempt、retest definition、program/test station |
| SPC / Capability | 监控长期稳定性 | control chart、capability summary、rule violation list | control limits、spec limits、sampling/aggregation |
| Correlation / Root Cause | 做多参数、多对象相关性和异常定位 | scatter、correlation matrix、ANOVA/table、guided checklist | selected variables、group by、statistical caveats |
| Optional Position | 仅在数据提供位置字段时分析空间分布 | position map / die map、site/bin overlay、linked table | WaferLot/WaferNo/X/Y availability、missing coordinate state |

所有 Analysis View 都必须：

- 显示 lot_scope、query summary、DataVersion set 或可展开引用。
- 显示 loading、empty、error、partial、stale、over budget 状态。
- 支持 Add Evidence、Export、Open related Lot、Reset selection。
- 将高成本查询转异步 job 或返回 partial data，不阻塞 UI。

### 5.14 Template Apply / Save Flow

| 项 | 设计 |
|----|------|
| 入口 | Analysis Workspace toolbar、Workspace Home、Governance Templates |
| 用户目标 | 复用工程分析路径，或将当前工作区保存为模板 |
| Apply | 显示模板适用条件：customer/product/TestType/TestStation/DataProfile/rule version；冲突时列出差异 |
| Save | name、description、scope、owner、required fields、default chart layout、expected query budget |
| 状态 | incompatible context、deprecated template、permission denied、draft vs published |
| API | governance/template APIs、workspace APIs |

### 5.15 Evidence Capture Flow

| 项 | 设计 |
|----|------|
| 入口 | Analysis Workspace、Alert Detail、Case Detail |
| 用户目标 | 将某个图表、表格、查询摘要或异常点保存为调查证据 |
| 内容 | Evidence title、source panel、QuerySnapshot、DataVersion set、generated time、selected filters、notes |
| 动作 | Add to existing case、Create case、Save evidence draft、Discard |
| 状态 | evidence version、permission changed、source data stale、recompute creates new version |
| API | `POST /api/v1/investigation/evidence`、case APIs |

### 5.16 Alert List

| 项 | 设计 |
|----|------|
| Route | `/app/alerts` |
| 用户目标 | 分流 yield/bin/parameter/data quality 告警，找到需要调查的对象 |
| Filter | customer、product、severity、status、rule version、time range、TestType、TestStation、owner |
| Table | severity、rule name、rule version、trigger metric、threshold、actual value、related LotNo、TestType/TestStation、LotEndTime、status、owner |
| 动作 | Acknowledge、Assign、Open Detail、Open Analysis、Create Case |
| 状态 | trigger context stale、permission hidden、rule deprecated |
| API | `/api/v1/alerts/*` |

### 5.17 Alert Detail / Rule Trigger Context

| 项 | 设计 |
|----|------|
| Route | `/app/alerts/:alert_id` |
| 用户目标 | 理解告警为什么触发，跳转到可复现分析 |
| 内容 | alert header、rule version、trigger context、related Lots、metric trend、threshold line、query/data refs、status timeline |
| 动作 | Acknowledge、Close、Reopen、Assign、Open Workspace、Add Evidence、Create Case |
| 状态 | rule version changed、related data hidden、snapshot stale |
| API | alert detail、query snapshot、evidence APIs |

### 5.18 Investigation Case List

| 项 | 设计 |
|----|------|
| Route | `/app/cases` |
| 用户目标 | 管理异常调查案例、责任人和状态 |
| Table | case id、title、severity、status、owner、related alerts/lots、latest evidence version、created/updated time |
| Filter | customer、product、status、owner、severity、time range、related rule |
| 动作 | Open、Create、Assign、Close |

### 5.19 Investigation Case Detail

| 项 | 设计 |
|----|------|
| Route | `/app/cases/:case_id` |
| 用户目标 | 在一个上下文中组织告警、证据、结论、任务和导出 |
| 布局 | Case header + evidence board + timeline + notes/conclusion panel + related objects |
| 内容 | Evidence cards、chart/table snapshots、QuerySnapshot summaries、DataVersion sets、recompute history、audit |
| 动作 | Add evidence、Recompute evidence、Link alert/job/export、Close/Reopen、Export case report |
| 状态 | evidence stale、permission changed、masked evidence、new evidence version |
| API | case APIs、evidence APIs、exports |

### 5.20 Evidence Detail

| 项 | 设计 |
|----|------|
| Route | `/app/cases/:case_id/evidence/:evidence_id` |
| 用户目标 | 查看一条证据的冻结查询、图表/表格结果和版本关系 |
| 内容 | evidence id/version、source page、QuerySnapshot、DataVersion set、generated by/time、visual/table payload、notes |
| 动作 | Open source workspace、Recompute as new version、Export evidence |
| 状态 | historical snapshot、current recompute available、data hidden/masked |

### 5.21 Jobs List

| 项 | 设计 |
|----|------|
| Route | `/app/jobs` |
| 用户目标 | 追踪摄入、分析、导出、重试、回放任务 |
| Filter | job type、status、customer、product、owner、created time、request id、correlation id |
| Table | job id、type、status、stage、progress、owner、created/updated/completed/expires、attempt、query snapshot、last error |
| 动作 | Open Detail、Cancel、Retry、Replay、Open Export、Open QuerySnapshot |
| 状态 | queued、running、succeeded、failed、canceling、canceled、expired、retry_scheduled、dead_letter |
| API | `/api/v1/jobs/*` |

### 5.22 Job Detail

| 项 | 设计 |
|----|------|
| Route | `/app/jobs/:job_id` |
| 用户目标 | 定位任务失败原因、查看阶段、执行重试或取消 |
| 内容 | stage timeline、request summary、principal/customer scope、query snapshot、DataVersion set、logs excerpt、last error、retry policy |
| 动作 | Cancel、Retry、Replay、Open related file/export/workspace |
| 状态 | cannot cancel reason、retry uses same QuerySnapshot vs re-resolve latest committed |

### 5.23 Export List / Detail

| 项 | 设计 |
|----|------|
| Route | `/app/exports`、`/app/exports/:export_id` |
| 用户目标 | 查看和下载导出结果，理解导出范围和权限状态 |
| List | export id、type、source、owner、status、format、rows estimate/actual、created/completed/expires、permission state |
| Detail | QuerySnapshot、DataVersion set、filters、columns、masking state、download history、expiry |
| 动作 | Download、Regenerate、Create share token、Open source workspace |
| 状态 | ready、expired、permission changed、file missing、masked、download denied |
| API | `/api/v1/exports/*` |

### 5.24 Export Wizard

| 项 | 设计 |
|----|------|
| 入口 | Data Explorer、Analysis Workspace、Case Detail |
| 用户目标 | 配置 Yield Summary、Raw Data、Parametric、Case Report 等导出 |
| Steps | Source Summary -> Fields/Columns -> Format/Masking -> Budget/Async -> Confirm |
| 状态 | over budget、requires async、permission masked、expired source snapshot |
| 验收 | 导出前显示影响范围、行数估计、DataVersion set、是否脱敏、过期时间 |

### 5.25 Dead Letter / Replay Console

| 项 | 设计 |
|----|------|
| Route | `/app/jobs/dead-letter` |
| 用户 | Admin |
| 目标 | 处理摄入、事件、导出或分析任务死信 |
| 内容 | event/job id、service、error code、payload summary、attempts、correlation id、created time |
| 动作 | Replay、Mark resolved、Open source object |
| 状态 | replay impact、permission/admin scope、audit required |

### 5.26 Users & Access

| 项 | 设计 |
|----|------|
| Route | `/app/governance/users` |
| 用户 | Admin |
| 目标 | 管理用户、角色、CustomerScope、permissions、会话和审计 |
| 内容 | users table、scope editor、permission matrix、session activity、audit trail |
| 状态 | role disabled、scope conflict、session expired、permission denied |
| 禁止 | 把 user role 当成普通工程页面选择器 |

### 5.27 Customers

| 项 | 设计 |
|----|------|
| Route | `/app/governance/customers` |
| 目标 | 管理客户、产品、设备、测试流程、可见范围和配置入口 |
| 内容 | customer list、product list、TestType/TestStation definitions、effective status、linked DataProfiles |
| 动作 | Create draft、Edit、Deprecate、Open Profile Resolution、View impact |
| 状态 | published/draft/deprecated、dependency exists、permission denied |

### 5.28 Products & Test Flow

| 项 | 设计 |
|----|------|
| Route | `/app/governance/products` |
| 目标 | 管理产品、TestType、TestStation、TestAttempt 口径和 Program 关联 |
| 内容 | product tree、test flow diagram、station definitions、attempt merge rules、program versions |
| 状态 | conflict、deprecated station、effective time |

### 5.29 DataProfiles

| 项 | 设计 |
|----|------|
| Route | `/app/governance/profiles` |
| 目标 | 管理解析、映射、规格、规则和模板组合配置 |
| Table | profile id、customer、product、TestType、TestStation、equipment、file format、program、version、state、effective time |
| Detail | parser/mapping/spec/rule/template refs、draft/published/deprecated、diff、impact analysis、validation |
| 动作 | Create draft、Validate、Publish、Schedule、Deprecate、Rollback、Copy/Fork |
| 状态 | blocking errors、warnings、version conflict、concurrent edit |

### 5.30 Profile Resolution

| 项 | 设计 |
|----|------|
| Route | `/app/governance/profile-resolution` |
| 目标 | 输入解析键，预览命中的 DataProfile、ParserRule、MappingRule、SpecRule、Template |
| 输入 | customer、product、TestType、TestStation、equipment_type、file_format、program_name、program_version、LotEndTime/effective time |
| 输出 | matched profile、fallback chain、overrides、warnings、why matched |
| 动作 | Open profile、Run sample parse、Save diagnostic |

### 5.31 Ingestion Sources

| 项 | 设计 |
|----|------|
| Route | `/app/governance/ingestion` |
| 目标 | 管理采集源、上传策略、文件格式、安全限制和摄入任务 |
| 内容 | source list、watch path/upload channel、format、customer binding、schedule、last job、failure rate |
| 动作 | Enable/disable、Test connection、Open jobs、Replay failed |
| 状态 | unavailable、permission denied、quarantined file、dead letter |

### 5.32 Parser / Mapping / Spec Rules

| 项 | 设计 |
|----|------|
| Route | `/app/governance/rules/parser`、`/mapping`、`/spec` |
| 目标 | 管理文件解析、字段映射、规格限和 Bin 语义 |
| 内容 | version list、rule editor、sample input/output、diff、validation、impact lots |
| 动作 | Validate、Publish、Fork、Deprecate、Rollback |
| 状态 | blocking validation、warning、dependent profiles、audit |

### 5.33 Alert Rules

| 项 | 设计 |
|----|------|
| Route | `/app/governance/rules/alerts` |
| 目标 | 管理告警规则、阈值、触发窗口、适用范围和版本 |
| 内容 | rule list、condition builder、scope、sample evaluation、online/offline mode、version history |
| 动作 | Test rule、Publish、Disable、Open triggered alerts |
| 状态 | query budget risk、deprecated metric、permission scope |

### 5.34 Analysis Templates

| 项 | 设计 |
|----|------|
| Route | `/app/governance/templates/analysis` |
| 目标 | 管理工程分析模板，供 Analysis Workspace 应用 |
| 内容 | template list、applicability、layout preview、parameters、default budget、version history |
| 动作 | Publish、Deprecate、Open in workspace、Impact analysis |
| 状态 | incompatible context、missing parameters、permission denied |

### 5.35 Report Templates

| 项 | 设计 |
|----|------|
| Route | `/app/governance/templates/reports` |
| 目标 | 管理导出报告、客户格式和字段脱敏策略 |
| 内容 | template list、format、columns、masking policy、customer/product scope、version |
| 动作 | Preview、Publish、Deprecate、Generate sample |

### 5.36 Feature Flags

| 项 | 设计 |
|----|------|
| Route | `/app/governance/feature-flags` |
| 目标 | 控制客户/产品/测试类型可见功能和灰度能力 |
| 内容 | flag list、scope、effective time、owner、audit、related pages |
| 动作 | Enable、Disable、Schedule、Rollback |
| 验收 | 前端不写客户硬编码分支；差异来自 profile、feature flag 或 options API |

### 5.37 Data Retention / DataVersion Governance

| 项 | 设计 |
|----|------|
| Route | `/app/governance/data` |
| 目标 | 管理数据保留、DataVersion 生命周期、lineage、清理策略 |
| 内容 | retention rules、DataVersion states、storage summary、dependent QuerySnapshots/Evidence/Exports |
| 动作 | Archive、Restore metadata、Open lineage、View impact |
| 状态 | cannot delete due to evidence/export、permission denied、stale references |

### 5.38 Audit Log

| 项 | 设计 |
|----|------|
| Route | `/app/governance/audit` |
| 目标 | 查询关键治理动作、发布、导出、重放、权限变化和 Case 变更 |
| Filter | actor、action、resource、customer、time、correlation id |
| Table | audit id、actor、action、resource、before/after summary、time、request id |
| 状态 | masked fields、retention expired、permission denied |

### 5.39 System Health

| 项 | 设计 |
|----|------|
| Route | `/app/governance/system-health` |
| 目标 | 查看 gateway、services、cache、queue、storage、analytics backend 的可用性 |
| 内容 | health cards、degraded dependencies、recent failures、preflight result、logs/correlation ids |
| 动作 | Open job failures、Open observability links、Run preflight if allowed |

## 6. 独立页面

| 页面 | Route | 触发 | 设计要求 |
|------|-------|------|----------|
| 403 Permission Denied | `/403` 或 route fallback | 无页面权限、无数据 scope | 说明权限缺失类型；提供申请权限/切换客户/返回入口；不泄露敏感对象 |
| 404 / Object Not Found | `/404` 或 resource fallback | 路由不存在、对象不存在或按策略不可见 | 不确认未授权对象是否存在；提供返回和搜索 |
| Expired Share / Export Link | `/share/expired` 或 export fallback | share/export token 过期或权限变化 | 显示过期、权限变化、文件缺失的区别；不显示死链接 |
| Session Expired | `/session-expired` 或 modal page | token 过期、refresh 失败 | 保留 intended route；重新登录后可返回 |
| Maintenance / Degraded Mode | `/maintenance` 或 global banner | 依赖不可用、系统降级 | 说明受影响范围、可用缓存、不可用动作 |
| Preflight / Environment Check | `/app/preflight` | Phase 0/0.5 本地环境验证 | 仅用于准备和环境闭环，不承载业务功能 |
| Not Suitable Viewport | responsive fallback | 过窄视口打开复杂工作区 | 说明建议使用桌面视口；保留只读摘要或返回入口 |

## 7. Route 基线

```text
/login
/app/data/lots
/app/data/lots/:lot_id
/app/data/lots/:lot_id/runs/:run_id
/app/data/files/:file_id
/app/data/versions/:data_version
/app/data/products
/app/data/programs
/app/data/saved-filters
/app/analysis
/app/analysis/:workspace_id
/app/alerts
/app/alerts/:alert_id
/app/cases
/app/cases/:case_id
/app/cases/:case_id/evidence/:evidence_id
/app/jobs
/app/jobs/:job_id
/app/jobs/dead-letter
/app/exports
/app/exports/:export_id
/app/governance/users
/app/governance/customers
/app/governance/products
/app/governance/profiles
/app/governance/profile-resolution
/app/governance/ingestion
/app/governance/rules/parser
/app/governance/rules/mapping
/app/governance/rules/spec
/app/governance/rules/alerts
/app/governance/templates/analysis
/app/governance/templates/reports
/app/governance/feature-flags
/app/governance/data
/app/governance/audit
/app/governance/system-health
/app/preflight
/403
/404
/session-expired
```

路由实现可以按开发阶段裁剪，但新增、删除或合并 route 前必须回答：

1. 是否改变页面族边界？
2. 是否影响 Required URL State、Session State、Forbidden URL State？
3. 是否影响 API 契约、权限、QuerySnapshot、DataVersion、Job 或 Evidence？
4. 是否需要更新功能切片交付卡？

## 8. URL 状态契约汇总

| 页面族 | Required URL State | Session State | Forbidden URL State |
|--------|--------------------|---------------|---------------------|
| Login | intended route 可选 | remember account | token、password、scope |
| Lot List | customer、product、test_type、test_station、test_attempt、lot_end_time_range、filters、page、page_size、sort | 大批量 selection、列配置、滚动位置 | 超长 lot_scope、未保存筛选草稿 |
| Lot Detail | lot_id、tab、data_version 或 snapshot ref 可选 | tab 展开、返回状态 | 下载 token、未授权 file id |
| Analysis | workspace_id 或 query_snapshot_id、核心查询参数引用 | 未保存布局、brush、draft notes | 未保存表单、超长 lot_scope、敏感字段 |
| Alerts | filters、alert_id、case_id、rule_version | 局部展开、排序 | 未授权告警摘要 |
| Jobs / Exports | job_id、export_id、filters | 临时下载状态 | 过期 download token |
| Governance | resource id、version、tab | draft form、局部展开 | secret、token、未发布敏感配置 |

## 9. API 与后端影响

本设计新增或细化了以下前端需求，后续 API 契约必须按 [API 契约规则](../backend-design/api-contract-rules.md) 补字段级表格：

| 需求 | 相关页面 | 后端事实来源 |
|------|----------|--------------|
| 全局搜索 | AppShell | API 契约、权限与脱敏 |
| Saved Filters | Data Explorer | API 契约、权限与 URL 状态 |
| Run Detail | Lot Detail | data APIs、DataVersion lineage |
| Product / Program Browser | Data Explorer | options APIs、data query |
| Workspace Home | Analysis | workspace APIs、template APIs |
| Template Apply / Save Flow | Analysis / Governance | governance/template APIs |
| Evidence Capture Flow | Analysis / Investigation | analytics engine、security/reliability |
| Case List | Investigation | workflow APIs、permissions |
| Export Wizard | Jobs & Exports | exports APIs、job state machine |
| Dead Letter / Replay Console | Jobs / Governance | event reliability、job state machine |
| System Health | Governance | deployment/observability |

字段级 API 文档必须覆盖：

- required/default/range/encoding/example/error。
- CustomerScope 和 permission/redaction state。
- QuerySnapshot、DataVersion set、job id、export id、Evidence version。
- pagination、sorting、filtering、query budget、async fallback。
- share/open/download 时的权限重校验。

## 10. Figma 与 Mockup 顺序

后续视觉和 Figma 设计按以下顺序推进：

1. Login：建立基础品牌和抽象测试数据视觉。
2. Data Explorer / Lot List：建立 AppShell、ContextBar、dense grid、filter、selection、drawer、table/chart/status 语言。
3. Lot Detail / DataVersion Trace：固化 detail tabs、lineage、version/status 模式。
4. Analysis Workspace：固化核心工作区、图表/表格联动、Evidence 模式。
5. Alerts & Investigation：固化 Case、Evidence、Rule Trigger Context。
6. Jobs & Exports：固化 job lifecycle、export wizard、failure diagnostics。
7. System Governance：固化 version diff、impact analysis、publish/rollback/audit。
8. Independent Pages：固化 permission、expired、maintenance、viewport fallback。

`docs/frontend-design/page-mockups/Proposed1` 当前只能作为 candidate input。除非用户明确确认，不作为最终 visual baseline。

Figma screen 每个页面至少要有：

- default ready state。
- loading / empty / error / permission denied 之一。
- data trust 状态示例。
- 关键 drawer 或 dialog。
- `1366x768` 和 `1440x900` 验收截图；复杂工作区另加 `1920x1080`。

## 11. 实施切片建议

| 阶段 | 页面范围 | 目标 |
|------|----------|------|
| F0 | Login、AppShell、403/404/session expired、Preflight | 建立路由、身份、壳层、基础状态和验证闭环 |
| F1 | Lot List、Lot Detail 基础 | 打通主查询和主对象浏览 |
| F2 | Run/File/DataVersion Trace、Saved Filters | 补齐数据可信与追溯 |
| F3 | Analysis Workspace Home/Detail、Yield/Bin/Parametric/Retest 基础视图 | 打通多 Lot 分析主链路 |
| F4 | Evidence Capture、Alerts、Case Detail | 打通异常调查和证据沉淀 |
| F5 | Jobs、Exports、Export Wizard | 打通异步、大导出和失败诊断 |
| F6 | Governance Customer/Profile/Rules/Templates | 打通治理发布和版本影响分析 |
| F7 | Feature Flags、Data Governance、Audit、System Health、Dead Letter | 补齐运维治理与回归验收 |

每个阶段进入代码实现前，必须先有对应页面设计、API 契约、权限策略和验收方式。文档阶段不执行前端 build/test；代码阶段必须按项目验证 gate 执行。

## 12. 页面验收总清单

新增或修改任意页面前必须检查：

- 页面是否归属明确页面族。
- 是否满足本页面的主任务和核心用户路径。
- 是否展示 customer、product、TestType、TestStation、TestAttempt、LotEndTime range 等必要上下文。
- 是否避免普通工程页面出现 DataProfile、DataVersion policy、user role 主选择器。
- 是否处理 loading、empty、error、permission denied、hidden、masked、partial、stale、snapshot、over budget、async running。
- 是否声明 Required URL State、Session State、Forbidden URL State。
- 是否支持 back/forward、详情返回、分享或历史打开时的权限重校验。
- 表格是否服务端分页/排序/筛选或虚拟滚动。
- 图表是否有单位、坐标轴、图例、QuerySnapshot/metadata 和 reset。
- 危险操作是否二次确认并展示影响范围。
- 导出、保存、发布、重试、回放是否展示最终状态和审计上下文。
- 是否明确 API、Options API、QuerySnapshot、job/export/evidence id。
- 是否有响应式降级或 Not Suitable Viewport 策略。
- 是否有 Figma screenshot checklist 和 docs delta。
