# 数据平台架构

当前阶段遵循 [ADR-0014](../architecture-design/adr/0014-gateway-modular-monolith.md)：`stdas-gateway` 是唯一后端运行服务，数据所有权先按 `modules/*` 逻辑边界治理。本文出现的 `*-service` 表示未来满足拆分触发条件后的服务形态，不表示当前必须启动多个 Rust 进程。

## 数据分层

| 层 | 存储 | 说明 |
|------|------|------|
| Raw Zone | 文件系统或对象存储 | 原始测试文件、压缩包、上传文件 |
| Staging Zone | PostgreSQL staging 表或临时文件 | 解析后的中间结构、校验结果 |
| Core Zone | PostgreSQL | Lot、Summary、Bin、用户、规则、任务、审计 |
| Analytics Zone | 聚合表、Parquet、DuckDB | 参数明细、交互式分析、图表数据 |
| Cache Zone | 内存或 Redis | Options、轻量查询快照、热点查询、token revocation；具体策略见 [cache-strategy.md](cache-strategy.md) |

## 客户配置域

OSAT 多客户场景下，PostgreSQL 还负责保存客户配置域：

- CustomerConfig。
- DataProfile。
- parser profile。
- field mapping。
- stage mapping。
- bin mapping。
- parameter specs。
- test type / test station profiles。
- equipment profiles。
- profile resolution rules。
- parser/mapping/spec rule bindings。
- rule fork lineage。
- alert rule sets。
- 查询快照、报表、导出模板。
- feature flags。

这些配置必须版本化，且入库数据必须记录当时使用的版本。

## 数据所有权

STDAS 允许在早期共享一个 PostgreSQL 实例，但逻辑上按 module schema/database 隔离。当前模块只能写自己的 schema/database，不能直接读写其他模块内部表；未来服务化后沿用同一所有权边界。

| 当前模块 | 默认数据域 | 未来服务 |
|------|------|------|
| `modules/identity` | 用户、角色、token family、会话、权限 | `identity-service` |
| `modules/customer` | CustomerConfig、DataProfile、ProfileResolutionKey、parser/mapping/spec/template 版本、规则复用/复制分叉、客户专属扩展声明 | `customer-service` |
| `modules/data_pipeline` | 文件登记、raw metadata、staging、归一化记录、Lot、LotRun、TestFile、Summary、Bin、参数索引、DataVersion、lineage | `data-pipeline-service` |
| `modules/analytics` | AnalysisResult、聚合表、查询缓存索引、OLAP 文件索引、告警规则、告警事件、分析会话、模板、导出记录 | `analytics-service` |
| `modules/workflow` | Saga、Process Manager、作业状态、重试和补偿 | `workflow-service` |
| `modules/integration` | 外部系统连接、同步 checkpoint、交换记录 | `integration-service` |

当前跨模块协作通过 Rust module API 和清晰 DTO 完成；未来跨服务查询通过 gRPC、事件构建的本地 projection 或 gateway 聚合完成。

## PostgreSQL 定位

PostgreSQL 是权威业务库，负责：

- 服务私有业务事实。
- 客户配置和规则版本。
- 作业状态、Saga 状态和 Inbox/Outbox。
- 审计索引和 lineage 索引。
- AnalysisResult 索引、缓存元数据。
- QuerySnapshot、Investigation Evidence 和导出元数据。

PostgreSQL 不应长期承担：

- 大范围参数明细扫描。
- 多 lot 多参数相关性计算。
- 高并发交互式 OLAP。
- 大文件导出临时结果存储。

## 数据版本

每次成功摄入形成一个 `DataVersion`。数据版本绑定：

- 原始文件 hash。
- parser id 和 parser version。
- data profile version。
- mapping version。
- spec version。
- test type。
- test station。
- equipment type。
- file format。
- program name 和 program version。
- 客户、产品、测试平台。
- 业务校验版本。
- 入库时间。
- 聚合状态。

分析结果必须记录所基于的数据版本，避免文件重解析或规则升级后结果不可追溯。

## Query Snapshot 与证据版本

`QuerySnapshot` 是分析、导出、workspace、case 和 evidence 的稳定引用边界。它不替代 DataVersion，而是记录一次查询实际使用的 DataVersion 集合和查询语义。

建议持久化字段：

- `query_snapshot_id`。
- `query_hash`。
- `created_by`。
- `created_at`。
- `customer_scope`。
- `lot_scope`。
- `data_version_policy`。
- `data_version_set`。
- `query_summary`。
- `budget_result`。
- `result_ref`。

Investigation Evidence 必须引用 QuerySnapshot，并有独立 `evidence_version`。重算 evidence 不覆盖旧版本。

## 配置版本表

建议将客户配置拆成可版本化表，而不是只放一个大 JSON：

| 配置 | 说明 |
|------|------|
| customer_configs | 客户身份、授权范围、默认策略和全局偏好 |
| data_profiles | 按客户、产品、测试类型、测试站点、设备类型、文件格式、程序和生效时间定位的数据规则集合 |
| test_type_profiles | FT / BI / BIT / SLT 等测试类型配置 |
| test_station_profiles | FT1 / FT2 / FTA / BI1 等测试站点配置 |
| equipment_profiles | ATE、Handler、BI 设备等设备类型配置 |
| parser_profiles | DataProfile 到 ParserRule 的绑定 |
| parser_rules | 可共享、可复制分叉的解析规则版本 |
| mapping_profiles | DataProfile 到 MappingRule 的绑定 |
| mapping_rules | 字段、阶段、Bin、参数归一规则 |
| spec_profiles | DataProfile 到 SpecRule 的绑定 |
| spec_rules | 参数规格限、产品、程序版本规则 |
| rule_forks | 规则复制分叉来源、原因和审计 |
| alert_rule_sets | 客户告警规则集版本 |
| template_profiles | 查询快照、报表、导出模板 |

每个配置版本应有生效时间、失效时间、创建人和审计记录。

## 参数数据演进

阶段 1：

- 参数明细可保存在 PostgreSQL。
- 高频查询使用聚合表。
- 同步接口强制查询预算。

阶段 2：

- 参数明细写入 Parquet。
- DuckDB 作为嵌入式分析引擎。
- PostgreSQL 保存文件索引、schema、统计摘要。

阶段 3：

- 引入 ClickHouse 支撑高并发、多维 OLAP。
- API contract 保持稳定，由 Analytics backend 切换实现。

## 客户与工厂范围隔离

- 所有业务表必须能追溯到 customer 或内部 factory/tenant 边界；这里的 tenant 只表示内部隔离边界，不表示对外 SaaS 租户。
- 所有业务查询必须显式接收 `CustomerScope`。
- 查询实现必须默认带 scope 条件。
- 高风险表可评估 PostgreSQL Row Level Security。

## 数据生命周期与保留策略

默认保留策略必须可按客户、环境和合规要求覆盖。覆盖策略不得破坏审计、lineage、DataVersion 和 QuerySnapshot 的可追溯性。

| 对象 | 默认保留 | 可配置 | 删除/归档规则 |
|------|----------|--------|---------------|
| Raw file | 长期保留或按客户策略 | 是 | 删除必须保留 file metadata、hash、操作者和审计记录 |
| Staging dataset | 成功后短期保留 | 是 | 失败解析保留诊断窗口；过期后可清理 staging 内容 |
| ParseAttempt | 长期保留索引 | 是 | 不删除审计索引；可清理大体积临时引用 |
| DataVersion | 长期保留索引 | 是 | 可 archived，不得静默删除；删除需客户策略和审计 |
| Aggregates | 可重建 | 是 | DataVersion archived 后可清理聚合数据，但保留索引和重建能力说明 |
| QuerySnapshot | 中长期保留 | 是 | expired 后结果文件可不可下载，但查询摘要、DataVersion 集合和审计保留 |
| Export file | 短 TTL | 是 | 到期删除对象，保留 export metadata、query_snapshot_id 和下载审计 |
| Investigation Evidence | 长期保留 | 是 | case 关闭后仍保留 evidence version 和 QuerySnapshot 引用 |
| Audit log | 长期保留 | 是 | 不允许业务删除直接清理审计链 |

重算与删除规则：

- 重解析历史文件必须生成新的 DataVersion，不能覆盖旧 DataVersion。
- 旧 DataVersion 可用于回放、对账和历史 case 查看，除非客户策略要求删除或归档。
- 客户要求删除数据时，必须同时处理 raw、staging、core、analytics、cache、export 和 evidence 引用，并记录删除审计。
- QuerySnapshot 过期不等于删除；过期只影响结果文件可下载性或缓存可用性。
- Evidence 所引用数据若被归档、删除或权限变化，API 必须返回可区分状态。
