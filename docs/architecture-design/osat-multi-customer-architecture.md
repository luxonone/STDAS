# OSAT 多客户客制化架构

STDAS 面向 OSAT 封测代工厂。OSAT 同时服务多个芯片客户，而每个客户的测试文件、数据定义、规格限、Bin 规则、报表格式、告警方式和可见范围都可能不同。

客户差异不是边缘需求，而是一级架构问题。STDAS 采用分布式微服务架构，但微服务边界不能按客户拆分，也不能按页面功能拆分。客户差异必须进入可治理的 Profile、Extension、Version 和 Lineage 体系。

## 已选架构

STDAS 采用以下组合：

- 原生分布式微服务。
- `stdas-gateway` 作为统一外部入口。
- `customer-service` 作为客户专属服务，承载客户配置、DataProfile、规则版本和受控扩展。
- `data-pipeline-service` 作为摄入、解析、归一化和 canonical test data 主链路。
- `analytics-service` 作为分析、告警、分析工作台和导出能力中心。
- `workflow-service` 作为跨服务流程编排和重试补偿中心。
- NATS JetStream 作为事件总线和持久化消息流。
- PostgreSQL 按服务 schema/database 隔离。
- MinIO/S3 保存原始文件、中间文件、导出文件。
- Parquet/DuckDB 承担分析型数据路径，必要时接 ClickHouse。

## 客户差异不等于客户服务

禁止为每个客户部署一套服务，例如：

```text
customer-a-ingestion-service
customer-b-ingestion-service
customer-c-analytics-service
```

正确方式是：

```text
customer-service
  -> CustomerConfig
  -> DataProfile
  -> ProfileResolutionKey
  -> ParserProfile
  -> ParserRule
  -> MappingProfile
  -> MappingRule
  -> SpecProfile
  -> SpecRule
  -> AlertRuleSet
  -> FeatureFlags
  -> Extension Registry
```

各处理服务通过 `customer-service` 获取当前客户、产品、测试类型、测试站点、设备、文件格式和程序版本对应的 DataProfile、规则版本和扩展声明。

## Profile Resolution

OSAT 场景下，解析和归一化不能只按客户选择规则，必须按更细的组合维度选择：

```text
ProfileResolutionKey
  ├── customer_code
  ├── product
  ├── test_type          # FT / BI / BIT / SLT
  ├── test_station       # FT1 / FT2 / FTA / BI1, can be inferred later if file envelope lacks it
  ├── equipment_type     # ATE / tester platform / handler family
  ├── file_format
  ├── program_name
  ├── program_version
  └── effective_time
```

规则选择流程：

```text
data-pipeline-service
  -> Detect File Envelope
  -> Infer customer_code / product / test_type / test_station / equipment_type / file_format
  -> Call customer-service ResolveDataProfile(ProfileResolutionKey)
  -> Store raw metadata
  -> Load DataProfile / ParserProfile / MappingProfile / SpecProfile
  -> Parse
  -> Normalize
  -> Commit Canonical TestData Model
  -> Publish CanonicalDataCommitted
```

`{客户 - 产品 - 测试类型 - 测试站点 - 测试设备类型}` 的差异允许存在，但只能存在于 DataProfile、Rule Registry 和 Extension 中。核心处理服务只依赖 `ProfileResolutionKey` 和标准解析结果，不直接写客户分支。

示例：

```text
YM / DeviceA / FT / FT1 / MV / summary_7z        -> ym_devicea_ft1_mv_summary_parser:v1
YM / DeviceA / FT / FTA / B6700 / csv            -> ym_devicea_fta_b6700_parser:v2
CustomerA / ProductX / CP / CP1 / V93K / stdf    -> customer_a_productx_cp1_v93k_parser:v1
CustomerB / ProductY / BI / BI1 / X900 / xlsx    -> customer_b_producty_bi1_x900_parser:v3
```

## DataProfile

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
  ├── parser_profile_id
  ├── mapping_profile_id
  ├── spec_profile_id
  ├── alert_rule_set_id
  ├── analysis_extension_set_id
  ├── report_template_set_id
  ├── export_profile_id
  └── feature_flags
```

`DataProfile` 是测试数据差异的主入口。它不是一个客户只有一个，而是一个客户可以按产品、测试类型、测试站点、设备类型、文件格式、程序和生效时间拥有多份 DataProfile。

客户级配置仍然存在，但它只描述客户身份、授权范围、默认策略和全局偏好；具体测试数据处理规则进入 DataProfile。

## 规则复用与复制分叉

解析规则既要支持复用，也要支持复制后独立维护。

```text
DataProfile A ─┐
               ├── references ParserRule P-100:v3
DataProfile B ─┘

DataProfile C ─── references ParserRule P-100-COPY:v1
```

规则治理模型：

| 模式 | 含义 | 适用场景 |
|------|------|------|
| 引用共享 | 多个 DataProfile 引用同一个 ParserRule / MappingRule / SpecRule | 同客户同测试类型/测试站点、不同设备但文件结构一致；OSAT 厂内通用数据类型 |
| 复制分叉 | 从已有规则复制出新 rule id，后续独立维护 | 客户要求临时差异、验证新规则、避免影响其他数据 |
| 继承覆盖 | 共享基础规则，只覆盖少数字段或参数 | 大部分一致，仅少数列名、Bin、规格限不同 |
| 冻结版本 | DataProfile 固定引用某个 rule version | 已入库数据追溯、客户验收版本锁定 |

规则对象必须有稳定 ID、版本、来源、适用范围和变更审计。复制分叉必须记录 `forked_from_rule_id` 和 `forked_from_version`，避免未来无法追溯规则来源。

## 客户专属能力隔离

客户强行要求加入的分析功能、报表逻辑或特殊规则可以存在，但必须隔离：

- 在 `customer-service` 中登记为 CustomerExtension。
- 通过 DataProfile 或 FeatureFlag 显式启用。
- 在 `analytics-service` 或 `data-pipeline-service` 的 extension sandbox / adapter 中执行。
- 输入输出必须是标准上下文、DataVersion、Canonical TestData 或标准 AnalysisResult。
- 不允许直接修改其他服务数据库。
- 不允许污染默认分析路径。
- 必须能关闭、回滚、审计和按客户范围隔离。

## Extension System

当配置无法覆盖客户差异时，再使用扩展点：

| 扩展点 | 所属服务 | 用途 |
|------|------|------|
| Parser Extension | `data-pipeline-service` | 客户私有文件格式、压缩结构、特殊 datalog |
| Mapping Extension | `data-pipeline-service` | 特殊字段转换、阶段推导、lot id 解析 |
| Analysis Extension | `analytics-service` | 客户特殊 KPI、特殊统计口径、客户强制要求的分析能力 |
| Report Extension | `analytics-service` | 客户指定报表格式 |
| Integration Extension | `integration-service` | 客户专属 MES/API/文件交换 |

扩展点必须有边界：只输入标准上下文，只输出标准模型或标准结果，不允许直接修改其他服务数据库。

Parser Extension 可以是客户/测试类型/测试站点/设备专用实现，也可以是多份 DataProfile 共享的通用实现。它必须通过 `DataProfile -> ParserProfile -> ParserRule(parser_id, parser_version)` 被选择。`data-pipeline-service` 的主流程不直接引用客户专用 parser 类型。

## 客制化优先级

1. 配置化：字段映射、阶段映射、Bin 映射、规格限、告警阈值、模板、feature flag。
2. Profile 化：将一组配置打包为客户版本。
3. 插件化：解析器、特殊指标、特殊报表、特殊外部接口。
4. 产品化：多个客户反复出现的定制沉淀为标准能力。

禁止策略：

- 禁止为客户复制一套服务。
- 禁止在核心分析逻辑中写 `if customer == "X"`。
- 禁止让 parser 直接写核心业务表。
- 禁止客户配置无版本生效。
- 禁止客户扩展绕过审计和 lineage。

## 标准数据模型

不同客户最终必须归一到 STDAS canonical model：

```text
Customer
Product
Lot
LotRun
TestStage
TestFile
Tester
Handler
Site
BinResult
ParametricResult
WaferOrPosition
SpecLimit
AnalysisResult
AlertEvent
```

客户原始字段可以保留在 raw/staging/metadata 中，但核心分析必须基于标准模型。

## 版本与追溯

每次数据摄入必须记录：

```text
customer_code
product
test_type
test_station
equipment_type
data_profile_version
parser_id
parser_version
mapping_version
spec_version
alert_rule_set_version
data_version
raw_file_hash
program_name
program_version
ingested_at
```

这些信息必须被写入业务事件、DataVersion 和 Lineage。客户规则变化后，历史数据仍可解释、可复算、可对账。

## 多客户隔离

隔离维度：

- API 层：所有请求必须带 `CustomerScope`。
- 认证层：工程师和管理员只拥有被授权客户范围。
- 数据层：所有核心表带 customer scope；必要时高敏客户使用独立 schema/database。
- 对象存储：按 customer、test_type、test_station、data_version 分前缀。
- 事件层：事件 payload 必须包含 customer scope；消费者必须做 scope 校验。
- 导出层：文件生成和下载链接必须绑定 scope、过期时间和审计记录。
