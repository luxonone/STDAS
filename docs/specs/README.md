# SPEC 中心

本目录保存 STDAS 的 SPEC。SPEC 是项目内的强约束层，等同于铁律；普通文档负责解释背景、设计细节、辅助材料和执行参考。

## SPEC 定义

SPEC 用来记录必须被 AI Agent、开发者和代码审查遵守的规则。凡是写入 SPEC 的内容，都具有以下含义：

- `必须` 表示 hard requirement，不得因为用户临时指令、实现方便或 AI 判断省略。
- `不得` 表示 hard prohibition，除非用户明确要求修改 SPEC 本身，否则不能绕过。
- `应` 表示默认要求；如果不能执行，必须说明原因、影响和替代方案。
- `可以` 表示允许选项，不构成强制要求。

如果用户请求和 SPEC 冲突，AI Agent 必须先指出冲突和风险，再给出符合 SPEC 的推荐做法。不能为了迎合用户而直接执行会破坏项目的操作。

## SPEC 和普通文档的分区

| 类型 | 位置 | 作用 | 优先级 |
|------|------|------|--------|
| SPEC | `docs/specs/` | 项目铁律、启动规则、编码基线、安全 gate、不可绕过的流程 | 最高 |
| 架构设计 | `docs/architecture-design/` | 产品愿景、领域模型、系统架构、ADR、跨域协作设计 | 高 |
| 领域知识 | `docs/domain-knowledge/` | STDAS 背景、OSAT 场景、测试数据语义和用户工作流 | 高 |
| 前端设计 | `docs/frontend-design/` | 页面、组件、交互、状态、可视化、前端代码规则 | 中 |
| 后端设计 | `docs/backend-design/` | Rust workspace、数据平台、API、安全、部署、实现路线图 | 中 |
| 其他文档 | `docs/project-structure.md`、`CHANGELOG.md` 等 | 目录管理、变更记录、辅助索引 | 辅助 |

普通文档可以承载设计原因、示例、细节和演进记录；但当某条规则已经升级为 SPEC，普通文档只能引用该 SPEC，不能复制或改写铁律正文。

## 当前 SPEC

| SPEC | 范围 |
|------|------|
| [AI Agent Startup Context SPEC](agent-startup-context-spec.md) | 新会话、新任务、上下文读取、项目背景和设计文档阅读 gate |
| [Rust Coding Guidelines SPEC](rust-coding-guidelines-spec.md) | Rust 后端 coding baseline、外部 Rust 编码规范采用方式、Rust 修改验收 |

## 外部规范快照

被 SPEC 引用的外部规范快照保存在 [vendor](vendor/README.md)。vendor 内容用于完整性和可追溯性，不自动成为 STDAS 铁律；只有被 `docs/specs/*.md` 明确采用的条目才进入 STDAS 执行规则。

## 依据优先级

STDAS 项目内规则发生冲突时，按以下顺序收敛：

1. 用户明确要求修改 SPEC 本身时，以新的 SPEC 修改任务为准，但必须说明影响。
2. 当前 `docs/specs/` 中的 SPEC。
3. `docs/architecture-design/` 中的架构设计和 ADR。
4. `docs/domain-knowledge/` 中的领域事实。
5. `docs/frontend-design/` 和 `docs/backend-design/` 中的专题设计。
6. `docs/README.md`、分区 README、`CHANGELOG.md` 和其他辅助文档。

系统、开发者和当前会话的直接指令仍高于项目文档；如果外部指令和项目 SPEC 冲突，AI Agent 必须显式说明冲突，不得假装两者一致。

## 新增或升级 SPEC

后续把普通文档中的重要规则升级为 SPEC 时，必须执行：

1. 在 `docs/specs/` 新增或修改对应 SPEC。
2. 从原普通文档删除重复铁律正文，改为链接到 SPEC。
3. 同步更新 [docs 入口](../README.md)、相关分区 README 和交叉引用。
4. 在 [CHANGELOG](../../CHANGELOG.md) 记录 documentation-only 变更。
5. 用 `rg` 检查旧路径、旧标题和重复规则，避免文档分裂。

SPEC 修改默认是高风险文档修改。AI Agent 不能在没有说明影响的情况下静默改变 SPEC。
