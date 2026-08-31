# ACT-2305 实现报告：最小本地 Actor Runtime

> 状态：Done<br>
> 日期：2026-08-31<br>
> 权威：Accepted `DEC-0270`、`DEC-0271`、`DEC-0272`、`DEC-0273`、`DEC-0274`<br>
> 范围：checked-Core-only、bounded、local、publish-disabled Experimental Rust embedding

## 结论

ACT-2305 已实现 DEC-0274 规定的最小本地 Actor Runtime。`ling-eval` 现在可以在内部
Rust embedding 中从一个成功的 `CheckedProgram` 创建 run-owned registry，分配 Actor、
接收 typed message、按显式 Actor ID 驱动单条 FIFO turn、原子发布状态、传播 Fault、响应根
Task 取消，并确定性完成 stop/shutdown 与 exactly-once cleanup。

这不是公开语言执行能力。源码尚无 `spawn`、`send`、`stop` 或 Actor entry；CLI、REPL、
bytecode、VM、Native、Wasm 和 artifact 路径均未接入该 runtime，现有双语
`L-ACTOR-0002` 边界保持不变。

## 规范覆盖

- DEC-0274 第 1—3 条：`ActorRuntime` 仅借用不可变 successful `CheckedProgram`，构造时
  复核每个 Actor Core 的 owner/type/schema/mailbox/turn/expression/binding/type/effect
  对应关系；run ID 非零，Actor ID 从一单调分配且停止或失败后不复用。
- 第 4 条：created/live/queued/event/command/turn/Fault/shutdown-work 均有非零显式上限，
  并验证 `live <= created`、Fault retention 和 shutdown work 等关系。资源耗尽先于 bounded
  mutation。
- 第 5—7 条：spawn 在 initializer 前退休 ID，只在 closed state value 类型一致后原子
  发布实例；send 依次验证 run、Actor、type、schema、lifecycle、payload 和 capacity；所有
  rejection 原样归还 payload，且不改变 queue 或 sender/admission sequence。
- 第 8—10 条：mailbox 使用 checked capacity 的 FIFO；每个 sender sequence 连续；
  `ready()` 以 Actor ID 排序，`step(id)` 只处理一条消息；候选状态在 normal return 并再次
  通过 checked state type 后一次发布，失败结果返回 previous state 证据并清除 terminal
  private state。
- 第 11—15 条：initializer/turn panic 被 containment 为无 unwind payload 的 typed Fault；
  turn Fault 关闭 admission、丢弃剩余消息、请求 `LocalTaskControl` 根取消，并按 Actor ID
  清理其余实例。stop/shutdown 清除 queue/state，保留 bounded terminal reason/Fault，重复
  操作不产生第二次 cleanup/event。
- 第 16—18 条：ready set、events、terminal records、Unicode/BOM/CRLF reconstruction 均
  确定；observations 只用于内部测试，不登记公共 schema/protocol；公开执行继续由
  `L-ACTOR-0002` 拒绝。

## 变更位置

- `crates/ling-eval/src/actor_runtime.rs`：run/Actor/sender identity、typed local reference、
  bounds、registry、envelope、FIFO admission、explicit dispatch、atomic state、Fault、Task
  cancellation bridge、stop/shutdown、events 与 snapshots。
- `crates/ling-eval/src/task_runtime.rs`：把既有 closed `TaskValue` type matcher 以
  `pub(super)` wrapper 复用于 Actor boundary，避免第二套普通值语义。
- `crates/ling-eval/src/lib.rs`：导出明确标注 Experimental/internal 的 Rust embedding types；
  不接入任何公开执行入口。
- `crates/ling-eval/tests/actor_runtime.rs`：12 个 positive、negative、boundary、ordering、
  failure-atomicity、cancellation、Fault、cleanup、determinism 和 Unicode 原始 span 测试。

实现遵循 KISS/YAGNI：只有一个本地串行 coordinator 和 DEC-0273 固定 turn profile，不加入
线程池、Supervisor、Replay 或 backend placeholder；遵循 DRY：Actor 与 Task 共享唯一
closed runtime Value/type matcher；遵循 SRP/DIP：checked contracts 决定可执行输入，
ActorRuntime 只管理本地生命周期，`LocalTaskControl` 作为根所有权取消抽象。

## 已执行验证

以下命令均于当前 worktree 实际执行并通过：

- `cargo test -p ling-eval --all-targets --locked --offline`：26 unit + 59 integration tests，
  其中 Actor runtime 12 tests。
- `cargo clippy -p ling-eval --all-targets --locked --offline -- -D warnings`。
- `cargo test -p ling-cli --test actor_boundary --locked --offline`：10 passed，确认
  `L-ACTOR-0002` 未被绕过。
- `cargo test --workspace --all-targets --locked --offline`：通过，1 个既有 bless test ignored。
- `cargo test -p xtask --locked --offline`：由 workspace gate 覆盖，174 passed。

治理、状态、文档、格式与 diff 门禁在里程碑状态更新后再次执行并记录。

## 兼容性影响

- Source/CLI/diagnostic：无变化；没有新语法、命令或错误码。
- Checked Core/Semantic ID/schema/protocol：无变化；runtime 只消费既有
  `ling.checked-actor-core/3`，不产生 wire format。
- Bytecode/VM/native/package ABI：无变化，Actor 仍不可进入这些路径。
- Determinism：容器使用 `BTreeMap` 与显式序列；跨 sender admission 是调用顺序输入，
  ready/cleanup 按 Actor ID；不读取 wall time、thread、allocation 或 hash-map order。
- Unicode：仍固定 Unicode 17.0.0；中文 identifier、BOM、LF/CRLF 与 original UTF-8 byte
  spans 的 differential evidence 通过。

## 明确延后

ACT-2306 的 Actor properties/stress、真实并行调度、公平性与 Replay，以及 SUP-2401、
REP-2501 以后、REM-2601 和 backend Actor ABI 均未由本任务实现。Source-level Actor API、
ActorRef language value、Supervisor、remote delivery、serialization、Resource/Managed transfer
与 Stable runtime protocol 仍需各自 Accepted authority。
