# STDAS Phase 0 Preflight

当前仓库只包含 STDAS 技术基线的最小验证切片。该切片仅用于前置验证，尚未实现半导体测试数据业务工作流、Domain Model、ingestion、cache、authentication 或正式功能行为。

## 技术栈

- Backend：Rust workspace，`stdas-gateway` 使用 Axum 和 Tokio。
- Frontend：Vite、React、TypeScript、pnpm。
- API prefix：`/api/v1`。
- Health endpoint：`GET /api/v1/system/health`。

## 项目历史

项目可见变更和本地 commit 编号约定见 [CHANGELOG.md](CHANGELOG.md)。该约定用于区分 code/configuration commit 与 documentation-only commit，避免回档时混淆代码和文档。

## 运行

安装 Frontend dependencies：

```bash
pnpm install
```

运行 Gateway：

```bash
cargo run -p stdas-gateway
```

运行 Frontend：

```bash
pnpm dev
```

打开 `http://127.0.0.1:5173`。

## 验证

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
pnpm lint
pnpm typecheck
pnpm test
pnpm build
```
