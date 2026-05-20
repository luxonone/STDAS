# 首批功能切片 V1

本文件定义进入代码实现前优先落地的功能切片交付卡。它是前端、后端、数据、权限和验收的索引文档；具体 UI/UX 规则、API 字段规则、权限矩阵和数据生命周期以对应事实来源为准。

事实来源：

- 页面族与 UI/UX：[UI/UX 约束](../frontend-design/ui-ux-constraints.md)
- 页面层级：[页面层级设计](../frontend-design/page-hierarchy-design.md)
- API 规则：[API 契约规则](../backend-design/api-contract-rules.md)
- API 分组：[API 契约原则](../backend-design/api-principles.md)
- QuerySnapshot / 查询预算：[分析引擎](../backend-design/analytics-engine.md)
- 权限与脱敏：[安全与可靠性](../backend-design/security-reliability.md)

## 1. 登录与当前上下文

### 用户目标

用户使用工厂内部账号密码登录后进入工作台。角色、权限和可见数据范围由账号配置和后端权限决定，不在页面上让用户选择。普通工程用户不需要确认或选择 DataProfile。

### 页面入口

- `/login`
- `/app/overview`
- Global Shell 当前用户与上下文区域

### Required URL State

- 登录页：无业务查询状态。
- 工作台页：`customer`、`product`、`test_type`、`test_station`、`lot_end_time_range` 按页面族声明。

### Session State

- access token。
- 当前用户。
- 当前权限范围。
- 当前默认业务上下文。

### Forbidden URL State

- token。
- refresh token。
- 临时权限。
- 未授权客户列表。

### API

| 方法 | 端点 | 说明 |
|------|------|------|
| `POST` | `/api/v1/auth/login` | 登录 |
| `POST` | `/api/v1/auth/refresh` | 刷新 access token |
| `POST` | `/api/v1/auth/logout` | 登出 |
| `GET` | `/api/v1/auth/me` | 当前用户、账号配置角色、权限范围、permissions |
| `GET` | `/api/v1/options/customers` | 当前用户可见客户选项 |

### 请求字段

`POST /api/v1/auth/login`：

- `username`
- `password`

`GET /api/v1/options/customers`：

- `query`
- `page`
- `page_size`

### 响应字段

`GET /api/v1/auth/me` 至少返回：

- `user_id`
- `display_name`
- `role`
- `customer_scope`
- `factory_scope`
- `permissions`
- `default_context`
- `session_expires_at`

### 错误

- `401xx`：未登录、token 过期、refresh 失败。
- `403xx`：无工作台访问权限。
- `422xx`：登录请求字段非法。
- `429xx`：登录限流。

### 权限

以 [权限矩阵](../backend-design/security-reliability.md) 为准。Admin 不自动等于业务全数据权限，必须通过显式 scope 表达。

### P0 UI 验收

- 当前用户和会话操作全局可达。
- 页面不提供 user role 选择器；角色由账号配置。
- 权限不足、未登录、无客户数据权限必须区分。
- 客户/产品/测试类型/测试站点变化后，后台必须按上下文重新解析规则。

### 契约测试

- 登录成功、登录失败、token 过期、refresh 轮换。
- `auth/me` 返回 CustomerScope 和 permissions。
- 无权限客户不出现在 options 中，或按 redaction policy 返回。

## 2. Overview Summary

### 用户目标

工程师进入 FT 工作台后快速判断授权范围内近期 FT 数据状态、风险 LotNo、open alerts、趋势和最近成功快照。CP 晶圆测试系统暂不进入该切片。

### 页面入口

- `/app/overview`

### Required URL State

- `customer`
- `product`
- `test_type`
- `test_station`
- `lot_end_time_range`

### Session State

- 卡片展开状态。
- 局部排序。

### Forbidden URL State

- 未授权客户或产品明文。
- 临时权限。

### API

| 方法 | 端点 | 说明 |
|------|------|------|
| `GET` | `/api/v1/overview/summary` | KPI、趋势、风险 Lot、open alerts、数据新鲜度 |

### 请求字段

- `customer_scope`
- `customer`
- `product`
- `test_type`
- `test_station`
- `test_attempt`
- `lot_end_time_range`

### 响应字段

- `overview_snapshot_id` 或 `query_hash`
- `context`
- `kpis`
- `trend_series`
- `open_alerts`
- `risky_lots`
- `held_lots`
- `data_freshness`
- `snapshot_ref`
- `permission_state`
- `partial_data`

### Snapshot / 数据语义

- Overview 可返回最近成功快照。
- 返回快照时必须包含 `snapshot_ref`、生成时间和原始查询条件摘要。
- 如果结果 stale、partial 或缓存命中，必须在响应中可区分。
- Overview 默认展示业务时间 `LotEndTime`，不以系统 `Updated` 作为主时间。
- Overview 普通 UI 不展示 DataProfile、DataVersion Policy、UserRole 选择器。
- Overview 不展示 Wafer Count、Wafer Map 或 CP 主维度。

### 错误

- `403xx`：无 CustomerScope 或数据被隐藏。
- `422xx`：LotEndTime 范围、customer/product/test_type/test_station 不合法。
- `429xx`：查询预算超限。
- `500xx`：聚合或缓存不可用。

### P0 UI 验收

- 首屏展示 KPI、近期趋势、open alerts、risky/held lots 和快捷入口。
- customer、product、TestType、TestStation、LotEndTime 范围和数据新鲜度可见。
- stale、snapshot、partial data、permission denied 区分展示。

### 契约测试

- 默认时间范围由 API 契约声明。
- 无数据、无权限、partial、stale、snapshot 分别可被前端区分。
- customer 切换后 overview 不复用旧上下文结果。

## 3. Data Explorer：Lot 列表

### 用户目标

工程师按 customer、product、TestType、TestStation、LotEndTime range 和主筛选查找 LotNo，并选择一个或多个 LotNo 进入 Analysis Workspace。

### 页面入口

- `/app/data/lots`

### Required URL State

- `customer`
- `product`
- `test_type`
- `test_station`
- `lot_end_time_range`
- `filters`
- `page`
- `page_size`
- `sort`

### Session State

- 大批量 Lot 选择。
- 列配置。
- 滚动位置。

### Forbidden URL State

- 未授权 Lot 明文。
- 未保存筛选草稿。
- 超长选择集合。

### API

| 方法 | 端点 | 说明 |
|------|------|------|
| `GET` | `/api/v1/data/lots` | Lot 列表 |
| `GET` | `/api/v1/options/products` | 产品选项 |
| `GET` | `/api/v1/options/test-types` | 测试类型选项，按客户/产品解析 |
| `GET` | `/api/v1/options/test-stations` | 测试站点选项，按客户/产品/测试类型解析 |

### 请求字段

- `customer_scope`
- `customer`
- `product`
- `test_type`
- `test_station`
- `test_attempt`
- `lot_end_time_range`
- `status`
- `program_name`
- `equipment_type`
- `filters`
- `page`
- `page_size`
- `sort`

### 响应字段

- `items[]`
- `total`
- `page`
- `page_size`
- `query_id`
- `data_freshness`
- `selection_capability`
- `permission_state`

`items[]` 至少包含：

- `lot_id`
- `lot_number`
- `customer_code`
- `product`
- `test_type`
- `test_station`
- `test_attempt`
- `lot_end_time`
- `yield_summary`
- `bin_summary_ref`
- `customer_lot_no`
- `permission_state`

### 用户可见数据语义

- 默认按 LotNo + TestType + TestStation + TestAttempt + LotEndTime 展示数据。
- DataVersion 等内部稳定引用保留在响应元数据或 lineage 中，不作为普通列表主列。
- WaferLot/WaferNo/X/Y 只有在客户 FT 数据实际提供且用户打开相关分析时出现。

### 错误

- `403xx`：无客户或 Lot 数据权限。
- `422xx`：筛选字段非法。
- `429xx`：查询预算超限。
- `500xx`：数据查询失败。

### P0 UI 验收

- 服务端分页。
- 核心列可见：LotNo、CustomerLotNo、Product、TestType、TestStation、TestAttempt、Yield、LotEndTime、状态。
- 当前页选择、手动选择、当前筛选结果选择必须区分。
- 多选数量、selection source 和跨页范围必须可见。
- 不显示 DataProfile/DataVersion Policy 选择器；不以 Updated 作为主时间列。

### 契约测试

- 分页、排序、筛选白名单。
- permission denied、hidden、masked、not found 可区分。
- options 级联字段在上游变化后重新校验已选值。

## 4. Lot Detail + DataVersion 追溯

### 用户目标

工程师查看单个 Lot 的 Run、File、DataVersion、Profile、Parser、Mapping、Spec、Summary、Bin 和 lineage，判断数据是否可信并可进入分析。

### 页面入口

- `/app/data/lots/:lot_id`
- `/app/data/files/:file_id`
- `/app/data/versions/:data_version`

### Required URL State

- `lot_id`
- `data_version` 或 `data_version_policy`
- `tab`
- 来源列表查询状态引用或返回状态。

### Session State

- tab 内局部展开。
- 返回列表的滚动位置。

### Forbidden URL State

- 未授权 file id 或 data version 明文，若安全策略禁止。
- 下载 token。

### API

| 方法 | 端点 | 说明 |
|------|------|------|
| `GET` | `/api/v1/data/lots/{lot_id}` | Lot 详情 |
| `GET` | `/api/v1/data/lots/{lot_id}/runs` | Run 列表 |
| `GET` | `/api/v1/data/files/{file_id}` | 文件详情 |
| `GET` | `/api/v1/data/versions/{data_version}` | DataVersion 详情 |
| `GET` | `/api/v1/data/versions/{data_version}/lineage` | lineage |

### 响应字段

Lot detail 至少返回：

- `lot_id`
- `lot_number`
- `customer_code`
- `product`
- `test_type`
- `test_station`
- `test_attempt`
- `lot_end_time`
- `runs[]`
- `data_versions[]`
- `current_data_version`
- `data_profile_version`
- `summary`
- `bin_summary`
- `permission_state`
- `data_freshness`

DataVersion detail 至少返回：

- `data_version`
- `status`
- `created_at`
- `ready_at`
- `raw_file_hash`
- `parser_version`
- `mapping_version`
- `spec_version`
- `data_profile_version`
- `lineage_ref`
- `aggregate_status`

### 错误

- `403xx`：无 Lot/File/DataVersion 权限。
- `404xx`：对象不存在或按策略不可见。
- `409xx`：DataVersion 状态不允许查看或分析。

### P0 UI 验收

- Lot、Run、File、DataVersion、Profile 追溯关系可见。
- 从详情返回列表时，筛选、排序、分页、滚动位置和已选项恢复。
- DataVersion 状态和 lineage 不得隐藏在次要区域不可达处。
- 无权限、被隐藏、不存在必须区分。

### 契约测试

- 历史 DataVersion 可打开。
- latest committed 变化时，历史详情不静默切换。
- lineage 字段可追溯到 raw file、parser/mapping/spec version。

## 5. Analysis Workspace 基础查询

### 用户目标

工程师选择单个或多个 Lot，执行基础良率、Bin、参数趋势或分布分析，并保存 workspace 或导出结果。

### 页面入口

- `/app/analysis`
- `/app/analysis/:workspace_id`

### Required URL State

- `workspace_id` 或 `query_snapshot_id`
- `lot_scope` 引用。
- `data_version_policy`
- 核心查询参数。

### Session State

- 未保存布局。
- 草稿注释。
- 临时 brush/zoom/legend 状态。

### Forbidden URL State

- 超长 lot_scope。
- 未保存表单。
- 未授权客户、Lot 或规则明文。

### API

| 方法 | 端点 | 说明 |
|------|------|------|
| `POST` | `/api/v1/analysis/queries/run` | 运行同步或可降级分析查询 |
| `GET` | `/api/v1/analysis/query-snapshots/{query_snapshot_id}` | 查询快照详情 |
| `POST` | `/api/v1/analysis/workspaces` | 保存 workspace |
| `GET` | `/api/v1/analysis/workspaces/{workspace_id}` | 打开 workspace |
| `POST` | `/api/v1/exports` | 基于 QuerySnapshot 导出 |

### 请求字段

- `customer_scope`
- `analysis_type`
- `lot_scope`
- `data_version_policy`
- `parameters`
- `time_range`
- `group_by`
- `filters`
- `budget_preference`
- `result_shape`

### 响应字段

- `query_id`
- `query_hash`
- `query_snapshot_id`
- `execution_mode`
- `budget_result`
- `data_version_set`
- `result`
- `partial_data`
- `stale`
- `cache_info`
- `job_id`，当转异步时返回。

### QuerySnapshot / DataVersion 语义

- 查询运行时必须固化每个 Lot 实际使用的 DataVersion。
- 历史 workspace 默认展示固化版本。
- 重新运行 latest committed 查询必须生成新的 QuerySnapshot。
- 结果区必须绑定 query id、query hash 或 query snapshot id，避免旧响应覆盖新结果。

### 错误

- `403xx`：CustomerScope、Lot 或 DataVersion 无权限。
- `409xx`：workspace 保存冲突或 DataVersion 状态冲突。
- `422xx`：分析参数非法。
- `429xx`：查询预算超限。

### P0 UI 验收

- lot_scope、DataVersion policy、查询摘要和结果可信状态可见。
- 保存目标、导出影响范围、退出保护在 workspace/case/template/export 场景为 P0。
- 图表、表格必须显示 loading、empty、error、partial data、over budget 状态。
- 刷新或重新运行时必须提示 DataVersion 可能变化。

### 契约测试

- 同步成功、partial data、over budget 转异步、失败错误可区分。
- 旧查询响应不能覆盖新查询结果。
- 导出复用原 QuerySnapshot，除非用户明确重新运行。
