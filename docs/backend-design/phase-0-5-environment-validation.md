# Phase 0.5 环境验证记录

日期：2026-05-18

复验：2026-05-18，当前 Windows 本机通过 Scoop 安装并验证 NATS Server、NATS CLI、MinIO、MinIO Client、PostgreSQL 和 Redis；Rust、Node.js、pnpm、Git 已可用。

本记录基于当前本机环境和当前仓库状态执行。当前仓库仍处于 V1 文档基线阶段，尚未创建 STDAS Rust workspace 或 React app，因此本次验证只确认本机工具链可用性，并记录项目级命令为何暂不可跑。

## 结论

| 项 | 结果 | 说明 |
|----|------|------|
| Rust toolchain | 通过 | `cargo`、`rustc`、`rustup`、`rustfmt`、`clippy` 可用 |
| Frontend toolchain | 通过 | `node`、`npm`、`pnpm` 可用 |
| Git | 通过 | `git` 可用 |
| PostgreSQL client/server binary | 通过 | `psql`、`postgres` 可用 |
| NATS CLI / server | 通过 | `nats-server`、`nats` 可用 |
| MinIO CLI/server | 通过 | `minio`、`mc` 可用 |
| Redis binary | 通过 | `redis-server`、`redis-cli` 可用 |
| Frontend package manager | 通过 | STDAS 前端统一使用 `pnpm` |
| Docker | 不使用 | Windows 本地开发不安装、不使用 Docker |
| STDAS backend commands | 暂不可执行 | 根目录尚无 `Cargo.toml` |
| STDAS frontend commands | 暂不可执行 | 根目录尚无 `package.json` |

当前状态可以进入 Phase 0 代码骨架创建；不能声称项目级 build/test 已经通过。Phase 0 完成 Rust workspace 和 React app 初始化后，必须重新执行本 gate。

## 已执行命令

| 命令 | 结果 |
|------|------|
| `cargo --version` | `cargo 1.94.0 (85eff7c80 2026-01-15)` |
| `rustc --version` | `rustc 1.94.0 (4a4ef493e 2026-03-02)` |
| `rustup --version` | `rustup 1.29.0 (28d1352db 2026-03-05)` |
| `rustfmt --version` | `rustfmt 1.8.0-stable (4a4ef493e3 2026-03-02)` |
| `cargo clippy --version` | `clippy 0.1.94 (4a4ef493e3 2026-03-02)` |
| `node --version` | `v25.8.0` |
| `npm --version` | `11.11.0` |
| `pnpm --version` | `10.33.0` |
| `git --version` | `git version 2.53.0.windows.2` |
| `psql --version` | `psql (PostgreSQL) 18.3` |
| `postgres` | 可执行，路径为 `C:\Users\UW00133\scoop\apps\postgresql\current\bin\postgres.exe` |
| `nats-server --version` | `nats-server: v2.14.0` |
| `nats --version` | `0.4.0` |
| `minio --version` | `RELEASE.2025-09-07T16-13-09Z` |
| `mc --version` | `RELEASE.2025-08-13T08-35-41Z` |
| `scoop list redis` | `redis 8.6.3` |
| `Get-Command redis-server` | `C:\Users\UW00133\scoop\shims\redis-server.exe` |
| `Get-Command redis-cli` | `C:\Users\UW00133\scoop\shims\redis-cli.exe` |
| `redis-server --version` | `Redis server v=8.6.3 sha=00000000:0 malloc=libc bits=64 build=623fcedff1aaa4a3` |
| `redis-cli --version` | `redis-cli 8.6.3` |
| `cargo check` | 失败：找不到 `Cargo.toml` |
| `cargo test` | 失败：找不到 `Cargo.toml` |
| `pnpm typecheck` | 失败：找不到 `package.json` |
| `pnpm build` | 失败：找不到 `package.json` |

## 当前本机环境基线

| 类别 | 当前选择 | 说明 |
|------|----------|------|
| OS | Windows | 当前开发机为 Windows，本地开发不安装、不使用 Docker |
| 安装方式 | Scoop | PostgreSQL、NATS、MinIO 等基础工具可通过 Scoop 管理 |
| Rust | stable toolchain | 使用当前 stable `cargo` / `rustc` / `rustfmt` / `clippy` |
| Frontend runtime | Node.js | 已安装 Node.js；前端项目创建后再验证版本兼容性 |
| Frontend package manager | pnpm | STDAS 前端统一使用 `pnpm-lock.yaml` |
| Database | PostgreSQL local binary | Phase 0 使用本机 PostgreSQL 可执行文件 |
| Event bus | NATS JetStream local binary | Phase 0 使用本机 `nats-server` |
| Object storage | MinIO local binary 或 local filesystem adapter | Phase 0 可先用本机 MinIO；必要时用本地文件适配器降低复杂度 |
| Cache | Redis available + interface first | Redis 已通过 Scoop 安装；使用时仍必须通过 CacheStore / OptionsCache / TokenRevocationStore / RateLimiter 接口 |

## Windows 本地开发边界

- 前端包管理器统一使用 pnpm。
- Windows 本地开发不安装、不使用 Docker。
- Redis 已安装，可在缓存 adapter、限流、token revocation 或 Options cache 需要时启用。
- Redis 不改变 QuerySnapshot、DataVersion、Evidence、Export metadata 的事实来源。

## 缺口

- 需要创建 STDAS Rust workspace 后重新运行：
  - `cargo fmt`
  - `cargo check`
  - `cargo clippy`
  - `cargo test`
- 需要创建 React + TypeScript app 后重新运行：
  - `pnpm install`
  - lint
  - typecheck
  - test
  - build
- NATS / MinIO 可执行文件已具备；后续还需在 Phase 0 代码骨架创建后验证：
  - NATS JetStream 启动和 publish/subscribe demo。
  - MinIO server 启动、bucket 创建和对象读写。
  - 或明确 Phase 0 先使用 local filesystem object-store adapter。
- Redis 版本命令已通过；运行连通性需在缓存 adapter 接入时验证，包括 server 启动、ping、TTL、delete 和基础 key 操作。
- Docker 不进入 Windows 本地开发验证范围。

## 下一步

1. 基于 V1 文档基线创建 Phase 0 代码骨架。
2. 初始化 Rust workspace、服务 crate、shared/libs crate 和基础 health endpoint。
3. 初始化 React + TypeScript + Vite app，并建立 lint/typecheck/test/build 脚本。
4. 验证 NATS JetStream、PostgreSQL、MinIO 或 local filesystem adapter 的本地启动方式。
5. 在缓存 adapter 接入时验证 Redis 本地启动和 `PING`。
6. 重新执行 Phase 0.5 gate，并把结果追加到本文件或新的验证记录。
