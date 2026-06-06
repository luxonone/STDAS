# stdas-gateway Changelog

本文件记录 STDAS 后端 Gateway `backend/services/stdas-gateway` 的 API、错误契约、模块边界、后端行为、运行时配置和后端交付变化。

当前后端 crate version 见根 [Cargo.toml](../../../Cargo.toml) 的 `workspace.package.version`。后端版本可以和前端版本相同，也可以在后续只更新后端时单独递增；如果后端需要脱离 workspace version，应先同步调整 Cargo 版本来源和本文件。

格式遵循 [Keep a Changelog](https://keepachangelog.com/)：

- 新变化先写入 `[Unreleased]`。
- 发布后端版本时，把 `[Unreleased]` 中已发布内容移动到对应版本号，例如 `## [0.1.0] - YYYY-MM-DD`。
- Changelog 只记录 notable changes，不复制每个 commit。

## [Unreleased]

### Added

- 新增 `identity` 模块的开发阶段最小 auth API。
- 新增 `POST /api/v1/auth/login`，当前初始账号为 `admin / admin@123`，用于 Phase 0 登录链路验证。
- 新增 `GET /api/v1/auth/me`，支持通过 `Bearer stdas-dev-admin-token` 读取当前开发阶段 admin 用户。
- 新增 auth request tests，覆盖登录成功、登录失败和有效 token 读取当前用户。

### Security

- 当前固定账号和 token 只用于 Phase 0 开发联调，不代表生产认证设计；正式认证仍需补齐持久化用户、密码存储、token 生命周期、refresh/logout、权限、审计和限流。
