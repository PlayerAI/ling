# TASK-2201 Implementation Report

## Outcome

TASK-2201 is complete under Accepted DEC-0264. Implementation commit
`54e4cecb6ad56685f93b155ef7395a1b0a7a7e26` adds the checked-only Structured
Task frontend and deliberately stops before state-machine lowering or runtime
execution.

## Normative clauses covered

- DEC-0264 clauses 1–2: contextual `task`, `scope`, `spawn`, `await`, `return`,
  and adjacent `let!` syntax lower through CST, AST, HIR, resolution, and type
  checking; spawn targets are direct resolved Task declarations and recursive
  spawn chains are rejected.
- Clauses 3–5: Task handles are non-first-class, lexically scope-owned, and
  path-checked for exact observation; explicit and fused observations publish
  stable scope, task, suspension, cancellation, and cleanup identities.
- Clauses 6–8: latent Task Effect rows contain `Task.Spawn` and `Task.Await`;
  suspension live sets reject mutable bindings, Handler continuations, and
  other live handles; immutable `CheckedTaskCore` is owned by `CheckedProgram`
  and retains original UTF-8 spans plus the DEC-0091 projection.
- Clause 9: the optional Experimental `x-ling-task` Semantic Graph extension
  is version 0.1, and checked Task evidence selects canonical Audit Source
  `ling.audit/0.3` without changing non-Task 0.1 or Handler-only 0.2 bytes.
- Clause 10: file/project run and test, REPL submission, project artifacts,
  interpreter entry, and bytecode 1.0–1.4 lowering stop with bilingual
  `L-TASK-0004` before evaluation or artifact publication.

## Evidence

- `tests/snapshots/structured-task/` freezes contextual tokens, CST, AST,
  direct spawn/await, fused `let!`, and nested scope spans.
- `crates/ling-types` covers direct/non-first-class targets and Task call
  arity/type failures.
- `crates/ling-effects/tests/structured_task_frontend.rs` covers Checked Task
  Core publication, exact observation, conditional paths, missing/non-final
  return, recursive spawn chains, unsafe suspension live sets, structural
  Effects, path independence, alpha-renaming, and BOM/CRLF/Chinese byte spans.
- `crates/ling-semantic` and `crates/ling-format` validate deterministic Task
  graph/Audit projections, including mutually exclusive suspension sites for
  one spawn and isolated Audit 0.3 round trips.
- `crates/ling-cli/tests/task_boundary.rs`, `test.rs`, and
  `project_commands.rs` cover checked publication plus file, test-runner, REPL,
  project run/test/build rejection without output or artifacts.
- `crates/ling-bytecode/tests/lowering.rs` covers every existing bytecode
  revision; direct interpreter evidence verifies rejection before host output.

Executed gates:

- `cargo fmt --all -- --check`
- `cargo check --workspace --all-targets --message-format short -q`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --all-targets --no-fail-fast`
- `cargo test -p xtask --bin xtask` (174 passed)
- `cargo xtask governance check-all`
- `cargo xtask governance check-error-codes`
- `cargo xtask governance check-protocols`
- `cargo xtask support verify`

## Specification gaps and conflicts

No conflict was resolved through implementation. DEC-0264 permits multiple
syntactic suspension sites for a single handle on mutually exclusive paths;
Audit validation therefore records all checked suspension identities and
requires every spawn to have at least one statically checked observation.
Path-complete exact-once enforcement remains in the checker rather than being
inferred by the protocol reader.

`GAP-STRUCTURED-TASK-001` remains open for TASK-2202 through TASK-2206. It no
longer blocks the completed source-to-checked slice, but still blocks executable
state machines, lifecycle cancellation/cleanup, Fault aggregation, detach,
schedulers, resource limits, and runtime conformance.

## Compatibility and determinism impact

- Diagnostics: adds Preview `L-TASK-0001` through `L-TASK-0004`; the generated
  compatibility lock and registry-count gates were updated.
- Schemas/protocols: adds optional Experimental Semantic Graph extension
  `x-ling-task` version 0.1 and Preview Audit Source `ling.audit/0.3`. Existing
  non-Task Semantic Graph, Audit 0.1/0.2, and bytecode 1.0–1.4 bytes remain
  unchanged.
- Semantic IDs: Task identities use canonical lexical owners and source-order
  ordinals; physical paths, source evidence, allocation order, and map order
  are excluded. Local alpha-renaming preserves checked Task bytes; Task
  declaration renaming changes its semantic identity.
- Unicode: original UTF-8 byte spans remain authoritative and Unicode stays
  pinned to 17.0.0.

## Intentionally deferred

TASK-2202 through TASK-2206 retain all state-machine, runtime join/cancel/
cleanup, child Fault, detach, scheduler, stress, resource, bytecode/VM Task
instruction, and production execution work. No placeholder executable API,
scheduler ordering promise, ABI, replay format, or Stable compatibility claim
was added.
