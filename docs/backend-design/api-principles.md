# API 契约原则

当前阶段只有 `stdas-gateway` 一个后端运行服务。当前 minimal app 已实现 `/api/v1/system/*` 与身份会话最小接口 `/api/v1/auth/login`、`/api/v1/auth/me`；其余端点分组是后续扩展原则，不是已经批准的 frontend/product design 或完整 API contract。

## 当前已实现最小 API

| 方法 | 端点 | 说明 |
|------|------|------|
| `GET` | `/api/v1/system/health` | Gateway 健康检查 |
| `GET` | `/api/v1/system/preflight` | 本地或环境预检 |
| `POST` | `/api/v1/auth/login` | 开发阶段最小登录；当前初始账号为 `admin / admin@123` |
| `GET` | `/api/v1/auth/me` | Bearer token 校验和当前用户读取；当前只返回 username、display name |

`/api/v1/auth/login` 和 `/api/v1/auth/me` 当前是 Phase 0 登录页联调用的开发契约，不代表完整生产认证设计。正式认证仍需补齐持久化用户、密码存储、token 生命周期、refresh/logout、权限、CustomerScope、审计和限流。

## 基本原则

- API 面向工作台任务，不按数据库表机械映射。
- 所有接口使用 `/api/v1` 前缀。
- 所有响应使用统一信封。
- 所有错误使用稳定业务错误码。
- 所有业务查询必须带认证上下文和客户范围。
- 大结果导出必须异步。
- 查询响应必须可关联 query id、query hash 或 query snapshot，避免前端旧响应覆盖新状态。
- 分享链接、导出下载和历史 workspace/case/export 打开时必须重新校验权限。
- 前端只访问 `stdas-gateway`，不直接访问内部模块或未来内部服务。
- gateway 负责外部 REST 契约；当前内部通过 Rust module API 协作，未来服务化后内部服务优先暴露 gRPC 契约。
- 字段取值范围、默认值、编码约束、版本兼容规则见 [api-contract-rules.md](api-contract-rules.md)。

## 响应格式

成功：

```json
{
  "code": 0,
  "message": "success",
  "data": {}
}
```

错误：

```json
{
  "code": 40001,
  "message": "invalid request",
  "data": null
}
```

分页：

```json
{
  "code": 0,
  "message": "success",
  "data": {
    "items": [],
    "total": 0,
    "page": 1,
    "page_size": 50
  }
}
```

## 端点组织

端点按工作台任务和功能切片组织，不按前端导航机械照搬。下面是候选分组，具体接口必须说明被哪个页面族或外部集成使用；在产品和前端设计重新确认前，不把这些分组当作当前必须实现的路由。

```text
/api/v1/auth/*
/api/v1/analysis/snapshots/*
/api/v1/data/lots/*
/api/v1/data/files/*
/api/v1/data/versions/*
/api/v1/analysis/queries/*
/api/v1/analysis/query-snapshots/*
/api/v1/analysis/yield/*
/api/v1/analysis/bin/*
/api/v1/analysis/parametric/*
/api/v1/analysis/{analysis_type}/*
/api/v1/analysis/workspaces/*
/api/v1/alerts/*
/api/v1/investigation/evidence/*
/api/v1/jobs/*
/api/v1/exports/*
/api/v1/governance/customers/*
/api/v1/governance/profiles/*
/api/v1/governance/ingestion/*
/api/v1/governance/rules/*
/api/v1/governance/templates/*
/api/v1/options/*
```

端点设计必须保持任务导向。新增端点前必须说明：

- 被哪个前端页面或外部集成使用。
- 是否同步返回还是返回 job id。
- 是否受权限范围、客户/产品、test_type、test_station、LotEndTime、内部解析规则或内部稳定数据引用影响。
- 查询预算和分页限制。
- 分析查询必须支持 `lot_scope`，用于表达单 Lot、多 Lot 或由筛选条件产生的 Lot 集合。
- 分析查询必须返回 query id、query hash 或 query snapshot id，前端据此校验响应是否仍匹配当前页面状态。
- `analysis_type` 只是能力分类入口，不能把分析能力限定为某几个固定算法；具体方法由分析注册表、模板和客户扩展决定。

分析查询必须显式处理 DataVersion：

- 默认策略是对每个 Lot 使用 latest committed DataVersion。
- 如果请求来自历史版本上下文，必须携带显式 DataVersion。
- 响应必须返回实际参与分析的 Lot 和 DataVersion 列表。
- 执行查询时必须固化实际使用的 DataVersion 集合到 query snapshot。
- 历史 workspace、case、export 默认读取固化 DataVersion，不能静默切换到新的 latest committed。
- 重新运行或刷新可以重新解析 latest committed，但响应必须暴露 DataVersion 变化。

## Options API

Options API 用于客户、产品、测试类型、测试站点、测试次数、设备类型、程序名、参数名、Bin、规则版本等受控字段。DataProfile 属于治理/诊断选项，不作为普通工程页面的主选项。

规则：

- options 请求必须接收上游上下文，例如权限范围、customer、product、test_type、test_station、LotEndTime range。
- options 响应必须区分 loading 之外的业务状态：empty、permission denied、deprecated、hidden、unauthorized、not found。
- 已选值变为 deprecated、hidden、unauthorized 或不存在时，后端必须返回可解释状态，不能只返回空列表。
- 超长 options 必须支持搜索、分页或游标。
- options 搜索响应必须带 request/query 标识，便于前端忽略过期响应。
- 除非契约明确允许 free text，否则提交字段必须来自 options API 或受控配置。

## 权限、脱敏与分享

- API 不得通过空响应、错误 message、文件名、URL 参数或导出元数据泄露未授权客户、产品、Lot 信息。
- 无权限、被隐藏、被脱敏、不存在必须使用可区分的错误码或状态。
- share token、workspace id、query snapshot id 打开时必须重新校验 Principal 和 CustomerScope。
- 导出文件下载时必须重新校验权限，不能只依赖生成时权限。
- 响应中的敏感字段是否明文、masked 或 hidden 必须由权限策略决定。

## Gateway 与内部服务

```text
React Analytical Workbench
  -> HTTPS REST
  -> stdas-gateway
  -> gRPC service clients
  -> internal services
```

规则：

- gateway 不直接读写服务数据库。
- gateway 可以聚合多个服务的查询结果，但不能承载业务状态机。
- 内部 gRPC 接口以服务能力建模，不按前端页面建模。
- 长耗时请求返回 job id，由 workflow 或目标服务推进状态。
- gateway 聚合结果时必须保留 query snapshot、request id、job id、DataVersion 集合和权限裁剪状态。

## 契约变更规则

- 新增字段默认可选。
- 修改字段语义必须版本化。
- 删除字段必须进入新 API 大版本。
- 枚举新增必须保持前端兼容。
- 错误码不得复用旧含义。

## 错误码范围

| 范围 | 含义 |
|------|------|
| `400xx` | 请求参数错误 |
| `401xx` | 认证失败 |
| `403xx` | 授权失败 |
| `404xx` | 资源不存在 |
| `409xx` | 状态冲突或重复请求 |
| `422xx` | 业务校验失败 |
| `429xx` | 限流或查询预算超限 |
| `500xx` | 服务内部错误 |
| `503xx` | 依赖不可用或系统降级 |
| `504xx` | 查询或外部调用超时 |


