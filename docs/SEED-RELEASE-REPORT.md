# `v0.0.1 Seed` 发布门禁报告 / Release Gate Report

> 结论 / Verdict: **NOT READY — candidate commit, remote CI, and real-TTY evidence pending**
> 报告日期 / Report date: 2026-08-19
> 仓库基线 / Repository baseline: `a88e5ef89abc3c26e0910016dc6305ee79c53e3e`
> 分支 / Branch: `main`
> Schema: `ling.semantic/0.1`
> Unicode: `17.0.0`

本文件是当前工作区的门禁快照，不是发布声明，也不授权 commit、tag 或 push。当前实现尚未形成候选 commit，因此不能填写候选 SHA 或远程 CI run。

This file is a gate snapshot for the current worktree, not a release declaration and not authorization to commit, tag, or push. The implementation is not yet a candidate commit, so no candidate SHA or remote CI run can be claimed.

## 本地环境 / Local environment

| 项目 / Item | 值 / Value |
| --- | --- |
| Stable Rust | `rustc 1.97.1 (8bab26f4f 2026-07-14)` |
| Cargo | `cargo 1.97.1 (c980f4866 2026-06-30)` |
| Fuzz nightly | `cargo 1.99.0-nightly (eb98b54bc 2026-08-11)` from `nightly-2026-08-15` |
| Host | Windows, PowerShell, Asia/Shanghai |
| License | Apache-2.0; dependency inventory in `DEPENDENCIES.md` |

## 已通过的本地门禁 / Passing local gates

2026-08-19 在当前工作区执行并通过：

- `cargo fmt --all -- --check`;
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings`;
- `cargo test --workspace --all-features --locked --offline`（138 个测试通过，另含 doc-tests）；
- `cargo doc --workspace --all-features --no-deps --locked --offline`;
- `cargo build --workspace --all-features --release --locked --offline`;
- `cargo +1.85 check --workspace --all-features --locked --offline`;
- `cargo check --manifest-path fuzz/Cargo.toml --bins --locked --offline`;
- `cargo run -p unicode-gen --locked --offline -- tools/unicode-gen/data target/unicode-generated-check.rs`，随后与跟踪表的 `git diff --no-index` 为空；
- 根 workspace 与 fuzz 锁文件的逐 package license、MSRV 和 `unsafe` 源码存在性清单；
- `examples/人物.ling`、`examples/adt-match.ling`、`examples/pipeline.ling` 的 check/run/semantic 进程级测试；
- `Option`/`Result` Prelude 的 resolve/type/eval/conformance 测试；
- Audit canonical renderer/parser、`L-AUDIT-*` 负例、round-trip 和两个独立进程逐 byte 确定性测试；
- REPL binding、编译/运行失败回滚、重定义 generation、旧 closure、confusable/type 重定义、多行中文、EOF、Console JSON event、human/JSON 脚本模式和 file/session Core 等价测试。
- RFC §6.11 全部 Semantic Graph 节点类别、deterministic owner/source IDs、reader/Audit round-trip 与悬空 owner/source 负例；
- `L-INTERNAL-0001` 稳定 incident ID、`ling.internal-incident/0.1` 本地重现报告、Semantic reader 独立 round-trip，以及 internal/snapshot/host failure 的 exit `5`/`6`/`4` 分离。
- Rustyline TTY 的显式 `Interrupted`/`Eof` 路径，以及 Windows 本地通过的 Ctrl-C session-state 单元测试；Linux/macOS 的真实 PTY 进程 fixture 已纳入 conformance，等待候选 SHA CI 执行。
- Seed literal 边界（非有限 `f64`、前导零、Char 显式拒绝）、结构相等性边界与 Value Restriction 来源 Facts。
- 作用域内 confusable、单个可疑混写名、module/import alias 安全检查，以及 built-in namespace 保护。
- local/higher-order Effect 传播、已解析 root type 的 `State<T>`，以及 Graph/Audit 中不泄漏 nominal DefinitionId 的用户可见类型文本。

Passed in the current worktree on 2026-08-19: formatting; offline Clippy with denied warnings; 138 locked, offline tests plus doc-tests; offline Rustdoc and release builds; Rust 1.85 MSRV verification; offline fuzz-target compilation; Unicode generation idempotence; process-level example coverage; Prelude conformance; complete RFC §6.11 Semantic Graph node coverage; deterministic Audit round-tripping; transactional human/JSON REPL coverage; stable internal incident reports; and distinct internal/snapshot/host failure handling.

## 明确阻断项 / Explicit blockers

1. Windows 本地 fuzz executable 以 `STATUS_DLL_NOT_FOUND (0xc0000135)` 退出；target 可编译，实际 ASan/libFuzzer smoke 仍由 Ubuntu CI 执行。
2. 当前工作区有预期中的未提交实现变更，不满足“发布工作区干净”。
3. 尚无同一候选 SHA 的 Windows、Linux、macOS 远程 CI 结果。
4. 未获得 commit、tag 或 push 的单独危险操作确认。
5. Linux/macOS 的真实 PTY Ctrl-C 进程 fixture 已实现但尚未在候选 SHA CI 执行；Windows 真实 TTY 按键证据尚未记录。

1. The Windows fuzz executable exits with `STATUS_DLL_NOT_FOUND (0xc0000135)`. Targets compile; the real ASan/libFuzzer smoke remains an Ubuntu CI gate.
2. The worktree intentionally contains uncommitted implementation changes and is not release-clean.
3. No Windows/Linux/macOS results exist for one candidate SHA.
4. No separate confirmation has authorized commit, tag, or push operations.
5. A real-PTY Ctrl-C process fixture exists for Linux/macOS but has not run in candidate-SHA CI; real Windows TTY keystroke evidence is not yet recorded.

## 已知限制 / Known limitations

- 当前 evaluator 只接受进程内 checker 构造的 `ProgramSnapshot`; Semantic JSON reader 不提供 JSON/Audit → executable conversion。
- 穷尽性分析有意只覆盖 Seed 承诺的 `Bool` 与 nominal variants；Int、List 和 guarded completeness 后置。
- REPL 的宿主 Capability 当前只通过 `--capability Console.Write` 配置；网络、文件、时间与随机能力不属于 Seed。
- 交互式 REPL 使用 Rustyline 的显式 `Interrupted`/`Eof` 事件；自动化测试覆盖 pending-buffer 清理和 committed-state 保留，Linux/macOS 真实 PTY fixture 等待远程执行，Windows 仍需手工按键证据。
- Semantic Schema 与 IDs 保持 experimental；不兼容变更必须升级版本并补迁移说明。
- Accepted Seed 当前定义 `f64` literal、类型、pattern 与 IEEE 相等性；算术/比较 operator 的 overload/defaulting 规则尚无 Accepted 决议，实现不猜测该语义。
- Seed 不以用户级高阶 Effect Row 多态为验收要求；当前已解析调用图覆盖 `map` callback 和直接用户 wrapper 传播。

- The evaluator accepts only in-process checked `ProgramSnapshot` values; the Semantic JSON reader exposes no JSON/Audit-to-executable conversion.
- Exhaustiveness intentionally covers only Seed `Bool` and nominal variants; Int, List, and guarded completeness remain deferred.
- The REPL host currently configures only `Console.Write` through `--capability Console.Write`; network, file, time, and random capabilities are outside Seed.
- The interactive REPL uses Rustyline's explicit `Interrupted`/`Eof` events. Automated tests cover pending-buffer clearing and committed-state preservation; the Linux/macOS real-PTY fixture awaits remote execution, while Windows still needs manual keystroke evidence.
- Semantic Schema and IDs remain experimental; incompatible changes require a version bump and migration notes.
- Accepted Seed currently defines `f64` literals, types, patterns, and IEEE equality. Arithmetic/comparison operator overloading and defaulting have no Accepted decision, so the implementation does not invent those semantics.
- User-level higher-order Effect Row polymorphism is not a Seed acceptance requirement; the resolved call graph covers `map` callbacks and direct user-wrapper propagation.

当阻断项全部关闭后，应从干净候选 commit 重新执行完整矩阵，把候选 SHA 与三平台 CI URL 回填到本报告，再由用户单独授权 commit/tag/push。

After every blocker is closed, rerun the complete matrix from a clean candidate commit, record its SHA and three-platform CI URLs here, and obtain separate user authorization for commit/tag/push.
