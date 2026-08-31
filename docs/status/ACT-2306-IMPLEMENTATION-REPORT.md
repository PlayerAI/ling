# ACT-2306 实现报告：Actor 性质与有界压力证据

> 状态：Done<br>
> 日期：2026-08-31<br>
> 权威：Accepted `DEC-0274`、`DEC-0275`<br>
> 实现提交：`3885ca9d74dcb6a05afe1c3fccc9abe578e843db`<br>
> 范围：checked-Core-only、内部 Experimental Rust 测试证据、最多 4 个 worker

## 结论

ACT-2306 已完成 DEC-0275 定义的本地 Actor 性质与有界压力证据。实现位于
`ling-eval` 的 test-only 模块：它构造真实 `ActorRuntime`，只从 successful
`CheckedProgram` 的既有 Actor Core 取输入，并在协调器保留 admission、reservation、
lifecycle、state publication、Fault、stop 与 shutdown 边界的前提下，使用真实 host thread
并行评估不同 Actor 的纯、non-suspending、normal-return turn。

该工作不是公开调度器或语言特性。并行驱动、barrier probe、panic injection、种子和命令序列均
只在 `cfg(test)` 下编译；不新增 Ling 语法、CLI 路径、`ActorRef` value、JSON/trace/replay
protocol、schema、diagnostic、bytecode、VM、Native 或性能承诺。公开 Actor execution 继续由
`L-ACTOR-0002` 拒绝。

## 规范覆盖

- DEC-0275 第 1—3 条：所有案例通过真实 checked frontend 与 `ActorRuntime`；结果 projection
  仅比较 run-relative Actor identity、accepted envelope sequence、committed state、lifecycle、
  cleanup、Fault facts 与原始 source evidence，不把 worker identity、wall time、allocation 或
  thread scheduling 纳入 oracle。
- 第 4—5 条：同一 Actor 的重复发送/step 证明一次只消费一条 FIFO envelope；测试驱动拒绝
  duplicate Actor reservation。不同 Actor 的 batch 先按 Actor ID canonicalize，最多 4 个
  worker 分组评估，所有 worker 完成后再按 Actor ID 提交。两 Actor barrier probe 证明两个
  reserved turn 可同时到达共同同步点，且一/两 worker 的 projection 相同。
- 第 6、8 条：parallel cases 只使用纯 normal-return Counter turn；Fault、owner cancellation、
  stop 和 shutdown 继续在既有 serial coordinator 上验证。test-only evaluator panic 被
  `catch_unwind` containment 成现有 `ActorFaultPhase::Turn` / `InvalidCheckedCore` Fault，既不
  unwind 到 test API，也不泄漏 panic payload。
- 第 7 条：slow consumer 由明确不调用 `step` 建模；案例覆盖 per-Actor `Full`、run-wide queue
  `Full`、原 payload 返回、drain 后 retry 与每 sender 连续 sequence；不使用 sleep、Wait、drop、
  coalescing 或 implicit retry。
- 第 9—10 条：test-local SplitMix64 以四个显式 seed 生成至多 48 条 command（实际 runtime
  command 不超过 256），最多 4 个 Actor、每 Actor mailbox capacity 8、global queue 32；同一
  complete schedule 在一和两个 worker 下比较同一 projection。失败断言携带完整 seed 和 command
  sequence，测试 source 为最小 checked fixture。
- 第 11—12 条：重建案例覆盖 source name、Actor declaration insertion order、Unicode actor name、
  BOM 与 LF/CRLF；zero、one、maximum valid runtime limits，以及 parallel command/event preflight
  exhaustion 均有 failure-atomic evidence。terminal cleanup 按 Actor ID，且每 Actor 仅一次。
- 第 13 条：正向、负向、边界、generated interleaving、parallel turn、cleanup、Fault、Unicode 与
  public-boundary gates 已实际执行并通过。

## 变更位置

- `crates/ling-eval/src/actor_runtime.rs`：仅在 `cfg(test)` 下插入一次 private evaluator panic
  seam，并加载测试模块；normal library build 和任何公开 API 不变。
- `crates/ling-eval/src/actor_runtime_properties.rs`：test-only bounded parallel-turn driver、
  canonical commit、barrier probe、SplitMix64 generator、projection oracle、resource/cleanup/
  Unicode/Fault/backpressure/stress tests。
- `crates/ling-eval/tests/actor_runtime.rs`：保留 ACT-2305 的真实 runtime positive/negative/
  ordering/Fault/Unicode coverage，作为本任务的既有 serial boundary evidence。
- `crates/ling-cli/tests/actor_boundary.rs`：保留公开 Actor execution 的 `L-ACTOR-0002` boundary
  evidence。

实现遵循 KISS/YAGNI：worker driver 不进入 normal build，不复用 Task scheduler，也不制造生产
thread pool；只覆盖 DEC-0275 授权的 pure normal-return profile。DRY/SRP 通过复用唯一
`ActorRuntime`、checked value matcher、events 与 snapshots 来达成；driver 只负责测试 reservation/
evaluation/commit evidence，运行时 coordinator 继续拥有生命周期与 mutation。

## 已执行验证

以下命令均于当前 worktree 实际执行并通过：

- `cargo test -p ling-eval --all-targets --locked --offline`：36 unit + 59 integration tests；其中
  ACT-2306 新增 10 个 property/stress unit tests，existing Actor runtime 12 tests。
- `cargo clippy -p ling-eval --all-targets --locked --offline -- -D warnings`。
- `cargo test -p ling-cli --test actor_boundary --locked --offline`：10 passed，确认公开 Actor
  execution 仍由 `L-ACTOR-0002` 拒绝。
- `cargo test --workspace --all-targets --locked --offline`：通过；不把任何 host timing 或
  worker identity 结果提升为语言语义。
- `cargo xtask governance check-all`、`cargo xtask status verify`、`cargo xtask docs verify` 与
  `cargo test -p xtask --locked --offline`：全部通过（xtask 174 passed）。
- `cargo fmt --all -- --check` 与 `git diff --check`：通过。

治理、状态、文档、format、diff、xtask 和 workspace gate 均在状态更新后再次执行。

## 兼容性影响

- Source/CLI/diagnostic：无变化；没有新语法、命令或错误码。
- Checked Core/Semantic ID/schema/protocol：无变化；驱动只消费既有 checked Actor Core，不产生
  wire format、Replay input 或 public trace。
- Runtime：normal build 无新增 public scheduler/worker setting；test build 仅新增有界 internal
  evidence，commit order 固定为 Actor ID。
- Determinism/Unicode：oracle 排除 host timing；explicit seed/schedule、BTreeMap order、BOM、LF/
  CRLF 与 original UTF-8 byte span evidence 已验证，Unicode 仍为 17.0.0。

## 明确延后

本任务不接受 source-level Actor operations、fairness/liveness、cross-sender global order、parallel
Fault/cancellation/restart resolution、生产 worker pool、watchdog、graceful drain、Supervisor、Replay、
serialization、remote delivery、bytecode/VM/Native Actor ABI 或 Stable compatibility。它们仍需要独立
Accepted authority；`GAP-ACTOR-MAILBOX-SUPERVISOR-001` 保持 Open。
