# AI Agent Startup Context SPEC

本文定义 AI Agent 在 STDAS 新会话或新任务中开始开发、设计、修改或审查前必须完成的上下文读取规则。该规则是 SPEC，优先级高于普通设计文档中的辅助说明。

## 适用范围

本 SPEC 适用于：

- 新 Codex 会话。
- 新 AI Agent 接手 STDAS 工作区。
- 经过 context compaction 后继续执行，但任务范围发生变化。
- 从问答切换到文档、设计、代码或 Git/GitHub 操作。
- 用户要求“继续开发”“开始设计”“修改代码”“提交代码”“整理项目”等会改变项目状态的任务。

如果只是回答一个不改变项目状态的轻量事实问题，可以不读取全部专题文档；但一旦准备修改文件、设计方案、执行验证或操作 Git/GitHub，就必须执行本 SPEC。

## 核心铁律

AI Agent 在 STDAS 中继续开发或设计前，必须先通读项目上下文文档。这里的“通读”不是只搜索关键词，也不是只依赖 memory、上一轮 summary 或模型记忆，而是在当前工作流中读取完整的必读文档内容，并基于当前仓库状态判断任务边界。

未完成上下文读取前，AI Agent 不得：

- 修改源码、配置、文档、设计稿或脚本。
- 生成具体实现方案并直接落地。
- 执行 `git add`、`git commit`、`git push`、`git reset`、`git clean`、`git restore`、`rebase` 或 force push。
- 删除、移动或重组项目目录。
- 宣称已经理解项目边界、架构、业务语义或验证要求。

## 新会话项目级必读

每个新会话在准备开发、设计或修改前，必须按顺序读取：

1. [SPEC 中心](README.md)。
2. 本 SPEC。
3. [docs 入口](../README.md)。
4. [AI Agent 运行时规则](../architecture-design/ai-agent-runtime-rules.md)。
5. [STDAS 背景知识总览](../domain-knowledge/stdas-domain-primer.md)。
6. [产品愿景](../architecture-design/product-vision.md)。
7. [领域模型](../architecture-design/domain-model.md)。
8. [系统架构](../architecture-design/system-architecture.md)。
9. [项目目录结构](../project-structure.md)。

这些文档提供项目背景、业务语义、系统边界、目录管理和 AI 执行 gate。它们不能被 memory 或旧会话经验替代。

## 任务级必读

完成项目级必读后，还必须按任务类型读取对应专题文档：

| 任务类型 | 额外必读 |
|----------|----------|
| 架构、服务边界、ADR | [架构设计 README](../architecture-design/README.md)、[ADR 目录](../architecture-design/adr/README.md) 和相关 ADR |
| 前端页面、UI/UX、React、TypeScript | [前端设计 README](../frontend-design/README.md) 以及 [docs 入口](../README.md) 中列出的前端任务文档 |
| 后端 Rust、API、数据、缓存、摄入、事件、安全 | [后端设计 README](../backend-design/README.md) 以及 [docs 入口](../README.md) 中列出的后端任务文档 |
| Rust 代码生成或修改 | [Rust Coding Guidelines SPEC](rust-coding-guidelines-spec.md)、[Rust 代码质量规则](../backend-design/rust-code-quality-rules.md)、[Rust AI 代码生成规则](../backend-design/rust-ai-code-generation-rules.md) |
| 领域概念、OSAT、FT、MES、测试数据语义 | [领域知识 README](../domain-knowledge/README.md) 和相关领域专题 |
| 端到端 feature slice | [前后端同步设计](../architecture-design/frontend-backend-sync-design.md)、[首批功能切片 V1](../architecture-design/feature-slices-v1.md) |
| Git/GitHub、提交、推送、回退 | [Git / GitHub 安全 SOP](../architecture-design/git-github-sop.md) |

如果任务同时跨多个类型，必须合并读取路径，不能只选其中一个。

## 读取后的执行要求

AI Agent 完成必读后，必须在内部判断：

1. 当前任务属于哪个 SPEC 或事实来源。
2. 是否存在用户请求和 SPEC 冲突。
3. 是否需要同步更新 docs 入口、分区 README、ADR、CHANGELOG 或交叉引用。
4. 是否需要代码验证、浏览器验证、文档引用验证或 Git 只读诊断。

如果发现冲突，必须先向用户说明冲突和推荐做法，再继续。

## 例外和恢复

- memory、rollout summary、聊天记录和旧 diff 只能帮助定位，不得替代当前文档读取。
- 如果当前会话已经完整读取了必读文档，且任务范围没有变化，可以继续使用本会话上下文。
- 如果 context compaction 后任务仍是同一任务，可以根据 summary 继续；但在编辑 SPEC、架构、Git/GitHub 或跨域规则前，必须重新读取相关关键文档。
- 如果文档路径缺失、链接断裂或内容冲突，必须先修复文档入口或提出阻塞点，不能继续假设。

## 验收

涉及本 SPEC 的文档变更至少要执行：

```bash
rg "agent-startup-context-spec|SPEC 中心|Rust Coding Guidelines SPEC" docs CHANGELOG.md
git diff -- docs
git status --short
```

如果只是 documentation-only 变更，可以不执行代码验证，但最终回复必须说明原因。
