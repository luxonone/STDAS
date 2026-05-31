# 架构决策记录

ADR 记录影响系统长期演进的重要决策。每条 ADR 只描述一个决策。

| ADR | 状态 | 决策 |
|-----|------|------|
| [0001](0001-rust-data-platform.md) | Accepted | STDAS 采用 Rust 数据平台架构 |
| [0002](0002-rust-distributed-microservices-platform.md) | Accepted | 采用 Rust 原生分布式微服务数据平台架构 |
| [0003](0003-customer-differences-via-dataprofile.md) | Accepted | 客户差异通过 DataProfile / FeatureFlag / Extension 治理 |
| [0004](0004-gateway-only-external-entry.md) | Accepted | `stdas-gateway` 是唯一外部 API 入口 |
| [0005](0005-nats-outbox-inbox-saga.md) | Accepted | 采用 NATS JetStream + Outbox/Inbox + Saga，不使用跨服务分布式事务 |
| [0006](0006-query-snapshot-freezes-analysis-semantics.md) | Accepted | 采用 QuerySnapshot 冻结分析语义 |
| [0007](0007-postgresql-not-infinite-olap.md) | Accepted | PostgreSQL 不作为无限 OLAP，引入 Parquet/DuckDB，必要时 ClickHouse |
| [0008](0008-feature-slice-delivery.md) | Accepted | 前后端按功能切片同步交付 |
| [0009](0009-gateway-loco-framework.md) | Accepted | `stdas-gateway` 采用 Loco 作为 HTTP 应用框架 |
| [0010](0010-monorepo-project-layout.md) | Accepted | 采用清晰的 `apps/` + `crates/` monorepo 目录结构 |
