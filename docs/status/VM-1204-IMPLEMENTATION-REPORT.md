# VM-1204 verifier-gated base VM implementation report

> Status: **Done**
>
> Completed: 2026-08-21
>
> Final verified implementation commit: `dfe2df79bcee020a30e178a568c8921b04aea346`
> Verified baseline: `main@dfe2df79bcee020a30e178a568c8921b04aea346`

## Outcome

VM-1204 adds the separate `ling-vm` crate and the first executable implementation of Accepted `ling.bytecode/1.0`. Its public entry point accepts only verifier-created `VerifiedProgramV1`, explicit step/frame/heap limits, and explicitly injected host Capabilities. It has no parser, resolver, checker, filesystem, environment, ambient console, bytecode bypass, or CLI integration.

The VM executes every version-1.0 scalar opcode and terminator in stored order using explicit `Unit`, `Bool`, arbitrary-precision `Int`, and `Text` values plus an iterative frame stack. Capability preflight occurs before the entry instruction. Steps are charged before each instruction and terminator; frame exhaustion is checked before a push; the implementation heap ceiling accounts for live dynamic value payloads. Runtime Faults retain the verified original `u64` UTF-8 byte span and whether an earlier/current host Effect may already be observable.

## Normative clauses covered

- Accepted [`RFC-0014`](../RFC-0014.md) §1: only `VerifiedProgramV1` crosses the executable trust boundary, with explicit capabilities and limits.
- RFC-0014 §§4–§5: version-1.0 SSA registers, stored block order, `Const`, both integer unary operations, all integer binary and comparison operations, all accepted scalar intrinsics, direct `Call`, `ConsoleWrite`, `Jump`, `Branch`, and `Return`.
- RFC-0014 §6: strict observable ordering, preflight against the verifier-proved transitive Effect closure, one logical LF-terminated Console operation, structured host failures, preserved prior Effects, deterministic limit Faults, and complete verified source-map lookup.
- RFC-0014 §7: explicit step, frame, and implementation heap-ceiling hooks without CLI defaults; iterative frames prevent Rust call-stack exhaustion for recursive direct-call artifacts.
- RFC-0014 §8: exact Experimental `ling.bytecode/1.0` boundary, no Rust layout/debug/allocator/physical-path leakage, unchanged `ling.diagnostic/0.1` framing, and no claim that bytecode is DEC-0012 canonical semantic identity.
- [`03-G1-V0.1-LIVING.md`](../ling_execution_plan/03-G1-V0.1-LIVING.md) VM-1204: verifier-gated execution, explicit value representation, deterministic left-to-right behavior, host adapter, execution budgets, and source-span Fault mapping.

## Implementation and tests

- [`execute.rs`](../../crates/ling-vm/src/execute.rs) owns the iterative engine, ordered opcode/terminator dispatch, checked register/frame access, explicit resource hooks, preflight, and source-map Fault construction.
- [`value.rs`](../../crates/ling-vm/src/value.rs) implements the private scalar value representation and reference-counted live-payload accounting without exposing Rust layout as language behavior.
- [`host.rs`](../../crates/ling-vm/src/host.rs) defines the injected `Console.Write` adapter and stable before/after-commit host-error categories.
- [`fault.rs`](../../crates/ling-vm/src/fault.rs) separates user-observable Runtime Faults from post-verification internal-invariant failures and renders the registered bilingual `L-RUNTIME-0001` facts without Rust `Debug` text.
- [`execution.rs`](../../crates/ling-vm/tests/execution.rs) covers all version-1.0 scalar operators, true/false branches, jump/return, direct and recursive calls, interpreter/VM Console differential, missing transitive Capability preflight, division/format Faults, step/frame/heap ceilings, original source spans, committed Effects, and host failures.
- [`wire.rs`](../../crates/ling-vm/tests/support/wire.rs) constructs independent valid scalar and recursive byte artifacts instead of granting authority through the compiler writer.
- `DiagnosticSpan` now preserves protocol offsets as `u64`; the existing compiler `Span` and `DiagnosticSpan::at(u32, u32)` paths remain available, while `at_u64` prevents RFC-0014 source-map clamping. A JSON test covers offsets beyond `u32::MAX`.

Validation completed on Windows against the implementation commit:

- `cargo test -p ling-vm --all-targets --all-features --locked --offline` — 9 passed.
- `cargo test --workspace --all-targets --all-features --locked --offline` — all workspace unit, integration, conformance, governance, and documentation tests passed; the xtask binary ran 92 tests.
- `cargo fmt --all -- --check`, full workspace Clippy with warnings denied, pinned Rust 1.85 workspace check, workspace documentation, and release build passed offline.
- Governance reported 46 documents, 26 gaps, 21 lifecycle records, 20 protocols, and 82 diagnostic codes; support, traceability, schema compatibility/corruption, CI-contract, pre-completion status, and eight-process Seed-reproduction gates passed.
- Production `ling-vm` sources contain no `panic!`, `unwrap`, `expect`, `unreachable!`, `unsafe`, TODO, or FIXME marker; `git diff --check` passed.

## Specification gaps or conflicts

No unresolved semantic conflict was encountered. Accepted RFC-0014 defines the complete VM-1204 behavior and permits `ling.bytecode/1.0` to transition from planned/Future to public Experimental only after this verifier-gated execution and differential evidence exists.

The RFC does not authorize a CLI bytecode command, a backend selector, default runtime limits, a common cross-backend logical heap model, or a full-Seed lowering contract. Those surfaces remain explicitly unsupported rather than being inferred from the library VM.

## Compatibility, dependencies, determinism, and Unicode impact

- `PROTO-BYTECODE` transitions to public Experimental version `ling.bytecode/1.0`; `BACKEND-VM` transitions to implemented Experimental Tier 2 library evidence. This is not a Stable 1.x, public CLI, artifact-distribution, or N-1 compatibility claim.
- Existing `L-RUNTIME-0001` code meaning and typed Facts are reused. Diagnostic JSON framing/schema is unchanged; the Rust `DiagnosticSpan` offset accessors widen from `u32` to `u64` so verified protocol spans are not truncated.
- No source-language rule, CLI command/exit, Semantic ID, canonical semantic byte, ABI/FFI contract, evaluator behavior, Unicode table, or third-party package version changed. `ling-vm` reuses the locked workspace `num-bigint` package.
- Stored instruction order, explicit block selection, deterministic logical step/frame counters, sorted verified source maps, injected host handles, and tests independent of physical paths/hash iteration keep observable execution deterministic within RFC-0014's stated boundary.
- Unicode remains 17.0.0. VM Text is not normalized; Runtime Faults use the verified source logical name and original zero-based half-open UTF-8 byte span.

## Intentionally deferred work

- VM-1205: accepted closure capture, recursive-frame, and broader function-lowering rules. The base engine already uses iterative frames for version-1.0 direct calls, but no closure or source-level recursion claim is added here.
- VM-1206 and VM-1207: aggregate/match and mutable-place/borrow bytecode only after accepted versioned extensions.
- VM-1208 through VM-1210: broader Effect/Capability/Fault coverage, full interpreter/VM differential corpus, cancellation, resource fuzzing, and any later common logical heap contract.
- VM-1202 lowering remains intentionally minimal; independently verified artifacts exercise scalar instructions not yet emitted from source.
- CLI bytecode input/output, backend selection, default limits, cache/artifact identity, signing, compression, JIT/debugger records, and distribution remain separate accepted-protocol tasks.
