# 前端技术架构

STDAS 前端是半导体测试数据分析工作台，不是通用后台管理系统、CRUD Admin Template 或展示型官网。第一阶段前端聚焦 FT 测试系统，必须支撑 LotNo / CustomerLotNo / TestType / TestStation / TestAttempt / LotEndTime / Bin / Parameter / QuerySnapshot / Investigation Case 等测试分析场景。

## 技术基线

| 项 | 选择 |
|----|------|
| Language | TypeScript |
| Framework | React + TypeScript |
| Build | Vite |
| Package Manager | pnpm |
| Rendering | CSR-first authenticated workbench |
| API | Typed API client over `stdas-gateway` `/api/v1/*` |
| Architecture | Feature-Sliced Analytical Workbench Architecture |
| UI Strategy | STDAS-owned semantic workbench components |
| Visualization | Grid Adapter + Chart Adapter；Wafer/position Adapter 仅作为可选分析能力 |

选择 React + TypeScript 的原因：

- 半导体测试分析工作台长期依赖复杂表格、图表、多图联动、wafer map、表单和工作区生态。
- React 在企业级数据网格、可视化、表单、测试和招聘维护方面风险更低。
- STDAS 需要长期演进，不应把核心工作台能力绑定到较弱的复杂组件生态。

禁止把 STDAS 前端实现成通用 Admin Template。React 只是渲染框架，产品架构仍然围绕半导体测试分析实体、功能和工作区组织。

前端包管理器统一使用 pnpm：

- 依赖安装使用 `pnpm install`。
- 脚本执行使用 `pnpm lint`、`pnpm typecheck`、`pnpm test`、`pnpm build`。
- 仓库只保留 `pnpm-lock.yaml`。

## 参考系统形态

STDAS 前端参考以下系统形态：

- Yield Management System。
- Semiconductor Test Data Analytics Workbench。
- Visual Analytics Platform。
- Root Cause Investigation Workspace。
- Report / Export / Case Workflow。

必须支持或预留的能力：

- STDF / ATDF / test file 数据浏览。
- LotNo / CustomerLotNo / Device 追溯。
- Yield / Bin / Parametric / Retest 分析。
- 可选 WaferLot / WaferNo / X / Y 分析，仅在客户 FT 数据实际提供且分析需要时启用。
- SPC / trend / distribution / correlation。
- Multi-lot comparison。
- Root cause investigation。
- QuerySnapshot / Evidence / Export。
- Customer-specific profile and rule governance。

## 架构模式

前端采用 Feature-Sliced Analytical Workbench Architecture。目录按页面工作区、分析功能、领域实体和共享基础能力组织，不按后端服务、数据库表或通用组件类型组织。

推荐结构：

```text
frontend/web/src/
├── app/
│   ├── routes/
│   ├── providers/
│   └── shell/
├── pages/
│   ├── data-explorer/
│   ├── analysis-workspace/
│   ├── alerts-investigation/
│   ├── jobs-exports/
│   └── governance/
├── widgets/
│   ├── context-bar/
│   ├── filter-panel/
│   ├── lot-grid/
│   ├── chart-panel/
│   ├── wafer-map/
│   ├── query-summary/
│   ├── result-state-banner/
│   ├── job-queue/
│   └── evidence-panel/
├── features/
│   ├── lot-filtering/
│   ├── lot-selection/
│   ├── data-version-picker/
│   ├── query-builder/
│   ├── yield-analysis/
│   ├── bin-analysis/
│   ├── parametric-analysis/
│   ├── wafer-map-analysis/
│   ├── export-job/
│   ├── evidence-capture/
│   └── profile-resolution/
├── entities/
│   ├── customer/
│   ├── product/
│   ├── lot/
│   ├── lot-run/
│   ├── test-file/
│   ├── data-version/
│   ├── query-snapshot/
│   ├── alert/
│   ├── job/
│   └── evidence/
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

依赖方向：

```text
app -> pages -> widgets -> features -> entities -> shared
```

规则：

- `shared` 不能依赖业务 feature、widget 或 page。
- `entities` 表达领域对象和轻量展示逻辑，不发起跨页面流程。
- `features` 表达用户动作，例如筛选、选择、运行查询、导出、保存证据。
- `widgets` 组合多个 feature/entity，形成工作台区域。
- `pages` 负责路由级组合、URL 状态和页面级错误边界。
- `app` 只承载全局 provider、shell、router 和启动配置。

## 状态模型

前端状态必须分层，不能混入单一全局 store。

| 状态类型 | 示例 | 位置 |
|----------|------|------|
| URL State | customer、product、test_type、test_station、lot_end_time_range、page、sort、filters、query_snapshot_id | `shared/url-state` + route |
| Server State | Lot、DataVersion、QuerySnapshot、Job、Options、Permission State | `shared/query` + `shared/api` |
| Workspace State | lot_scope、DataVersion policy、layout、unsaved changes、chart selections、draft notes | `features/*` 或 `pages/analysis-workspace` |
| Local UI State | drawer open、tab、hover、temporary menu、popover position | component local state |
| Permission / Redaction State | hidden、masked、unauthorized、not_found、disabled reason | `shared/permissions` + API response |

规则：

- Required URL State 必须可刷新、分享和 back/forward 恢复。
- Session State 不进入 URL，但必须在当前工作流内可恢复。
- Forbidden URL State 不得进入 URL，包括 token、临时权限、未保存表单、敏感字段和超长选择集合。
- Server State 不直接塞进全局 store。
- Workspace State 必须能表达未保存状态和退出保护。
- 修改筛选、DataVersion policy、lot_scope 或时间范围后，旧请求不得覆盖新结果。

## API Client

前端只访问 `stdas-gateway` 的 `/api/v1/*`。页面、widget 和 feature 不得散写 raw `fetch`。

Typed API client 必须负责：

- 统一 base URL。
- 统一 request id / correlation id。
- 统一响应信封解析。
- 统一 401 / 403 / 404 / 409 / 422 / 429 / 500 / 503 / 504 处理。
- 统一 permission state / redaction state。
- 统一 query id、query hash、query snapshot id 校验。
- 统一过期响应忽略。
- 统一 abort / cancellation。
- 统一 Options API 的 loading、empty、error、permission denied、deprecated、hidden、unauthorized、not_found 状态。

前端不直接理解数据库表结构，不实现后端规则，不拼接内部服务 URL。

## 分析适配层

STDAS 前端必须通过 adapter layer 接入复杂表格、图表和 wafer map，避免页面和业务逻辑直接绑定第三方库 API。

### Grid Adapter

表格层必须支持：

- server-side pagination。
- server-side sorting。
- server-side filtering。
- virtualized rows。
- pinned columns。
- column groups。
- row selection。
- current page selection vs current filter result selection vs manual selection。
- permission-aware cell rendering。
- LotEndTime / test dimension / stale / partial / hidden 状态展示。DataVersion / DataProfile 默认仅在治理、诊断、Evidence、Export 元数据中展示。

React 技术候选可以评估 AG Grid、TanStack Table 或自研 grid shell。具体库选择必须通过 adapter 封装。

### Chart Adapter

图表层必须支持：

- yield trend。
- bin pareto。
- bin transition。
- retest analysis。
- parametric trend。
- histogram。
- box plot。
- scatter。
- correlation matrix。
- SPC / control chart。
- CPK / capability。
- outlier view。

图表组件只消费后端准备好的 dataset 和 metadata。前端不得计算 yield/bin/spec/DataVersion policy/customer-specific rule 等业务语义。

### Wafer/Position Adapter

WaferLot / WaferNo / X / Y 等位置字段不是 FT 第一阶段页面默认主维度。只有客户 FT 数据实际提供且分析场景需要时，位置分析能力才通过独立 adapter 启用：

- wafer layout。
- die coordinate。
- bin / parametric value coloring。
- brush / selection。
- linked filtering。
- missing die / invalid coordinate / hidden die 状态。
- drilldown 到 LotNo、CustomerLotNo、WaferLot、WaferNo、X/Y、TestFile 和内部数据引用。

第一版可以预留 adapter，不要求立即实现全部 wafer map 能力。

## 语义组件

STDAS 组件库应围绕业务语义组织，而不是只围绕 Button、Card、Modal、Table。

核心语义组件包括：

- `ContextBar`
- `CustomerScopeSwitcher`
- `TestDimensionBadge`
- `LotEndTimeBadge`
- `QuerySnapshotSummary`
- `LotScopeSelector`
- `LotGrid`
- `PositionMap`
- `DieMap`
- `BinParetoChart`
- `YieldTrendChart`
- `ParametricDistributionChart`
- `SpcChart`
- `CorrelationMatrix`
- `OutlierPanel`
- `EvidenceCard`
- `JobQueueDrawer`
- `ExportStatusBadge`
- `PermissionStateBanner`
- `ResultStateBanner`

通用 UI primitive 可以存在，但不得替代业务语义组件。

## 前后端职责边界

前端负责：

- 页面组织。
- URL 状态和工作区状态。
- 交互、选择、联动和布局。
- 数据可信状态展示。
- 权限、脱敏、partial、stale、snapshot、over budget 等状态表达。
- 小图前端图片导出。

后端负责：

- 认证、授权和 CustomerScope。
- QuerySnapshot / Evidence / Export 内部数据引用。
- yield、bin、spec、DataVersion policy、profile resolution 等业务语义。
- 聚合、采样、查询预算和异步任务。
- 导出文件生成。
- Investigation Evidence 版本和审计。

前端不得通过隐藏 UI 实现数据安全。所有安全边界以后端权限校验为准。

## 测试与验证

前端实现必须支持：

- TypeScript typecheck。
- lint。
- unit test。
- component test。
- route / workflow E2E test。
- visual regression 或关键页面截图验收。

Phase 0.5 环境验证必须确认前端 `pnpm install`、lint、typecheck、build 和测试命令可运行，且失败输出可被 AI Agent 读取。
