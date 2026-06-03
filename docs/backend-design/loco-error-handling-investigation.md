# Loco Error Handling 调查日志

日期：2026-06-01

## 状态

本文是历史调查日志。STDAS 当前活跃 HTTP 技术基线已经改为直接使用 Axum；Loco 不再作为 `stdas-gateway` 的实现框架。本文只保留早期技术评估证据，不再作为后续错误处理实现的事实来源。

本文记录 STDAS 对 Loco 默认错误处理行为的调查结论。本文只是调查日志，不是实现决策。等 STDAS 开始实现非平凡 request parsing、validation、domain error、authentication、ingestion 或前后端 API 联调时，再基于实际痛点决定是否实现项目级错误处理层。

## 触发背景

在 STDAS 早期采用 Loco 的过程中，我们注意到 Loco 默认错误处理对开发调试和前端/API 联调不够友好。直接触发点是一篇博客，博客描述了 custom message 被隐藏在泛化错误响应后的问题：

- Blog reference: <https://blog.gitcode.com/85c83bbca176b8ca0058f7be2604eada.html>

## 相关 Loco Issue

主要 issue：

- [loco-rs/loco#1138 - Error::Message always results in bad request](https://github.com/loco-rs/loco/issues/1138)

相关 issue：

- [loco-rs/loco#1383 - Expose Detailed Error Information in API Responses During Development](https://github.com/loco-rs/loco/issues/1383)
- [loco-rs/loco#545 - Add Error processing hook](https://github.com/loco-rs/loco/issues/545)
- [loco-rs/loco#1231 - Inconsistent Error Responses for Missing Fields in API Request](https://github.com/loco-rs/loco/issues/1231)

相关 PR：

- [loco-rs/loco#1140 - fix: errors with custom messages are now properly handled](https://github.com/loco-rs/loco/pull/1140)

注意：PR #1140 已关闭且未 merge。Loco 后续版本仍然改动过相关行为，所以不能只根据这个 PR 判断当前版本。

## 当前 STDAS 版本

STDAS 当前使用：

```text
loco-rs 0.16.4
```

来源：

- `backend/services/stdas-gateway/Cargo.toml`
- `Cargo.lock`

## Loco 0.16.4 当前行为

本次检查基于本机 Cargo registry 中的 `loco-rs-0.16.4` 源码。

### 已比博客描述有所改善

`bad_request("message")` 现在会映射到 `Error::BadRequest`，并在 response 中返回 `description`：

```json
{
  "error": "Bad Request",
  "description": "message"
}
```

因此，博客中关于 `bad_request("some text")` 只返回泛化 `Bad Request` 的说法，对 Loco 0.16.4 已经不完全成立。

### 仍然存在风险

`Error::Message("message")` 仍然不会把 message 作为 developer-visible API response 返回。它会落到 generic fallback 分支，最终返回：

```json
{
  "error": "internal_server_error",
  "description": "Internal Server Error"
}
```

这意味着 custom message 细节仍然会从 API client 视角消失。

`JsonRejection` 对 client 也仍然是泛化响应。Loco 会把 JSON extraction 的详细错误写到内部日志，但 response body 仍然是：

```json
{
  "error": "Bad Request"
}
```

这对 production 安全是合理的，但会降低 development 阶段的调试效率，尤其会影响 frontend/API 联调。

## 对 STDAS 的影响

当前 Phase 0 minimal preflight application 还不受影响，因为当前 API 只有：

- `GET /api/v1/system/health`
- `GET /api/v1/system/preflight`

等 STDAS 开始实现以下能力时，这个问题会变得重要：

- JSON request body。
- Query filter。
- DataProfile 选择。
- Ingestion job 创建。
- File upload metadata。
- Authentication / authorization。
- Domain validation。
- Frontend form / API integration。

主要风险是：developer 或 frontend 只能看到 `Bad Request`、`Internal Server Error` 这类泛化错误，而真正有用的错误细节只能在 backend logs 中查找。

## 暂缓处理方向

当前阶段不 fork Loco，也不修改 Loco source。

等这个问题在实际开发中造成明显阻碍时，优先在 STDAS 内部建立项目级 API error layer：

1. 业务/API 错误不要直接使用 `Error::Message`。
2. 使用 `Error::CustomError(StatusCode, ErrorDetail)` 或 STDAS helper function 构造 developer-visible API error。
3. 定义项目级 helper，例如：

```text
api_bad_request(code, message)
api_unauthorized(code, message)
api_forbidden(code, message)
api_conflict(code, message)
api_internal(code, message)
```

4. Production response 保持安全、稳定、不过度暴露内部细节。
5. Development 环境在安全前提下允许返回 debug detail。
6. Validation error 优先返回 field-level structured errors，不返回泛化 `Bad Request`。
7. 在大范围实现前，先把最终 error envelope 写入 API principles 和 API contract rules。

## 后续决策点

在实现第一个非平凡 POST / PUT / PATCH endpoint 前，重新评估是否需要：

- small helper module；
- full API error enum；
- development-only response detail toggle；
- JSON / query validation extractor wrapper；
- tests 固定 error response envelope。
