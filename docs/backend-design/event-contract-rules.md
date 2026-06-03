# 事件契约规则

本文件定义 STDAS 后端事件契约。当前阶段遵循 [ADR-0014](../architecture-design/adr/0014-gateway-modular-monolith.md)，事件契约先作为 module boundary、长流程、审计和未来服务化的设计约束；不要求 Phase 0 立即启用 NATS JetStream 或多个服务进程。未来拆分服务后，事件用于跨服务异步协作、长流程推进、投影构建和审计追踪。事件契约是后端事实来源；前端不得直接消费内部事件。

## 通用事件信封

所有通过 NATS JetStream 发布的业务事件必须包含统一信封：

| 字段 | 要求 |
|------|------|
| `event_id` | 全局唯一事件 ID |
| `event_type` | 稳定事件类型，例如 `FileRegistered` |
| `schema_version` | 事件 schema 版本 |
| `occurred_at` | RFC3339 时间 |
| `producer_service` | 发布服务 |
| `correlation_id` | 串联一次用户请求或业务流程 |
| `causation_id` | 触发本事件的 request id、event id 或 job id |
| `customer_scope` | 事件影响的数据范围；不允许空值表达全部客户 |
| `idempotency_key` | 消费幂等键 |
| `payload` | 事件业务负载 |

规则：

- payload 不得包含 token、password、cookie、连接串或未授权敏感字段。
- 事件 schema 只能向后兼容扩展；破坏性变化必须新增 schema version。
- consumer 必须使用 inbox 记录 `event_id` 或 `idempotency_key`，保证幂等。
- 可重放事件不得依赖当前 mutable 配置推断历史语义，必须携带版本或稳定引用。

## 事件清单

| Event | Producer | Consumer | Idempotency Key | 必须字段 |
|------|----------|----------|-----------------|----------|
| `FileRegistered` | `stdas-gateway` / `data-pipeline-service` | `data-pipeline-service`、`workflow-service` | `customer_scope + file_hash + source_ref` | `file_id`、`raw_ref`、`file_hash`、`customer_scope`、`source_ref` |
| `FileStored` | `data-pipeline-service` | `workflow-service` | `file_id + raw_ref` | `file_id`、`raw_ref`、`object_store_ref`、`size_bytes` |
| `FileValidated` | `data-pipeline-service` | `workflow-service` | `file_id + validation_version` | `file_id`、`validation_status`、`file_format`、`detected_metadata`，其中可识别时包含 customer/product/test_type/test_station |
| `ProfileResolved` | `data-pipeline-service` | `workflow-service` | `file_id + profile_resolution_key` | `file_id`、`profile_resolution_key`、`data_profile_version`、`parser_profile_version`、`mapping_profile_version`、`spec_profile_version` |
| `FileParsed` | `data-pipeline-service` | `workflow-service` | `file_id + parse_attempt_id` | `file_id`、`parse_attempt_id`、`parser_version`、`staging_ref` |
| `DataNormalized` | `data-pipeline-service` | `workflow-service` | `parse_attempt_id + mapping_version` | `parse_attempt_id`、`normalized_ref`、`mapping_version`、`validation_summary` |
| `CanonicalDataCommitted` | `data-pipeline-service` | `analytics-service`、`workflow-service` | `lot_run_id + data_version` | `customer_code`、`product`、`test_type`、`test_station`、`lot_id`、`lot_run_id`、`data_version`、`data_profile_version`、`lineage_ref` |
| `AggregatesRequested` | `workflow-service` | `analytics-service` | `data_version + aggregate_type` | `data_version`、`aggregate_type`、`priority` |
| `AggregatesBuilt` | `analytics-service` | `workflow-service` | `data_version + aggregate_type + aggregate_version` | `data_version`、`aggregate_type`、`aggregate_ref`、`aggregate_version` |
| `AlertEvaluationRequested` | `workflow-service` | `analytics-service` | `data_version + alert_rule_set_version` | `data_version`、`alert_rule_set_version`、`trigger_reason` |
| `AlertRaised` | `analytics-service` | `workflow-service`、notification projection | `rule_id + rule_version + data_version + lot_scope_hash` | `alert_id`、`rule_id`、`rule_version`、`trigger_context`、`data_version_set` |
| `AlertCleared` | `analytics-service` | `workflow-service`、notification projection | `alert_id + cleared_at` | `alert_id`、`rule_version`、`cleared_at`、`reason` |
| `DataVersionReady` | `workflow-service` | `analytics-service`、frontend projection | `data_version + ready_at` | `data_version`、`customer_code`、`product`、`test_type`、`test_station`、`lot_id`、`lot_run_id`、`ready_at`、`aggregate_status` |
| `ExportReady` | `analytics-service` / `workflow-service` | `workflow-service`、frontend projection | `export_id + file_ref` | `export_id`、`query_snapshot_id`、`file_ref`、`expires_at`、`redaction_state` |
| `JobDeadLettered` | `workflow-service` | ops projection | `job_id + attempt` | `job_id`、`correlation_id`、`last_error_code`、`dead_letter_reason` |

## 重放与 Dead Letter

- 事件重放必须保留原始 `event_id`、`occurred_at`、`correlation_id` 和 payload。
- 如果需要重新生成业务结果，必须创建新的 job、QuerySnapshot 或 DataVersion，不得覆盖历史对象。
- Dead letter 事件必须可按 `correlation_id`、`job_id`、`event_type`、`customer_scope` 查询。
- 重放前必须展示影响范围和幂等键。

## 事件验收

新增事件必须提交：

- 事件类型和 schema version。
- producer 和 consumer。
- 信封字段示例。
- payload 字段表。
- idempotency key。
- 可重放性说明。
- 失败和 dead letter 处理。
