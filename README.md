# STDAS Phase 0 Preflight

This repository currently contains a minimal verification slice for the STDAS
technical baseline. It is preparation work only: no semiconductor test data
business workflow, domain model, ingestion, cache, authentication, or formal
feature behavior is implemented here.

## Stack

- Backend: Rust workspace with `stdas-gateway` using Axum and Tokio.
- Frontend: Vite, React, TypeScript, pnpm.
- API prefix: `/api/v1`.
- Health endpoint: `GET /api/v1/system/health`.

## Run

Install frontend dependencies:

```bash
pnpm install
```

Run the gateway:

```bash
cargo run -p stdas-gateway
```

Run the frontend:

```bash
pnpm dev
```

Open `http://127.0.0.1:5173`.

## Verify

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

