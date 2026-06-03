# 分析引擎

当前阶段遵循 [ADR-0014](../architecture-design/adr/0014-gateway-modular-monolith.md)：分析能力先落在 `stdas-gateway` 进程内的 `modules/analytics`，不是独立 `analytics-service`。本文提到的 `analytics-service` 仅表示未来满足拆分触发条件后的服务形态；页面名只作为历史示例，不作为当前 frontend/product design 决策。

## 查询类型

| 类型 | 场景 | 执行方式 |
|------|------|----------|
| 在线轻查询 | 列表/详情类轻量分析、小范围趋势 | `stdas-gateway` 同步调用 `modules/analytics` |
| 交互式中查询 | 多 lot、多参数图表 | 同步优先，超预算转 workflow 异步 job |
| 离线重查询 | 大导出、复杂相关性、批量报告 | NATS event + workflow job |

## 分析能力边界

`modules/analytics` 不穷举所有分析方法。架构只定义分析执行框架，具体分析能力通过内置模块、配置模板、算法 registry 和客户专属扩展持续演进。未来如果拆成独立服务，该边界升级为 `analytics-service`。

分析能力可以覆盖但不限于：

- 良率、Bin、Retest、参数、规格限、趋势、分布、相关性、异常点、设备/TestStation/Site/批次/程序对比。
- 工程统计、过程控制、质量监控、失效聚类、报表型分析、根因辅助、客户定义指标。
- 交互式查询、看板快照、异步导出、批量报告、告警评估。
- 未来新增的算法、模型、工程规则或客户指定分析逻辑。

任何新增分析能力必须遵守统一上下文：

- `CustomerScope`。
- `DataVersion`。
- `DataVersionPolicy`。
- `QuerySnapshot`。

面向普通工程用户的查询 API 应优先使用业务语义：`LotNo`、`CustomerLotNo`、`TestType`、`TestStation`、`TestAttempt`、`LotEndTime`。`DataVersion` 和 `DataVersionPolicy` 是内部可复现和追溯语义，不应在普通列表/详情类页面作为主筛选控件暴露。
- `lot_scope`。
- 查询预算。
- 权限和审计。
- 同步/异步执行策略。
- 结果版本和缓存策略。

## Query Snapshot

分析查询执行时必须生成或引用 `QuerySnapshot`。它用于冻结一次分析的语义，避免 latest committed 变化、缓存命中或异步重试导致结果不可追溯。

`QuerySnapshot` 至少记录：

- query snapshot id。
- query hash。
- 创建人和创建时间。
- CustomerScope。
- lot_scope。
- DataVersion policy。
- 实际参与分析的 Lot 和 DataVersion 列表。
- 查询条件摘要。
- 查询预算、执行方式和降级结果。
- 结果缓存或导出引用。

规则：

- 默认内部数据引用由服务端按业务上下文解析；查询运行时必须冻结为具体内部数据引用集合。
- 历史 workspace、case、export 打开时默认读取冻结版本，不重新解析 latest committed。
- 用户重新运行查询时可以生成新的 QuerySnapshot，并提示 DataVersion 变化。
- 图表、表格、导出、Investigation Evidence 必须引用 QuerySnapshot。

## 查询预算

每个分析端点必须定义：

- 最大 lot 数。
- 最大参数数。
- 最大时间窗口。
- 最大返回点数。
- 最大同步执行时间。
- 是否允许异步 fallback。

示例：

```text
max_lots_per_sync_query = 200
max_parameters_per_sync_query = 20
max_points_per_chart = 5000
max_sync_query_seconds = 8
```

## 缓存

| 数据 | 策略 |
|------|------|
| Options | 短 TTL，配置变更主动失效 |
| 轻量查询快照 | 快照缓存，可返回最近成功版本 |
| 图表查询 | query hash + data version |
| QuerySnapshot | query hash + lot_scope + data version set |
| 大导出 | 异步结果文件 |
| MES 补充信息 | TTL cache + 数据新鲜度 |

## 查询竞态与幂等

- 每次分析查询响应必须携带 query id、query hash 或 query snapshot id。
- gateway 和前端可以用该标识判断响应是否对应当前页面状态。
- 异步 fallback 必须复用同一 QuerySnapshot，除非用户明确重新运行。
- 重新运行 latest committed 查询必须生成新的 QuerySnapshot，不能覆盖历史结果。

## Investigation Evidence

Investigation Case 中的证据默认引用 QuerySnapshot，不引用可变查询条件。

- 保存 evidence 时必须记录 QuerySnapshot、生成时间、操作者和证据版本。
- evidence 重算必须生成新的 evidence version。
- 删除 evidence、修改结论、关闭 case 必须写审计。
- 如果 evidence 引用的数据被隐藏、过期或权限变化，API 必须返回可区分状态。

## OLAP backend

`modules/analytics` 必须把查询语义和执行后端分离。初始可用 PostgreSQL 聚合表；参数分析规模增长后切换 DuckDB；高并发场景再接 ClickHouse。未来服务化后同一边界升级为 `analytics-service`。

## 已选数据演进路线

STDAS 的数据层采用以下路线：

1. 第一阶段：PostgreSQL 保存业务事实、客户配置、作业状态、审计和聚合表。
2. 第二阶段：参数明细和中等规模交互式分析进入 Parquet + DuckDB。
3. 第三阶段：当多客户、多产品、跨批次分析并发压力超过 DuckDB 时，引入 ClickHouse。

不把 PostgreSQL 设计成无限 OLAP 引擎。
