# Phase 0 Preflight Verification

This note records the minimal project surface used before formal STDAS feature
development starts.

## Scope

- Rust workspace root with `crates/services/stdas-gateway`.
- Loco-based gateway health endpoint at `GET /api/v1/system/health`.
- Standard API success envelope with `code`, `message`, and `data`.
- Vite React TypeScript frontend at the repository root.
- Frontend API access isolated under `apps/web/src/shared/api`.
- A preflight workbench page that verifies the frontend can reach the gateway.

## Out of Scope

- OSAT, FT, MES, test data, ingestion, cache, authentication, authorization,
  persistence, and production deployment behavior.
- Formal domain workflows and feature slices after M0.

## Expected Commands

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

当前仓库根目录已配置 Cargo alias。Gateway 启动和路由检查命令为：

```bash
cargo loco start
cargo loco routes
```
