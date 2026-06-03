# 缓存与 Redis 使用策略

本文件定义 STDAS 缓存策略和 Redis 使用边界。当前阶段遵循 [ADR-0014](../architecture-design/adr/0014-gateway-modular-monolith.md)：缓存能力先作为 `stdas-gateway` 内的 adapter/interface 设计，不把 Redis、NATS 或多服务部署作为 Phase 0 强依赖。当前 Windows 本机已通过 Scoop 安装 Redis，Redis 可在缓存 adapter、限流、token revocation 或 Options cache 需要时启用；但缓存能力从第一版开始仍必须通过接口抽象，避免业务代码绑定具体缓存实现。

## 结论

STDAS Phase 0 / 0.5 可以使用 Redis，但不把 Redis 作为业务正确性的强依赖。Redis 是 Cache Zone 的可选实现，不是 QuerySnapshot、DataVersion、Evidence、Export metadata 的事实来源。

第一版允许使用：

- PostgreSQL 保存权威元数据、快照、作业状态、审计和缓存索引。
- 进程内短 TTL 缓存 Options 和热点配置。
- 本地文件系统或对象存储 adapter 保存 raw、staging、export 文件。

但必须遵守：

- 所有缓存访问必须通过接口，例如 `CacheStore`、`OptionsCache`、`TokenRevocationStore`、`RateLimiter`。
- 不得在 handler、repository、page 或 use case 中散写 `HashMap` 缓存。
- 缓存 key 必须包含业务语义和权限上下文，不能直接使用 URL 或 SQL 字符串。
- QuerySnapshot 不得只存在缓存中。
- Cache miss 不得改变业务语义。
- 权限变化、DataProfile 发布、DataVersion 变化、Options 上游字段变化必须能失效或重新解析。
- 多实例或生产限流、token revocation 需要共享状态时，应启用 Redis adapter。

## 缓存不是事实来源

| 对象 | 是否可只放缓存 | 事实来源 |
|------|----------------|----------|
| QuerySnapshot | 否 | PostgreSQL |
| DataVersion | 否 | PostgreSQL / Data Pipeline metadata |
| Investigation Evidence | 否 | PostgreSQL |
| Export metadata | 否 | PostgreSQL |
| Export file | 否 | 本地文件或对象存储，PostgreSQL 保存元数据 |
| 轻量查询 snapshot metadata | 否 | PostgreSQL |
| Options result | 可以短 TTL 缓存 | PostgreSQL + DataProfile / permission rules |
| Token revocation | 第一版可 PostgreSQL，生产多实例建议 Redis | PostgreSQL 审计 + Redis 加速 |
| Rate limit counter | 第一版可进程内或 PostgreSQL，生产多实例建议 Redis | RateLimiter adapter |

核心原则：

> Redis 可以缓存结果，但不能定义结果语义。QuerySnapshot 才定义分析语义。

缓存可以过期、清理、重建；QuerySnapshot、DataVersion、Evidence、Export metadata 不能被缓存失效语义覆盖。

## 推荐接口

第一版必须按可替换 adapter 设计缓存能力。

```rust
#[async_trait::async_trait]
pub trait CacheStore {
    async fn get(&self, key: &CacheKey) -> Result<Option<Vec<u8>>, CacheError>;
    async fn set(&self, key: &CacheKey, value: Vec<u8>, ttl: Duration) -> Result<(), CacheError>;
    async fn delete(&self, key: &CacheKey) -> Result<(), CacheError>;
    async fn delete_by_tag(&self, tag: &CacheTag) -> Result<(), CacheError>;
}

#[async_trait::async_trait]
pub trait TokenRevocationStore {
    async fn is_revoked(&self, token_id: &TokenId) -> Result<bool, AuthError>;
    async fn revoke(&self, token_id: &TokenId, ttl: Duration) -> Result<(), AuthError>;
}

#[async_trait::async_trait]
pub trait RateLimiter {
    async fn check(&self, key: &RateLimitKey) -> Result<RateLimitDecision, RateLimitError>;
}
```

第一版实现：

- `NoopCache`
- `ProcessMemoryCache`
- `PostgresTokenRevocationStore`
- `PostgresRateLimitStore`
- `LocalFileObjectStore`

后续实现：

- `RedisCache`
- `RedisTokenRevocationStore`
- `RedisRateLimiter`
- `MinioObjectStore`

## 缓存 key 规则

缓存 key 必须按 Redis 兼容格式设计，即使某些阶段使用进程内缓存。

通用组成：

```text
factory/customer scope
customer_code
product
test_type
test_station
DataProfile version
DataVersion set
lot_scope hash
query parameters hash
permission/redaction context hash
result shape/version
```

建议 key：

```text
analysis:{analysis_type}:{query_hash}:{data_version_set_hash}:{permission_context_hash}
options:{option_type}:{customer_scope_hash}:{upstream_context_hash}:{profile_version}:{search_hash}:{page}
analysis_snapshot:{customer_scope_hash}:{customer}:{product}:{test_type}:{test_station}:{time_range}:{profile_version}:{snapshot_version}
revoked_token:{token_family_id}:{token_id}
rate_limit:{scope}:{principal_or_ip}:{action}:{window}
```

规则：

- 分析缓存必须包含 DataVersion set 或 QuerySnapshot 引用。
- Options 缓存必须包含上游上下文、DataProfile/Profile version 和权限上下文。
- 轻量查询快照缓存必须包含时间范围、profile version 和 snapshot version。
- 权限裁剪会影响结果时，必须纳入 key 或禁止共享缓存。

## 失效规则

以下变化必须触发缓存失效或重新解析：

- DataProfile 发布新版本。
- parser、mapping、spec、rule、template 发布新版本。
- 用户切换 CustomerScope。
- 用户权限变化。
- latest committed DataVersion 变化。
- Options 上游字段变化。
- 用户选择重新运行分析，而不是查看历史结果。
- 安全策略要求 token revocation 立即生效。

历史 workspace、case、export 默认继续读取旧 QuerySnapshot，不因 latest committed 变化自动失效到新结果。

## 分阶段策略

| 阶段 | Redis 策略 |
|------|------------|
| Phase 0 / 0.5 | Redis 已可用；可用于验证缓存 adapter，但业务代码必须先建立缓存接口 |
| Phase 1 | 建立 `TokenRevocationStore`、`RateLimiter`、`OptionsCache` 接口；需要共享状态时可接入 Redis |
| Phase 2 / 3 | 继续以 PostgreSQL + NATS + workflow 保证正确性和幂等 |
| Phase 4 | 若 Options、轻量查询快照、热点分析结果有性能压力，接入 Redis 作为共享缓存 |
| 生产多实例 | 建议启用 Redis adapter，尤其用于限流、token revocation、Options cache、热点快照 |

## Redis 启用触发条件

出现任一条件，应启用 Redis adapter：

| 触发条件 | 原因 |
|----------|------|
| gateway 或服务多实例部署 | 进程内缓存不共享 |
| 登录或刷新接口需要全局限流 | 单进程限流不可靠 |
| token revocation 需要跨实例立即生效 | 进程内 blacklist 不够 |
| Options API 延迟影响交互 | 需要共享热点缓存 |
| 轻量查询快照访问量高，PostgreSQL 压力上升 | 需要缓存热点快照 |
| Analysis query snapshot reuse rate 高 | 可以缓存结果引用或小结果 |
| 系统进入生产前且 NATS/workflow 已稳定 | Redis 运维成本可以接受 |
| 安全要求必须快速撤销 token | Redis blacklist 适合该场景 |

## Windows 本地开发

当前 Windows 开发阶段不安装、不使用 Docker。基础设施优先采用：

- PostgreSQL 本机安装。
- NATS JetStream 原生 `nats-server.exe`。
- MinIO 原生 `minio.exe`，或 Phase 0 使用 local filesystem object-store adapter。
- Redis 已通过 Scoop 安装；需要缓存 adapter、限流、token revocation 或 Options cache 时可直接接入本机 Redis。

Docker 不进入 Windows 本地开发验证范围。
