# ADR-0002：采用 Rust 原生分布式微服务数据平台架构

状态：Superseded
日期：2026-05-17

> 当前第一阶段 runtime 形态已由 [ADR-0014](0014-gateway-modular-monolith.md) 调整为 `stdas-gateway` 单一运行服务 + 强模块边界。本文保留为历史设计记录和未来服务化参考。

## 背景

STDAS 服务对象是一个 OSAT 封测代工厂。系统需要支撑多客户、多测试类型、多测试站点、多设备类型、多文件格式、多规则版本的数据摄入、归一化、分析和追溯。

项目允许从第一版开始采用微服务，但微服务边界必须明确、稳定，并且服务划分要服务于 OSAT 数据平台能力，而不是简单照搬通用 CRUD、页面功能、单个处理步骤或按客户复制系统。

## 决策

STDAS 采用 Rust 原生分布式微服务数据平台架构：

- 每个服务是独立 Rust 进程。
- 每个服务有独立端口、配置、日志、指标和健康检查。
- 单节点部署是分布式拓扑的 `n=1` 形态，不是架构退让。
- 多节点部署通过配置改变服务地址，不改变代码。
- 外部访问统一经过 `stdas-gateway`。
- 服务间同步调用使用 gRPC。
- 长流程和异步任务使用 NATS JetStream。
- 跨服务一致性采用 Outbox/Inbox、Saga、Process Manager、幂等消费和补偿。
- 数据按服务 schema/database 隔离。
- 原始文件和分析文件进入 MinIO/S3 与 Parquet/DuckDB，必要时引入 ClickHouse。

## 服务边界

第一版固定以下服务类别：

- `stdas-gateway`
- `identity-service`
- `customer-service`
- `data-pipeline-service`
- `analytics-service`
- `workflow-service`
- `integration-service`

其中 `customer-service` 是客户专属服务，负责 CustomerConfig、DataProfile、规则复用/分叉、FeatureFlags 和客户专属扩展登记；`data-pipeline-service` 承担摄入、解析、归一化和 canonical TestData 提交；`analytics-service` 承担分析执行框架、告警、分析会话、模板和导出；`workflow-service` 是跨服务流程编排中心。

不把 ingestion、normalization、testdata、alerting、workspace、observability 在第一版拆成独立服务。它们分别作为 `data-pipeline-service`、`analytics-service` 或各服务基础能力的一部分存在。

## 原因

微服务适合 STDAS 的原因：

- OSAT 数据处理链路天然存在长流程、异步任务和可重试步骤。
- 数据流水线、分析查询、流程编排、外部集成的扩展压力不同，适合独立进程扩展。
- Rust 单二进制部署成本低，适合 Windows / Linux 原生进程部署。
- 客户现场部署可能从单机演进到多机，配置即拓扑更清晰。
- 独立服务便于隔离高风险处理，例如 parser、外部集成、大查询和导出。

必须控制的风险：

- 不按客户拆服务。
- 不允许服务之间直接共享数据库表。
- 不允许同步调用链形成循环依赖。
- 不允许为了“服务数量”而拆出没有业务所有权的服务。
- 不允许把强顺序、强一致、同一数据生命周期内的步骤过早拆成多个服务。
- 不允许把跨服务一致性交给隐式约定，必须使用 Outbox/Inbox 和 workflow 编排。

## 后果

正面：

- 服务边界清晰，运行拓扑明确。
- 单节点和多节点部署模型一致。
- 不同能力可以独立扩展、独立观测、独立重启。
- 长流程具备重试、补偿、回放和审计基础。
- DataProfile 和客户专属服务能系统性承接客户差异，同时避免一个客户一个 Profile 的歧义。

代价：

- 第一版就需要建设 NATS、gRPC、配置、健康检查、日志指标和进程管理。
- 本地开发会启动更多进程。
- 跨服务数据一致性必须通过事件、Saga 和幂等机制治理。
- 测试需要覆盖单服务、契约、事件流和端到端流程。

## 验证方式

- 所有服务能在 Windows / Linux 以独立进程启动。
- 所有服务暴露 health、metrics、version。
- gateway 不直接访问服务数据库。
- 服务之间只通过 gRPC、NATS event、对象存储引用协作。
- 所有事件 consumer 具备 inbox 幂等。
- 文件摄入到 DataVersionReady 的主链路可重试、可追踪、可回放。
- ProfileResolutionKey 可解析到 DataProfile，并进一步绑定 parser/mapping/spec rule。
- 不允许为客户 fork 代码分支或复制服务。
