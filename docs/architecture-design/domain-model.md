# 领域模型

## 能力域

| 能力域 | 职责 |
|------|------|
| Identity | 用户、角色、会话、token、权限 |
| Tenant | 内部隔离边界、工厂范围、客户、产品、测试平台、数据范围；不是对外 SaaS 租户模型 |
| Ingestion | 文件接收、校验、解析、入库、数据版本 |
| TestData | Lot、测试阶段、Summary、Bin、Parametric、位置数据 |
| Analytics | 分析执行框架、良率/Bin/参数等基础分析、扩展算法、客户专属分析 |
| Alerting | 规则、事件、确认、上下文调查入口 |
| Workspace | 分析会话、模板、案例、导出 |
| Admin | 用户、客户、规则、采集任务、数据治理 |
| Integration | MES、设备信息、外部数据源 |
| Observability | 日志、指标、追踪、审计、诊断 |

## 关注点优先

系统能力按照工程分析关注点组织，而不是按照企业岗位硬编码。初始权限只区分工程师和管理员；页面导航、筛选、模板和告警上下文围绕具体分析关注点设计。

| 关注点 | 对应能力域 |
|------|------------|
| Lot 详情 | TestData、Analytics |
| 良率趋势 | Analytics |
| Bin 分布 | TestData、Analytics |
| 参数能力 | Analytics、TestData |
| 告警追溯 | Alerting、Workspace |
| 模板复用 | Workspace |
| 系统治理 | Identity、Tenant、Admin |

## 核心实体

| 实体 | 说明 |
|------|------|
| Tenant | 内部数据隔离和组织边界，不表示对外 SaaS 租户 |
| Customer | 客户或业务方 |
| DataProfile | 按客户、产品、测试类型、测试站点、设备类型、文件格式、程序和生效时间定位的数据规则集合 |
| Product | 产品型号或产品族 |
| TestType | 客户/产品定义的测试类型或测试阶段，例如 FT、BI、BIT、SLT |
| TestStation | 测试类型下的具体测试站点或流程节点，例如 FT1、FT2、FTA、BI1 |
| TestSite | ATE 并行测试位置，不能与 TestStation 混用 |
| TestPlatform | 测试机、Handler、平台类型 |
| EquipmentType | ATE、Handler、Burn-in 设备或其他测试设备类型 |
| Lot | 生产/测试批次 |
| LotRun | 一次测试运行，包含阶段、设备、时间、结果 |
| TestFile | 原始文件及其 hash、来源、parser、版本 |
| IngestionJob | 摄入作业及状态机 |
| MeasurementSet | 一次解析得到的测试数据集合 |
| BinSummary | Hard Bin / Soft Bin 汇总 |
| ParametricMeasurement | 参数测试明细或参数明细引用 |
| AnalysisQuery | 分析请求、筛选、查询预算 |
| AnalysisResult | 分析结果快照、缓存或导出结果 |
| AlertRule | 告警规则 |
| AlertEvent | 告警事件 |
| WorkspaceSession | 工程师分析会话 |
| AnalysisTemplate | 可复用分析模板 |
| InvestigationCase | 异常调查案例 |

## 关键值对象

Rust 代码中应使用 newtype 或强类型表示关键业务值，避免全系统裸用 `String`、`i64`。

```text
CustomerCode
DataProfileVersion
FactoryCode
TestType
TestStation
EquipmentType
FileFormat
ProgramName
ProgramVersion
ProductName
LotNumber
TesterId
HandlerId
SiteIndex
BinCode
ParameterName
SpecLimit
TimeRange
CustomerScope
DataVersion
IdempotencyKey
CorrelationId
```

## 数据范围

所有业务查询必须携带显式数据范围：

```text
CustomerScope::All
CustomerScope::Some(Vec<CustomerCode>)
```

`CustomerScope::All` 只能由认证和授权上下文显式授予，不能由请求缺省推导。禁止使用空数组、空字符串或缺省值表达“全部客户”。

## 客户客制化模型

OSAT 场景下，测试数据差异通过 `DataProfile` 管理：

```text
DataProfile
  ├── customer_code
  ├── product
  ├── test_type
  ├── test_station
  ├── equipment_type
  ├── file_format
  ├── program_name
  ├── program_version
  ├── effective_time
  ├── data_profile_version
  ├── parser_profile
  ├── mapping_profile
  ├── spec_profile
  ├── alert_rule_set
  ├── analysis_extension_set
  ├── report_template_set
  └── feature_flags
```

所有入库数据必须记录使用的 data profile version 和 rule version，保证客户规则变化后仍可追溯。

## Profile Resolution Key

客户规则选择必须使用显式解析键：

```text
ProfileResolutionKey
  ├── customer_code
  ├── product
  ├── test_type
  ├── test_station
  ├── equipment_type
  ├── file_format
  ├── program_name
  ├── program_version
  └── effective_time
```

该 key 用于选择 parser、mapping、spec、alert rule set 和模板。`test_station` 可按文件或客户规则推导后补齐；如果解析前无法确定，必须在 ProfileResolutionRule 中声明回退和二次解析策略。系统允许客户专用 parser/mapping，但不允许核心业务流程中散落客户判断。

## 聚合边界

| 聚合 | 聚合根 | 包含 | 不包含 |
|------|--------|------|--------|
| Identity | User / Principal | role、permission、token family、session | 业务数据明细 |
| Customer Config | CustomerConfig | DataProfile、ProfileResolutionRule、FeatureFlag、parser/mapping/spec/template 绑定 | TestData、AnalysisResult |
| Test Data | Lot / LotRun | TestFile、ParseAttempt、Summary、Bin、DataVersion 引用、lineage | AnalysisResult、Investigation conclusion |
| Analysis | QuerySnapshot | lot_scope、data_version_set、query_summary、budget_result、result_ref | 原始测试明细、可变筛选草稿 |
| Alerting | AlertEvent | rule version、trigger context、ack/close 状态 | Investigation conclusion |
| Investigation | InvestigationCase | EvidenceVersion、结论、审计 | 可变查询条件、可静默重算结果 |
| Workflow | Job / Saga | 状态、重试、补偿、dead letter 引用 | 业务事实所有权 |
| Export | ExportRecord | query_snapshot_id、file_ref、format、expires_at、download audit | QuerySnapshot 本体 |

聚合边界用于约束数据库写入和服务所有权。跨聚合访问必须通过 API、事件投影或稳定引用完成，不能直接读写其他服务内部表。

## 关键关系与基数

```text
Tenant 1..n Customer
Customer 1..n Product
Customer 1..n DataProfile
Customer 1..n ProfileResolutionRule
Product 1..n Lot
Lot 1..n LotRun
LotRun 1..n TestFile
TestFile 1..n ParseAttempt
Successful ParseAttempt 1 DataVersion
DataVersion 1..1 LotRun
DataVersion n..1 DataProfileVersion
QuerySnapshot n..n DataVersion via data_version_set
QuerySnapshot n..n Lot via lot_scope
AnalysisResult n..1 QuerySnapshot
ExportRecord n..1 QuerySnapshot
AlertEvent n..1 RuleVersion
AlertEvent 0..1 QuerySnapshot
InvestigationCase 1..n EvidenceVersion
EvidenceVersion 1..1 QuerySnapshot
AnalysisTemplate n..1 DataProfile or template scope
```

默认约束：

- `Product` 归属于 `Customer`，跨客户同名产品不能合并为同一业务对象。
- `Lot` 归属于 `Product` 和 `Customer`，不能跨 Customer。
- `LotRun` 表示一次测试运行；一个 Lot 可以有多个测试阶段或重测运行。
- `TestFile` 可以产生多次 `ParseAttempt`，但只有成功提交的解析尝试生成 `DataVersion`。
- `QuerySnapshot` 不直接拥有原始测试数据，只引用实际使用的 DataVersion 集合。
- `InvestigationCase` 第一版不跨 Customer；跨客户调查需要显式授权和单独 ADR。

## 核心状态生命周期

```text
DataProfile: draft -> published -> deprecated
ProfileResolutionRule: draft -> published -> superseded -> deprecated
IngestionJob: queued -> running -> succeeded / failed / dead_letter
ParseAttempt: created -> validating -> parsed -> normalized -> committed / failed
DataVersion: pending -> committed -> ready -> superseded / archived
QuerySnapshot: created -> materialized / failed -> expired
AlertEvent: raised -> acknowledged -> closed / reopened
InvestigationCase: open -> investigating -> resolved -> closed
Export: queued -> running -> ready -> expired / failed
Job: queued -> running -> succeeded / canceling -> canceled / failed / expired / dead_letter
```

状态变化必须可审计。会影响用户可见数据、DataVersion、规则版本、权限或导出文件的状态变化，必须带 `request_id`、`correlation_id`、操作者和时间。

## 领域不变量

- 所有业务查询必须携带显式 `CustomerScope`。
- 空值、空数组或缺省字段不能表示“全部客户”。
- 所有入库测试数据必须能追溯到 DataProfile version、parser version、mapping version 和 spec version。
- successful parse 生成新的 DataVersion；重解析历史文件不能覆盖旧 DataVersion。
- 分析、导出、workspace、case 和 evidence 必须引用 QuerySnapshot 或等价稳定引用。
- Evidence 重算必须生成新的 evidence version，不能覆盖旧证据。
- 客户差异必须通过 DataProfile、FeatureFlag、规则绑定或受控 extension 表达，不能在核心流程散落客户判断。
- `TestStation` 是流程/站点维度，`Site` 或 `SiteIndex` 是 ATE 并行测试位置，两者不能混用。
