# DEC-0088: Effect handler execution rejection gate / Effect Handler 执行拒绝门

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: effect-system-design
> Related RFC/gaps: `RFC-0006` | `DEC-0063` | `DEC-0066` | `GAP-EFFECT-HANDLER-001`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision authorizes only negative conformance evidence that unresolved
handler HIR cannot reach a checked snapshot or execution entry point. It does
not define handler operation semantics, continuations, interpreter behavior,
bytecode, VM ABI, Faults, cancellation, or a public language feature.

本决定只授权负向一致性证据：未解析 Handler HIR 不能进入已检查 snapshot 或执行入口。不定义
Handler operation 语义、continuation、解释器行为、字节码、VM ABI、Fault、取消或公共语言特性。

## Question

DEC-0066 requires the resolver to reject unresolved handlers before any
evaluator, bytecode, or VM path can consume them. The existing compiler
pipeline has a registered bilingual `L-EFFECT-0004` diagnostic, but the
cross-front-end rejection boundary needs an executable fixture. A negative
fixture can prove the boundary without choosing any missing runtime semantics.

DEC-0066 要求 resolver 在任何 evaluator、bytecode 或 VM 路径消费未解析 Handler 之前拒绝它。
现有编译流水线已有注册的双语 `L-EFFECT-0004` diagnostic，但跨前端拒绝边界需要可执行
fixture。负向 fixture 可以证明边界，而不选择任何缺失的运行时语义。

## Decision

1. `ling-cli::compile_source` may be exercised with the accepted unresolved
   handler HIR shape and must return only the registered
   `UNSUPPORTED_HANDLER`/`L-EFFECT-0004` diagnostic with its original source
   span; it must not publish a `ProgramSnapshot`.
2. The fixture is a rejection gate, not a handler evaluator test. It must not
   assert operation dispatch, resume cardinality, continuation lifetime,
   residual runtime rows, State/Fault behavior, cancellation, bytecode, VM,
   ABI, or protocol output.
3. Existing DEC-0066 resolver rejection remains the semantic authority. The
   child only checks the shared CLI compilation boundary and diagnostic
   serialization, without allocating a new diagnostic or changing Seed
   behavior.
4. Public `EFF-2104` remains `BlockedSpec` until handler execution and VM
   authorities are Accepted.

1. `ling-cli::compile_source` 可以使用已接受的未解析 Handler HIR 形状进行测试，并且只能返回
   注册的 `UNSUPPORTED_HANDLER`/`L-EFFECT-0004` diagnostic 及其原始源码 span；不得发布
   `ProgramSnapshot`。
2. fixture 是拒绝门，不是 Handler evaluator 测试。不得断言 operation dispatch、resume 基数、
   continuation 生命周期、残余 runtime row、State/Fault 行为、取消、bytecode、VM、ABI 或
   protocol 输出。
3. 现有 DEC-0066 resolver 拒绝仍是语义权威。子任务只检查共享 CLI 编译边界与 diagnostic
   序列化，不分配新 diagnostic，也不改变 Seed 行为。
4. 公开 `EFF-2104` 仍为 `BlockedSpec`，直到 Handler 执行与 VM 权威被 Accepted。

## Conformance plan

- Compile the accepted unresolved handler source through the shared CLI
  compiler and assert `L-EFFECT-0004`, bilingual message text, and a non-empty
  original source span.
- Assert no checked snapshot is returned and keep all positive handler/runtime,
  bytecode, VM, Fault, cancellation, differential, and migration fixtures
  deferred.

## Compatibility impact

- Adds one internal negative CLI fixture and no language, diagnostic registry,
  schema, Semantic ID, runtime, bytecode, VM, ABI, dependency, or Unicode
  17.0.0 behavior changes.
- Reuses the existing registered `L-EFFECT-0004`; no new public code or
  protocol field is introduced.

## Unresolved alternatives

Checked handler operation signatures, binding and resume typing, nested
propagation, continuation storage, runtime Fault/State/cancellation semantics,
interpreter reference behavior, bytecode/VM ABI and verification, differential
equivalence, public syntax stabilization, and migration remain open under
EFF-2103 through EFF-2105 and the registered Effect gaps.

已检查 Handler operation 签名、绑定与 resume 类型、嵌套传播、continuation 存储、runtime
Fault/State/取消语义、解释器参考行为、bytecode/VM ABI 与验证、差分等价性、公共语法稳定化
和迁移仍由 EFF-2103 至 EFF-2105 及已登记 Effect 缺口决定。

## Supersession

- Supersedes: `None`
- Superseded by: `None`
