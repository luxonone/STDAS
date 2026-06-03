# 部署与可观测性

当前阶段遵循 [ADR-0014](../architecture-design/adr/0014-gateway-modular-monolith.md)：后端只部署一个 Rust 运行服务 `stdas-gateway`，内部通过强 module boundary 保持可拆分性。本文保留的多进程部署内容是未来满足拆分触发条件后的目标形态，不是当前 Phase 0 的执行要求。

当前部署拓扑：

```text
runtime/
├── stdas-gateway
├── postgresql
├── config/
└── logs/
```

未来服务化后，STDAS 采用原生分布式进程部署。每个服务是独立 Rust 二进制，可在 Windows 或 Linux 上直接运行。单节点部署使用 `localhost`，多节点部署通过配置切换为实际 IP。

## 未来运行进程

```text
runtime/
├── stdas-gateway
├── identity-service
├── customer-service
├── data-pipeline-service
├── analytics-service
├── workflow-service
├── integration-service
├── nats-server
├── postgresql
├── minio
├── config/
└── logs/
```

## 配置即拓扑

当前 `stdas-gateway` 只需要自身配置和直接依赖配置。未来服务化后，每个服务通过 TOML 配置获知依赖服务地址，不依赖外部注册中心。

```toml
[server]
service_name = "data-pipeline-service"
host = "0.0.0.0"
grpc_port = 50053
metrics_port = 9103

[discovery]
customer_service = "http://localhost:50052"
workflow_service = "http://localhost:50055"
nats_server = "nats://localhost:4222"

[database]
url = "postgres://stdas:stdas@localhost:5432/stdas"
schema = "data_pipeline"

[object_store]
type = "s3"
endpoint = "http://localhost:9000"
bucket = "stdas-data"

[security]
jwt_issuer = "stdas"
```

单节点和多节点只改变地址：

```diff
- customer_service = "http://localhost:50052"
+ customer_service = "http://10.0.1.20:50052"
```

## 跨平台数据目录

服务不在配置中硬编码平台路径。数据目录通过环境变量或启动参数指定。

```rust
pub fn data_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("STDAS_DATA_DIR") {
        return std::path::PathBuf::from(dir);
    }

    dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("stdas")
}
```

## 未来 Linux 进程管理

未来多服务生产环境使用 systemd。

```ini
[Unit]
Description=STDAS Data Pipeline Service
After=network.target

[Service]
Type=simple
User=stdas
ExecStart=/opt/stdas/bin/data-pipeline-service --config /etc/stdas/data-pipeline-service.toml
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

## 未来 Windows 进程管理

未来多服务生产环境使用 Windows Service。

```powershell
New-Service -Name "STDASDataPipeline" `
    -BinaryPathName "`"C:\Program Files\STDAS\bin\data-pipeline-service.exe`" --config `"C:\Program Files\STDAS\config\data-pipeline-service.toml`"" `
    -DisplayName "STDAS Data Pipeline Service" `
    -Description "Handles ingestion, parsing, normalization and canonical data commit for STDAS" `
    -StartupType Automatic
```

## 未来开发启动脚本

服务拆分后，开发环境可以直接启动所有进程。

```text
scripts/start-all.ps1
scripts/stop-all.ps1
scripts/start-all.sh
scripts/stop-all.sh
```

未来服务化启动顺序：

1. PostgreSQL。
2. MinIO。
3. NATS JetStream。
4. Control Plane services。
5. Data Plane services。
6. `stdas-gateway`。

## 日志字段

所有关键日志应包含：

- `service_name`
- `process_id`
- `request_id`
- `correlation_id`
- `causation_id`
- `event_id`
- `job_id`
- `query_snapshot_id`
- `user_id`
- `customer_code`
- `test_type`
- `test_station`
- `equipment_type`
- `lot_number`
- `data_version`
- `data_version_policy`
- `permission_result`
- `redaction_state`
- `duration_ms`
- `error_code`

## 指标

- API latency by route。
- gRPC latency by method。
- DB query latency by service。
- NATS publish/consume latency。
- JetStream consumer lag。
- Outbox pending count。
- Inbox duplicate count。
- Worker success/failure/retry。
- Job lifecycle count by state。
- Query snapshot creation count。
- Query snapshot reuse rate。
- Parser duration。
- Ingestion throughput。
- Normalization throughput。
- Analysis query duration。
- Analysis over-budget count。
- Stale/partial/snapshot response count。
- Options API latency and error count。
- Cache hit ratio。
- Alert evaluation duration。

## 健康检查

每个服务都必须暴露健康检查：

| 端点 | 用途 |
|------|------|
| `/health/live` | 进程存活 |
| `/health/ready` | 依赖可用，可接流量 |
| `/health/version` | 构建版本、配置摘要 |
| `/metrics` | Prometheus 指标 |

## 运维工具

`stdas-cli` 应提供：

- 服务健康聚合。
- 数据版本查询。
- 事件回放。
- 作业重跑。
- parser 回放。
- profile 解析诊断。
- 分析结果对账。
- query snapshot 查询。
- investigation evidence 查询。
- 导出权限诊断。
- 用户和权限诊断。
- NATS consumer lag 查询。
- 缓存清理。

## SLO 与告警阈值

阈值是第一版默认目标，具体数值可以按部署规模调整，但调整必须记录原因和生效范围。

| 指标 | 目标 | 告警 |
|------|------|------|
| gateway light query p95 latency | < 500ms | > 1s 持续 5 分钟 |
| internal gRPC p95 latency | < 300ms | > 800ms 持续 5 分钟 |
| analysis sync p95 latency | < 8s | 超过同步预算时必须转异步或拒绝 |
| ingestion to DataVersionReady | 按文件规模定义 | 超过配置阈值进入 job warning |
| Options API p95 latency | < 300ms | > 1s 持续 5 分钟 |
| outbox pending count | 接近 0 | 超过配置阈值持续 5 分钟 |
| JetStream consumer lag | 接近 0 | 超过配置阈值持续 5 分钟 |
| dead_letter count | 0 | > 0 立即告警 |
| job retry rate | 低于基线 | 异常升高告警 |
| analysis over-budget rate | 低于基线 | 异常升高需要评估查询预算或索引 |
| stale query snapshot age | < 默认新鲜度 | 超过阈值显示 stale 并告警 |
| export expired download attempts | 监控 | 异常增长告警，可能代表链接或保留策略问题 |

告警必须带 `service_name`、`correlation_id` 或可追踪对象 ID。无法定位到请求、job、event、query snapshot 或 DataVersion 的告警不可验收。
