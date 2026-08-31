# ACT-2304 实现报告：Checked non-suspending Actor turn

> 日期：2026-08-31  
> 权威：Accepted `DEC-0270`、`DEC-0271`、`DEC-0272`、`DEC-0273`  
> 范围：checked-only one-message turn、no suspension、no reentry、failure-atomic state-publication evidence、Experimental `x-ling-actor/0.3`

## 结果

ACT-2304 已实现 DEC-0273 规定的首个本地 Actor turn profile。每个
`CheckedActorCore` 现在包含一个不可变 `ling.checked-actor-turn/1` 合同：一次只处理一条
消息、禁止 suspension、禁止 reentry、仅在 normal return 时发布完整候选状态，未来
self-send 只能走同一 bounded mailbox。

本实现仍是 checked-only 数据边界。它不会创建 Actor instance、queue、state cell、
scheduler、continuation 或 runtime turn，也不会执行 `receive`。Actor-bearing 程序继续在
执行前以双语 `L-ACTOR-0002` 停止。

## 规范条款与实现

- DEC-0273 第 1–5、8–12 条：
  [`turn_contract.rs`](../../crates/ling-concurrency/src/turn_contract.rs) 定义固定的
  one-message、non-suspending、no-reentry、mailbox-only profile；合同记录 Actor owner/type、
  transition、state/message binding 以及原始 receive/body spans。构造器拒绝空 owner、零
  Actor type、binding alias 与不一致 spans。
- 第 6–7 条：`ActorTurnCompletion` 到 `ActorTurnStatePublication` 的纯分类只允许
  `NormalReturn -> PublishCandidate`；任何 `Unsuccessful` 完成都保留 previous state。
  这是 checked evidence，不提交或存储状态。
- 第 13 条：canonical bytes 使用 `ling.checked-actor-turn/1`，编码固定模式与 governing
  checked identities，排除 span、路径、trivia、时间、thread、allocation、queue 与 runtime
  state。Checked Actor Core 精确升级到 `/3`，并把 turn bytes 纳入 Actor Body/Program
  identity。
- 第 14 条：未增加诊断码或公开 reason。effectful transition 继续使用双语
  `L-ACTOR-0001`；`await`/`send` 仍不能进入 checked Actor Core。
- 第 15–16 条：file-mode `ling.semantic/0.1` 的 Actor extension 从
  `x-ling-actor/0.2` 显式升级到 `x-ling-actor/0.3`。Writer 投影完整 turn 合同；isolated
  reader 校验 exact version、owner/type、全部 mode、transition/binding、canonical bytes、
  source/span order 与 Actor order，且只能返回 data-only graph。
- 第 17 条：测试覆盖 normal/unsuccessful completion、one-message/no-reentry 模式、
  effect/await/send rejection、owner/type/binding/span/canonical corruption、Unicode Actor
  名称、BOM、CRLF、source-name independence、schema/protocol compatibility，以及
  no-execution boundary。

## 变更位置

- `crates/ling-concurrency/src/turn_contract.rs`：单一职责的 checked turn 合同、typed
  construction errors、canonical bytes 与纯 completion classification。
- `crates/ling-effects/src/actor_core.rs`：从已完成类型与 Effect 检查的 Actor declaration
  原子构造 turn 合同；`CheckedActorCore` 升级到 `/3`。
- `crates/ling-semantic/src/lib.rs`、`schemas/semantic/0.1/schema.json`：
  `x-ling-actor/0.3` writer/schema/isolated reader。
- `crates/ling-cli/tests/actor_boundary.rs`：Checked Core 对应关系、completion 分类、
  Unicode/BOM/CRLF、unsupported operation 与 `L-ACTOR-0002` 证据。
- `schemas/registry.toml`、protocol inventory、support matrix：登记新 canonical domain 与
  Experimental protocol migration。

实现遵循 KISS/YAGNI：只有一个固定 profile，没有可配置 suspension/reentry 策略或未来
runtime placeholder；遵循 DRY：writer 与 reader 共享同一 `ActorTurnContract` 构造和
canonical encoding；遵循 SRP/DIP：concurrency crate 只拥有合同数据，effects 负责 checked
construction，semantic crate 负责 wire projection，CLI 只验证执行边界。

## 验证

以下命令均在 locked、offline workspace 中实际执行：

- `cargo test -p ling-concurrency --all-targets --locked --offline`
- `cargo test -p ling-effects --lib --locked --offline`
- `cargo test -p ling-semantic --lib --locked --offline`
- `cargo test -p ling-cli --test actor_boundary --locked --offline`
- `cargo clippy -p ling-concurrency -p ling-effects -p ling-semantic -p ling-cli --all-targets --locked --offline -- -D warnings`
- `cargo test --workspace --all-targets --locked --offline`
- `cargo xtask schema validate-all`
- `cargo xtask governance check-all`
- `cargo xtask support verify`
- `cargo xtask status verify`
- `cargo fmt --all -- --check`
- `git diff --check`

## 兼容性影响

- Source/diagnostic：无语法或诊断码变化。
- Checked Core：Actor Core `/2 -> /3`；仅 Actor-bearing identity 加入 turn contract。
- Schema/protocol：`x-ling-actor/0.2 -> /0.3`，必须显式迁移，无自动 adapter。
- 非 Actor `ling.semantic/0.1`、package-aware `ling.semantic/0.2`、bytecode、VM、ABI 与
  Unicode 17.0.0：不变。
- Determinism：canonical bytes 不包含 source evidence 或 host/runtime facts；Unicode、BOM、
  LF/CRLF 与 source-name differential evidence 通过。

## 明确延后

ACT-2305 及后续 authority 仍需定义并实现 Actor runtime、instance/registry、实际 queue 与
dequeue、typed send outcome、state cell/atomic commit、lifecycle/Fault、cancellation、
shutdown、scheduler/fairness、watchdog observation、supervision、Replay、serialization、
remote delivery、interpreter/bytecode/VM/native execution，以及任何新的 suspending/reentrant
profile。本任务没有为这些能力创建 placeholder API。
