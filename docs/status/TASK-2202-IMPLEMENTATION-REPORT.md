# TASK-2202 Implementation Report

## Outcome

TASK-2202 is complete under Accepted DEC-0265. Implementation commit
`450ec1bad6403a03a702713d80464fa6bbd83172` adds the checked-only,
non-executable `ling.task-machine/0.1` lowering boundary. It does not add Task
execution, lifecycle semantics, bytecode, a scheduler, or a public protocol.

## Normative clauses covered

- DEC-0265 clauses 1–2: effect checking atomically lowers every successful
  `CheckedTaskCore` into one immutable `CheckedTaskMachine` owned by
  `CheckedProgram`; the exact internal version is `ling.task-machine/0.1`.
- Clauses 3–4: deterministic Entry, Suspend, reasoned cleanup, and terminal
  states retain Checked Core scope, task, suspension, continuation,
  cancellation, cleanup, original UTF-8 span, binding, and `TypeId` evidence.
  Suspension frames are sorted by canonical binding identity and liveness is
  never recomputed by the lowering pass.
- Clauses 5–6: reverse continuation lowering preserves expression evaluation
  order, sequential suspension, mutually exclusive `if` and `match` branches,
  nested scope-local returns, normal cleanup, and explicit cancellation and
  Fault exits from every active state.
- Clauses 7–8: validation rejects malformed versions, state/edge identities,
  roles, endpoints, frames, exit skeletons, terminals, reachability, and normal
  paths without return cleanup. Canonical bytes exclude paths, source IDs,
  spans, allocation order, and debug output while source evidence retains exact
  original spans.
- Clauses 9–10: the existing Semantic Graph and Audit Source projections are
  unchanged. File/project execution, tests, REPL, interpreter, bytecode 1.0
  through 1.4, and VM paths continue to reject checked Tasks with
  `L-TASK-0004` before evaluation or artifact publication.

## Evidence

- `crates/ling-effects/src/task_machine.rs` contains checked lowering,
  validation, deterministic canonical encoding, and the DEC-0092 structural
  projection. Unit tests cover malformed models and a synthetic checked
  back-edge boundary without introducing a source loop form.
- `crates/ling-effects/tests/task_state_machine.rs` covers zero and repeated
  suspensions, exact typed frames, `if` and `match` branch exclusivity and
  convergence, nested scope-local continuation, explicit return/cancel/Fault
  cleanup topology, path/source-ID/BOM/CRLF independence, Chinese identifiers,
  and exact source-span retention.
- Existing Task boundary tests prove that checked machine publication does not
  create an execution route or bytecode representation.

Executed gates:

- `cargo fmt --all -- --check`
- `cargo check --workspace --all-targets --message-format short -q`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --all-targets --no-fail-fast`
- `cargo test -p ling-cli --test task_boundary`
- `cargo test -p ling-bytecode --test lowering every_bytecode_revision_rejects_checked_tasks_before_lowering -- --exact`
- the workspace run included all 174 `xtask` tests

## Specification gaps and conflicts

No specification conflict was resolved in code. During conformance work,
nested-scope evidence confirmed the Accepted DEC-0264 rule that a nested
scope's final `return` targets that lexical scope's continuation rather than
prematurely completing the owning Task. The machine lowering carries the
current scope return target explicitly and the regression test requires an
inner suspension to resume into a later outer-scope suspension.

`GAP-STRUCTURED-TASK-001` remains open for TASK-2203 through TASK-2206. It no
longer blocks checked state-machine lowering, but continues to block executable
scope lifecycle, child registration/join, cancellation propagation, cleanup
execution and precedence, child Fault aggregation, scheduling, resource
limits, detach, and Task interpreter/VM behavior.

## Compatibility and determinism impact

- Diagnostics: no code or meaning changed; internal lowering disagreement uses
  the existing `L-TASK-0001` checked-structure domain.
- Schemas/protocols: no public schema, protocol inventory entry, Semantic Graph
  revision, Audit Source revision, bytecode revision, or artifact was added.
- Semantic IDs: existing Semantic Graph and Audit identities are unchanged.
  Machine bytes are internal Experimental evidence and include canonical
  checked identities only.
- Determinism: ordered maps/sets and explicit ordinals remove insertion and
  allocation order. Equivalent reconstruction, physical paths, source IDs,
  BOM/CRLF representation, and source spans do not change canonical bytes.
- Unicode: original UTF-8 spans remain authoritative and Unicode stays pinned
  to 17.0.0.

## Intentionally deferred

TASK-2203 through TASK-2206 retain all executable scope creation/closure,
child registration and default join, cancellation propagation, cleanup code
and precedence, child Fault aggregation, timeout/Clock races, deterministic
and production schedulers, resource/stress behavior, detach, Task bytecode/VM
ABI, and interpreter/VM differential work. No placeholder executable API or
Stable compatibility claim was added.
