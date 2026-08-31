# ACT-2303 实施报告：有界 mailbox 与 backpressure

> 状态：Done  
> 日期：2026-08-31  
> 授权：Accepted `DEC-0272`  
> 范围：checked-only bounded local mailbox、`Reject` admission、Experimental `x-ling-actor/0.2`

## 结论

ACT-2303 已实现 `DEC-0272` 第 1—15 条规定的最小完整纵向切片。每个已检查 Actor 必须声明一个容量为 `1..=65_535`、overflow policy 为 `Reject` 的本地 mailbox；编译器发布不可变 checked contract、纯 `Accepted`/`Full` admission classification 和 data-only Semantic Graph evidence。实现不创建 Actor 实例或队列，也不增加 `send`、调度、turn、suspension、supervision、serialization、remote delivery、bytecode/VM/native 执行路径。

## 规范覆盖

- 第 1—2、14 条：CST、AST、HIR 接受唯一且位于 `state` 前的 `mailbox capacity <decimal> overflow Reject` clause；旧 Experimental Actor 源码必须显式迁移，缺失、重复或重排 clause 不获得默认值。
- 第 3—6 条：`MailboxCapacity` 精确限制 `1..=65_535`；`Reject` 是唯一 policy。纯 admission classifier 在 queue length 小于容量时返回 `Accepted`、等于容量时返回 `Full`，大于容量时返回 typed invariant error，不分配或持有 payload。
- 第 7—10 条：`CheckedActorCore` 原子持有 Actor owner/type、capacity、policy、canonical bytes 及原始 clause/capacity/policy UTF-8 spans；Actor 执行和生命周期 outcome 仍不可构造。
- 第 11 条：`ling.checked-local-mailbox/1` canonical bytes 仅编码版本、语义容量和 policy，并进入 Actor checked canonical bytes，进而影响 Actor-bearing Body/Program identity；source name、span、trivia、host representation 和 queue state 均不参与。
- 第 12 条：完整但非法的容量或 policy 使用既有双语 `L-ACTOR-0001`，分别携带稳定 reason `mailbox_capacity_out_of_range` 与 `mailbox_overflow_policy_unsupported`，并定位原始 token span；结构错误继续使用 syntax diagnostics。
- 第 13 条：Actor file-mode graph 精确升级为 `x-ling-actor/0.2`，writer 发布 mailbox capacity、policy、canonical bytes 与 spans；isolated reader 独立校验版本、owner、bounds、policy、bytes、span order/source 与 message schema correspondence，且不能构造 executable Core。
- 第 15 条：positive、boundary、migration、determinism、Unicode/BOM/CRLF、reader corruption、bounded stress 与 no-execution 证据均已通过。

## 实现与证据

- `crates/ling-syntax/src/{cst,parser}.rs`：mandatory contextual mailbox clause 与严格结构顺序。
- `crates/ling-ast/src/lib.rs`、`crates/ling-hir/src/lib.rs`：保留 clause、keyword、capacity、policy 的原始 spans 与 normalized policy。
- `crates/ling-concurrency/src/mailbox_contract.rs`：validated capacity、唯一 `Reject` policy、纯 admission classification、typed invariant error 与 path-free canonical bytes。
- `crates/ling-effects/src/actor_core.rs`：`CheckedActorMailboxContract`、Actor Core v2 identity input、稳定 `L-ACTOR-0001` reasons 与原始 spans。
- `crates/ling-semantic/src/lib.rs`、`schemas/semantic/0.1/schema.json`：`x-ling-actor/0.2` writer/isolated reader 与 mailbox object。
- `crates/ling-cli/tests/actor_boundary.rs`、`crates/ling-semantic/src/lib.rs` tests、`tests/fixtures/actor-message-schema.ling`：capacity 1/65,535/0/65,536、unsupported policy、migration、canonical identity、reader corruption、Unicode/BOM/CRLF 与执行边界证据。
- `docs/governance/protocol-inventory.toml`、`docs/governance/support-matrix.toml`、`schemas/registry.toml`：协议、支持状态与 canonical domain 登记。

## 兼容性影响

- Source：所有旧 Experimental Actor declaration 必须在 `state` 前加入 `mailbox capacity 1 overflow Reject` 或其他合法容量；不提供隐式迁移。
- Diagnostics：不新增错误码；`L-ACTOR-0001` 新增两个稳定 machine reasons，`L-ACTOR-0002` 执行边界不变。
- Schema/protocol：Actor extension 从 `x-ling-actor/0.1` 不兼容升级到 `x-ling-actor/0.2`；无自动 JSON adapter，因为 0.1 没有容量。非 Actor `ling.semantic/0.1` bytes 与 package-aware `ling.semantic/0.2` 不变。
- Semantic IDs：checked Actor Core domain 从 `/1` 升至 `/2`，mailbox canonical bytes 进入 Actor-bearing identity；改变 capacity 会改变 Program ID，source name、LF/CRLF 和 span evidence 不会。
- Determinism/Unicode：canonical bytes 使用固定 big-endian length/value encoding，不依赖 Rust layout、allocation 或 map order；Unicode 仍固定为 17.0.0，没有新增 normalization/security/table 行为。

## 已执行验证

- `cargo test -p ling-concurrency --all-targets --locked --offline`：通过。
- `cargo test -p ling-effects --lib --locked --offline`：29 passed。
- `cargo test -p ling-cli --test actor_boundary --locked --offline`：9 passed。
- `cargo test -p ling-semantic --locked --offline`：28 passed。
- `cargo test -p ling-format --lib --locked --offline`：27 passed。
- `cargo clippy -p ling-concurrency -p ling-syntax -p ling-ast -p ling-hir -p ling-effects -p ling-semantic -p ling-format -p ling-cli --all-targets --locked --offline -- -D warnings`：通过。
- `cargo xtask governance check-protocols`、`cargo xtask support verify`、`cargo xtask schema validate-all`：通过。
- `cargo test --workspace --all-targets --locked --offline`：通过（1 个既有 bless test ignored）。
- `cargo fmt --all -- --check`、`git diff --check`、`cargo xtask governance check-all`、`cargo xtask status verify`、`cargo test -p xtask --locked --offline`：通过。

## 明确延期

实际 queue storage、payload byte/heap quota、`send` syntax/outcome、closed/stopped/Fault receiver outcomes、`Wait`、drop/coalesce、waiter fairness、Actor turn/reentry、自发消息、cancellation、scheduler ordering、lifecycle、supervision、Replay、serialization、remote delivery，以及 interpreter/bytecode/VM/native Actor execution 仍受 ACT-2304—ACT-2306 和后续 Accepted authority 约束。本任务没有为这些能力加入 placeholder public API。
