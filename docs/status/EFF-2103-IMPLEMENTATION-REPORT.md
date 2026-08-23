# EFF-2103 Implementation Report: Checked Handler Core

## Status

Implemented under `SEMANTICS.md` §15.7, Accepted RFC-0006, DEC-0062 through
DEC-0066, and DEC-0260. This closes the source-to-checked-Core representation
task only. EFF-2104 still owns interpreter/VM execution and continuation
behavior; EFF-2103 publishes existing Semantic Graph identities and the bounded
`ling.audit/0.2` Handler projection.

## Normative clauses covered

- RFC-0006 §§4–7 and §9: canonical operation/row model, explicit first-order
  contracts, residual rows, lexical nesting, resume cardinality, Capability
  separation, original spans, and deterministic path-free projection.
- SEMANTICS §15.7 and DEC-0260 §§1–4: exact operation registry, independent
  clause scopes, parameter and `Output -> R` resume types, common result type,
  and `Once`/`Many` source-reference limits.
- DEC-0260 §§5–7: nested/transitive Effect subtraction, clause-body
  propagation, visible State, unmasked Capability closure, and atomic
  `HandlerCore` publication keyed by checked expression identity.
- DEC-0260 §§8–10: evaluator/bytecode/VM rejection, Graph identity traversal,
  Audit Source 0.2 round-trip, contextual-source compatibility, deterministic
  canonical bytes, original UTF-8 spans, and Unicode 17.0.0 preservation.

## Implementation

- `ling-hir` assigns a real `BindingId` to the optional resume name while
  retaining the already accepted clause/body spans and source order.
- `ling-resolve` owns the fixed DEC-0260 registry, creates fresh lexical clause
  scopes, resolves parameter/resume references, records exact resume-use
  counts, and rejects unknown operations, arity mismatch, duplicate handled
  labels, and invalid continuation use with `L-EFFECT-0005`.
- `ling-types` checks operation input patterns, gives resume the type
  `Output -> R`, unifies every clause body with `R`, returns `R` for the handler,
  and traverses handler bodies for existing Trait obligations.
- `ling-effects` records handled-label sets on call edges, computes residual
  rows through the transitive fixed point, separately computes unmasked rows
  for Capability authorization, keeps `State<T>` visible, and publishes
  deterministic checked `HandlerCore` values.
- `ling-semantic` hashes and traverses checked Handler bodies, clauses,
  parameter/resume bindings, and references through existing graph node kinds,
  then projects checked Core evidence into `AuditHandler` records.
- `ling-format` adds the isolated canonical `ling.audit/0.2` Handler grammar,
  validator, renderer, and parser. Handler-free models retain exact 0.1 output;
  evaluator and bytecode/VM lowering remain rejecting under EFF-2104.

## Executable evidence

`crates/ling-effects/tests/checked_handlers.rs` covers:

- source-to-Core compilation and repeated canonical-byte equality;
- direct and transitive `Console.Write` subtraction;
- Capability requirements after Effect masking;
- operation lookup, arity, and `Once` versus `Many` resume use;
- clause result typing and clause-body Effect propagation;
- nested handler composition and two Core publications;
- visible `State<Int>` residuals and source-path exclusion from canonical bytes.

Resolver, Semantic Graph, Audit, and CLI boundary tests additionally prove
lexical resume identities, no checked publication on invalid contracts,
Handler graph/reference integrity, 0.2 canonical round-trip, registered
bilingual diagnostics, and original spans.

Focused commands executed successfully during implementation:

```text
cargo check -p ling-hir -p ling-resolve -p ling-types --all-targets --locked --offline
cargo check -p ling-effects --all-targets --locked --offline
cargo check -p ling-cli -p ling-semantic -p ling-db -p ling-eval --all-targets --locked --offline
cargo test -p ling-resolve --lib --locked --offline --quiet
cargo test -p ling-types -p ling-effects --all-targets --locked --offline --quiet
cargo test -p ling-effects --test checked_handlers --locked --offline --quiet
cargo test -p ling-cli --test handler_boundary --locked --offline --quiet
cargo test -p ling-semantic checked_handler_publishes_graph_identity_and_audit_evidence --locked --offline --quiet
cargo test -p ling-format handler_audit_revision_round_trips_eliminated_and_residual_rows --locked --offline --quiet
cargo test -p ling-eval checked_handler_execution_remains_a_structured_eff_2104_boundary --locked --offline --quiet
cargo test -p ling-bytecode every_bytecode_revision_rejects_checked_handlers_atomically --locked --offline --quiet
```

Repository-wide gates executed successfully on 2026-08-24:

```text
cargo test --workspace --all-targets --locked --offline --quiet
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo xtask ci verify
cargo xtask governance check-all
cargo xtask lsp verify
cargo xtask support verify
cargo xtask status verify
cargo xtask rc0 verify
cargo xtask traceability verify --release v0.0.1
cargo fmt --all -- --check
git diff --check
manual SHA-256 verification of docs/ling_execution_plan/SHA256SUMS.txt
```

## Compatibility and determinism

- **Source:** activates only DEC-0064's contextual handler form and the exact
  three-operation registry; ordinary identifiers and non-handler Seed input are
  unchanged.
- **Diagnostics:** adds `L-EFFECT-0005`; the provisional, never-emitted
  `L-EFFECT-0006` allocation is retained as retired compatibility evidence.
  Existing emitted codes, Facts, severities, spans, and meanings are unchanged.
- **Effects/Capabilities:** successful handler expressions expose checked
  residual rows in-process; host authorization remains derived from unmasked
  reachable uses.
- **Schema/identity:** adds `ling.audit/0.2` only for Handler-bearing models and
  Handler-aware semantic body identity; no public Semantic Graph field,
  Definition ID, or canonical non-handler graph/Audit/program byte changes.
- **Runtime/ABI:** no evaluator behavior, continuation, opcode, VM stack,
  bytecode format, ABI, Task, Actor, Fault, or cancellation behavior is added.
- **Determinism/Unicode:** operation order, reference counts, rows, and Core
  bytes use ordered checked facts; source spans remain original UTF-8 bytes and
  normalization remains Unicode 17.0.0.

## Intentionally deferred

User-declared or polymorphic operations, `Never` source operations,
proof-carrying State masking, dynamic handlers, continuation capture/storage,
runtime resume counts, Fault/cancellation and mutable-State execution,
Task/Actor crossing, interpreter/VM execution and differential behavior,
package-aware Handler Audit, migrations, and Stable compatibility remain
outside EFF-2103.
