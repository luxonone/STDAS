# Rust Coding Guidelines SPEC

本文定义 STDAS Rust 后端编码规范的外部基线。STDAS 采用 [Rust 编码规范 V 1.0 beta](https://rust-coding-guidelines.github.io/rust-coding-guidelines-zh/) 作为 Rust 项目的通用编码规范参考，并在此基础上叠加 STDAS 自有的架构、API、错误处理、权限、数据平台、Axum HTTP 层和 SQLx 持久化规则。

STDAS 已保存该规范的完整上游快照：[vendor/rust-coding-guidelines-zh](vendor/rust-coding-guidelines-zh/)。后续 Rust 代码生成、修改和 review 不能只依赖本 SPEC 的摘要，必须按任务读取完整快照中的相关条目。

本文是 SPEC，适用于：

- `crates/services/**`
- `crates/libs/**`
- `crates/shared/**`
- `crates/tools/**`
- 后续所有 Rust backend crate

## 依据优先级

Rust 代码生成、修改、评审和验收时，按以下优先级处理：

1. [SPEC 中心](README.md) 中定义的 SPEC 优先级。
2. STDAS 架构、领域、API、权限、数据和功能切片文档。
3. [Rust 代码质量规则](../backend-design/rust-code-quality-rules.md)。
4. [Rust AI 代码生成规则](../backend-design/rust-ai-code-generation-rules.md)。
5. 本 SPEC 引用的 [Rust 编码规范 V 1.0 beta](https://rust-coding-guidelines.github.io/rust-coding-guidelines-zh/)。
6. Rust Book、Rust API Guidelines、rustfmt、Clippy、Tokio、Axum、SQLx、thiserror、tracing 等一手资料。

如果外部 Rust 编码规范和 STDAS 文档冲突，必须先指出冲突，并按 STDAS 项目事实来源收敛。外部规范不能反向覆盖 STDAS 的服务边界、API envelope、错误码、权限、数据版本、功能切片或部署规则。

## 完整规范快照

| 项 | 内容 |
|----|------|
| 上游仓库 | <https://github.com/Rust-Coding-Guidelines/rust-coding-guidelines-zh> |
| 在线版本 | <https://rust-coding-guidelines.github.io/rust-coding-guidelines-zh/> |
| 本地快照 | [vendor/rust-coding-guidelines-zh](vendor/rust-coding-guidelines-zh/) |
| 目录入口 | [vendor/rust-coding-guidelines-zh/src/SUMMARY.md](vendor/rust-coding-guidelines-zh/src/SUMMARY.md) |
| License | [MIT](vendor/rust-coding-guidelines-zh/LICENSE) |
| 固定 commit | `6b3fc48b285b4f87696634a3e18572d010b30fd4` |
| 下载日期 | 2026-06-02 |

该快照是上游 tracked archive，不包含 `.git` 目录。它覆盖代码风格、命名、格式、注释、常量、变量、数据类型、表达式、控制流程、字符串、集合、函数设计、泛型、trait、错误处理、内存管理、模块、Cargo、macro、代码生成、多线程、async、Unsafe Rust、FFI、I/O、安全、测试、工具链和附录。

AI Agent 在处理 Rust 任务时，必须先读取 [SUMMARY.md](vendor/rust-coding-guidelines-zh/src/SUMMARY.md)，再按任务读取相关具体条目。例如：

- 命名和格式问题：读取 `code_style/**`。
- 业务类型、转换、整数、浮点、元组、集合问题：读取 `coding_practice/data-type/**`、`collections/**`。
- 函数、泛型、trait、macro、模块和 Cargo 问题：读取对应 `coding_practice/**` 条目。
- async、多线程、Unsafe Rust、FFI、I/O、安全问题：读取对应专题条目。
- 测试、lint、rustfmt、clippy、deny 配置问题：读取 `Appendix/**` 中的测试、工具和模板条目。

本 SPEC 负责说明 STDAS 如何采用上游规范；vendor 快照负责提供完整原文。不得把 vendor 快照中的所有条目机械等同为 STDAS 自有规则。

## 采用范围

STDAS 采用该规范的完整目录作为 Rust coding baseline 参考。执行时先按完整快照定位相关条目，再按下表收敛为 STDAS 可执行规则：

| 规范范围 | STDAS 采用方式 |
|----------|----------------|
| 代码风格 | 作为 rustfmt、Clippy 和人工 review 的补充依据。 |
| 命名 | 约束 identifier、crate feature、getter、iterator、conversion function 等命名习惯。 |
| 格式 | 以 `cargo fmt` / rustfmt 为主，规范条目用于解释格式期望。 |
| 注释 | 要求注释简洁，解释为什么；public API、`Result`、`panic`、`unsafe` 必须有必要文档。 |
| 数据类型 | 鼓励使用类型表达语义，避免用 primitive type、string 或 magic number 表达受约束业务概念。 |
| 控制流程 | 鼓励清晰的 `match`、pattern matching、`if let`、`let else`，避免滥用 iterator pipeline。 |
| 函数设计 | 控制参数数量，避免 bool 参数膨胀，返回值表达结果，不用隐式全局状态传递结果。 |
| 泛型 / trait / macro | 只在真实复用和语义稳定时引入，避免为少量重复制造复杂抽象。 |
| 多线程 / async | 避免死锁、跨 `.await` 持同步锁、不必要 async、阻塞操作和无边界并发。 |
| Unsafe Rust | 默认禁止；确需使用时必须说明 safety invariant，并符合 `unsafe` 文档要求。 |
| I/O / 安全 | 文件、外部输入、FFI、原始句柄和不可信数据必须显式校验边界。 |

## 对 STDAS 现有规则的补强

本 SPEC 不重复完整外部规范，而是把关键条目收敛为 STDAS 可执行规则。未在下方展开的上游条目仍可作为 review 和实现参考，但不得覆盖 STDAS 自有 SPEC、架构、领域、API、权限和数据规则。

### 命名

- 同一 crate 内 identifier 的词序必须一致。
- 作用域越大，命名越精确；局部变量可以更短，但不能含糊。
- Getter 类方法通常不使用 `get_` 前缀，除非项目或框架约定确实需要。
- Iterator 相关方法遵循 `iter` / `iter_mut` / `into_iter` 语义。
- 不使用 Rust keyword、内置类型、常见 trait 名称作为业务 identifier。
- 不在变量名中附加无意义类型标识，例如 `user_string`、`items_vec`。

### 格式

- 格式统一交给 `cargo fmt`。
- 缩进使用空格，不使用 tab。
- `match` 分支、import 分组、macro 分支必须保持可读。
- 结构体初始化必须写出字段名，不用位置含义表达业务意图。
- 函数参数过多时优先引入 request/config/context struct。

### 类型和数据表达

- 对 Customer、Lot、Wafer、Test、DataProfile、QuerySnapshot、Job、DataVersion 等业务概念，优先使用 domain type、newtype 或 enum。
- 不用裸 `String` 表达有限状态或有约束业务 ID。
- 数字转换优先使用 `TryFrom` / `try_from`，避免不安全的 `as`。
- 整数计算必须考虑 overflow、wrap、truncate 风险。
- 对浮点计算、比较和单位转换要保留精度风险说明。
- 元组不宜表达超过 3 个业务字段；超过时使用 struct。

### 控制流程

- 优先使用 pattern matching 处理 `Option`、`Result`、enum。
- 复杂业务逻辑优先清晰的 `for` 循环，不写难以审查的 iterator pipeline。
- `if` / `else if` 链过长时，考虑 `match`、`cmp` 或拆分函数。
- 需要表达失败时使用 `Result`，需要表达缺失时使用 `Option`，不使用伪默认值掩盖问题。

### 函数设计

- 函数职责单一，不同时承担 parsing、validation、permission、SQL、business logic、response formatting。
- 参数过多或 bool 参数过多时，引入明确的 struct 或 enum。
- 不随意使用 `return`；除非 early return 能显著提升错误路径可读性。
- 不盲目 `#[inline(always)]`。
- 小且 `Copy` 的值可以 by-value；大对象或不需要所有权时按引用传递。

### Async / 多线程

- 异步编程不适合所有场景；纯计算、纯转换、纯 validation 不应为了形式写成 `async`。
- 不在 `.await` 期间持有同步锁、`RefCell` borrow、数据库事务外的不必要大对象借用。
- 不在 async path 中执行阻塞 I/O。
- 后台 task 必须有生命周期、取消、错误上报、tracing 字段和测试入口。

### Unsafe Rust

- STDAS 默认禁止 `unsafe`。
- 不为绕过 borrow checker、lifetime 或性能猜测使用 `unsafe`。
- 每个 `unsafe` block 前必须有 `SAFETY:` 注释。
- public `unsafe fn` 必须有 `# Safety` 文档。
- FFI、裸指针、`MaybeUninit`、`repr(packed)`、manual `Drop` 必须先写设计说明再实现。

## 与现有 STDAS 文档的关系

- 本 SPEC 是外部 Rust coding baseline。
- [Rust 代码质量规则](../backend-design/rust-code-quality-rules.md) 是 STDAS 后端 Rust 代码质量事实来源。
- [Rust AI 代码生成规则](../backend-design/rust-ai-code-generation-rules.md) 是 AI 编写 Rust 代码前后的执行规则。
- [Rust 高质量项目参考与模式](../backend-design/rust-reference-projects-and-patterns.md) 负责参考 Tokio、axum、SQLx、Meilisearch、Qdrant 等项目时的抽取边界。

因此，AI Agent 或开发者不能只引用外部规范来绕开 STDAS 规则。正确做法是：

```text
SPEC 定铁律
STDAS 事实来源定边界
Rust Coding Guidelines 提供通用 coding baseline
rustfmt / Clippy / cargo test 提供本地验证
code review 判断业务语义和可维护性
```

## AI Agent 执行要求

涉及 Rust 后端代码时，AI Agent 必须：

1. 先读取 [AI Agent Startup Context SPEC](agent-startup-context-spec.md) 和本 SPEC。
2. 读取 [vendor/rust-coding-guidelines-zh/src/SUMMARY.md](vendor/rust-coding-guidelines-zh/src/SUMMARY.md)，并按任务读取上游完整规范中的相关条目。
3. 再读取 [Rust 代码质量规则](../backend-design/rust-code-quality-rules.md) 和 [Rust AI 代码生成规则](../backend-design/rust-ai-code-generation-rules.md)。
4. 非平凡实现还要读取 [Rust 高质量项目参考与模式](../backend-design/rust-reference-projects-and-patterns.md)。
5. 在设计说明中明确 Rust Coding Guidelines 影响了哪些选择，例如命名、类型设计、函数参数、`unsafe` 禁止、async 边界、错误处理。
6. 如果用户要求和本 SPEC 冲突，必须先提醒风险并给出推荐替代方案。

## 验收

Rust 变更至少需要按风险执行以下检查：

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

如果只是 documentation-only 变更，可以不执行代码验证，但最终回复必须说明原因。

vendor 快照更新还必须执行：

```bash
rg "rust-coding-guidelines-zh|Rust Coding Guidelines SPEC|vendor/rust-coding-guidelines-zh" docs CHANGELOG.md
git status --short docs/specs
```
