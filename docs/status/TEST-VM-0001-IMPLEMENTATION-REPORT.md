# TEST-VM-0001 failing-first corpus implementation report

> Status: **Done**
>
> Completed: 2026-08-21
>
> Final verified implementation commit: `5bd49583c9160cd2067a7124bc014ebc3b4bcf95`
> Verified baseline: `main@5bd49583c9160cd2067a7124bc014ebc3b4bcf95`

## Outcome

TEST-VM-0001 establishes the first VM slice's observable interpreter baseline before any VM is executable. Every source fixture passes through the existing Source, CST, AST, HIR, resolution, type, Effect/Capability, and semantic-snapshot pipeline. The harness never hands an unchecked AST to a backend.

The corpus freezes exact UTF-8 Console output, scalar values including an integer larger than 128 bits, direct function calls and local bindings, and the current structured `Text.format` Runtime Fault. Twenty-two named malformed scenarios define the decoder/verifier test backlog. The interpreter-to-VM differential test is present but explicitly ignored with a VM-1204 reason; it cannot be mistaken for passing VM evidence.

## Normative clauses covered

- Accepted [`RFC-0014`](../RFC-0014.md) trust boundary and conformance plan: checked input, scalar values, direct calls, Console behavior, Runtime Fault parity, original source spans, malformed artifacts, and deferred differential execution.
- Accepted DEC-0002: the fault assertion uses the original zero-based half-open UTF-8 byte span.
- Accepted DEC-0010, DEC-0011, and DEC-0013: Console Effect/Capability behavior, scalar built-ins, `Main.main`, and `L-RUNTIME-0001` remain the interpreter authority.
- [`14-FIRST-SPRINT-CODEX-TASKS.md`](../ling_execution_plan/14-FIRST-SPRINT-CODEX-TASKS.md) TEST-VM-0001: the required constants, binding/call, Console, return, Fault, checked-input, skip-reason, and malformed-case evidence is present.

## Evidence

- [`differential_baseline.rs`](../../crates/ling-bytecode/tests/differential_baseline.rs) compiles every program through the checked pipeline and freezes interpreter results.
- [`tests/bytecode`](../../tests/bytecode/README.md) documents the trust boundary and contains four source programs plus 22 stable malformed scenarios.
- The `Text.format` case asserts `L-RUNTIME-0001`, category `invalid_format`, the logical source name, no committed Console output, and the exact original expression byte span.
- `cargo test -p ling-bytecode --locked --offline` passes six tests; the single VM differential test is intentionally ignored until VM-1204.
- Full workspace tests, Clippy with warnings denied, Rust 1.85 checking, documentation, release build, and governance/schema/support/reproducibility gates pass.

## Specification gaps or conflicts

No unresolved semantic conflict was found. RFC-0014 defines the required slice and the existing interpreter supplies the observable reference. The implementation uses the existing diagnostic category `invalid_format`; it does not invent the earlier draft spelling `invalid_format_placeholder_count`.

## Compatibility, determinism, and Unicode impact

- No source syntax, type, Effect, Capability, evaluation, CLI, schema, diagnostic code, Semantic ID, ABI, or dependency version changed.
- Fixtures contain no physical checkout path or host-dependent expected value.
- Unicode remains 17.0.0. The Chinese output and source-span assertions operate on exact original UTF-8 bytes.

## Intentionally deferred work

- VM-1202: deterministic Checked Core lowering, byte encoding, and disassembly goldens.
- VM-1203: independent decoding, verification, corrupt binary fixtures, and fuzzing.
- VM-1204: verifier-gated execution and enabled interpreter/VM differential evidence.
