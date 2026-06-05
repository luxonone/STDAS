# 首批功能切片 V1

本文定义进入代码实现前优先落地的能力切片交付卡。它是前端、后端、数据、权限和验收的索引文档，不定义固定前端页面、固定 route、固定导航或固定 mockup 顺序。

前端页面形态必须在具体切片中通过 [AI Mockup Prompt Workflow](../frontend-design/mockup-prompt-workflow.md) 和用户审阅确认。后端契约、数据语义和权限策略仍以对应后端文档为事实来源。

## 事实来源

- 前端代码架构：[frontend-tech-architecture.md](../frontend-design/frontend-tech-architecture.md)
- UI/UX 通用护栏：[ui-ux-constraints.md](../frontend-design/ui-ux-constraints.md)
- AI 设计流程：[mockup-prompt-workflow.md](../frontend-design/mockup-prompt-workflow.md)
- API 规则：[api-principles.md](../backend-design/api-principles.md) + [api-contract-rules.md](../backend-design/api-contract-rules.md)
- QuerySnapshot / 查询预算：[analytics-engine.md](../backend-design/analytics-engine.md)
- DataVersion / lineage：[data-architecture.md](../backend-design/data-architecture.md)
- 权限与脱敏：[security-reliability.md](../backend-design/security-reliability.md)
- 当前 runtime：[system-architecture.md](system-architecture.md) + [ADR-0014](adr/0014-gateway-modular-monolith.md)

## 切片约束

- 切片名称按用户任务或后端能力命名，不按页面命名。
- 页面、route、导航、布局和视觉稿只能作为切片内设计产物，不能反向成为架构事实。
- 每个切片必须声明所需后端模块、API、数据语义、权限策略、异步策略和验收方式。
- 前端可以用 AI 重新探索形态；但必须遵守 CustomerScope、权限、DataVersion、QuerySnapshot、Evidence、Job 和导出安全。

## 1. 身份、会话与授权上下文

### 当前落地状态

该切片当前已完成 M1 最小主链路：登录页通过 `POST /api/v1/auth/login` 使用开发阶段初始账号 `admin / admin@123` 获取 Bearer token，前端保存本地 session，刷新时通过 `GET /api/v1/auth/me` 验证 token 并读取当前用户。登录成功后进入临时空白工作区，仅用于证明 auth 链路已经接通；正式登录后工程入口、AppShell、CustomerScope、permissions 和默认业务上下文仍等待后续设计与契约确认。

### 用户目标

用户使用工厂内部账号密码进入系统，前端获得当前用户、权限范围、默认业务上下文和会话状态。角色、权限和可见数据范围由后端返回，不在前端让用户自行选择角色。

### 后端能力域

- `identity`
- `customer`
- `system`

### API 状态

| 方法 | 端点 | 状态 | 说明 |
|------|------|------|------|
| `POST` | `/api/v1/auth/login` | 已实现最小开发契约 | 登录；当前仅支持 `admin / admin@123` 开发账号 |
| `GET` | `/api/v1/auth/me` | 已实现最小开发契约 | 当前用户；当前仅返回 username 和 display name |
| `POST` | `/api/v1/auth/refresh` | 后续契约 | 刷新 access token |
| `POST` | `/api/v1/auth/logout` | 后续契约 | 登出并让服务端会话失效 |
| `GET` | `/api/v1/options/customers` | 后续契约 | 当前用户可见客户选项 |

### 前端设计要求

- 允许 AI 重新设计登录页、登录后入口和壳层，不预设固定首页。
- 当前登录页视觉以用户确认的 Pencil / imagegen 方向为准；登录后空白工作区不是正式页面设计。
- 必须表达未登录、会话过期、无权限、权限范围受限。
- token、refresh token、临时权限不得进入 URL。
- 当前用户和会话操作必须可达。

### 当前验收

- 使用 `admin / admin@123` 登录成功后进入临时空白工作区。
- 密码错误时返回稳定错误信封，前端展示失败状态。
- 刷新已有 session 时，前端调用 `auth/me` 校验 token；token 无效则清理本地 session 并回到登录页。

### 后续验收

- 登录成功、失败、锁定/限流、token 过期、refresh 失败可区分。
- `auth/me` 返回 CustomerScope、permissions、default context。
- 无权限客户不出现在 options 中，或按权限策略脱敏。

## 2. 系统健康与本地 Preflight

### 用户目标

开发者或管理员确认 `stdas-gateway`、配置、依赖和本地开发闭环是否可用。

### 后端能力域

- `system`
- `telemetry`

### 当前已验证端点

| 方法 | 端点 | 说明 |
|------|------|------|
| `GET` | `/api/v1/system/health` | Gateway 健康检查 |
| `GET` | `/api/v1/system/preflight` | 本地或环境预检 |

### 前端设计要求

- 该能力服务开发/运维验证，不要求成为产品导航固定入口。
- 必须展示健康状态、降级原因、request id 或 correlation id。

### 验收

- 前端能区分 healthy、degraded、dependency unavailable、permission denied。
- 后端错误不被前端吞掉。

## 3. 受控选项与上下文解析

### 用户目标

用户在查询、分析或治理流程中选择客户、产品、测试类型、测试站点、测试次数、设备、程序、参数等受控选项。

### 后端能力域

- `customer`
- `data_pipeline`
- `analytics`

### 候选 API

| 方法 | 端点 | 说明 |
|------|------|------|
| `GET` | `/api/v1/options/customers` | 授权客户 |
| `GET` | `/api/v1/options/products` | 产品选项 |
| `GET` | `/api/v1/options/test-types` | 测试类型选项 |
| `GET` | `/api/v1/options/test-stations` | 测试站点选项 |
| `GET` | `/api/v1/options/parameters` | 参数选项 |
| `GET` | `/api/v1/options/bins` | Bin 选项 |

### 前端设计要求

- Options 字段必须支持 loading、empty、error、permission denied。
- 上游字段变化后，下游字段必须重新加载并校验已选值。
- 已选值 deprecated、hidden、unauthorized 或不存在时，不得静默清空。
- DataProfile 只在治理或诊断任务中作为选项出现，不作为普通查询默认主控件。

### 验收

- options 支持搜索、分页或游标。
- 权限变化和 stale options 可解释。
- 提交字段不得绕过受控选项，除非 API 契约明确允许 free text。

## 4. Lot / Run / File 查询与追溯

### 用户目标

工程师查找授权范围内的 LotNo、Run、TestFile、DataVersion 和 lineage，理解测试数据是否可信。

### 后端能力域

- `data_pipeline`
- `customer`
- `analytics`

### 候选 API

| 方法 | 端点 | 说明 |
|------|------|------|
| `GET` | `/api/v1/data/lots` | Lot 查询 |
| `GET` | `/api/v1/data/lots/{lot_id}` | Lot 详情 |
| `GET` | `/api/v1/data/lots/{lot_id}/runs` | Run 列表 |
| `GET` | `/api/v1/data/files/{file_id}` | 文件详情 |
| `GET` | `/api/v1/data/versions/{data_version}` | DataVersion 详情 |
| `GET` | `/api/v1/data/versions/{data_version}/lineage` | lineage |

### 数据语义

- LotNo、CustomerLotNo、TestType、TestStation、TestAttempt、LotEndTime 是核心业务语义。
- DataVersion、parser/mapping/spec version、DataProfile version 是追溯语义。
- FT 数据中可选位置字段只在数据实际提供且当前任务需要时出现。

### 前端设计要求

- 页面形态由 AI 设计，不预设列表/详情/抽屉模式。
- 必须表达 permission、hidden、masked、partial、stale、latest committed、historical。
- 必须保留从结果进入分析、导出或证据流程所需的内部稳定引用。

### 验收

- 服务端分页、排序、筛选可验证。
- 历史 DataVersion 可打开，不静默切换到 latest committed。
- 无权限、不可见、不存在可区分。

## 5. 分析查询与结果可信

### 用户目标

工程师对 Lot、Bin、参数、测试维度或其它后端支持的分析对象运行查询，获得可复现、可导出或可沉淀为证据的结果。

### 后端能力域

- `analytics`
- `data_pipeline`
- `evidence`
- `workflow`

### 候选 API

| 方法 | 端点 | 说明 |
|------|------|------|
| `POST` | `/api/v1/analysis/queries/run` | 运行分析查询 |
| `GET` | `/api/v1/analysis/query-snapshots/{query_snapshot_id}` | 查询快照详情 |
| `POST` | `/api/v1/analysis/workspaces` | 保存分析工作流状态，名称可在实现时调整 |
| `GET` | `/api/v1/analysis/workspaces/{workspace_id}` | 打开保存的分析状态 |

### 数据语义

- 查询必须固化实际使用的 DataVersion set。
- 响应必须带 query id、query hash、query snapshot id 或 job id。
- over budget 可以返回 partial、采样、拒绝或异步 job。

### 前端设计要求

- AI 可自由探索画布式、向导式、表格优先、图表优先或调查式分析体验。
- 每个结果区必须能表达查询摘要、数据范围、数据可信状态和权限状态。
- 旧响应不得覆盖新查询条件。

### 验收

- 同步成功、partial、over budget 转异步、失败错误可区分。
- 历史结果默认展示固化 DataVersion。
- 重新运行 latest committed 必须生成新的查询引用。

## 6. Evidence / Investigation 证据沉淀

### 用户目标

用户把某次分析结果、图表、表格片段或结论保存为可追溯证据，并在调查流程中复用。

### 后端能力域

- `evidence`
- `analytics`
- `workflow`

### 候选 API

| 方法 | 端点 | 说明 |
|------|------|------|
| `POST` | `/api/v1/investigation/evidence` | 保存证据 |
| `GET` | `/api/v1/investigation/evidence/{evidence_id}` | 查看证据 |
| `POST` | `/api/v1/investigation/evidence/{evidence_id}/recompute` | 重算为新版本 |

### 前端设计要求

- 证据必须显示 evidence version、来源、生成时间、QuerySnapshot、DataVersion set。
- 重算不得覆盖旧证据，必须形成新版本。
- 权限变化导致数据 hidden/masked 时必须可见。

### 验收

- Evidence 可复现其冻结上下文。
- 重新计算生成新版本。
- 无权限时不泄露敏感摘要。

## 7. Job / Export 生命周期

### 用户目标

用户追踪摄入、分析、导出、重试、回放和过期任务，并下载仍有权限的导出结果。

### 后端能力域

- `workflow`
- `analytics`
- `data_pipeline`
- `evidence`

### 候选 API

| 方法 | 端点 | 说明 |
|------|------|------|
| `GET` | `/api/v1/jobs` | 任务查询 |
| `GET` | `/api/v1/jobs/{job_id}` | 任务详情 |
| `POST` | `/api/v1/jobs/{job_id}/cancel` | 取消 |
| `POST` | `/api/v1/jobs/{job_id}/retry` | 重试 |
| `GET` | `/api/v1/exports` | 导出查询 |
| `GET` | `/api/v1/exports/{export_id}` | 导出详情 |
| `POST` | `/api/v1/exports` | 创建导出 |

### 前端设计要求

- 页面形态不固定，但必须展示 job id、状态、阶段、进度、发起人、创建/更新/完成/过期时间。
- 失败任务必须展示失败阶段、错误原因、是否可重试。
- 下载必须显示权限重校验状态。
- 过期文件显示元数据但不显示死链接。

### 验收

- queued、running、succeeded、failed、canceling、canceled、expired、retry_scheduled、dead_letter 可区分。
- retry 说明复用原 QuerySnapshot 还是重新解析 latest committed。
- 下载权限变化可被前端识别。

## 8. Customer / Profile / Rule 治理

### 用户目标

管理员管理客户、产品、测试流程、DataProfile、parser/mapping/spec rule、alert rule、template、feature flag 和审计。

### 后端能力域

- `customer`
- `data_pipeline`
- `analytics`
- `integration`
- `audit`

### 候选 API

| 方法 | 端点 | 说明 |
|------|------|------|
| `GET` | `/api/v1/governance/customers` | 客户治理 |
| `GET` | `/api/v1/governance/profiles` | DataProfile 治理 |
| `GET` | `/api/v1/governance/rules/*` | 规则治理 |
| `GET` | `/api/v1/governance/templates/*` | 模板治理 |
| `GET` | `/api/v1/governance/audit` | 审计查询 |

### 前端设计要求

- 治理任务可以显式展示 DataProfile、规则版本、ProfileResolutionKey、diff、影响范围和审计。
- 发布、覆盖、删除、回放必须显示影响范围。
- 多人并发编辑必须提示冲突。

### 验收

- draft、published、deprecated 状态可区分。
- 发布前有 validation、diff、impact analysis。
- 关键动作有审计上下文。
