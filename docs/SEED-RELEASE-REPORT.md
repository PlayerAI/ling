# `v0.0.1 Seed` 发布门禁报告 / Release Gate Report

> 结论 / Verdict: **READY FOR TAG AUTHORIZATION — all required local and remote gates pass**
> 报告日期 / Report date: 2026-08-19
> 候选提交 / Candidate commit: `652d19b9eaec2ab607edfe1a1e7ea742c861cf91`
> 分支 / Branch: `main`
> CI: [run #4](https://github.com/PlayerAI/ling/actions/runs/32247366834), `success`
> Schema: `ling.semantic/0.1`
> Unicode: `17.0.0`

本文件记录已经验证的候选提交与发布门禁，但不是发布声明，也不授权 commit、tag 或 push。候选提交的本地工作区在审计时干净，且 `origin/main` 指向同一 SHA。

This file records the verified candidate and release gates, but it is not a release declaration or authorization to commit, tag, or push. At audit time the candidate worktree was clean and `origin/main` resolved to the same SHA.

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
- Rustyline TTY 的显式 `Interrupted`/`Eof` 路径、Windows 本地通过的 Ctrl-C session-state 单元测试，以及在候选 SHA 的 Linux/macOS CI 中通过的真实 PTY 进程 fixture。
- Seed literal 边界（非有限 `f64`、前导零、Char 显式拒绝）、结构相等性边界与 Value Restriction 来源 Facts。
- 作用域内 confusable、单个可疑混写名、module/import alias 安全检查，以及 built-in namespace 保护。
- local/higher-order Effect 传播、已解析 root type 的 `State<T>`，以及 Graph/Audit 中不泄漏 nominal DefinitionId 的用户可见类型文本。

Passed for the candidate on 2026-08-19: formatting; offline Clippy with denied warnings; 138 Windows tests plus doc-tests; offline Rustdoc and release builds; Rust 1.85 MSRV verification; offline fuzz-target compilation; Unicode generation idempotence; process-level example coverage; Prelude conformance; complete RFC §6.11 Semantic Graph node coverage; deterministic Audit round-tripping; transactional human/JSON REPL coverage; stable internal incident reports; and distinct internal/snapshot/host failure handling.

## 远程候选证据 / Remote candidate evidence

CI run #4 由候选 SHA `652d19b9eaec2ab607edfe1a1e7ea742c861cf91` 的 push 触发。所有 job 均以 `success` 完成：

CI run #4 was triggered by candidate SHA `652d19b9eaec2ab607edfe1a1e7ea742c861cf91`. Every job completed with `success`:

| Job | Result |
| --- | --- |
| [ubuntu-latest](https://github.com/PlayerAI/ling/actions/runs/32247366834/job/96050714148) | `success` — includes the real-PTY Ctrl-C fixture |
| [macos-latest](https://github.com/PlayerAI/ling/actions/runs/32247366834/job/96050714297) | `success` — includes the real-PTY Ctrl-C fixture |
| [windows-latest](https://github.com/PlayerAI/ling/actions/runs/32247366834/job/96050714138) | `success` |
| [fuzz corpus smoke](https://github.com/PlayerAI/ling/actions/runs/32247366834/job/96050713868) | `success` — pinned nightly, all three corpora |
| [Rust 1.85 MSRV](https://github.com/PlayerAI/ling/actions/runs/32247366834/job/96050714055) | `success` |

## 剩余受控操作 / Remaining controlled action

唯一剩余发布操作是创建并推送指向候选 SHA 的 annotated tag `v0.0.1`。该操作尚未执行，必须由用户单独明确授权。

The only remaining release action is creating and pushing the annotated `v0.0.1` tag at the candidate SHA. It has not been performed and requires separate explicit user authorization.

## 已知限制 / Known limitations

- 当前 evaluator 只接受进程内 checker 构造的 `ProgramSnapshot`; Semantic JSON reader 不提供 JSON/Audit → executable conversion。
- 穷尽性分析有意只覆盖 Seed 承诺的 `Bool` 与 nominal variants；Int、List 和 guarded completeness 后置。
- Windows 本地 ASan/libFuzzer executable 受工具链运行时限制不能作为发布证据；候选 SHA 的三个 corpus 已在 Ubuntu nightly fuzz job 实际执行并通过。
- REPL 的宿主 Capability 当前只通过 `--capability Console.Write` 配置；网络、文件、时间与随机能力不属于 Seed。
- 交互式 REPL 使用 Rustyline 的显式 `Interrupted`/`Eof` 事件；自动化测试覆盖 pending-buffer 清理和 committed-state 保留，Linux/macOS 真实 PTY fixture 已在候选 CI 通过。Windows 完整测试矩阵已通过，但真实 Windows Console 的手工按键 smoke 尚未记录；它是后续平台验证建议，不是 Accepted Seed 验收阻断项。
- Semantic Schema 与 IDs 保持 experimental；不兼容变更必须升级版本并补迁移说明。
- Accepted Seed 当前定义 `f64` literal、类型、pattern 与 IEEE 相等性；算术/比较 operator 的 overload/defaulting 规则尚无 Accepted 决议，实现不猜测该语义。
- Seed 不以用户级高阶 Effect Row 多态为验收要求；当前已解析调用图覆盖 `map` callback 和直接用户 wrapper 传播。

- The evaluator accepts only in-process checked `ProgramSnapshot` values; the Semantic JSON reader exposes no JSON/Audit-to-executable conversion.
- Exhaustiveness intentionally covers only Seed `Bool` and nominal variants; Int, List, and guarded completeness remain deferred.
- A local Windows ASan/libFuzzer executable is not usable as release evidence because of toolchain runtime constraints; all three candidate-SHA corpora executed successfully in the Ubuntu nightly fuzz job.
- The REPL host currently configures only `Console.Write` through `--capability Console.Write`; network, file, time, and random capabilities are outside Seed.
- The interactive REPL uses Rustyline's explicit `Interrupted`/`Eof` events. Automated tests cover pending-buffer clearing and committed-state preservation, and the Linux/macOS real-PTY fixture passed candidate CI. The complete Windows matrix is green, but a manual real-Windows-Console keystroke smoke has not been recorded; it is recommended follow-up platform evidence, not an Accepted Seed release blocker.
- Semantic Schema and IDs remain experimental; incompatible changes require a version bump and migration notes.
- Accepted Seed currently defines `f64` literals, types, patterns, and IEEE equality. Arithmetic/comparison operator overloading and defaulting have no Accepted decision, so the implementation does not invent those semantics.
- User-level higher-order Effect Row polymorphism is not a Seed acceptance requirement; the resolved call graph covers `map` callbacks and direct user-wrapper propagation.

全部功能与证据门禁已经关闭。后续不得改变候选提交内容；只有获得用户单独明确授权后，才能创建并推送 `v0.0.1` tag。

All functional and evidence gates are closed. The candidate content must not change; create and push the `v0.0.1` tag only after separate explicit user authorization.
