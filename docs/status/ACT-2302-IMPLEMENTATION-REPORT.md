# ACT-2302 实施报告：Actor 消息可发送性检查

> 状态：Done<br>
> 日期：2026-08-30  
> 授权：Accepted `DEC-0271`  
> 范围：checked-only `SendableLocal`、本地消息 schema、Experimental `x-ling-actor/0.1`

## 结论

ACT-2302 已实现 `DEC-0271` 第 1—14 条规定的最小完整纵向切片，并通过仓库级门禁。实现只扩展已检查 Actor Core 与 file-mode `ling.semantic/0.1` 数据投影；它没有增加 `send` 语法、Actor runtime、mailbox、serialization、bytecode/VM/native 执行或远程交付能力。

## 规范覆盖

- 第 1、14 条：Actor-bearing 程序仍在所有执行入口以 `L-ACTOR-0002` 停止；新增结构均为 checked-only、data-only。
- 第 2—6 条：`ling-types` 发布封闭、单态、结构化的 `SendableLocal(Value)` 判断。primitive、tuple、`List<T>`、record、variant 按闭包递归判定；function、Task、Task handle、Actor、Trait member、开放变量、error 与未知类别默认拒绝。当前类型系统没有 Borrow、Resource、Managed、Capability 或 source-level `ActorRef` payload 表示，因而这些类别不能通过占位 API 进入消息边界。
- 第 7—10 条：从 checked message type 构造不可变、规范化的完整可达 schema graph；使用 `ling.actor-message-schema-id/v1` 域与 lowercase `experimental:blake3:` digest。递归 nominal graph 使用稳定引用，schema ID 相同但 canonical bytes 不同会阻止 checked Actor snapshot 发布。
- 第 11 条：不支持的源码消息类型继续使用双语 `L-ACTOR-0001`，定位原始 UTF-8 message-type span，并按类别提供稳定 `reason`；内部 schema 不变量保持 typed Rust error。
- 第 12—13 条：Actor-bearing file-mode graph 增加可选 `x-ling-actor/0.1`；writer 仅消费 `CheckedActorCore`，isolated reader 检查版本、ownership、排序、闭包、边、span 与 digest。Actor message canonical bytes 进入 Actor Body/Program identity；无 Actor graph 与 package-aware `ling.semantic/0.2` 保持原行为。

## 实现与证据

- `crates/ling-types/src/actor_message.rs`：SendableLocal schema graph、canonical encoding、public schema identity 与结构化失败原因。
- `crates/ling-types/src/lib.rs`：以 checked type graph 为唯一判断来源，并保留 generic nominal substitution 所需的内部参数映射。
- `crates/ling-effects/src/actor_core.rs`：原子构造 `CheckedActorMessageContract`、碰撞注册、Actor Core canonical identity 与原始 span。
- `crates/ling-semantic/src/lib.rs`：`x-ling-actor/0.1` writer/isolated reader、Actor identity 输入以及对 Audit/package projection 的隔离。
- `schemas/semantic/0.1/schema.json` 与 `schemas/registry.toml`：扩展字段、节点形状和 hash domain 登记；Actor extension 作为 `ling.semantic/0.1` 的 namespaced extension 由该主 schema 覆盖，不声明第二份重复顶层 public schema。
- `crates/ling-cli/tests/actor_boundary.rs`、`crates/ling-semantic/src/lib.rs` tests、`tests/fixtures/actor-message-schema.ling`：positive、negative、generic recursive、deterministic、Unicode/BOM/CRLF、reader corruption、schema collision 与 no-execution 证据。

## 兼容性影响

- Diagnostics：未分配新错误码；`L-ACTOR-0001` 增加更精确的 machine `reason`，`L-ACTOR-0002` 执行边界不变。
- Schema/protocol：`ling.semantic/0.1` 对 Actor program 增加可选 Experimental `x-ling-actor/0.1`；协议 inventory 与 support matrix 已登记。无 Actor JSON bytes 不变。
- Semantic IDs：checked Actor Core canonical domain 从 `/0` 升至 `/1`；只有 Actor-bearing Body/Program identity 纳入 message schema。非 Actor ID 不变。
- Determinism：canonical bytes 排除 source path、source identity、span、TypeId、arena/insertion/allocation/Rust debug 信息；字段与 case 使用规范化语义顺序，递归边使用连续 canonical node ID。
- Unicode：仍固定 Unicode 17.0.0；本变更没有新增 Unicode table、normalization 或 security 规则。

## 已执行验证

- `cargo test -p ling-effects --lib --locked --offline`：29 passed。
- `cargo test -p ling-cli --test actor_boundary --locked --offline`：7 passed。
- `cargo test -p ling-semantic --lib --locked --offline`：21 passed。
- `cargo clippy -p ling-types -p ling-effects -p ling-semantic -p ling-cli --all-targets --locked --offline -- -D warnings`：通过。
- `cargo xtask governance check-gaps`：通过。
- `cargo xtask governance check-protocols`：通过。
- `cargo xtask support verify`：通过。
- `cargo xtask schema validate-all`：通过。

- `cargo test --workspace --all-targets --locked --offline`：通过（1 个既有 bless test ignored）。
- `cargo xtask governance check-all`：通过。
- `cargo xtask status verify`：通过。

## 明确延期

Resource move/失败回滚、Managed sharing、Borrow lifetime、Capability transfer、公开 `ActorRef`、mailbox/backpressure、turn/reentry、Actor runtime、serialization、remote delivery、schema evolution/migration、Audit Source Actor projection，以及 bytecode/VM/native Actor 执行仍未实现。它们分别受后续 Accepted decision、ACT-2303—ACT-2305 与已登记 gap 约束。
