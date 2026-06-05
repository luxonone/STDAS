# Changelog

本文件记录 STDAS 的用户可见变化、接口变化、架构/维护规则变化和重要迁移说明。

未来 changelog 遵循 [Keep a Changelog](https://keepachangelog.com/) 的主流结构，并配合 [Conventional Commits](https://www.conventionalcommits.org/) 维护提交历史：

- 新变化先写入 `[Unreleased]`。
- 每个版本按 `Added`、`Changed`、`Deprecated`、`Removed`、`Fixed`、`Security` 分类。
- Changelog 记录“发生了什么”和“对使用者/维护者有什么影响”，不记录每个 commit 的流水账。
- 提交说明和 PR 描述负责记录 Code / Docs / Validation 细节；changelog 只保留值得长期追踪的结果。
- 历史 `C###` / `D###` 条目保留为 legacy 记录，后续不再新增本地编号条目。

## [Unreleased]

### Added

- 新增登录页和最小会话链路：前端通过 `shared/api` typed client 调用 `POST /api/v1/auth/login`，保存本地 session，并在刷新时调用 `GET /api/v1/auth/me` 校验 token。
- 新增 `identity` 模块的开发阶段最小 auth API。当前初始账号为 `admin / admin@123`；该账号和固定 token 只用于 Phase 0 登录链路验证，不代表生产认证设计。
- 新增登录成功后的临时空白工作区，用于确认 auth 链路已经打通；正式登录后工程入口等待下一张页面设计稿确认。

### Changed

- Git/GitHub SOP 调整为主流多人协作口径：新提交采用 Conventional Commits；提交拆分按 atomic change、reviewability、revertability、bisectability 判断，不再使用本地 `C###` / `D###` 编号作为新 commit subject。
- 文档入口、前后端同步设计、首批功能切片、API 契约和前端设计入口已对齐“身份、会话与授权上下文”最小落地范围。
- 前端/产品文档状态改为“登录页与身份会话切片部分恢复”。固定 Overview、固定登录后 route 和 Data Explorer 默认入口不再作为当前事实来源。

## Legacy Local-Numbered Entries

以下历史条目保留原本本地编号格式，仅用于追溯早期 Phase 0 记录；未来不再新增同类编号条目。

### C004 - Backend App Layout

- 将 `stdas-gateway` crate 从 `crates/services/stdas-gateway` 移到 `apps/api`，让当前项目外层结构变为 `apps/api` + `apps/web`。
- 保留 package name `stdas-gateway` 和现有 API 契约；`cargo gateway`、`cargo gateway-routes`、`cargo run -p stdas-gateway` 命令不变。
- 新增 ADR-0012，明确 `apps/api/src` 参考 Melrose《Rust + Axum 后端架构设计文档》的 Axum API 内部分层；`crates/` 以后只放真实共享库、基础设施库、工具或内部服务，不提交空目录。

### D009 - Backend API Layout Documentation

- 同步 `apps/api` 后端应用目录说明、根 README、项目目录结构、后端 workspace 文档、ADR 索引和验证文档。
- 明确 `crates/` 是未来共享库、基础设施库、工具 crate 或内部服务 crate 的位置；当前没有真实 crate 时不提交空目录。
- 记录 Melrose《Rust + Axum 后端架构设计文档》的采用边界：采用 Axum API app 内部分层，不照搬 DB、Redis、JWT、OpenAPI、metrics、Docker/K8s 或后台任务等未落地能力。

### C003 - Gateway Axum and SQLx Baseline

- 将 `stdas-gateway` 重新定基线为直接使用 Axum，不再使用 Loco scaffold、Loco CLI 或 Loco YAML 配置。
- 按 Melrose《Rust + Axum 后端架构设计文档》将 Gateway 从扁平文件结构调整为目录化分层：`app.rs`、`routes/`、`handlers/`、`services/`、`repositories/`、`dto/`、`models/`、`middleware/`、`config/`、`errors/`、`telemetry.rs`。
- 将后端持久化技术决策明确为 SQLx + PostgreSQL，不采用 ORM；当前最小 Gateway 暂不引入 unused SQLx dependency。
- 新增 ADR-0011，并将 ADR-0009 标记为历史记录；同步项目结构、后端设计、验证命令和 preflight UI 文案。

### D008 - SPEC Governance Partition

- 新增 `docs/specs/` 作为 SPEC 专区，明确 SPEC 是项目铁律，普通文档只承担背景、设计、索引和辅助说明。
- 新增 `docs/specs/agent-startup-context-spec.md`，规定新会话或新任务开始开发、设计、修改或审查前必须通读项目上下文文档。
- 将 Rust coding baseline 从后端设计目录迁移到 `docs/specs/rust-coding-guidelines-spec.md`，并同步 docs 入口、AI Agent 运行时规则、后端规则和项目目录结构。
- 下载 `rust-coding-guidelines-zh` 上游完整规范快照到 `docs/specs/vendor/rust-coding-guidelines-zh/`，固定 commit `6b3fc48b285b4f87696634a3e18572d010b30fd4`，并更新 Rust Coding Guidelines SPEC 的读取要求。

### D007 - Rust Coding Guidelines SPEC

- 新增 Rust Coding Guidelines SPEC，将 Rust 编码规范中文站作为 STDAS Rust coding baseline，并同步后端 README、Rust 质量规则、AI 生成规则、参考项目规则和 docs 入口。

### D006 - Backend Investigation Log

- 新增 Loco error handling 调查日志，记录 `loco-rs 0.16.4` 下 `Error::Message`、`bad_request`、`JsonRejection` 的当前行为、相关 GitHub issue 和 STDAS 后续处理方向。

### D005 - Git and GitHub Safety SOP

- 新增 Git / GitHub 安全 SOP，覆盖远端绑定、提交、推送、AI 生成代码、diff 审查和安全回退。
- 明确 `reset --hard`、`clean -fdx`、`push --force` 等高风险命令不得由 AI Agent 默认执行。
- 在 docs 入口和 AI Agent 运行时规则中加入 Git/GitHub 任务的必读路径。
- 补充 AI 执行门禁，区分 Codex GitHub 绑定、GitHub CLI 登录和本地仓库 `origin` remote，要求提交/推送/回退前先做只读诊断。

### C002 - Gateway Framework Migration

- 将 `stdas-gateway` 从手写 Axum 启动迁移到 Loco app hooks、controller routes、service-local YAML 配置和 `cargo loco` 命令入口。
- 将前端工作台移动到 `apps/web`，根目录保留 repo-level pnpm scripts 和 workspace 文件。
- 保留 `GET /api/v1/system/health` 与 `GET /api/v1/system/preflight` 的 preflight API 契约。
- 新增 ADR-0009 / ADR-0010，并同步 README、项目结构、后端技术选型和验证文档中的 Loco 启动/路由检查命令。
- 补充一级目录清理规则，明确根目录 Cargo/pnpm workspace 文件、`node_modules/`、`target/`、`dist/`、`tmp/` 等路径的管理方式，并要求 AI Agent 对新手提出的方向先做专业判断。

### D004 - Documentation

- 新增 `docs/frontend-design/frontend-page-design-v1.md`，定义第一阶段全量页面、次级页面、独立页面和页面级设计契约。
- 将公开行业参考、本地 SAS/OneData/Exensio 参考转译为 STDAS 自有前端信息架构，不复制品牌或受保护界面。
- 更新前端文档入口和 docs 总入口，确保页面设计事实来源可被后续实现阶段读取。

### D002 - Documentation

- 新增本 changelog。
- 记录本地 commit 编号约定，用于区分 code/configuration commit 与 documentation-only commit。
- 在项目 README 和 docs index 中链接 changelog。

### C001 - Code and Configuration Review

- 在不做大范围 refactor 的前提下收紧最小 preflight slice。
- 显式处理 React root lookup，避免依赖 non-null assertion。
- 让 preflight page request flow 更明确地处理 abort，并提升可维护性。
- 为 `GET /api/v1/system/preflight` 增加 Backend contract test。
- 将 Frontend build tooling packages 移入 development dependencies。
- 忽略 local agent guidance/state 和外部 reference material。

### 000 - Baseline

- 记录 best-practice review pass 之前的最小 STDAS Phase 0 preflight validation project。
