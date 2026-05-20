# ADR-0004：`stdas-gateway` 是唯一外部 API 入口

状态：Accepted
日期：2026-05-18

## 背景

STDAS 前端需要访问身份、客户配置、测试数据、分析、任务、导出和治理能力。如果前端直接访问多个内部服务，认证、授权、错误码、日志、限流、版本兼容和聚合响应会分散。

## 决策

`stdas-gateway` 是唯一外部 HTTP API 入口：

- 前端只访问 gateway 的 `/api/v1/*`。
- gateway 负责认证校验、外部 REST 契约、错误信封、request id 和聚合响应。
- 内部服务优先暴露 gRPC 契约。
- gateway 不直接读写服务数据库，不承载业务状态机。

## 后果

正面：

- 外部 API 契约稳定。
- 权限、错误、日志、限流和版本兼容统一。
- 内部服务可以独立演进。

代价：

- gateway 需要维护聚合层和内部 client。
- gateway 可能成为入口瓶颈，需要指标、限流和水平扩展准备。

## 替代方案

- 前端直连多个服务：实现快，但权限和契约分散。
- API BFF 按页面拆多个 gateway：第一版复杂度过高。

## 验证方式

- 前端代码不得配置内部服务地址。
- 内部服务端口默认只对受信网络开放。
- gateway 日志必须保留 request id、correlation id、user id、CustomerScope 和 permission result。
