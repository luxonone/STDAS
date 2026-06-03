# ADR-0014：`stdas-gateway` 采用单一运行服务与强模块边界

## 状态

Accepted

## 背景

STDAS 当前仍处于最小应用阶段。项目维护者预计很长时间内不会拆分后端运行服务，同时明确要求不能因为单服务而牺牲领域边界和未来可拆分性。

此前 ADR-0002 把第一版描述为明确的分布式微服务拓扑，会过早引入多进程、gRPC、NATS、Outbox/Inbox、MinIO、独立配置、服务间鉴权和多服务调试成本。该复杂度不适合当前 minimal app、学习阶段和可信数据闭环的近期目标。

本 ADR 不否定长期可能服务化，而是重新定义当前阶段的 runtime 形态和未来拆分触发机制。

## 决策

STDAS 当前采用 **modular monolith**：

```text
frontend/web
  -> backend/services/stdas-gateway
       -> internal business modules
```

`stdas-gateway` 保留名称。它在当前阶段同时承担：

- 唯一后端运行服务。
- 唯一外部 HTTP API 入口。
- Axum application assembly。
- 内部业务模块容器。

未来如果触发服务拆分，`stdas-gateway` 再逐步收敛为更薄的 Gateway / BFF。

## 内部模块边界

`stdas-gateway` 内部按未来服务边界组织模块：

```text
src/
  modules/
    identity/
    customer/
    data_pipeline/
    analytics/
    evidence/
    workflow/
    integration/

  telemetry/
  audit/
  middleware/
  errors/
  shared/
  system/
```

模块职责：

| Module | 职责 | 未来可能升级为 |
|--------|------|----------------|
| `identity` | 用户、角色、session、permission、CustomerScope | `identity-service` |
| `customer` | CustomerConfig、DataProfile、ProfileResolutionKey、rule binding、feature flags、customer extension registry；不是普通客户 CRUD | `customer-service` |
| `data_pipeline` | file registration、raw metadata、parser selection、normalization、DataVersion、lineage | `data-pipeline-service` |
| `analytics` | query、aggregation、QuerySnapshot、analysis result、export | `analytics-service` |
| `evidence` | DataVersion、lineage、QuerySnapshot、analysis result、export/report 的证据链视图 | 可独立 crate；是否独立服务以后按压力判断 |
| `workflow` | job state、retry、compensation、long-running process coordination | `workflow-service` |
| `integration` | MES schema reference、未来 MES runtime connector、客户接口、外部文件交换 | `integration-service` |

横切能力：

- `telemetry`：tracing、metrics、request id、correlation id。
- `audit`：记录谁或哪个系统对业务对象做了什么。
- `lineage`：归属 `data_pipeline`，因为它描述数据生命周期来源链。
- `shared`：只放稳定、低业务含义、跨模块确实共用的基础类型；不得成为垃圾桶。
- `system`：health、preflight 等运维端点，不属于业务服务边界。

## 拆分触发机制

默认不拆运行服务。未来拆分必须由触发条件驱动，不由预设时间表、文档蓝图或“微服务更高级”驱动。

推荐演进路径：

```text
module -> independent crate -> runtime service
```

黄色观察信号：

- 某个模块代码明显膨胀，已经难以在 `stdas-gateway` 内阅读。
- 模块需要独立 fixture、mock、domain type 或测试边界。
- handler、service、repository 开始跨模块直接调用。
- 某模块引入 object storage、DuckDB、ClickHouse、MES client 等重依赖，但只有它使用。
- 一个模块的改动频繁扩大整个后端测试范围。

红色拆分信号：

- parser、normalization 或 analytics query 占用 CPU / memory，影响普通 API。
- parser、MES connector、analytics timeout 等失败会拖垮整个后端。
- 某模块需要长期 worker，而 Gateway 需要保持轻量同步 API。
- 某模块需要独立扩容或独立部署。
- 多个模块直接写同一核心对象，例如 DataVersion、DataProfile、Evidence。
- 某模块需要特殊账号、特殊网络、敏感数据隔离或强审计边界。
- 多人团队长期按 identity、data pipeline、analytics 等区域分工。

## MES 和字段策略

第一阶段 MES 只作为字段语义和命名参考源，不作为运行时依赖。

在提供可读 MES 数据库前，不把数据库字段、Rust struct 字段、API field 或 frontend label 定死。后续字段治理采用：

```text
MES source field
  -> STDAS canonical field
  -> API field
  -> Frontend display label
  -> Evidence / lineage source reference
```

本 ADR 不定义具体字段名，也不记录前端页面、路由、筛选或 UI 设计。

## 影响

- ADR-0002 对“第一版固定多运行服务”的判断被本 ADR supersede。
- ADR-0005 的 NATS JetStream、Outbox/Inbox、Saga 机制保留为未来多服务阶段的架构方向；当前阶段不作为 Phase 0 必须实现项。
- ADR-0011 的 Axum + SQLx 技术选择继续有效，但 `stdas-gateway` 内部结构由全局 `handlers/services/repositories/dto/models` 分层调整为 business module boundary + cross-cutting boundary。
- ADR-0013 的 `backend/` + `frontend/` 物理分区继续有效，但未来服务引入顺序由触发条件驱动。

## 验证

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo run -p stdas-gateway -- routes`
