# 系统架构

## 架构定位

STDAS 采用 **Rust 原生分布式微服务数据平台架构**，但服务边界采用粗粒度平台能力划分。

分布式是架构模式，不等于必须上 Kubernetes，也不等于第一天就要多台机器。STDAS 从第一版开始就按独立进程、独立端口、独立健康检查、独立配置和明确数据边界设计。单机部署只是分布式拓扑在 `n=1` 时的部署形态，多节点部署只改变配置和进程位置，不改变代码结构。

STDAS 服务对象是 OSAT 封测代工厂。核心复杂度来自多客户、多测试类型、多测试站点、多设备类型、多文件格式、多规则版本的数据摄入、归一化、分析和追溯。因此服务不按页面、CRUD 或单个技术步骤拆分，而按稳定平台能力拆分。

## 服务抽象原则

- 第一版采用明确微服务架构，但服务数量保持克制。
- 服务边界按业务所有权和数据所有权划分，不按函数名拆分。
- 高频协作、强事务相邻、同一数据生命周期内的能力放在同一服务内。
- 需要独立扩展、独立故障隔离、独立对外集成的能力才拆成独立服务。
- Observability、Outbox、Inbox、health、metrics 是所有服务的基础能力，不作为独立业务服务起步。

## 服务边界总览

```text
React Analytical Workbench
      |
      | HTTPS
      v
stdas-gateway / BFF
      |
      | gRPC
      v
+----------------------+--------------------------------------+
| Control Plane        | Data & Analysis Plane                |
+----------------------+--------------------------------------+
| identity-service     | data-pipeline-service                |
| customer-service     | analytics-service                    |
| workflow-service     | integration-service                  |
+----------------------+--------------------------------------+
      |
      | NATS JetStream events / commands
      v
Event Bus
      |
      v
Data Platform
  |-- PostgreSQL schemas/databases by service
  |-- MinIO / S3 object storage
  |-- Parquet data lake
  |-- DuckDB embedded query engine
  |-- ClickHouse for high-volume OLAP if needed
```

## 第一版服务大类

| 层级 | 服务 | 职责 |
|------|------|------|
| Edge | `stdas-gateway` | HTTPS API、BFF、认证上下文注入、请求聚合、限流、OpenAPI |
| Control Plane | `identity-service` | 用户、工程师/管理员角色、token、会话、权限、CustomerScope |
| Control Plane | `customer-service` | 客户专属服务：客户配置、DataProfile、规则版本、扩展注册、客户专属能力隔离 |
| Control Plane | `workflow-service` | Saga / Process Manager、作业状态、事件编排、重试、补偿、Dead Letter 处理入口 |
| Data Plane | `data-pipeline-service` | 文件登记、raw metadata、解析、归一化、canonical TestData、DataVersion、lineage |
| Analysis Plane | `analytics-service` | 分析查询、聚合、OLAP adapter、告警规则评估、分析会话、模板、导出，以及可扩展分析能力 |
| Integration | `integration-service` | MES、客户接口、外部文件交换、设备数据同步 |

## 为什么这样合并

| 原细分能力 | 合并到 | 原因 |
|------|------|------|
| ingestion / normalization / testdata | `data-pipeline-service` | 三者属于同一条数据生命周期，强顺序、强幂等、强 lineage，过早拆开会制造大量跨服务事务和契约成本 |
| alerting | `analytics-service` | 告警本质是分析结果或规则评估，依赖聚合、数据版本和分析上下文 |
| workspace | `analytics-service` | 分析会话、模板、案例和导出紧贴分析体验，第一版不需要独立服务 |
| 客户配置与规则解析 | `customer-service` | 该服务的本质不是客户 CRUD，也不是一个客户一个 Profile，而是承载客户专属配置、DataProfile、规则版本和受控扩展 |
| observability-service | 所有服务基础能力 | tracing、metrics、audit、lineage 写入必须内建；后续只有在审计查询规模很大时才独立 |

## 进程与端口

端口是默认建议，可通过配置修改。

| 进程 | 默认端口 | 对外协议 | 存储 |
|------|------:|------|------|
| `stdas-gateway` | 8080 | HTTPS / REST | 无状态 |
| `identity-service` | 50051 | gRPC | PostgreSQL `identity` |
| `customer-service` | 50052 | gRPC | PostgreSQL `customer` |
| `data-pipeline-service` | 50053 | gRPC + NATS | PostgreSQL `data_pipeline` + MinIO |
| `analytics-service` | 50054 | gRPC + NATS | PostgreSQL `analytics` + Parquet/DuckDB/ClickHouse |
| `workflow-service` | 50055 | gRPC + NATS | PostgreSQL `workflow` |
| `integration-service` | 50056 | gRPC + NATS | PostgreSQL `integration` |
| `nats-server` | 4222 | NATS | JetStream storage |
| `minio` | 9000 | S3 API | object storage |
| `postgresql` | 5432 | PostgreSQL | service schemas/databases |

## 服务间通信

| 模式 | 技术 | 适用场景 |
|------|------|------|
| 外部 API | HTTPS REST | 前端 Workbench、第三方系统调用 |
| 同步内部调用 | Tonic gRPC | 用户鉴权、Profile 解析、元数据查询、作业状态查询 |
| 异步事件 | NATS JetStream | 数据流水线、分析构建、告警评估、集成同步 |
| 大文件/中间文件 | MinIO/S3 | 原始文件、staging 文件、Parquet、导出文件 |

同步调用只用于短请求。长耗时处理必须转为事件或作业。

## 事务驱动事件流水线

STDAS 不使用跨服务分布式事务。每个服务在本地事务中提交状态和 Outbox Event，由 NATS JetStream 发布事件，由订阅服务通过 Inbox 做幂等消费。

```text
Command / Event
  -> Local Transaction
  -> Write Local State
  -> Write Outbox Event
  -> Commit
  -> Outbox Publisher
  -> NATS JetStream
  -> Consumer Inbox
  -> Local Transaction
```

主事件链：

```text
FileRegistered
  -> FileStored
  -> FileValidated
  -> ProfileResolved
  -> FileParsed
  -> DataNormalized
  -> CanonicalDataCommitted
  -> AggregatesRequested
  -> AggregatesBuilt
  -> AlertEvaluationRequested
  -> AlertRaised / AlertCleared
  -> DataVersionReady
```

`workflow-service` 负责跨服务流程编排、超时、重试、失败状态和补偿动作。单个服务只负责自己的本地一致性。

## Profile Resolution

客户、产品、测试类型、测试站点、设备和文件格式差异通过统一 key 解析：

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

`customer-service` 根据该 key 返回 DataProfile、ParserProfile、MappingProfile、SpecProfile、AlertRuleSet、FeatureFlags 和客户专属扩展声明。`data-pipeline-service` 只依赖解析结果，不直接引用客户专用实现。

`DataProfile` 不是“一个客户一个 Profile”。它描述某类测试数据在特定解析上下文下的规则集合，通常由 `{customer_code, product, test_type, test_station, equipment_type, file_format, program_name, program_version, effective_time}` 定位。一个客户可以拥有多个 DataProfile；多个 DataProfile 也可以引用同一个 ParserRule、MappingRule 或 SpecRule。

## 扩展能力边界

架构文档只定义能力边界，不穷举所有未来功能。分析能力、客户专属功能、报表模板、数据接入类型和外部集成都必须通过 registry / profile / contract 扩展，而不是把每一种能力写死在总体架构里。

`analytics-service` 只承诺分析执行框架：

- 统一查询上下文、CustomerScope、DataVersion 和权限校验。
- 查询预算、超时、同步/异步切换。
- 分析算法 registry。
- OLAP backend adapter。
- 结果版本、缓存和导出。
- 客户专属分析扩展隔离。

具体分析方法可以持续演进，例如良率、Bin、参数分布、规格限、相关性、趋势、异常检测、设备/TestStation/Site 对比、报表型分析、工程经验规则、客户专属模型或其他后续定义的能力。总体架构不把分析能力限制在某几个固定算法上。

## 数据边界

默认采用一个 PostgreSQL 实例，按服务 schema 或 database 隔离。多节点或高负载时，可把高压力服务迁移到独立 PostgreSQL 实例。

```text
identity.*
customer.*
data_pipeline.*
analytics.*
workflow.*
integration.*
```

规则：

- 服务只能写自己的 schema/database。
- 服务不能直接 join 其他服务内部表。
- 跨服务查询通过 gRPC query API、只读 projection 或事件构建的本地视图完成。
- 参数明细和大规模 OLAP 不长期压在 PostgreSQL 上，进入 Parquet/DuckDB，必要时进入 ClickHouse。

## 单节点与多节点

单节点部署：

```text
localhost:
  stdas-gateway
  identity-service
  customer-service
  data-pipeline-service
  analytics-service
  workflow-service
  integration-service
  nats-server
  postgresql
  minio
```

多节点部署只改变配置：

```diff
- customer_service = "http://localhost:50052"
+ customer_service = "http://10.0.1.20:50052"

- nats_server = "nats://localhost:4222"
+ nats_server = "nats://10.0.1.30:4222"
```

服务代码不因单节点或多节点而改变。

## 架构约束

- 不允许绕过 `stdas-gateway` 暴露内部服务给前端。
- 不允许服务直接写其他服务的数据库对象。
- 不允许跨服务同步调用形成循环依赖。
- 不允许把长耗时任务放在 HTTP/gRPC 请求生命周期内等待完成。
- 不允许为每个客户 fork 系统。
- 不允许在核心流程中写客户/测试类型/测试站点/设备硬编码分支。
- Parser 只产生标准解析结果，不直接写 analytics 或 integration 数据。
- Analytics 查询必须有预算、超时和异步任务通道。
- PostgreSQL 不作为无限 OLAP 引擎。
