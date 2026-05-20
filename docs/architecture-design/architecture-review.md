# 架构合理性审查

## 审查结论

当前 STDAS 架构方向总体合理：采用 Rust 原生分布式微服务、粗粒度服务边界、DataProfile 规则治理、事件驱动流水线和分析能力扩展框架，能覆盖 OSAT 多客户、多测试类型、多测试站点、多设备、多规则版本的数据平台复杂度。

当前设计的主要风险不在服务拆分，而在后续前端和后端是否能围绕同一功能切片同步推进。如果前端只按页面发散、后端只按服务发散，会导致 API 契约、状态流、错误语义和验收标准脱节。因此下一阶段必须以“功能切片 + API 契约 + UI/UX 约束 + 可验证 milestone”为共同边界。

## 六问审查

| 问题 | 当前状态 | 结论 | 后续动作 |
|------|------|------|------|
| 1. 项目规则是否沉淀？ | 已沉淀 | 架构规则、AI Agent 运行规则、前后端同步规则已分别落入文档 | 后续修改前读取 `docs/architecture-design/ai-agent-runtime-rules.md` |
| 2. build/test 是否可跑？ | 本机工具链已验证，项目级命令暂不可跑 | Phase 0.5 已确认 Rust、Node、pnpm、PostgreSQL、NATS、MinIO、Redis 等工具可用；但当前尚无 `Cargo.toml` / `package.json`，不能声称项目级 build/test 通过 | Phase 0 代码骨架创建后重新执行环境验证 gate |
| 3. API 契约是否严格？ | 已补齐设计规则 | 已明确响应信封、错误码、字段取值范围、默认值、编码约束和版本兼容要求 | 具体功能设计时继续补端点级契约 |
| 4. 是否跟着主流架构走？ | 基本符合 | Rust 微服务、gRPC、NATS、PostgreSQL、S3/MinIO、React + TypeScript 分析工作台方向合理 | 文档中要求优先采用对应技术官方/主流推荐布局 |
| 5. UI/UX 约束是否写明？ | 已写明 | 已补表单长内容、关键按钮、结构化输入、高级筛选、首屏主任务等硬约束 | 具体页面设计时逐项验收 |
| 6. 任务是否小步可验证？ | 已补齐设计规则 | 已按骨架、主链路、编辑流、优化体验、回归验证组织同步 milestone | 每个功能切片必须有独立验收矩阵 |

## 服务边界审查

| 服务 | 审查 |
|------|------|
| `stdas-gateway` | 合理。前端只访问 gateway，内部服务不暴露给前端，能稳定外部 API 契约。 |
| `identity-service` | 合理。身份、权限、CustomerScope 独立，避免业务服务重复处理认证细节。 |
| `customer-service` | 合理，但必须持续强调它不是简单客户 CRUD，而是客户专属配置、DataProfile、规则治理和扩展隔离中心。 |
| `data-pipeline-service` | 合理。摄入、解析、归一化、canonical TestData 同属强顺序数据生命周期，合并能减少跨服务事务。 |
| `analytics-service` | 合理。必须保持“分析执行框架”定位，不把分析能力写死为少数算法清单。 |
| `workflow-service` | 合理。跨服务长流程、重试、补偿、Dead Letter 应集中治理。 |
| `integration-service` | 合理。外部系统接入风险高，应独立隔离。 |

## 架构风险与约束

### 风险 1：DataProfile 规则治理复杂度上升

DataProfile 支持共享、复制分叉、继承覆盖、冻结版本，这会带来规则治理复杂度。必须要求每个规则对象具备：

- 稳定 ID。
- 版本号。
- 适用范围。
- 来源。
- `forked_from_rule_id` / `forked_from_version`。
- 生效时间和失效时间。
- 审计记录。

### 风险 2：客户专属扩展污染默认路径

客户强制要求的分析能力或解析逻辑可能与统一架构冲突。必须通过 `CustomerExtension + FeatureFlag + sandbox/adapter` 隔离，默认路径不能出现客户硬编码分支。

### 风险 3：前端和后端不同步

后续所有功能必须同时定义：

- 前端页面/组件/状态。
- 后端 API/gRPC/event/data。
- API 字段取值范围、默认值、错误码。
- UI/UX 约束。
- 验收方式。

### 风险 4：项目级 build/test 仍待代码骨架创建后验证

Phase 0.5 已完成本机工具链验证，但当前仓库仍是文档基线，尚未创建 Rust workspace 或 React app。正式项目级 `cargo` / `pnpm` build、test、lint、typecheck 必须在 Phase 0 代码骨架创建后重新执行，不能用工具链验证替代项目验证。

## 审查结论

架构可以作为下一阶段前后端同步设计的基础。当前不需要改变服务边界；设计规则、API 契约、UI/UX 约束和小步可验证 milestone 已补齐到文档，后续重点是按功能切片同步推进前端与后端设计。
