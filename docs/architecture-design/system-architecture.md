# 系统架构

## 架构定位

STDAS 采用 **Rust + Axum + SQLx 的分阶段数据平台架构**。当前阶段采用 `stdas-gateway` 单一运行服务和强模块边界；未来只有在触发条件满足时，才把模块升级为独立 crate 或 runtime service。

本架构不再把第一版定义为多运行服务微服务拓扑。服务化仍是长期可选演进方向，但不是当前实现目标。当前目标是用一个可运行、可调试、可验证的 `stdas-gateway` 承载清晰业务模块，避免在 minimal app 阶段引入 gRPC、NATS、Outbox/Inbox、MinIO、多服务配置和多进程调试成本。

STDAS 服务对象是 OSAT 封测代工厂。核心复杂度来自多客户、多测试类型、多测试站点、多设备类型、多文件格式、多规则版本的数据摄入、归一化、分析和追溯。因此服务不按页面、CRUD 或单个技术步骤拆分，而按稳定平台能力拆分。

## 服务抽象原则

- 当前阶段采用 single runtime modular monolith。
- 模块边界按业务所有权和数据所有权划分，不按函数名、页面或 CRUD 拆分。
- 高频协作、强事务相邻、同一数据生命周期内的能力放在同一模块内。
- 需要独立扩展、独立故障隔离、独立对外集成、独立部署或明确团队边界的能力，才考虑从 module 升级为 crate 或 runtime service。
- Observability、health、metrics、audit 是横切能力，不作为独立业务服务起步。
- 服务拆分由 [ADR-0014](adr/0014-gateway-modular-monolith.md) 的黄色/红色触发条件驱动，不由预设时间表驱动。

## 服务边界总览

```text
Frontend Workbench
      |
      | HTTPS
      v
stdas-gateway
  |-- system
  |-- identity
  |-- customer
  |-- data_pipeline
  |-- analytics
  |-- evidence
  |-- workflow
  |-- integration
  |-- audit / telemetry / shared
      |
      v
Data Platform, introduced only when real use cases exist
  |-- PostgreSQL + SQLx
  |-- object storage, Parquet, DuckDB, ClickHouse as future expansion
```

未来满足拆分条件时，模块可演进为：

```text
frontend/web
  -> stdas-gateway / BFF
       -> identity-service
       -> customer-service
       -> data-pipeline-service
       -> analytics-service
       -> workflow-service
       -> integration-service
```

## 当前模块与未来服务

| 层级 | 当前模块 | 职责 | 未来可能升级为 |
|------|----------|------|----------------|
| Edge | `stdas-gateway` runtime | HTTPS API、BFF、认证上下文注入、请求聚合、限流、OpenAPI | 保持 Gateway / BFF |
| Control Plane | `modules/identity` | 用户、工程师/管理员角色、token、会话、权限、CustomerScope | `identity-service` |
| Control Plane | `modules/customer` | CustomerConfig、DataProfile、ProfileResolutionKey、规则版本、扩展注册、客户专属能力隔离；不是普通客户 CRUD | `customer-service` |
| Control Plane | `modules/workflow` | 作业状态、事件编排、重试、补偿、Dead Letter 处理入口 | `workflow-service` |
| Data Plane | `modules/data_pipeline` | 文件登记、raw metadata、解析、归一化、canonical TestData、DataVersion、lineage | `data-pipeline-service` |
| Analysis Plane | `modules/analytics` | 分析查询、聚合、QuerySnapshot、分析结果、导出，以及可扩展分析能力 | `analytics-service` |
| Evidence | `modules/evidence` | 证据链视图：DataVersion、lineage、QuerySnapshot、analysis result、export/report 引用 | crate 或 service，按压力判断 |
| Integration | `modules/integration` | MES schema reference、未来 MES runtime connector、客户接口、外部文件交换 | `integration-service` |

## 为什么这样合并

| 原细分能力 | 合并到 | 原因 |
|------|------|------|
| ingestion / normalization / testdata | `data-pipeline-service` | 三者属于同一条数据生命周期，强顺序、强幂等、强 lineage，过早拆开会制造大量跨服务事务和契约成本 |
| alerting | `analytics-service` | 告警本质是分析结果或规则评估，依赖聚合、数据版本和分析上下文 |
| workspace | `analytics-service` | 分析会话、模板、案例和导出紧贴分析体验，第一版不需要独立服务 |
| 客户配置与规则解析 | `customer-service` | 该服务的本质不是客户 CRUD，也不是一个客户一个 Profile，而是承载客户专属配置、DataProfile、规则版本和受控扩展 |
| observability-service | `telemetry/` + `audit/` + 各模块内 lineage | tracing、metrics、audit 必须内建；lineage 归属 `data_pipeline`；后续只有在审计查询规模很大时才考虑独立 |

## 当前进程与未来端口

当前阶段只启动 `stdas-gateway`。端口是默认建议，可通过配置修改。

| 进程 | 默认端口 | 对外协议 | 存储 |
|------|------:|------|------|
| `stdas-gateway` | 8080 | HTTPS / REST | PostgreSQL；当前已用于 `c_users`、`c_roles`、`c_user_rl`、`r_user_session` 最小身份会话模型 |

未来服务化后才引入独立服务端口、gRPC、NATS 和对象存储：

| 未来进程 | 默认端口 | 对外协议 | 存储 |
|----------|------:|------|------|
| `identity-service` | 50051 | gRPC | PostgreSQL `identity` |
| `customer-service` | 50052 | gRPC | PostgreSQL `customer` |
| `data-pipeline-service` | 50053 | gRPC + NATS | PostgreSQL `data_pipeline` + object storage |
| `analytics-service` | 50054 | gRPC + NATS | PostgreSQL `analytics` + Parquet/DuckDB/ClickHouse |
| `workflow-service` | 50055 | gRPC + NATS | PostgreSQL `workflow` |
| `integration-service` | 50056 | gRPC + NATS | PostgreSQL `integration` |
| `nats-server` | 4222 | NATS | JetStream storage |
| `minio` | 9000 | S3 API | object storage |
| `postgresql` | 5432 | PostgreSQL | module schemas or future service schemas |

## 服务间通信

| 模式 | 技术 | 适用场景 |
|------|------|------|
| 外部 API | HTTPS REST | 前端 Workbench、第三方系统调用 |
| 当前内部协作 | Rust module call / service usecase | 同一 `stdas-gateway` runtime 内的模块协作 |
| 未来同步内部调用 | Tonic gRPC | 服务拆分后的用户鉴权、Profile 解析、元数据查询、作业状态查询 |
| 未来异步事件 | NATS JetStream | 服务拆分后的数据流水线、分析构建、告警评估、集成同步 |
| 未来大文件/中间文件 | object storage / S3 compatible | 原始文件、staging 文件、Parquet、导出文件 |

同步调用只用于短请求。长耗时处理必须转为事件或作业。

## 未来事务驱动事件流水线

当前阶段不引入 NATS、Outbox/Inbox 或 Saga runtime。只要仍是单一运行服务，优先在 owning module 的 usecase/repository 边界管理本地一致性。

未来拆分 runtime service 后，STDAS 不使用跨服务分布式事务。每个服务在本地事务中提交状态和 Outbox Event，由 NATS JetStream 发布事件，由订阅服务通过 Inbox 做幂等消费。

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

当前阶段 `modules/workflow` 可以先作为内部 job/process boundary；只有跨模块长流程、重试、补偿和失败隔离压力持续出现时，才升级为 `workflow-service`。

## Profile Resolution

客户、产品、测试类型、测试站点、设备和文件格式差异通过统一 key 解析。下列字段名是概念性示例，不是最终数据库字段、Rust 字段、API field 或 frontend label；正式字段必须等待 MES schema 审查后确定。

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

当前阶段 `modules/customer` 根据该 key 返回 DataProfile、ParserProfile、MappingProfile、SpecProfile、AlertRuleSet、FeatureFlags 和客户专属扩展声明。未来服务化后由 `customer-service` 承担该职责。`data_pipeline` 只依赖解析结果，不直接引用客户专用实现。

`DataProfile` 不是“一个客户一个 Profile”。它描述某类测试数据在特定解析上下文下的规则集合，通常由 `{customer_code, product, test_type, test_station, equipment_type, file_format, program_name, program_version, effective_time}` 定位。一个客户可以拥有多个 DataProfile；多个 DataProfile 也可以引用同一个 ParserRule、MappingRule 或 SpecRule。

## 扩展能力边界

架构文档只定义能力边界，不穷举所有未来功能。分析能力、客户专属功能、报表模板、数据接入类型和外部集成都必须通过 registry / profile / contract 扩展，而不是把每一种能力写死在总体架构里。

`modules/analytics` 只承诺分析执行框架；未来服务化后由 `analytics-service` 承担：

- 统一查询上下文、CustomerScope、DataVersion 和权限校验。
- 查询预算、超时、同步/异步切换。
- 分析算法 registry。
- OLAP backend adapter。
- 结果版本、缓存和导出。
- 客户专属分析扩展隔离。

具体分析方法可以持续演进，例如良率、Bin、参数分布、规格限、相关性、趋势、异常检测、设备/TestStation/Site 对比、报表型分析、工程经验规则、客户专属模型或其他后续定义的能力。总体架构不把分析能力限制在某几个固定算法上。

## 数据边界

当前阶段可以采用一个 PostgreSQL 实例，并按 module ownership 组织 schema/table 边界。未来服务化后，按服务 schema 或 database 隔离。多节点或高负载时，可把高压力服务迁移到独立 PostgreSQL 实例。

```text
identity.*
customer.*
data_pipeline.*
analytics.*
workflow.*
integration.*
```

规则：

- 当前模块只能写自己拥有的数据对象。
- 未来服务只能写自己的 schema/database。
- 未来服务不能直接 join 其他服务内部表。
- 跨模块或跨服务查询通过 owning module query API、gRPC query API、只读 projection 或事件构建的本地视图完成。
- 参数明细和大规模 OLAP 不长期压在 PostgreSQL 上，进入 Parquet/DuckDB，必要时进入 ClickHouse。

## 服务化与多节点

当前阶段是单运行服务：

```text
localhost:
  stdas-gateway
```

未来服务拆分后，多节点部署只改变配置：

```diff
- customer_service = "http://localhost:50052"
+ customer_service = "http://10.0.1.20:50052"

- nats_server = "nats://localhost:4222"
+ nats_server = "nats://10.0.1.30:4222"
```

服务代码不因单节点或多节点而改变；服务拆分本身必须先经过 ADR-0014 的触发条件审查。

## 架构约束

- 不允许绕过 `stdas-gateway` 暴露内部服务给前端。
- 不允许模块直接写其他模块拥有的数据对象；未来服务不得直接写其他服务数据库对象。
- 不允许跨模块或跨服务同步调用形成循环依赖。
- 不允许把长耗时任务放在 HTTP/gRPC 请求生命周期内等待完成。
- 不允许为每个客户 fork 系统。
- 不允许在核心流程中写客户/测试类型/测试站点/设备硬编码分支。
- Parser 只产生标准解析结果，不直接写 analytics 或 integration 数据。
- Analytics 查询必须有预算、超时和异步任务通道。
- PostgreSQL 不作为无限 OLAP 引擎。
