# API 契约规则

本文件定义 STDAS 后端 API 契约的严格规则。所有外部 REST API 由 `stdas-gateway` 暴露，内部服务优先使用 gRPC 契约。

## 契约优先原则

- API 契约先于实现。
- 前后端必须围绕同一契约同步设计。
- 契约必须稳定、可版本化、可测试。
- 前端不得依赖数据库表结构。
- 后端不得返回未定义字段、模糊枚举或隐式默认值。

## 字段定义要求

每个字段必须定义：

| 属性 | 要求 |
|------|------|
| 名称 | snake_case 或约定格式，保持稳定 |
| 类型 | string、number、integer、boolean、array、object、enum |
| 必填性 | required / optional / nullable 必须区分 |
| 默认值 | 如果可省略，必须声明默认值 |
| 取值范围 | 数值范围、长度范围、数组数量范围 |
| 编码约束 | 字符集、大小写、格式、trim 规则 |
| 示例 | 至少给一个正常值示例 |
| 错误 | 不合法时返回的错误码 |

## 通用编码约束

| 类型 | 规则 |
|------|------|
| `customer_code` | 大写字母、数字、下划线或短横线，长度 1-64 |
| `product` | 客户/工厂定义的产品型号或产品族，使用稳定 code 或受控名称，trim 后长度 1-128 |
| `test_type` | 客户/产品定义的测试类型，例如 `FT`、`BI`、`BIT`、`SLT`，第一阶段用于 FT 部门系统；不写死为固定枚举 |
| `test_station` | 客户/产品定义的单个测试站点，例如 `FT1`、`FT2`、`FTA`、`BI1` |
| `test_attempt` | 同一 LotNo 在同一 test_type-test_station 下的测试次数 |
| `lot_end_time_range` | 按业务作业结束时间筛选，优先来自测试数据或 MES 过账记录 |
| `equipment_type` | 字符串枚举或受控字典，长度 1-64 |
| `file_format` | 小写标识，例如 `stdf`、`csv`、`xlsx`、`summary_7z` |
| `program_name` | UTF-8 字符串，trim 后长度 1-128 |
| `program_version` | UTF-8 字符串，trim 后长度 0-128，空值必须语义明确 |
| `data_version` | 稳定 ID，不暴露数据库自增含义 |
| `query_snapshot_id` | 稳定 ID，用于引用一次固化查询，不暴露数据库自增含义 |
| `job_id` | 稳定 ID，用于异步任务状态查询 |
| `time` | ISO-8601 / RFC3339，必须带时区或明确 UTC |
| `date` | `YYYY-MM-DD` |
| `page` | integer，默认 1，最小 1 |
| `page_size` | integer，默认 50，最大值按接口声明 |

## 默认值规则

- 默认值必须由后端契约声明，前端不能猜。
- 默认时间范围必须按页面场景声明，例如最近 7 天、最近 30 天、当前班次。
- 默认排序必须声明字段和方向。
- 默认 CustomerScope 必须来自认证上下文，不能用空值表达全部客户。
- 可选字段省略和显式 null 必须语义不同或明确等价。
- 普通工程接口默认不暴露 DataVersion policy 作为用户输入；内部稳定数据引用由服务端按业务上下文解析并在 snapshot、lineage、export 或 evidence 元数据中保留。

## 枚举规则

- 枚举必须有稳定 code 和展示 label。
- API 传输使用 code，不使用 label。
- 枚举扩展必须向后兼容。
- 前端必须能处理未知枚举：显示 code 或 fallback label，不崩溃。

示例：

```json
{
  "code": "FT",
  "label": "Final Test"
}
```

## 分页、排序、筛选

分页请求：

```json
{
  "page": 1,
  "page_size": 50
}
```

分页响应：

```json
{
  "items": [],
  "total": 0,
  "page": 1,
  "page_size": 50
}
```

规则：

- `page_size` 必须有最大值。
- 排序字段必须白名单。
- 筛选字段必须声明类型、操作符和默认行为。
- 大范围查询必须受查询预算控制。
- 高成本筛选必须声明是否显式运行查询，不能依赖前端猜测。

## Query Snapshot 契约

分析查询、跨 Lot 对比、导出和调查证据必须能关联 query snapshot。

Query snapshot 至少包含：

- `query_snapshot_id`。
- `query_hash`。
- `created_at`。
- `created_by`。
- `customer_scope`。
- `lot_scope`。
- `data_version_policy`。
- 实际参与查询的 LotNo、test_type、test_station、test_attempt、LotEndTime 和内部稳定数据引用列表。
- 查询条件摘要。
- 查询预算和降级结果。

响应返回图表、表格、导出或 evidence 时，必须返回 `query_snapshot_id` 或等价引用。

## Options 契约

Options 响应必须使用稳定 code，不使用 label 作为传输值。每个 option 应包含：

- `code`。
- `label`。
- `status`，例如 active、deprecated、hidden、unauthorized、not_found。
- 可选的 `reason`。

Options 请求必须声明上游上下文字段和分页/搜索规则。搜索型 options 必须支持过期响应识别，例如返回 `request_id` 或 `query_hash`。

## 错误契约

错误响应必须稳定：

```json
{
  "code": 42201,
  "message": "invalid data profile",
  "data": {
    "field": "data_profile_id",
    "reason": "not found or not effective"
  }
}
```

规则：

- `code` 面向程序判断，稳定不可随意改。
- `message` 面向人类，可优化但不能改变语义。
- `data.field` 指向错误字段。
- 批量错误必须返回可定位列表。
- 权限、隐藏、脱敏、不存在必须能区分，不能全部映射成空列表或通用 404。

## 版本兼容

- `/api/v1` 内不得做破坏性字段变更。
- 新字段默认可选。
- 删除字段必须进入下一个大版本。
- 枚举新增必须兼容旧前端。
- 响应结构变化必须写迁移说明。

## 契约验收

每个 API 文档必须包含：

- 请求示例。
- 成功响应示例。
- 错误响应示例。
- 字段表。
- 取值范围。
- 默认值。
- 错误码。
- 权限和 CustomerScope。
- DataVersion policy 和 query snapshot。
- Options API 依赖和级联字段。
- 权限脱敏策略。
- 同步/异步说明。
- job id、过期时间和重试/取消语义。
- 前端页面使用场景。
