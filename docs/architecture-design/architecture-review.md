# 架构合理性审查

本文是架构审查记录，主要约束 `stdas-gateway` modular monolith 和后端能力边界。前端/产品设计的当前有效范围以 [docs 入口](../README.md) 和 [前端设计 README](../frontend-design/README.md) 为准。

## 审查结论

当前 STDAS 架构方向调整为：**Rust + Axum + SQLx、`stdas-gateway` 单一运行服务、强模块边界、服务拆分触发条件驱动**。

该方向比第一版直接铺开多运行服务更适合当前 minimal app 和学习阶段。它降低多进程、gRPC、NATS、Outbox/Inbox、MinIO 和部署配置的早期复杂度，同时通过 `modules/` 保留未来服务边界，避免单服务退化为大泥团。

当前架构审查重点不再是“服务数量是否完整”，而是：

- `stdas-gateway` 是否保持唯一外部 API 入口。
- 内部业务模块是否按未来服务边界组织。
- `customer` 是否持续表达 DataProfile / rule governance，而不是普通客户 CRUD。
- `data_pipeline` 是否拥有 ingestion / normalization / DataVersion / lineage。
- `evidence` 是否独立表达可信结果证据链。
- `telemetry`、`audit`、`shared` 是否保持横切能力边界。
- 服务拆分是否只由触发条件驱动。

本审查不讨论前端页面、路由、筛选、字段显示或产品设计细节。

## 六问审查

| 问题 | 当前状态 | 结论 | 后续动作 |
|------|----------|------|----------|
| 1. 项目规则是否沉淀？ | 已沉淀 | SPEC、AI Agent 运行规则、Git SOP、Rust 规则、ADR 已有入口 | 后续修改前读取 `docs/architecture-design/ai-agent-runtime-rules.md` 和任务相关文档 |
| 2. build/test 是否可跑？ | 已可跑最小项目命令 | 当前 Rust workspace 和前端 workspace 已存在；后端最小路由已验证过 | 架构/代码变更后继续执行 cargo 验证 |
| 3. API 契约是否严格？ | 框架已建立，具体契约待功能切片 | 当前不提前定义字段；MES schema 到位后再校准字段和 API | 功能切片设计时补端点级契约 |
| 4. 是否跟着主流架构走？ | 基本符合 | Axum app assembly、handler/usecase/repository 分层、SQLx 显式 SQL、modular monolith 均是主流可维护方向 | 不照搬微服务复杂度，不创建 fake code |
| 5. UI/UX 约束是否写明？ | 不在本轮处理 | 本审查不承载前端/产品事实来源 | 以 docs 入口和前端设计 README 的当前状态为准 |
| 6. 任务是否小步可验证？ | 已重新收敛 | Phase 0 先验证 Axum 单服务与 module boundary，不把 NATS/MinIO/gRPC 作为当前验收 | roadmap 按 ADR-0014 调整 |

## 模块边界审查

| 当前边界 | 审查 |
|----------|------|
| `stdas-gateway` | 合理。当前唯一 backend runtime service，也是唯一外部 API 入口。未来服务拆分后再收敛为薄 Gateway / BFF。 |
| `modules/identity` | 合理。身份、权限、CustomerScope 需要独立边界，但当前不急于拆成 runtime service。 |
| `modules/customer` | 合理。必须持续强调它不是简单客户 CRUD，而是 CustomerConfig、DataProfile、ProfileResolutionKey、规则治理和扩展隔离中心。 |
| `modules/data_pipeline` | 合理。摄入、解析、归一化、DataVersion、lineage 同属强顺序数据生命周期，当前放在同一 module。 |
| `modules/analytics` | 合理。必须保持“分析执行框架”定位，不把分析能力写死为少数算法清单，也不在可信数据基础前做重 OLAP。 |
| `modules/evidence` | 合理。Evidence 连接 data pipeline、analytics 和前端证据展示，不应被某一侧吞掉。 |
| `modules/workflow` | 合理。当前作为内部 job/process boundary；跨模块长流程复杂后再评估 runtime service。 |
| `modules/integration` | 合理。MES 第一阶段只作字段语义参考；runtime connector 出现后再评估独立服务。 |

## 架构风险与约束

### 风险 1：单服务退化为大泥团

缓解方式：

- 所有业务代码必须进入 owning module。
- 不恢复全局 `handlers/services/repositories/dto/models` 大杂烩。
- 跨模块调用必须通过明确 query/usecase boundary。
- `shared/` 不得存放具体业务流程。

### 风险 2：DataProfile 规则治理复杂度上升

DataProfile 支持共享、复制分叉、继承覆盖、冻结版本时，必须要求每个规则对象具备：

- 稳定 ID。
- 版本号。
- 适用范围。
- 来源。
- `forked_from_rule_id` / `forked_from_version`。
- 生效时间和失效时间。
- 审计记录。

具体字段等待 MES schema 和真实样例文件校准。

### 风险 3：客户专属扩展污染默认路径

客户强制要求的分析能力或解析逻辑可能与统一架构冲突。必须通过 CustomerExtension、FeatureFlag、sandbox/adapter 隔离，默认路径不能出现客户硬编码分支。

### 风险 4：过早引入未来基础设施

NATS、Outbox/Inbox、MinIO、gRPC、DuckDB、ClickHouse、Redis adapter 都可能有长期价值，但不能作为 Phase 0 必做项。只有真实功能、数据规模或拆分触发条件出现后才引入。

### 风险 5：字段假设沉淀

数据库字段、Rust struct 字段、API field、frontend label 都必须等待 MES schema 和真实样例文件校准。当前文档中的字段名如果没有经过 MES 审查，只能视为概念性示例。

## 审查结论

当前架构可以作为下一阶段后端实现和文档清理的基础。重点是坚持 `stdas-gateway` modular monolith、业务模块边界、MES 字段校准和服务拆分触发机制；不要在当前阶段继续扩写前端/产品设计或过早实现未来分布式基础设施。
