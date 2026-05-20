# ADR-0007：PostgreSQL 不作为无限 OLAP

状态：Accepted
日期：2026-05-18

## 背景

STDAS 第一版可以用 PostgreSQL 保存业务事实、聚合和部分参数数据。但多 Lot、多参数、长时间窗口和高并发分析会超过 PostgreSQL 作为 OLAP 引擎的合理边界。

## 决策

PostgreSQL 是权威业务库，不承担无限 OLAP：

- 阶段 1：PostgreSQL 保存业务事实、聚合表和受预算控制的参数数据。
- 阶段 2：参数明细进入 Parquet，DuckDB 承担中等规模交互式分析。
- 阶段 3：高并发、多维 OLAP 引入 ClickHouse。

API contract 保持稳定，由 analytics backend 切换实现。

## 后果

正面：

- 第一版实现成本可控。
- 后续数据规模增长时有明确迁移路径。
- 避免把 PostgreSQL 优化成不合适的无限分析引擎。

代价：

- 数据索引、schema、lineage 和对象存储引用需要从早期设计。
- 分析服务必须隔离查询语义和执行后端。

## 替代方案

- 第一版直接上 ClickHouse：部署和运维成本过高。
- 永久只用 PostgreSQL：规模增长后查询和导出风险高。

## 验证方式

- 所有分析端点定义查询预算。
- 超预算请求必须拒绝、降级或转异步。
- analytics-service 不把 PostgreSQL 大范围扫描作为兜底策略。
