# VM-1201 bytecode model implementation report

> Status: **Done**
>
> Completed: 2026-08-21
>
> Final verified implementation commit: `5bd49583c9160cd2067a7124bc014ebc3b4bcf95`
> Verified baseline: `main@5bd49583c9160cd2067a7124bc014ebc3b4bcf95`

## Outcome

VM-1201 introduces the dependency-free production crate `ling-bytecode` as the data-only Rust representation of Accepted `ling.bytecode/1.0`. It models the fixed versions, header facts, resource maxima, typed table indexes and digests, scalar tables, SSA blocks and registers, instruction/terminator variants, Effects, Capabilities, and source maps defined by RFC-0014.

The public boundary is deliberately named `UnverifiedProgram`. Construction only preserves data and confers no validity or execution authority. There is no encoder, decoder, verifier, lowering API, `VerifiedProgram`, or VM entry point in this milestone.

## Normative clauses covered

- Accepted [`RFC-0014`](../RFC-0014.md) §1: distinct untrusted data and typed identifier domains.
- RFC-0014 §2–§3: protocol/magic/version constants, table domains, explicit tags, source digest/length data, and hard resource maxima.
- RFC-0014 §4–§5: the version-1.0 SSA register/block, instruction, operator, intrinsic, and terminator model with explicit wire values independent of Rust layout.
- RFC-0014 §6: Effect, Capability, Runtime Fault baseline, and original UTF-8 source-map domain.
- RFC-0014 §7–§8: a model that cannot bypass the future independent verifier and an inventory that continues to report the protocol/backend as unavailable.
- [`03-G1-V0.1-LIVING.md`](../ling_execution_plan/03-G1-V0.1-LIVING.md) VM-1201: format/version, tables, encoding model, order, register model, Fault/source map, verifier guarantees, limits, and compatibility boundary are represented without serializing Rust memory layout.

## Implementation and tests

- [`format.rs`](../../crates/ling-bytecode/src/format.rs) owns typed versions, exact magic/header constants, `NO_INDEX`, Unicode 17.0.0, and the RFC hard-limit set.
- [`model.rs`](../../crates/ling-bytecode/src/model.rs) separates every table/index/digest domain and supplies explicit tag/opcode methods rather than discriminant casts or serialization-derived wire values.
- [`model.rs` tests](../../crates/ling-bytecode/tests/model.rs) freeze every version-1.0 type, operator, intrinsic, instruction, terminator, capability/effect tag, limit, and a representative unverified Hello model.
- TEST-VM-0001 supplies the checked interpreter baseline and malformed-verifier plan.
- The crate has no production dependency. Existing compiler crates are development-only dependencies used by the baseline harness.

Validation completed on Windows against the implementation commit:

- `cargo test -p ling-bytecode --locked --offline` — six pass, one VM-1204 differential test explicitly ignored.
- `cargo test --workspace --all-features --locked --offline` — all unit, integration, conformance, governance, and documentation tests pass.
- `cargo fmt --all -- --check`, full workspace Clippy with warnings denied, pinned Rust 1.85 workspace check, workspace documentation, and release build pass.
- Governance reports 46 documents, 26 gaps, 21 lifecycle records, 20 protocols, and 76 diagnostic codes; support, traceability, schema, CI-contract, status, and Seed-reproduction gates pass.
- The execution-plan checksum manifest verifies all 27 entries after the VM-1201 transition, and the completion registry verifies 31 tasks as Done.

## Specification gaps or conflicts

RFC-0014 is sufficient for this model slice. The accepted Runtime Fault category is aligned with the existing interpreter value `invalid_format`. No implementation decision expands Author Source behavior or resolves a future wire-version question beyond the RFC.

## Compatibility, determinism, and Unicode impact

- `ling.bytecode/1.0` remains planned-public and Future. No artifact bytes or reader compatibility are claimed yet.
- No `L-BYTECODE-*` diagnostic is registered or emitted because there is no reader, lowerer, or verifier in VM-1201.
- Source language behavior, existing public schemas, diagnostics, CLI behavior, Semantic IDs, ABI/FFI, and third-party dependency versions are unchanged.
- Explicit tags and typed domains avoid Rust layout and debug-output leakage. No unordered collection participates in the data model.
- Unicode remains pinned to 17.0.0; source spans remain original UTF-8 byte ranges and source records cannot carry physical paths.

## Intentionally deferred work

- VM-1202: deterministic lowering, encoding, disassembly, and Hello round-trip golden.
- VM-1203: bounded independent decoder/verifier, stable bilingual bytecode diagnostics, malformed binary fixtures, and fuzzing.
- VM-1204: immutable verifier-produced executable state, explicit host limits/capabilities, and differential VM execution.
- VM-1205 onward: recursion/closures, aggregates/match, mutation, and the remaining Seed surface under their assigned tasks and any required accepted protocol revision.
