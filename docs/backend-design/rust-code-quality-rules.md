# Rust 代码质量规则

本文件是 STDAS 后端 Rust 代码生成和修改的事实来源。AI Agent、开发者和代码审查都应以本文件约束 Rust 实现；架构、API、事件、权限等业务规则仍以对应设计文档为准。

> STDAS 同时采用 [Rust Coding Guidelines SPEC](../specs/rust-coding-guidelines-spec.md) 作为通用 Rust coding baseline。本文件负责把外部规范收敛为 STDAS 后端的具体质量规则；如果两者冲突，以本文件和 STDAS 项目事实来源为准。

> AI 生成或修改 Rust 代码还必须读取 [Rust AI 代码生成规则](rust-ai-code-generation-rules.md)；偏航提醒、替代方案和 `TODO(AI-OPTIMIZE, ...)` 机制见 [AI 代码生成治理机制](../architecture-design/ai-code-generation-governance.md)。

## 目标

生成或修改的 Rust 代码必须安全、清晰、惯用、可维护。优先级如下：

1. 正确性。
2. 内存安全。
3. 可读性。
4. Rust 惯用写法。
5. 可测试性。
6. 可维护性。
7. 性能。
8. 简洁性。
9. 避免炫技。

不要为了展示技巧牺牲可读性。好的 STDAS Rust 代码应该让所有权、错误路径、数据状态、权限边界、异步边界和业务意图都清楚可见。

## 总体原则

- 代码应能通过 `cargo check`、`cargo fmt`、`cargo clippy`、`cargo test`。
- 优先使用稳定 Rust，不使用 nightly 特性，除非项目已经明确依赖 nightly。
- 遵循当前 workspace、crate、模块、错误处理、命名和测试风格。
- 不为了“架构感”创建不必要的 trait、泛型、宏、模块或依赖。
- 不随意改变公共 API、DTO 字段、错误类型、事件契约、gRPC 契约或模块边界，除非任务明确要求。
- 不隐藏复杂度。复杂逻辑应拆成清楚的小函数或 use case。
- 不制造隐式行为。所有权、错误、状态转换、权限裁剪、并发共享都应显式表达。
- 不引入不必要的第三方依赖；确实需要新依赖时，必须说明原因，并优先选择成熟、轻量、常用的 crate。

## 所有权与借用

- 函数只读取数据时，优先使用借用参数。
- 输入参数优先使用 `&str`、`&[T]`、`&Path`、`&T` 或 `&mut T`。
- 只有函数确实需要拥有、存储、移动或修改所有权时，才使用 owned 类型。
- 避免不必要的 `.clone()`、`.to_string()`、`.to_owned()`。
- 必须 clone 时，clone 的位置和原因应清楚可见。
- 不为了绕过借用检查器滥用 `Rc<RefCell<T>>`、`Arc<Mutex<T>>`。

优先：

```rust
fn resolve_profile(customer_code: &str) -> Result<DataProfileVersion, ProfileError> {
    // ...
}
```

避免：

```rust
fn resolve_profile(customer_code: String) -> Result<DataProfileVersion, ProfileError> {
    // ...
}
```

除非函数确实需要持有 `String`。

## 错误处理

- 生产代码中不要随意使用 `unwrap()` 或 `expect()`。
- 可能失败的函数必须返回 `Result<T, E>`。
- 可能不存在的值必须使用 `Option<T>`。
- 使用 `?` 传播错误，避免重复和模糊的 `match`。
- 错误类型应表达真实业务含义，不要统一塞进字符串。
- 不吞掉错误，不用空返回值掩盖错误。
- 对外错误必须能映射到稳定 API 错误码；内部错误可以使用枚举错误类型。

允许 `unwrap()` 的场景：

- 测试代码。
- 示例代码。
- 已由前置逻辑证明安全的地方。

允许 `expect()` 的场景：

- 明确不可能失败的静态初始化，并且 message 说明不变量。

```rust
let regex = Regex::new(r"^\d+$").expect("static regex pattern must be valid");
```

推荐：

```rust
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to read config file")]
    Io(#[from] std::io::Error),
    #[error("config file is empty")]
    Empty,
}

pub fn load_config(path: &Path) -> Result<String, ConfigError> {
    let content = std::fs::read_to_string(path)?;

    if content.trim().is_empty() {
        return Err(ConfigError::Empty);
    }

    Ok(content)
}
```

避免：

```rust
fn load_config(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}
```

## 类型设计

- 用类型表达业务约束。
- 用 `enum` 表达有限状态，不用裸字符串或魔法数字。
- 对 ID、状态、权限、DataVersion、QuerySnapshot、CustomerScope、配置版本等有业务含义的值，必要时使用 newtype。
- 避免非法状态可表示。
- 不用过宽类型表达过窄业务概念。

优先：

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobStatus {
    Queued,
    Running,
    Succeeded,
    Canceling,
    Canceled,
    Failed,
    Expired,
    DeadLetter,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuerySnapshotId(String);
```

避免：

```rust
pub struct Job {
    pub status: String,
}
```

## `Option` 与 `Result`

- `Option<T>` 只表示值可能不存在。
- 不对 `Option<T>` 直接 `unwrap()`。
- 使用 `match`、`if let`、`let else`、`map`、`and_then`、`ok_or`、`unwrap_or` 等清楚处理缺失。
- 找不到数据时，返回 `None` 或业务错误，不返回伪造默认值，除非默认值有明确业务含义。
- `Result<T, E>` 表示操作可能失败，错误应保留上下文。
- 不把错误转成 `()` 或模糊 `String`，除非局部边界明确要求。

```rust
let user = find_user(user_id).ok_or(UserError::NotFound)?;
```

## 函数设计

- 函数职责单一，名称准确表达行为。
- 参数数量不宜过多；参数过多时引入请求、配置或上下文结构体。
- 返回值应表达结果，不通过隐藏全局状态传递结果。
- 不让一个函数同时做解析、校验、业务处理、IO、日志、持久化和事件发布。
- 复杂函数应拆成更小的私有函数，但不要机械拆分。
- 纯计算、纯校验、纯转换函数应保持同步，不无意义地标成 async。

## 迭代器、循环与匹配

- 简单转换、过滤、收集时可以优先使用迭代器。
- 复杂业务逻辑中，优先选择更清楚的 `for` 循环。
- 不为链式调用写难以理解的超长 iterator pipeline。
- 对 `enum`、`Option`、`Result` 优先使用 `match`、`if let`、`let else`。
- 不用字符串比较或魔法数字模拟状态机。

## Trait、泛型与宏

- trait 应描述清楚、稳定、小而明确的能力。
- 不为了测试或抽象过早创建 trait。
- 不创建只有一个实现且没有明确扩展需求的 trait，除非用于隔离外部依赖或测试边界。
- 泛型应服务真实复用需求；复杂 trait bound 使用 `where` 子句。
- 如果泛型显著降低可读性，而运行时动态分发足够清楚，可以使用 `Box<dyn Trait>` 或 `&dyn Trait`。
- 不滥用宏。普通函数、泛型、trait 能解决的问题，不使用宏。
- 宏必须明显减少重复或表达必要 DSL，并有测试。

## 生命周期

- 不显式写不必要的生命周期。
- 能让编译器自动推导时，不手写生命周期参数。
- 生命周期标注必须表达真实引用关系。
- 如果生命周期让代码难以维护，应重新审视所有权设计。

## 并发与异步 Rust

- 不滥用共享可变状态。
- 优先使用清晰的所有权转移、消息传递或局部可变状态。
- 只有确实需要跨线程共享状态时，才使用 `Arc<Mutex<T>>` 或 `Arc<RwLock<T>>`。
- 不在持有锁时执行耗时操作。
- 不在持有锁时调用可能阻塞或可能回调用户代码的逻辑。
- 异步代码中不要跨 `.await` 持有 `std::sync::MutexGuard`。
- async 函数中不要执行阻塞 IO，不使用 `std::thread::sleep`。
- 不创建不必要的 task，不忽略 `JoinHandle` 的错误。
- async 边界优先放在 IO 层，不污染纯业务逻辑。

避免：

```rust
let guard = state.lock().expect("state lock poisoned");
some_async_call().await;
drop(guard);
```

优先：

```rust
let value = {
    let guard = state.lock().expect("state lock poisoned");
    guard.clone()
};

some_async_call(value).await;
```

## `unsafe`

- 默认禁止使用 `unsafe`。
- 只有无法用安全 Rust 实现且确有必要时，才允许使用。
- `unsafe` 范围必须尽量小。
- 不把业务逻辑放进 `unsafe` 块。
- 每个 `unsafe fn` 必须包含 `# Safety` 文档。
- 每个 `unsafe` 块附近必须说明为什么安全。
- 对外暴露 API 应尽量安全，把 unsafe 封装在内部。
- 不用 `unsafe` 绕过借用检查器或生命周期问题。

```rust
/// # Safety
///
/// `ptr` must be valid for reads of `len` bytes.
unsafe fn read_raw(ptr: *const u8, len: usize) -> Vec<u8> {
    // ...
}
```

## 模块结构与可见性

- 模块结构服务可读性，不为形式拆分。
- 小模块不要过早拆分太多文件。
- 大模块按业务边界或技术边界组织。
- 不创建空洞的 `utils`、`common`、`helpers`，除非内容稳定且通用。
- 私有实现细节不要暴露为 `pub`。
- 能用 `pub(crate)` 就不要用 `pub`。
- 公共 API、DTO、错误、事件和配置结构要谨慎设计。

## 命名

- 类型名使用 `UpperCamelCase`。
- 函数、变量、模块使用 `snake_case`。
- 常量使用 `SCREAMING_SNAKE_CASE`。
- 布尔变量使用 `is_`、`has_`、`can_`、`should_` 等语义前缀。
- 避免过度缩写和无意义命名。
- 错误类型使用 `XxxError`。
- 配置类型使用 `XxxConfig`。

## 注释与文档

- 注释解释“为什么”，不是重复“做了什么”。
- 公共 API 提供必要文档。
- 复杂业务规则、错误语义、边界条件、状态转换应适当说明。
- `unsafe` 必须有安全说明。
- 不写空泛注释。

## 性能

- 不过早优化。
- 不为了微小性能收益牺牲清晰度。
- 避免明显不必要的分配、clone、字符串转换、锁和动态分发。
- 热路径上应关注内存分配、锁粒度、响应大小和查询预算。
- 性能优化优先基于测量，不基于猜测。
- 对大集合，避免不必要的中间 `Vec`。

## 测试

- 新增核心逻辑时，应同时新增测试。
- 测试覆盖正常路径、错误路径和边界条件。
- 修复 bug 时，应添加回归测试。
- 纯函数优先单元测试。
- IO、网络、数据库等逻辑应通过抽象、临时资源或集成测试覆盖。
- 测试代码可以使用 `unwrap()`，但失败信息应清楚。
- 测试可观察行为，不只测试实现细节。

```rust
#[test]
fn parse_age_rejects_underage_user() {
    let result = parse_age("17");
    assert_eq!(result, Err(AgeError::TooYoung));
}
```

## 修改现有代码

修改现有 Rust 项目时必须遵守：

- 先理解现有模块结构和错误处理风格。
- 保持现有代码风格一致。
- 不随意重构无关代码。
- 不改动与任务无关的公共 API。
- 不删除已有测试，除非测试明显错误且需要替换。
- 不引入大规模架构变化，除非任务明确要求。
- 修改后检查是否需要更新测试、文档、示例和错误信息。
- patch 应小而聚焦。

## 禁止行为

禁止生成以下 Rust 代码：

- 无理由的 `unwrap()`、`expect()`、`.clone()`。
- 无理由的 `unsafe`。
- 使用字符串或魔法数字表示有限业务状态。
- 为绕过所有权问题滥用 `Rc<RefCell<T>>`。
- 为绕过并发设计滥用 `Arc<Mutex<T>>`。
- async 代码中执行阻塞操作。
- 持锁状态下 `.await`。
- 吞掉错误或返回模糊错误。
- 过度泛型化、过度 trait 化、过度模块化。
- 炫技式 iterator 链。
- 未经要求引入新依赖。
- 未经要求改变项目架构或公共 API。

## 推荐行为

生成 Rust 代码时优先：

- 使用 `&str`、`&[T]`、`&Path` 作为输入参数。
- 使用 `Result<T, E>` 表达失败。
- 使用 `Option<T>` 表达缺失。
- 使用 `enum` 表达状态。
- 使用 newtype 表达业务 ID 或受约束值。
- 使用 `?` 传播错误。
- 使用小函数表达清楚步骤。
- 使用清晰的 `match` 处理分支。
- 使用 `pub(crate)` 限制可见性。
- 使用测试覆盖核心逻辑。
- 使用 `cargo fmt` 风格。
- 使用 Clippy 友好的惯用写法。

## 生成代码前自检

输出 Rust 代码前必须自检：

1. 这个函数真的需要拥有参数吗？能否改成借用？
2. 有没有不必要的 clone？
3. 有没有不必要的 unwrap 或 expect？
4. 错误路径是否被清楚表达？
5. 是否应该返回 Result 或 Option？
6. 是否能用 enum 替代字符串状态？
7. 是否存在非法状态可表示的问题？
8. 生命周期标注是否必要？
9. trait 是否真的有必要？
10. 泛型是否真的有必要？
11. 是否引入了不必要依赖？
12. 是否改变了无关代码？
13. 是否有测试覆盖核心逻辑？
14. 是否符合当前项目风格？
15. 是否有隐藏 panic？
16. 是否有隐藏阻塞？
17. 是否有持锁 await？
18. 是否有不必要的 unsafe？
19. 是否能通过 `cargo fmt`、`cargo clippy`、`cargo test`？
20. 读者能否快速理解这段代码的意图？

核心原则：

> 让类型系统表达约束，让 Result 表达失败，让 Option 表达缺失，让 enum 表达状态，让所有权表达数据流向，让测试表达行为。
