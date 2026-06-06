# @stdas/web Changelog

本文件记录 STDAS 前端工作台 `frontend/web` 的用户可见页面、交互、前端 API client、状态管理、运行时资源和前端构建/交付变化。

当前前端 package version 见 [package.json](package.json)。前端版本可以和后端版本相同，也可以在后续只更新前端时单独递增。

格式遵循 [Keep a Changelog](https://keepachangelog.com/)：

- 新变化先写入 `[Unreleased]`。
- 发布前端版本时，把 `[Unreleased]` 中已发布内容移动到对应版本号，例如 `## [0.1.0] - YYYY-MM-DD`。
- Changelog 只记录 notable changes，不复制每个 commit。

## [Unreleased]

### Added

- 新增登录页视觉实现、登录表单、错误状态、记住账号选项和登录页资源。
- 新增前端 `shared/api` typed auth client，支持调用 `POST /api/v1/auth/login` 和 `GET /api/v1/auth/me`。
- 新增本地 session 保存、刷新时 session 校验和无效 token 清理。
- 新增登录成功后的临时空白工作区，用于确认 auth 链路已经打通；正式登录后工程入口等待下一张页面设计稿确认。
- 新增 auth client 单元测试，覆盖登录成功、登录失败和 `auth/me` bearer token 传递。
