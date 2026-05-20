# 实施路线图

## Phase 0：分布式骨架

目标：

- Rust workspace。
- `stdas-gateway` health/readiness。
- `identity-service`、`customer-service`、`workflow-service` 最小 gRPC 服务。
- NATS JetStream 接入。
- PostgreSQL schema/database 初始化。
- MinIO/Object Storage 接入。
- Redis 已在 Windows 本机安装；缓存能力必须按 [cache-strategy.md](cache-strategy.md) 先建立接口，再按需要接入 Redis adapter。
- Outbox/Inbox 基础表和 publisher/consumer 框架。
- 统一 tracing、request id、correlation id。
- Windows / Linux 启停脚本。

验收：

- `cargo fmt` 通过。
- `cargo clippy` 通过。
- `cargo test` 通过。
- Rust 代码满足 [Rust 代码质量规则](rust-code-quality-rules.md)。
- gateway health endpoint 可访问。
- 所有服务能按配置启动并注册日志、metrics、health。
- NATS publish/subscribe demo 通过。
- 单节点 localhost 拓扑可运行。
- Windows 本地开发不安装、不使用 Docker；NATS/MinIO/Redis 优先使用原生二进制或本地 adapter。
- 前端包管理器统一使用 pnpm。
- 只在进入代码实现阶段执行 build/test；当前 docs 阶段只记录该 gate。

## Phase 0.5：环境验证 Gate

触发条件：前端设计和后端设计大体确定后，进入代码实现前。

当前本机验证记录见 [phase-0-5-environment-validation.md](phase-0-5-environment-validation.md)。该记录显示本机工具链已具备，但正式项目级 build/test 需在 Phase 0 代码骨架创建后重新执行。

目标：

- 验证 Rust、前端工具链、包管理器和本地配置。
- 验证 Rust 代码质量规则能通过 fmt、clippy、test 形成反馈闭环。
- 验证 React + TypeScript + pnpm 前端工具链能通过 install、lint、typecheck、test、build 形成反馈闭环。
- 验证 AI Agent 能捕获编译/测试错误。
- 跑通写代码 -> 编译 -> 报错 -> 修复 -> 再验证闭环。

验收：

- 后端 `cargo check` 或等价命令可运行。
- 后端测试命令可运行或明确待补齐。
- 前端 `pnpm install`、lint、typecheck、test、build 可运行或明确待补齐。
- 失败输出可被 AI Agent 读取。
- 验证结果写入 docs 或开发记录。

## Phase 1：Identity + Customer Specialization

目标：

- 工程师/管理员两类角色。
- 登录、刷新、登出、当前用户。
- CustomerScope。
- 权限脱敏和数据隐藏策略。
- CustomerConfig 基础表。
- DataProfile 基础表和版本模型。
- TestType、TestStation、EquipmentType、ProfileResolutionKey。
- ParserProfile、MappingProfile、SpecProfile。
- ParserRule、MappingRule、SpecRule。
- 规则共享引用和复制分叉审计。
- Feature flags。
- `customer-service.ResolveDataProfile` gRPC API。
- 审计日志事件。

验收：

- 未登录访问返回 401。
- 权限不足返回 403。
- token refresh 可轮换。
- token revocation、登录限流和 Options cache 通过接口实现；是否接 Redis 按 [cache-strategy.md](cache-strategy.md) 的触发条件决定。
- DataProfile 可按版本查询。
- ProfileResolutionKey 可解析到 parser/mapping/spec profile。
- 无权限、hidden、masked、unauthorized 状态可区分。
- gateway 不直接读取 identity/customer 数据库。

## Phase 2：Data Pipeline

目标：

- `data-pipeline-service` 文件登记。
- Raw storage。
- FileRegistered / FileStored / FileValidated 事件。
- Parser registry。
- ParserProfile 选择。
- Staging 输出。
- MappingProfile 归一。
- Canonical TestData commit。
- DataVersion 和 lineage。

验收：

- 重复文件幂等。
- 格式错误可诊断。
- 客户/产品/测试类型/测试站点/设备 parser 可通过 profile 选择。
- 主流程不直接写客户分支。
- Parser 不直接写 analytics 或 integration 数据。
- 所有结果记录 profile/parser/mapping/spec version。

## Phase 3：Workflow

目标：

- 作业状态。
- queued、running、succeeded、failed、canceling、canceled、expired 生命周期。
- Saga / Process Manager。
- 超时、失败、重试、补偿。
- Dead Letter 查询和重放。
- 文件摄入到 DataVersionReady 的流程追踪。

验收：

- workflow 能追踪一个摄入作业的完整状态。
- 任务状态包含时间、过期时间、request id、correlation id 和可诊断错误。
- 任意步骤失败后可诊断、可重试、可进入 dead letter。
- 重放事件不会造成重复入库。

## Phase 4：Analytics

目标：

- Lot 列表、Lot 详情、Summary/Bin 查询。
- Yield/Bin/Retest。
- Parametric 基础统计。
- 分析能力 registry 和一组基础分析能力。
- 查询预算。
- QuerySnapshot 和 DataVersion 冻结。
- Options API 基础能力。
- 聚合表和缓存。
- 如 Options、Overview 或热点分析结果有性能压力，按 [cache-strategy.md](cache-strategy.md) 接入 Redis。
- Parquet/DuckDB 分析路径。
- Overview 最小 KPI。

验收：

- 查询必须带 scope。
- 超预算请求被拒绝或转异步。
- 图表返回点数受控。
- 分析结果记录 query snapshot 和实际使用的 DataVersion 集合。
- options 支持 loading 之外的 empty、permission denied、deprecated、hidden、unauthorized 状态。
- 大导出通过事件和对象存储交付。

## Phase 5：Workspace + Alerting

目标：

- 分析会话。
- workspace query snapshot。
- 模板。
- 案例。
- Investigation Evidence 版本。
- 告警规则、事件、确认。
- AlertEvaluationRequested / AlertRaised 事件。
- 异步导出。

验收：

- 告警可从规则生成事件。
- 告警可跳转到分析上下文。
- case/evidence 引用固化 QuerySnapshot，不静默重算。
- 模板可复用并受 Profile 控制。
- 告警评估失败可重试且幂等。

## Phase 6：Integration + 多节点演练

目标：

- MES 同步。
- 客户接口或文件交换。
- 多节点配置演练。
- ClickHouse PoC。
- Windows Service / systemd 安装脚本。

验收：

- 把任一服务迁移到另一台机器时，只需要改配置地址。
- Integration 通过事件和 gRPC 协作，不直接读写其他服务数据库。
- 参数明细可从 PostgreSQL 扫描路径迁出。
- API contract 不因 backend 切换而变化。
