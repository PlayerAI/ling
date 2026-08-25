# EFF-2105 Implementation Report: Effect Property Generator and Differential Oracle

## Outcome

EFF-2105 is complete under Accepted RFC-0006, DEC-0262, and DEC-0263. Commit
`3517ffcccc8204a528c9768b0642aface4fcec29` adds a repository-owned,
deterministic checked-source generator, bounded shrinker, canonical residual-row
oracle, and interpreter/VM differential property suite.

The harness is test-only. It starts from UTF-8 source, re-enters every normal
compiler stage, and obtains bytecode only through the 1.4 lowerer, encoder,
decoder, and independent verifier. It cannot fabricate `CheckedProgram`, typed
identities, Handler Core, or verified bytecode and exposes no CLI, schema,
seed-replay protocol, or persistent corpus writer.

## Normative clauses covered

- DEC-0263 clauses 1–3: checked-source-only generation, the closed accepted
  scalar/Handler/State/Fault domain, fixed seeds, exact structural/source/output
  bounds, canonical logical case names, and no ambient entropy.
- DEC-0263 clauses 4–5: canonical checked definition rows versus verified
  bytecode rows, deterministic Program IDs and bytecode, and interpreter/VM
  comparison of ordered committed events, Unit completion, stable Fault
  categories/operations, committed state, and original UTF-8 spans.
- DEC-0263 clauses 6–7: deterministic ordered shrinking, a 256-attempt ceiling,
  complete checked-pipeline revalidation, same-failure projection, rejection of
  unchecked candidates, no worktree writes, and no public corpus protocol.
- RFC-0006 and DEC-0262: lexical Handler elimination, zero/one Once resume,
  propagation, nested handlers, visible `State<T>`, shared Cells, and verified
  bytecode 1.4 execution.
- DEC-0013 and RFC-0020: stable Runtime Fault projections and the existing
  deterministic cancellation/resource evidence retained by the VM suite.

## Implementation

- `crates/ling-vm/tests/support/effect_property.rs` owns eight documented
  64-bit seeds and twelve scenario families, producing 96 replayable cases.
  SplitMix64 is repository-owned and observes no clock, environment, network,
  filesystem order, allocation identity, or thread schedule.
- Generated cases cover direct calls, closures, conditionals, exhaustive
  matches, immutable and mutable bindings, Console output, zero/one resume,
  propagation, repeated-resume Fault, clause Fault, shared Handler State,
  nested handlers, BOM, CRLF, Chinese identifiers, combining marks, and emoji.
- Every case is rebuilt twice through Source → CST → AST → HIR → resolution →
  type → Effect/Capability → Semantic snapshot. The suite compares Program ID,
  exact bytecode bytes, verified models, named checked/bytecode Effect Rows,
  interpreter/VM events, completion, and Runtime Fault projection.
- Shrinking proposes declaration, sequence, Handler/body, literal/text, and
  lexical-name simplifications in deterministic size/UTF-8 order. The caller
  rechecks candidates and retains only the same failure projection.
- The residual-row oracle exposed a bytecode 1.4 defect for ordinary mutable
  lexical bindings. The 1.4 lowerer now selects all mutable lexical bindings as
  private Cells, registers their payload/Cell types, retains exact `State<T>`
  rows, and remains independently verifier-checkable. Revisions 1.0–1.3 are
  unchanged.

## Executable evidence

- `fixed_effect_property_seeds_replay_through_checked_source_and_bytecode`
  runs all 96 generated cases twice and compares checked identity, exact
  encoded bytes, verified rows, events, completion, Faults, commits, and spans.
- `deterministic_shrinking_rechecks_candidates_and_preserves_failure_projection`
  freezes shrink ordering and bounds, rejects unchecked source, rechecks every
  retained candidate, replays an intentionally divergent event oracle, and
  verifies the repository manifest bytes remain unchanged.
- `generated_shapes_enforce_every_dec_0263_bound` covers every generator bound
  and explicit one-over failures for definition and source-byte limits.
- `bytecode_1_4_retains_state_for_non_captured_mutable_bindings` proves exact
  `CellNew`/`CellSet`, `State<Int>`, encoding, and independent verification for
  the conformance defect fixed by this milestone.
- Existing VM execution tests retain cancellation-before/after-commit,
  exact resource boundaries, malformed bytecode, missing capability, host
  panic containment, resume cardinality, shared mutation, and Fault evidence.

Executed on 2026-08-25 against the implementation commit:

```text
cargo test --workspace --locked --offline
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo xtask governance check-all
cargo xtask status verify
cargo xtask docs verify
cargo xtask ci verify
cargo fmt --all --check
git diff --check
```

All commands passed. The workspace suite retained its intentionally ignored
fixture-blessing tests; no failing or newly ignored test was observed.

## Compatibility impact

- Source language, diagnostics, schemas, Semantic IDs, Program IDs, CLI, LSP,
  packages, Native/Wasm, and Unicode 17.0.0 are unchanged.
- Bytecode 1.0–1.3 output is unchanged. Bytecode 1.4 remains Experimental, but
  newly lowered ordinary mutable bindings now use existing 1.4 Cell operations
  and retain their accepted `State<T>` rows. This intentionally changes 1.4
  canonical bytes for such source while preserving Ling results and events.
- No seed, replay, shrink, corpus, or differential format is public. Failure
  text and generated logical names are internal test evidence.
- Determinism improves through fixed seeds, canonical comparisons, explicit
  limits, repeated reconstruction, and path-free logical names.

## Specification gaps and intentionally deferred work

No specification conflict was encountered. DEC-0263 authorized the property
boundary and the exact row comparison that exposed the 1.4 lowering defect.
The following remain outside EFF-2105:

- Task, Actor, Supervisor, Clock, Random, user-defined operation production,
  `Many`, Native/Wasm, packages, and unchecked or malformed source generation;
- public replay/corpus formats, coverage-guided subprocess fuzzing, automatic
  corpus writes, and Stable compatibility;
- Task/Actor crossing and later runtime semantics governed by TASK-2201 through
  TASK-2206 and their own accepted decisions.
