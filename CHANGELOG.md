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
