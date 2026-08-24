# DEC-0263: Effect property generator and differential oracle / Effect 性质生成器与差分判定器

> 状态：Proposed<br>
> 提出日期：2026-08-24<br>
> 决定日期：Pending<br>
> Owner role：effect-runtime-design<br>
> 相关 RFC/缺口：RFC-0006 | DEC-0067 | DEC-0262 | EFF-2105<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This proposal defines only an internal, bounded evidence harness over semantics
already accepted by RFC-0006 and DEC-0262. It does not add a source construct,
runtime behavior, public fuzz protocol, or compatibility promise.

本提案仅定义一个内部、有界的证据工具，覆盖 RFC-0006 与 DEC-0262 已接受的
语义；不增加源码构造、运行时行为、公共 fuzz 协议或兼容性承诺。

## Question

How may EFF-2105 generate reproducible small checked programs, minimize a
failure, and compare residual Effect Rows plus interpreter/VM observations
without fabricating Typed Core, interpreting unresolved trees, or making host
randomness and test implementation details part of Ling semantics?

## Decision

1. **Checked input boundary.** Every generated program starts as bounded UTF-8
   source and must pass the normal Source → CST → AST → HIR → resolution → type
   → Effect/Capability pipeline. The harness may evaluate only the resulting
   immutable checked snapshot. It must not construct `CheckedProgram`, typed
   identities, `HandlerCore`, or verified bytecode by bypassing their owning
   validators, and it must discard rejected candidates rather than interpret
   an earlier representation.

2. **Closed generated domain.** Generation is limited to the already accepted
   DEC-0262 execution domain: `Unit`, `Bool`, `Int`, and `Text`; direct and
   closure calls; conditionals and checked exhaustive matches; immutable and
   mutable lexical bindings; `Console.write`; single and nested lexical
   handlers; propagation; zero/one `Once` resume; committed `State<T>` mutation;
   and deterministic Runtime Fault cases already defined by DEC-0013,
   RFC-0020, and DEC-0262. Task, Actor, Supervisor, Clock, Random, user-defined
   operation production, `Many`, Native/Wasm, packages, and unchecked or
   malformed source are outside this generator.

3. **Bounds and reproducibility.** A case contains at most 4 definitions,
   24 expressions, nesting depth 8, 2 handlers, 3 operation clauses, 2 mutable
   bindings, 4 KiB of UTF-8 source, and 8 KiB of logical Console output. The
   suite uses a documented fixed 64-bit seed set and a repository-owned
   deterministic generator; it must not read clocks, entropy, filesystem
   iteration order, allocation addresses, thread scheduling, network state, or
   ambient environment variables. A failure reports the seed, ordinal, and
   canonical logical source name.

4. **Residual-row oracle.** For every checked expression and definition, the
   reference residual row is the canonical checked Effect Row authorized by
   RFC-0006/DEC-0262: a Handler body row minus its handled labels, union every
   clause row, while retaining all `State<T>` labels. The lowered and verified
   bytecode declaration must encode the same canonical row. Comparison uses
   canonical effect identity and payload type, never display/debug strings,
   table insertion order, Rust type names, or allocation identity.

5. **Runtime equivalence.** The interpreter and verifier-created VM run from
   the same checked snapshot under equal explicit limits and capability input.
   Success requires equal logical result values, ordered committed host events,
   resume counts, and externally observable committed mutations. Failure
   requires the same stable Runtime Fault category, reason/facts projection,
   and original UTF-8 source span. Cancellation probes compare the last
   committed observation at an explicit deterministic poll boundary. Resource
   exhaustion is equivalent only when both sides reach the same named limit;
   wall-clock duration, panic text, and internal stack/heap layout are ignored.

6. **Deterministic shrinking.** Shrinking considers, in order: removable
   declarations, removable sequence elements, handler nesting, clause/body
   subexpressions, literal magnitude/text length, and lexical-name simplification.
   Every candidate re-enters the full checked pipeline and is retained only if
   it remains checked, stays inside this domain, and preserves the same
   differential failure projection. The harness performs at most 256 shrink
   attempts and reports the lexicographically smallest surviving UTF-8 source
   among equal-size candidates. It does not write the worktree; a reviewed
   minimized regression becomes a committed fixture in a later change.

7. **Failure and corpus boundary.** A host panic, process abort, output above
   the bound, nondeterministic repeated observation, or exhausted harness bound
   is a test failure, not a Ling Fault. Existing arbitrary-byte decoder fuzzing
   remains responsible for malformed bytecode. The property harness and its
   fixed seeds are internal conformance evidence and expose no CLI command,
   schema, Semantic ID, diagnostic, or public protocol.

## Conformance plan

- Add a deterministic generator module under `crates/ling-vm/tests/support/`
  and prove identical `(seed, ordinal)` inputs produce identical source,
  checked rows, bytecode bytes, and observation projections in repeated
  locked-offline runs.
- Cover pure/effectful direct and closure calls, single/nested handlers,
  propagation, zero/one resume, handler Faults, shared mutable Cell state,
  cancellation before/after commit, missing/unhandled operations, and exact
  resource boundaries.
- Compare checked residual rows with verified bytecode rows and compare
  interpreter/VM values, events, resume counts, mutation observations, Fault
  facts/spans, and Program IDs without debug-string or physical-path equality.
- Exercise BOM, CRLF, Chinese identifiers, combining marks, and emoji while
  preserving original UTF-8 byte spans and Unicode 17.0.0 behavior.
- Unit-test deterministic shrinking, rejection of unchecked candidates,
  no-worktree-write behavior, exact generation/shrink limits, and stable
  replay of at least one intentionally divergent test-double fixture.
- Retain the existing bounded DEC-0067 model properties and bytecode malformed
  fuzz corpus as separate evidence; run workspace, Clippy, governance, status,
  fuzz, fault, security, traceability, formatting, and diff gates.

## Compatibility impact

- Source language, runtime semantics, Capability rules, bytecode 1.0–1.4,
  diagnostics, schemas, Semantic/Definition/Program IDs, CLI, LSP, packages,
  Native/Wasm, and Unicode 17.0.0: unchanged; the proposal tests only accepted
  behavior.
- Adds no public seed, corpus, replay, or property-test protocol. Generator
  source and failure text are internal test evidence and are not compatibility
  surfaces.
- Determinism improves through fixed seeds, canonical projections, explicit
  bounds, and path-free logical names; no host entropy or timing is observed.

## Unresolved alternatives

- Direct construction of Typed Core is rejected because it can fabricate
  identities and invariants that no checked source can produce.
- General grammar fuzzing and malformed source remain owned by existing source
  and parser targets; malformed bytecode remains owned by decoder/verifier
  targets.
- Random seed injection, persistent automatic corpus writes, coverage-guided
  subprocess fuzzing, public replay formats, `Many`, new effect producers,
  Task/Actor crossing, Native/Wasm comparison, and Stable compatibility remain
  separate work requiring their own authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
