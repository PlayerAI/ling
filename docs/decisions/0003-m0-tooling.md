# DEC-0003：M0 工具选择

> 状态：Accepted  
> 日期：2026-08-18

## 决议

- CLI 参数在 M0 使用小型手写解析器。当前只有五个命令和一个 `--format` 选项；出现共享 option group、shell completion 或复杂约束时再重新评估 `clap`。
- v0.0.1 REPL 以标准输入输出和可脚本化行为为协议基线，不引入终端行编辑依赖。交互历史与补全只能作为不改变脚本语义的外层能力加入。
- Conformance runner 位于 `crates/ling-cli/tests/conformance.rs`，fixture 位于根目录 `tests/conformance/<case>/`，并由 `cargo test --workspace` 实际执行。
- 运行时直接依赖集中在 workspace manifest；测试专用依赖必须保持在 `dev-dependencies`。每次新增或升级依赖都要更新 `docs/DEPENDENCIES.md`、提交 `Cargo.lock` 并通过 `--locked` CI。

## 理由

这些选择保持 M0 简单、离线且可测试，避免在语法和 REPL 语义尚未关闭前引入不可逆框架约束。

