# 页面层级设计

本文件定义 STDAS 前端页面层级。页面层级先于具体页面和 API 契约确认，用来约束后续前端设计、后端契约、功能切片和 UI/UX 验收。

STDAS 不是展示型官网，也不是通用 CRUD 后台。它是面向测试工程师的高密度数据工作台，页面组织必须围绕工程分析关注点和系统治理任务，而不是围绕后端服务、数据库表或临时 route 清单。

## 设计原则

- 关注点优先：一级导航围绕用户要解决的问题组织。
- 工程分析和系统治理分离：工程师日常分析入口不混入治理配置噪音。
- 工作区承载分析：良率、Bin、参数、重测等分析能力进入 Analysis Workspace，不拆成孤立一级页面。
- 多批次是一等工作流：工程师必须能从 Lot 列表或告警上下文选择多个批次进入同一 Analysis Workspace。
- 上下文连续：从数据对象、告警、模板进入分析时，客户、产品、测试类型、测试站点、测试次数、LotEndTime 时间范围和来源上下文不能丢。
- 用户语义优先：普通工程页面不暴露 DataProfile、DataVersion policy、UserRole 选择等开发/治理概念；这些由账号、权限和解析规则在后台决定。
- 任务导向契约：后端 API 按工作台任务和功能切片设计，不按页面组件或数据库表机械映射。

## 总体层级

```text
STDAS
├── Global Shell
│   ├── Login / current user / session
│   ├── Current user / permissions
│   ├── Current context: customer, product, test type, test station, LotEndTime range
│   ├── Global search
│   ├── Notifications
│   └── Task / export queue
│
├── Engineering Workbench
│   ├── Data Explorer
│   ├── Analysis Workspace
│   ├── Alerts & Investigation
│   └── Jobs & Exports
│
└── System Governance
    ├── Users & Access
    ├── Customer & Profile
    ├── Ingestion Config
    ├── Rules & Templates
    └── Data Governance
```

## Global Shell

Global Shell 提供所有页面共享的身份、数据范围和运行上下文。

| 能力 | 规则 |
|------|------|
| 登录和会话 | 未登录进入 `/login`；当前登录成功后进入临时空白工作区，正式登录后入口待下一张页面设计确认 |
| 当前用户 | 显示用户身份和会话操作；角色/权限来自账号配置，不作为页面选择器 |
| 授权范围 | 由后端权限控制可见客户/产品/数据范围；普通页面不展示 CustomerScope 技术词 |
| 客户/产品筛选 | 用户按业务需要筛选客户、产品、测试类型和测试站点；切换后后台重新解析适用规则 |
| 当前上下文 | 页面应按场景显示 customer、product、test type、test station、LotEndTime range |
| 全局任务 | 长耗时分析、摄入和导出任务可从全局任务队列进入 |

## Engineering Workbench

Engineering Workbench 是工程师日常入口，承载数据浏览、分析、告警调查和异步结果管理。

| 页面族 | 职责 | 典型页面 |
|------|------|----------|
| Data Explorer | 找到并理解测试数据对象，支持多 LotNo 选择并进入分析 | Lot List、Lot Detail、Run/File/Lineage、Product/Program/Equipment 浏览 |
| Analysis Workspace | 执行跨批次、多参数、多图联动分析并沉淀结果 | Multi-lot Compare、Yield Trend、Bin Distribution、Parametric Capability、Retest Analysis、Cross-lot Compare、模板应用、案例保存、导出 |
| Alerts & Investigation | 从告警进入可解释调查流程 | Alert List、Alert Detail、Alert Context、Investigation Case、确认、关闭、追溯 |
| Jobs & Exports | 管理异步任务和长耗时结果 | Ingestion Jobs、Analysis Jobs、Export Jobs、失败诊断、重试、下载 |

删除原固定首页后，不再预设固定登录后 route。Data Explorer / Lot List 是候选工程入口之一；开班检查、异常入口和快捷分析最终由后续确认的页面设计决定，并可能由 Data Explorer、Alerts & Investigation 与 Analysis Workspace 共同承担。

Analysis Workspace 是分析能力的核心容器。Yield、Bin、Parametric、Retest 等不是一级导航，而是工作区内的分析视图、模板配置或保存后的 workspace state。多 Lot 分析是 Analysis Workspace 的核心场景，不是 Lot Detail 的附属能力。

## 多 Lot 工作流

工程师必须能同时查看和分析多个批次。默认工作流：

```text
Data Explorer / Lot List
  -> 选择一个或多个 Lot
  -> 检查 customer、product、test type、test station、test attempt、LotEndTime 上下文
  -> Add to Analysis Workspace
  -> 选择分析视图或模板
  -> 运行跨批次分析
  -> 保存为 workspace、case、template 或 export
```

规则：

- Lot List 必须支持多选，并允许将当前筛选结果或手动选择结果加入 Analysis Workspace。
- 多 Lot 选择必须显示上下文摘要，包括 customer、product、test type、test station、test attempt、LotEndTime range。
- 如果所选 Lot 的上下文不一致，系统可以允许比较，但必须明确标记差异和可能影响。
- Analysis Workspace 必须保留每个 LotNo 对应的测试类型、测试站点、测试次数、LotEndTime 和内部稳定数据引用，不能只传 Lot id。
- 多 Lot 查询超过预算时，必须返回 job id 或 partial data 状态，不能阻塞工作区。

## System Governance

System Governance 是管理员入口，承载可审计、可版本化、可受控的系统配置。

| 页面族 | 职责 |
|------|------|
| Users & Access | 用户、角色、CustomerScope、会话和审计 |
| Customer & Profile | 客户、产品、测试类型、测试站点、EquipmentType、DataProfile、Profile Resolution 预览 |
| Ingestion Config | Parser Profile、Mapping Profile、Spec Profile、文件格式、接入源配置 |
| Rules & Templates | Alert Rules、Analysis Templates、Report Templates、版本发布记录 |
| Data Governance | 数据保留、DataVersion 管理、Lineage、Dead Letter、重放治理 |

模板具有双重入口：工程师侧用于应用、保存和复用分析；治理侧用于维护、发布、绑定客户和版本管理。

## 路由基线

```text
/login
/app/data/lots
/app/data/lots/:lot_id
/app/data/files/:file_id
/app/analysis
/app/analysis/:workspace_id
/app/alerts
/app/alerts/:alert_id
/app/cases/:case_id
/app/jobs
/app/exports
/app/governance/users
/app/governance/customers
/app/governance/profiles
/app/governance/ingestion
/app/governance/rules
/app/governance/templates
/app/governance/data
```

路由可以随实现细化，但不能反向改变页面族边界。新增页面必须先归属到一个页面族，再设计 route、组件和 API 契约。

## 上下文流转

| 来源 | 目标 | 必须携带的上下文 |
|------|------|------------------|
| Data Explorer | Analysis Workspace | lot scope、product、test type、test station、test attempt、LotEndTime range、internal data refs |
| Alerts & Investigation | Analysis Workspace | alert id、rule version、trigger context、related lots、test dimension |
| Analysis Workspace | Jobs & Exports | query id、workspace id、export config、snapshot/internal data refs |
| System Governance | Engineering Workbench | 发布后的 profile/rule/template version，不直接复用未发布草稿 |

## 后续文档派生

- `workbench-design.md` 描述 Engineering Workbench 的体验、状态、组件和图表策略。
- `ui-ux-constraints.md` 按页面族补充可验收约束。
- [前后端同步设计](../architecture-design/frontend-backend-sync-design.md) 从页面族派生功能切片。
- 后端 API 文档必须说明接口被哪个页面族或功能切片使用。
