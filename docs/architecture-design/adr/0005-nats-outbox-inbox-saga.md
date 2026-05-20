# ADR-0005：采用 NATS JetStream + Outbox/Inbox + Saga

状态：Accepted
日期：2026-05-18

## 背景

STDAS 的摄入、解析、归一化、聚合、告警、导出和重算都是长流程，跨多个服务和存储系统。系统需要可重试、可恢复、可回放，但不应引入跨服务分布式事务。

## 决策

跨服务异步协作采用：

- NATS JetStream 作为事件传输。
- Outbox 发布事件。
- Inbox 幂等消费。
- `workflow-service` 承担 Saga、Process Manager、超时、补偿和 dead letter。
- 每个事件包含 event id、correlation id、causation id、schema version 和 idempotency key。

## 后果

正面：

- 长流程可追踪、可重试、可补偿。
- 服务本地事务边界清晰。
- 事件可用于构建 projection 和审计链。

代价：

- 需要维护事件契约、消费者幂等和 dead letter 运维。
- 最终一致性需要在 UI 和 API 中明确状态。

## 替代方案

- 跨服务分布式事务：复杂且不适合文件、对象存储和长流程。
- 同步调用串联全流程：失败恢复困难，容易形成循环依赖。

## 验证方式

- 每个发布事件的服务必须有 outbox。
- 每个事件 consumer 必须有 inbox 幂等记录。
- dead letter 可查询、可诊断、可重放。
- 文件摄入到 DataVersionReady 全链路可用 correlation id 串起。
