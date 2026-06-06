# Git Commit and Collaboration SPEC

> **定位**：STDAS Git 提交、推送、Pull Request、主分支合并和回退的强制规则。
> **稳定性**：最高稳定 — 除非用户明确要求修改本 SPEC，否则 AI Agent 和维护者不得绕过。
> **适用范围**：所有 Git remote，包括 GitHub、GitLab、Gitea、Azure Repos、自建 Git 服务和只使用本地 Git 的情况。

本 SPEC 采用主流轻量分支协作方式作为默认：短生命周期 topic branch、结构化 commit、PR/MR review、验证后合并主分支。执行细节见 [Git / GitHub 安全 SOP](../architecture-design/git-github-sop.md)。

## 参考规范

- [GitHub Flow](https://docs.github.com/en/get-started/using-github/github-flow)：短分支、提交、PR、review、merge、删除分支。
- [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/)：`<type>[optional scope]: <description>`，用 `feat`、`fix`、`BREAKING CHANGE` 等表达变更意图。
- [Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/)：changelog 面向人，使用 `[Unreleased]` 和 `Added` / `Changed` / `Deprecated` / `Removed` / `Fixed` / `Security` 分类，不把 git log 直接倒进 changelog。
- [Semantic Versioning 2.0.0](https://semver.org/)：当组件声明了 API 或发布产物时，使用 `MAJOR.MINOR.PATCH` 表达不兼容变更、向后兼容功能和向后兼容修复。

## 基本原则

1. `main` 是可集成主线。默认不得直接在 `main` 上提交并推送功能变更。
2. 所有非平凡变更默认使用短生命周期 branch，并通过 PR/MR 合并到 `main`。
3. 用户只说“提交 git”时，AI Agent 必须先诊断当前状态并推荐提交方式，不得直接 `git add .`、`git commit` 或 `git push`。
4. 工作区有混合变更时，必须按意图分组，显式列出包含和排除文件。
5. 提交必须可 review、可回退、可 bisect。不得为了省事把无关变更塞进同一个 commit。
6. 已推送到共享远端的历史默认不得改写。修正已推送问题默认使用新 commit 或 `git revert`。
7. 公开仓库或可能公开的仓库推送前必须进行敏感信息检查。

## AI Agent Git 请求响应规则

当用户说“提交 git”、“提交 github”、“推送”、“合并主分支”、“回退”、“换分支”或同义表达时，AI Agent 必须先执行只读诊断：

```text
1. 当前分支和 upstream
2. remote 列表和默认 remote
3. 当前 HEAD 和 main/default branch 的关系
4. 工作区未提交变更、未跟踪文件、删除文件
5. staged 与 unstaged 的差异
6. 提交作者 identity
7. 是否存在已知敏感信息、生成目录、构建产物或本地草稿
8. 当前平台能力：GitHub connector、gh、gitlab CLI 或纯 git remote
```

诊断后必须给出推荐动作：

```text
推荐提交方式:
推荐分支/remote:
是否需要 PR/MR:
是否可以直接进 main:
包含文件:
排除文件:
验证命令:
风险和注意事项:
```

只有在诊断和计划完成后，才允许进入暂存、提交、推送或合并阶段。

## 分支策略

| 情况 | 标准动作 |
|------|----------|
| 当前在 `main`，有功能、修复、重构、文档规范或跨文件改动 | 新建短分支：`codex/<topic>`、`feat/<topic>`、`fix/<topic>` 或 `docs/<topic>` |
| 当前已在任务分支，且改动属于同一 PR/MR 目标 | 继续使用当前分支 |
| 当前已在任务分支，但新需求与当前 PR/MR 无关 | 暂停当前分支，切回 `main` 同步后另开新分支 |
| 只做本地保存，不准备远端协作 | 可以只 commit 到本地分支，不 push；必须说明未远端备份 |
| 用户明确要求直接推 `main` | 仍必须诊断、说明风险；只有极小低风险改动或紧急修复且维护者确认时才可执行 |
| hotfix 需要快速进入主线 | 从最新 `main` 开 `hotfix/<topic>`，提交最小修复，验证后 PR/MR 合并；生产事故由维护者决定是否走 emergency direct push |

短分支必须表达变更目标，不得使用 `temp`、`test`、`new`、`update` 这类不可追溯名称。

## Commit 规范

STDAS 新提交采用 Conventional Commits，并使用中文优先描述：

```text
<type>(<scope>): <中文摘要，专有名词可用英文>

[optional body]

[optional footer(s)]
```

常用 type：

| Type | 使用场景 |
|------|----------|
| `feat` | 新增用户可见功能、API、能力切片 |
| `fix` | 修复 bug、错误行为、回归 |
| `docs` | 只修改文档、SPEC、ADR、changelog |
| `test` | 只新增或调整测试 |
| `refactor` | 不改变外部行为的代码结构调整 |
| `perf` | 性能优化且行为不变 |
| `style` | 纯格式、空白、lint 自动格式化且不改变行为 |
| `chore` | 仓库维护、ignore、脚本、小工具、非产品行为 |
| `build` | build system、依赖、lockfile、打包配置 |
| `ci` | CI/CD workflow、检查策略 |
| `revert` | 回退已有提交 |

破坏性变更必须使用 `!` 或 `BREAKING CHANGE:` footer：

```text
feat(api)!: 调整 auth/me 响应结构

BREAKING CHANGE: auth/me 不再返回旧字段 displayName。
```

## Commit 拆分标准

拆分 commit 按变更意图，不机械按文件类型。判断顺序：

1. Atomic change：是否只表达一个可理解的行为、功能切片或维护动作。
2. Reviewability：reviewer 是否能在一个 diff 中看清原因、实现和影响。
3. Revertability：回退该 commit 是否会误伤仍然正确的无关工作。
4. Bisectability：每个 commit 是否保持可构建、可测试或至少不破坏主线诊断。

可以合并到一个 commit：

- 同一功能切片的 code、test、配置、runtime assets、相关文档。
- 同一 bugfix 的实现、回归测试、错误说明和 changelog。
- 同一文档规范调整涉及的入口、分区 README、SPEC 和 changelog。

必须拆分 commit：

- 两个无关功能。
- 代码实现和独立长期文档策略，且任一方可以单独回退。
- 大规模格式化和行为改动。
- 依赖升级和业务功能实现，除非依赖升级是该功能的最小必要条件。
- 生成产物和源文件，除非产物是项目约定必须提交的 runtime asset。
- 高风险安全/认证修改和普通 UI 微调。

## Changelog 规则

1. Changelog 不按 commit 数量维护，不要求每个 commit 都写 changelog。
2. 每个 PR/MR 必须执行 changelog gate：判断本次变更是否有用户、维护者、集成方或发布说明价值。
3. 有长期追踪价值的变化必须写入对应 changelog 的 `[Unreleased]`；无 release note 价值的内部改动可以不写。
4. 使用 `Added`、`Changed`、`Deprecated`、`Removed`、`Fixed`、`Security` 分类。
5. 根 [CHANGELOG](../../CHANGELOG.md) 只记录仓库级、跨前后端、SPEC、ADR、CI、目录结构、发布流程和迁移变化。
6. 前端 [CHANGELOG](../../frontend/web/CHANGELOG.md) 记录 `frontend/web` 页面、交互、前端 API client、状态管理、前端资源、前端构建和前端交付变化。
7. 后端 [CHANGELOG](../../backend/services/stdas-gateway/CHANGELOG.md) 记录 `stdas-gateway` API、错误契约、模块边界、后端行为、运行时配置和后端交付变化。
8. 同一 PR/MR 同时影响前端和后端时，分别更新前端和后端 changelog；如果还改变跨端契约、SPEC、ADR 或发布流程，再更新根 changelog。
9. 修改 Git 流程、SPEC、架构规则、API 契约、用户可见页面、数据语义、安全策略时必须写入对应 changelog。

### Changelog gate

| 变更类型 | 是否写 changelog | 写入位置 |
|----------|------------------|----------|
| 前端用户可见页面、交互、状态、前端 API client | 必须 | `frontend/web/CHANGELOG.md` |
| 后端 API、错误码、认证、权限、数据语义、运行时行为 | 必须 | `backend/services/stdas-gateway/CHANGELOG.md` |
| 前后端契约同步、端到端功能切片、跨端迁移 | 必须 | 前端 + 后端；必要时根 changelog |
| SPEC、ADR、Git/CI/发布流程、目录结构 | 必须 | 根 `CHANGELOG.md` |
| 安全修复、敏感信息处理、权限/认证变化 | 必须 | 受影响组件；跨端时根 changelog 也写 |
| 纯测试重排、内部重命名、无行为变化的小 refactor | 通常不写 | 不写，除非影响维护者升级或排查 |
| typo、格式化、lint-only、注释微调 | 不写 | 不写 |
| 试验草稿、临时验证脚本、未提交生成物 | 不写 | 不写且不得混入提交 |

### 版本轨道

- 前端版本轨道属于 `frontend/web`，当前版本以 `frontend/web/package.json` 为准。
- 后端版本轨道属于 `stdas-gateway`，当前版本以 Cargo package version 为准；当前阶段由根 `Cargo.toml` 的 `workspace.package.version` 提供。
- 前端和后端当前可以同为 `0.1.0`，但后续只更新其中一端时，只递增受影响组件版本。
- 根仓库不作为产品版本轨道；根 `CHANGELOG.md` 不替代前端或后端 changelog。
- 发布组件版本时，把该组件 changelog 的 `[Unreleased]` 移动到 `## [x.y.z] - YYYY-MM-DD`；未发布的另一端保持自己的 `[Unreleased]` 和版本不变。
- 如果后端后续需要独立于 workspace 版本演进，必须先把 `stdas-gateway` 改为 package-local version，并同步更新本 SPEC、后端 changelog 和发布脚本。

## 推送策略

| 情况 | 标准动作 |
|------|----------|
| 本地无 remote | 只做本地 commit 或先配置 remote；不得声称已推送 |
| remote 是 GitHub 且有权限 | push 当前分支到 GitHub，默认创建 draft PR |
| remote 是 GitLab/Gitea/Azure/self-hosted | push 当前分支到对应 remote，按平台创建 MR/PR；如缺少平台工具，提供 remote branch URL 和后续手动步骤 |
| fork 工作流 | push 到 fork 的 topic branch，再向 upstream `main` 创建 PR/MR |
| 只需要远端备份，未准备 review | push topic branch，但 PR/MR 标记 draft 或暂不创建；必须说明未进入主线 |
| 用户要求发布到多个 remote | 逐一确认 remote URL、权限和目标分支，不得默认所有 remote 都安全 |

推送命令必须使用当前分支或显式目标，不得盲推所有分支：

```bash
git push -u origin <branch>
```

## PR/MR 规则

默认使用 PR/MR 承接协作和审查：

- 新功能、bugfix、重构、规范文档、SPEC、依赖升级默认开 draft PR/MR。
- 内容完成、自检通过、描述完整后才标记 Ready for review。
- PR/MR 描述必须包含：变更范围、影响、验证、风险、后续事项。
- 混合提交必须在 PR/MR 描述中标出 Code / Docs / Validation。
- review 反馈通过追加 commit 处理；是否 squash 由合并策略决定。

## 合并 `main` 的条件

只有满足以下条件，才可以把 PR/MR 合并到 `main`：

1. PR/MR 不再是 draft。
2. 变更范围符合用户目标，没有混入无关文件。
3. 必要测试、lint、typecheck、build 或文档检查已通过；未执行项有合理说明。
4. 敏感信息检查已完成，公开仓库没有真实密钥、token、连接串、客户数据或内部截图。
5. Changelog 和相关 SPEC/docs 已同步。
6. 无冲突，或冲突已按当前 `main` 重新解决并重新验证。
7. 需要 review 的场景已经获得维护者或 reviewer 确认。
8. 用户明确要求合并，或项目规则允许维护者自行合并。

合并后应删除已完成的 topic branch。合并方式按仓库规则选择：

- `Squash and merge`：适合多个修正型 commit，需要主线简洁。
- `Create a merge commit`：适合保留完整分支历史。
- `Rebase and merge`：适合线性历史，但必须确认不会破坏协作分支。

## 直接推 `main` 的例外

默认不得直接推 `main`。只有以下情况可以考虑：

- 仓库还未进入多人协作，维护者明确要求直接推。
- 极小文档 typo、ignore 规则或本地配置说明，且无分支保护要求。
- 紧急生产修复，维护者明确接受绕过 PR/MR 的风险。

即使属于例外，也必须完成诊断、计划、验证和敏感信息检查。直接推 `main` 后必须在 changelog、issue 或后续 PR 中补齐可追溯说明。

## 回退规则

| 情况 | 标准动作 |
|------|----------|
| 未提交改动错误 | 优先按文件 `git restore <path>`；禁止默认整仓恢复 |
| 已暂存但不想提交 | `git restore --staged <path>`，保留工作区内容 |
| 本地 commit 未 push | 优先新 commit 修正；如需改写本地历史，必须说明影响并再次确认 |
| commit 已 push 到共享 remote | 默认 `git revert <commit>`，不得默认 reset/force push |
| PR/MR 分支错误 | 在分支上追加修正 commit 或关闭 PR/MR 后重开正确分支 |
| 主分支已合并错误 PR/MR | 新建 revert PR/MR 或 hotfix PR/MR |

禁止默认执行：

```bash
git reset --hard
git clean -fd
git clean -fdx
git checkout -- .
git restore .
git push --force
git push --force-with-lease
```

## 完成报告

每次完成 Git 相关操作后，AI Agent 必须报告：

- 当前分支和 commit hash。
- 是否已 push，push 到哪个 remote/branch。
- 是否创建 PR/MR，链接是什么，是否 draft。
- 是否进入 `main`；如果没有，说明何时可以进入。
- 执行的验证命令。
- 排除的文件或目录。
- 任何未解决风险或需要维护者确认的事项。
