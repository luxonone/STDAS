# 后端实施路线图

本路线图以 [ADR-0014](../architecture-design/adr/0014-gateway-modular-monolith.md) 为当前事实来源：STDAS 当前采用 `stdas-gateway` 单一运行服务与强模块边界。服务拆分、NATS、Outbox/Inbox、MinIO、gRPC 和多进程部署是未来触发条件满足后的扩展方向，不是 Phase 0 必做项。

本路线图不定义前端页面、路由、筛选、字段显示或产品设计细节。字段命名等你提供可读 MES 数据库后再校准。

## Phase 0：Axum 单服务基线

目标：

- Rust workspace。
- `backend/services/stdas-gateway` 作为唯一 backend runtime service。
- Axum app assembly、server、route catalog、CORS、config、state。
- `system` health / preflight。
- `modules/` 建立未来服务边界：
  - `identity`
  - `customer`
  - `data_pipeline`
  - `analytics`
  - `evidence`
  - `workflow`
  - `integration`
- 顶层横切边界：
  - `telemetry`
  - `audit`
  - `middleware`
  - `errors`
  - `shared`

验收：

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo run -p stdas-gateway -- routes`
- health / preflight route 保持可访问。
- 不引入 unused SQLx pool、Redis、NATS、MinIO、gRPC 或 fake business implementation。

## Phase 0.5：本地验证 Gate

当前本机验证记录见 [phase-0-5-environment-validation.md](phase-0-5-environment-validation.md)。

目标：

- 验证 Rust toolchain、cargo alias 和最小后端命令可运行。
- 验证代码质量规则能通过 fmt、check、test、clippy 形成反馈闭环。
- 验证 AI Agent 能捕获编译/测试错误并修复。
- 记录 Windows 本地开发不使用 Docker 的实际限制。

验收：

- 后端 `cargo check` 可运行。
- 后端测试命令可运行。
- 失败输出可被 AI Agent 读取。
- 验证结果写入 docs 或开发记录。

## Phase 1：字段语义与治理准备

触发条件：用户提供可读 MES 数据库或真实字段资料。

目标：

- 建立 `MES source field -> STDAS canonical field -> API field -> frontend display label -> evidence/lineage reference` 的字段治理流程。
- 明确哪些字段来自 MES，哪些字段来自 test file，哪些字段由 STDAS normalization 生成。
- 校准 `customer` module 中的 CustomerConfig、DataProfile、ProfileResolutionKey、rule binding 语义。
- 不在 MES schema 审查前定死数据库字段、Rust field、API field 或 frontend label。

验收：

- 每个关键字段都有 source、canonical、display、lineage 关系说明。
- 临时字段不能沉淀为正式 schema。
- 字段命名能贴合 MES 业务语义，同时不被历史字段名完全绑死。

## Phase 2：真实样例文件接入准备

触发条件：用户提供真实或代表性 FT test file。

目标：

- 原始文件只进入 Git ignored 的本地目录。
- 原始文件不得直接提交 Git。
- 基于真实文件识别 metadata、文件 fingerprint、格式特征和敏感字段。
- 制定脱敏 fixture 生成流程。
- 在 `data_pipeline` module 内明确 file register、raw metadata、parser selection、normalization、DataVersion、lineage 的 ownership。

验收：

- raw file 不出现在 Git tracked changes。
- 可说明真实文件中的 source field 与未来 canonical field 的关系。
- 可从真实文件派生脱敏 fixture；fixture 才能进入项目测试。
- 不凭空实现 parser，不伪装成已经支持真实格式。

## Phase 3：可信数据闭环后端主链路

目标：

- 在 `stdas-gateway` 单服务内实现可信数据闭环的最小后端能力。
- `customer` module 提供 DataProfile / rule resolution 的最小可验证边界。
- `data_pipeline` module 提供 file register、parse/normalize 边界、DataVersion、lineage。
- `evidence` module 提供 evidence view / citation 边界。
- `analytics` module 只提供轻量 query / QuerySnapshot / result contract，不提前实现重 OLAP。
- `audit` 记录关键业务动作。

验收：

- 真实或脱敏样例文件能从登记走到 DataVersion。
- 所有结果能说明使用的 source、profile/rule version、normalization version 和 evidence reference。
- parser 不直接写 analytics 或 integration 数据。
- analytics 不绕过 DataVersion / lineage / evidence。
- handler 保持协议适配，不直接写 SQL 或承载业务流程。

## Phase 4：查询、分析与导出扩展

触发条件：可信 DataVersion 和 evidence 基础已稳定。

目标：

- 扩展 `analytics` module 的 query、aggregation、analysis registry、result materialization。
- 引入查询预算、QuerySnapshot、DataVersion 冻结和 evidence-aware result。
- 按实际数据规模决定是否接入 Redis、DuckDB、Parquet 或 ClickHouse。
- 大查询和导出根据压力进入 job 化。

验收：

- 查询必须带 scope。
- 超预算请求被拒绝或转异步。
- 分析结果记录 QuerySnapshot 和实际使用的 DataVersion 集合。
- 大导出不阻塞普通 API。

## Phase 5：服务拆分审查

触发条件：ADR-0014 的黄色或红色信号持续出现。

目标：

- 判断是继续 module、升级 crate，还是拆 runtime service。
- 优先考虑 `module -> independent crate -> runtime service`。
- 不因为文档里有未来服务名、目录名像服务、微服务更高级或 AI 建议而拆分。

验收：

- 有明确资源隔离、失败隔离、数据所有权、安全边界、独立部署、外部系统隔离或团队边界证据。
- 先写新的 ADR，再改代码结构。
- 拆分后仍保持 `stdas-gateway` 是唯一外部 API 入口。

## Future：外部集成与多服务运行

仅当真实需要出现后再引入：

- MES runtime connector。
- gRPC service clients。
- NATS JetStream。
- Outbox/Inbox。
- MinIO/S3 object storage。
- 多节点配置演练。
- Windows Service / systemd 安装脚本。
- ClickHouse PoC。

这些能力属于 future direction，不是当前 Phase 0 / Phase 1 的验收要求。
