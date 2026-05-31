# Changelog

本文件记录 Phase 0 preflight validation 期间的项目可见变更。内容保持简洁：实现细节保留在 code 中，设计依据保留在 `docs` 中。

## Commit 编号

commit 编号写入 commit subject。Git commit hash 基于内容生成，不应为了业务编号手动塑形。

- `000`：repository baseline 或 import point。
- `C###`：code 或 configuration change。
- `D###`：documentation-only change。

code/configuration commit 与 documentation commit 必须分离。这样回档更清晰：revert 生成代码时不应自动丢弃仍然准确的文档，更新文档时也不应把代码改动隐藏在同一个 commit 中。

该规则建立前已有的 commit 保持原样，避免重写本地历史。`001 review: align preflight project with best practices` 在 changelog 语义上视为 `C001`。

## Unreleased

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
