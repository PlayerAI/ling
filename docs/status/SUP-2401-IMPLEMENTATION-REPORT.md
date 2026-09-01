# SUP-2401 实现报告：最小本地 Supervisor 故障包含

> 状态：Done<br>
> 日期：2026-09-01<br>
> 权威：Accepted `DEC-0274`、`DEC-0275`、`DEC-0276`<br>
> 实现提交：`10f8e0c1d09177c97adbfcef283b2ed6c314649d`<br>
> 范围：checked-Core-only、crate-private、内部 Experimental、固定 child、`ContainOne`

## 结论

SUP-2401 已完成 DEC-0276 定义的最小本地 Supervisor 纵切。实现位于
`ling-eval` 内部，拥有一个 run-scoped Actor runtime 和按 `ActorTypeId` 排序的固定
child slots。一个 child 的 turn Fault 会先由真实 Actor runtime 原子关闭、清空 mailbox、
保留有界 Fault 并完成一次 cleanup；随后 Supervisor 在同一 coordinator boundary
同步验证报告。精确匹配的报告只把该 slot 标记为 `Contained`，其他 siblings 继续运行。

该实现没有 restart、replacement、state restore、mailbox transfer、budget、backoff、
circuit breaker、escalation、dynamic/nested tree 或并行 Fault recovery。Supervisor module
没有从 `ling-eval` re-export，因此不新增 public Rust embedding API；Ling source、CLI、REPL、
bytecode、VM、Native、Wasm、LSP 和 editor 仍不能执行 Actor/Supervisor，公开边界保持
`L-ACTOR-0002`。

## 规范覆盖

- DEC-0276 第 1—4 条：`LocalActorSupervisor::start` 只接受一个 successful
  `CheckedProgram`、既有 `ActorRuntimeLimits`、root `LocalTaskControl` 和非空 definition
  集合；先解析并按 `ActorTypeId` canonicalize，拒绝 unknown 与 duplicate type，每个 type
  只创建一个 incarnation。没有树、dynamic child、global registry 或 public identity。
- 第 5—7 条：构造预检 created/live/Fault/shutdown work、command 与 event reserve；按 canonical
  slot 顺序 spawn。initializer Fault 复用 DEC-0274 的 ID retirement、root cancel 和 canonical
  sibling cleanup，错误携带 bounded construction evidence。published lifecycle 实际经过
  `Starting -> Running`，唯一策略是无 restart 的 `ContainOne`。
- 第 8—9 条：`ActorRuntime` 增加私有 `ActorFaultPolicy`；默认 `new` 固定为 `CancelRoot`，只有
  crate-private `new_supervised` 选择 `SupervisorContainment`。turn Fault 返回后，Supervisor
  对 run、Actor、type、definition、expression、phase、original UTF-8 span、registered category、
  discard、cleanup、retained Fault 和唯一 `ActorFaulted` event 做同步一致性检查。只有确认成功
  才抑制默认 root fallback。
- 第 9、13 条：cross-run、unknown Actor/type、wrong definition/expression/phase/category、stale、
  duplicate、discard/cleanup mismatch 均使 Supervisor 与 runtime 进入 `Failed`，停止其余 live
  children 并取消 root Task；Fault report 不携带 payload、path、wall time、thread、address、
  panic text 或 Rust debug output。
- 第 10—12 条：contained slot 终结且后续 send 为 `Closed`；多个 children 可依次 contained，
  但没有 replacement 或 restore。dispatch 仍只接受显式 `step(ActorId)`；explicit stop 和 owner
  cancellation 按 Actor ID 清理 live children，contained child 不会二次 cleanup，repeated stop
  返回 `AlreadyStopped`。
- 第 14—15 条：supervised send/turn 在成功前保留所有 live children 加一个 terminal event 的
  shutdown 空间；边界失败不 dequeue、不 publish state，之后 stop 仍成功。snapshot 只投影
  canonical run-relative identity、slot/Actor lifecycle、queue 与 cleanup count，不包含 host
  timing、allocation、source ID/path 或 worker identity。
- 第 16—17 条：Supervisor 没有 public re-export、语法、diagnostic、schema、Semantic ID、protocol、
  package/ABI 或 backend 入口；原 no-Supervisor integration tests 继续证明 turn Fault 取消 root，
  CLI actor-boundary tests 继续证明 `L-ACTOR-0002`。

## 变更位置

- `crates/ling-eval/src/actor_runtime.rs`：增加私有 `ActorFaultPolicy`、crate-private supervised
  constructor、有效报告前的 deferred root fallback，以及 supervised send/turn shutdown-event
  reserve。默认 `ActorRuntime::new` 行为不变。
- `crates/ling-eval/src/actor_supervisor.rs`：固定 slots、failure-atomic construction、synchronous
  `ChildFaultReport`、`ContainOne` acknowledgement、invalid-report fallback、stop/cancellation、
  canonical snapshot 和 9 个 unit tests（其中 report mutation matrix 有 10 个独立变体）。
- `crates/ling-eval/src/lib.rs`：只声明 private module，不 re-export Supervisor 类型或函数。
- `crates/ling-eval/tests/actor_runtime.rs` 与 `crates/ling-cli/tests/actor_boundary.rs`：保留默认
  no-Supervisor 和 public execution negative evidence。

实现遵循 KISS/YAGNI：只增加两值 private policy 和一个 fixed-child coordinator，不引入通用
strategy hierarchy、restart engine 或预留 variants。DRY/SRP 通过复用唯一 Actor registry、spawn、
send、step、Fault、events、snapshots 和 shutdown 实现；Supervisor 只负责 ownership、report
validation 与 slot transition。

## 已执行验证

以下命令均在实现提交前由当前 worktree 实际执行并通过：

- `cargo test -p ling-eval --all-targets --locked --offline`：45 unit + 59 integration tests；
  Supervisor 新增 9 个 unit tests，原 Actor runtime 12 tests 全部通过。
- `cargo clippy -p ling-eval --all-targets --locked --offline -- -D warnings`。
- `cargo test -p ling-cli --test actor_boundary --locked --offline`：10 passed。
- `cargo test --workspace --all-targets --locked --offline`：完成且无失败；仓库既有一个 fixture
  bless test 保持 ignored。
- `cargo fmt --all -- --check` 与 `git diff --check`：通过。

本报告及状态更新完成后又实际执行并通过：

- `cargo test -p xtask --locked --offline`：174 passed；
- `cargo xtask governance check-all`：343 documents、29 gaps、318 lifecycle records、
  50 protocols、104 diagnostic codes；
- `cargo xtask status verify`：499 tasks、348 `Done`；
- `cargo xtask docs verify`：12 manuals、46 exact evidence paths；
- `cargo fmt --all -- --check` 与 `git diff --check`。

## 兼容性影响

- Source/CLI/diagnostic：无变化；没有新语法、命令、公开执行或错误码。
- Checked Core/Semantic ID/schema/protocol：无变化；Supervisor 只消费现有 immutable checked
  Actor Core，不创建 wire format、Replay input 或 public lifecycle/Fault channel。
- Runtime：默认 no-Supervisor `ActorRuntime::new` 仍在 turn Fault 后取消 root 并关闭 siblings；
  只有 crate-private supervised construction 延迟该 fallback 到同步 acknowledgement。
- Determinism/Unicode：child/cleanup 按 accepted identity 排序；报告排除 host facts，BOM、LF/CRLF、
  Unicode identifier 和 original UTF-8 span evidence 已验证；Unicode 保持 17.0.0。

## 明确延后

SUP-2401 不实现 automatic restart、replacement identity、restart budget/window、backoff、jitter、
circuit breaker、snapshot/restore、mailbox transfer、dynamic/nested Supervisor、lifetime classes、
`OneForOne` restart、`RestForOne`、`OneForAll`、escalation、parallel Fault/recovery、public Fault
channel、Replay、remote delivery、backend Actor execution、fairness/liveness、performance 或 Stable
compatibility。`GAP-ACTOR-MAILBOX-SUPERVISOR-001` 保持 Open，SUP-2402 和 SUP-2403 保持
`BlockedSpec`，直至相应 Accepted authority 存在。
