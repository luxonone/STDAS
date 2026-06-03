# 安全与可靠性

当前阶段遵循 [ADR-0014](../architecture-design/adr/0014-gateway-modular-monolith.md)：安全规则先约束 `stdas-gateway` 和内部 `modules/*`，未来服务化后再扩展到跨服务 gRPC、NATS subject、mTLS 和服务身份。

## 身份认证

- Access token 使用短 TTL。
- Refresh token 使用 HttpOnly Cookie；生产环境必须启用 Secure，本地 HTTP 开发只能通过明确的 localhost 开发配置例外。
- Refresh token 必须轮换。
- 使用 token family 检测 refresh token reuse。
- 密码使用 Argon2id。
- 登录和刷新接口必须限流。

## 授权模型

```text
Principal
  ├── user_id
  ├── role
  ├── customer_scope
  ├── factory_scope
  └── permissions
```

初始只设置两类角色：

| 角色 | 说明 |
|------|------|
| engineer | 工程师，查看授权范围内的数据并执行分析 |
| admin | 管理员，管理用户、客户、采集、规则和系统配置 |

授权同时检查角色和数据范围。Admin 权限不自动等于业务全数据权限，必须通过显式 scope 表达。

如果后续需要只读用户、质量用户、管理层用户或外部客户用户，优先增加权限点和 scope，再评估是否新增角色。

## 权限矩阵

| 能力 | engineer | admin | Scope 要求 | 审计 |
|------|----------|-------|------------|------|
| 查看轻量查询快照 | allow | allow | CustomerScope | no |
| 查看 Lot 列表 | allow | allow | CustomerScope | no |
| 查看 Lot / Run / File / DataVersion 详情 | allow | allow | CustomerScope + data access | no |
| 运行分析查询 | allow | allow | CustomerScope + DataVersion access | yes |
| 保存 workspace | allow | allow | CustomerScope | yes |
| 导出分析结果 | allow if permission | allow if permission | CustomerScope + export permission | yes |
| 创建 Investigation Case | allow | allow | CustomerScope | yes |
| 修改 Investigation Evidence / 结论 | allow if owner or permission | allow | CustomerScope | yes |
| 确认或关闭告警 | allow if permission | allow | CustomerScope + rule scope | yes |
| 管理用户 | deny | allow | admin scope | yes |
| 管理客户配置 | deny | allow | customer admin scope | yes |
| 发布 DataProfile | deny | allow if permission | customer/profile scope | yes |
| 发布规则或模板 | deny | allow if permission | customer/profile/rule scope | yes |
| 回放 ingestion job | deny | allow if permission | customer/data scope | yes |
| 查看 dead letter | deny | allow | system scope | yes |

权限矩阵是后端鉴权和前端按钮状态的共同事实来源。前端可以隐藏或禁用无权限操作，但 API 必须始终重新校验权限。

## Redaction Policy

| 状态 | 含义 | API 返回 | UI 展示 |
|------|------|----------|---------|
| `hidden` | 用户不应知道对象存在 | 不返回对象，或只返回策略允许的 hidden count | 不显示明细，不泄露名称 |
| `masked` | 用户可知道对象存在但不能看敏感字段 | 返回对象，敏感字段为 mask token 或掩码值 | 显示掩码并说明受限 |
| `unauthorized` | 用户请求了无权限对象 | 403 或权限状态对象 | 明确权限不足和可申请动作 |
| `not_found` | 对象不存在或按策略不可见 | 404 或统一不可见策略 | 不泄露对象是否存在 |

不得通过空状态、错误文案、导出文件名、URL 参数、选项命中数或下载失败原因泄露未授权客户、产品、Lot 或规则信息。

## 数据保护

- 生产环境强制配置 JWT secret。
- Cookie secure 在生产必须开启。
- CORS 必须显式白名单。
- 日志禁止记录 token、password、cookie、连接串。
- 上传文件必须隔离存储。
- 关键操作必须写审计日志。
- API 不得通过空状态、错误文案、导出文件名、URL 参数泄露未授权客户、产品或 Lot 信息。
- 分享链接、workspace、query snapshot、case 和导出下载必须在访问时重新校验权限。
- 响应中的敏感字段必须按权限策略返回明文、masked、hidden 或 unauthorized。

## 内部边界安全

- 外部请求只能进入 `stdas-gateway`。
- 当前跨模块调用必须显式传递或引用已验证的 `Principal` 和 `CustomerScope`，不得在业务模块内重新推断权限。
- 未来服务化后，内部服务端口默认只对受信网络开放。
- 未来内部 gRPC 调用必须携带 service identity、request id、correlation id。
- 未来生产多服务环境应启用 mTLS 或等效的内网身份校验。
- 未来 NATS subject 按服务和事件域命名，发布/订阅权限最小化。
- 对象存储 bucket/prefix 按环境和客户范围隔离；当前本地文件 adapter 也必须保留同样的隔离语义。

## Job 状态机

```text
queued
  -> running
  -> succeeded
  -> canceling
  -> canceled
  -> expired
  -> retry_scheduled
  -> failed
  -> dead_letter
```

运行中任务字段：

- `locked_by`
- `locked_until`
- `heartbeat_at`
- `attempt`
- `max_attempts`
- `next_retry_at`
- `last_error_code`
- `correlation_id`
- `created_at`
- `updated_at`
- `completed_at`
- `expires_at`
- `query_snapshot_id`

## 重试策略

| 错误类型 | 策略 |
|------|------|
| 文件格式错误 | 不重试，进入 failed |
| 业务校验错误 | 不重试，进入 failed |
| 数据库短暂不可用 | 指数退避重试 |
| 外部系统超时 | 重试，可降级使用缓存 |
| Worker 崩溃 | lock 过期后重新领取 |
| 聚合刷新失败 | 可重试，不阻塞原始数据入库 |

重试任务必须说明复用原 QuerySnapshot，还是按当前 latest committed 重新解析。导出任务默认复用原 QuerySnapshot，除非用户明确重新运行。

## 事件可靠性

- 事件发布必须来自本地 outbox。
- 事件消费必须写入 inbox 幂等记录。
- 每个事件必须包含 `event_id`、`correlation_id`、`causation_id`、`occurred_at` 和 schema version。
- Consumer 失败后按指数退避重试，超过上限进入 dead letter。
- 当前 `modules/workflow` 负责跨模块长流程、超时、补偿和人工介入状态；未来服务化后升级为 `workflow-service`。

## 降级策略

- 外部系统不可用时使用缓存并标注数据新鲜度。
- Analytics backend 不可用时返回明确错误，不退化为无限扫 PostgreSQL。
- 轻量查询快照可返回最近成功版本。
- 告警计算失败不阻塞摄入主流程。

## 查询和任务可靠性

- 前端可通过 query id、query hash 或 query snapshot id 识别过期响应。
- 后端异步任务必须保留 request id、correlation id、job id 和 query snapshot id。
- 取消任务不保证总能立即成功；无法取消时必须返回原因。
- 任务完成、失败、取消、过期必须可查询，不能只依赖一次性通知。
