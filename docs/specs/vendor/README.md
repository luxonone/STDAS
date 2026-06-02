# SPEC Vendor References

本目录保存被 STDAS SPEC 引用的外部规范快照。这里的内容是外部参考材料，不是 STDAS 自有规则；STDAS 自有铁律仍以 `docs/specs/*.md` 为准。

## 管理原则

- vendor 快照必须保留上游 `LICENSE`。
- vendor 快照必须记录 source URL、commit 和下载日期。
- 更新 vendor 快照时，必须同步更新引用它的 SPEC 和 `CHANGELOG.md`。
- 不得在 vendor 快照内部直接修改上游文件。需要解释 STDAS 如何采用时，应修改 STDAS 自有 SPEC。
- 如果上游规范和 STDAS SPEC 冲突，以 STDAS SPEC 为准，并在实现或 review 中说明冲突。

## 当前快照

| 名称 | 本地路径 | 上游 | Commit | License | 下载日期 |
|------|----------|------|--------|---------|----------|
| Rust Coding Guidelines 中文版 | [rust-coding-guidelines-zh](rust-coding-guidelines-zh/) | <https://github.com/Rust-Coding-Guidelines/rust-coding-guidelines-zh> | `6b3fc48b285b4f87696634a3e18572d010b30fd4` | MIT | 2026-06-02 |

该快照包含上游 tracked archive 的 355 个文件，不包含上游 `.git` 目录。规范目录入口是 [rust-coding-guidelines-zh/src/SUMMARY.md](rust-coding-guidelines-zh/src/SUMMARY.md)。
