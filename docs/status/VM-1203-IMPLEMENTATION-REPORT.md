# VM-1203 independent bytecode decoder and verifier implementation report

> Status: **Done**
>
> Completed: 2026-08-21
>
> Final verified implementation commit: `e08940ef511cbcb1416e4b32e0c0805601d5c160`
> Verified baseline: `main@e08940ef511cbcb1416e4b32e0c0805601d5c160`

## Outcome

VM-1203 implements a bounded, writer-independent `ling.bytecode/1.0` decoder and a failure-atomic verifier whose successful result is the only public construction path for `VerifiedProgramV1`. The decoder treats every artifact byte as untrusted, applies RFC hard limits and smaller caller limits before slicing or allocation, and retains byte offsets needed for deterministic verification diagnostics.

The verifier checks canonical tables, references and signatures, control flow, reachability, dominance, single assignment, exact register types, transitive Effects, module Capabilities, the `Main.main (Unit) -> Unit` entry contract, and complete source-owned source maps. It publishes no executable state on failure. This milestone deliberately adds no VM, CLI backend, bytecode command, or source-language behavior.

## Normative clauses covered

- Accepted [`RFC-0014`](../RFC-0014.md) §§1–3: untrusted artifact boundary, exact format/language/Unicode envelope, bounded length-framed decoding, canonical tables, names, constants, and source identities.
- RFC-0014 §§4–5: exact function, block, register, instruction, intrinsic, operator, terminator, operand, tag, opcode, and reserved-field decoding.
- RFC-0014 §6: Effect/Capability authorization facts and complete original UTF-8 source-map provenance without executing the artifact.
- RFC-0014 §7: ordered failure-atomic verification, hard and caller resource limits, CFG/reachability/dominance/SSA/type/signature checks, transitive Effect closure, Capability and entry validation, and complete source-map coverage.
- RFC-0014 §8: exact version rejection, deterministic first-error identity, bounded bilingual bytecode diagnostics, canonical verified re-encoding, and exclusion of Rust layout, debug text, physical paths, and host state.
- [`03-G1-V0.1-LIVING.md`](../ling_execution_plan/03-G1-V0.1-LIVING.md) VM-1203: independent verifier checks, malformed-input coverage, valid/corrupt fuzz seeds, bounded behavior, and no panic for arbitrary bytes.

## Implementation and tests

- [`decode.rs`](../../crates/ling-bytecode/src/decode.rs) owns the untrusted byte reader, exact envelope and record framing, checked count arithmetic, pre-allocation hard/caller limits, explicit tag/opcode decoding, UTF-8 rejection, and byte-offset provenance.
- [`verify.rs`](../../crates/ling-bytecode/src/verify.rs) is independent of lowering and encoding and is the sole constructor of immutable `VerifiedProgramV1`.
- [`error.rs`](../../crates/ling-bytecode/src/error.rs) supplies stable phase/reason/index/resource facts for six registered bilingual `L-BYTECODE-*` diagnostics.
- [`path.rs`](../../crates/ling-bytecode/src/path.rs) centralizes canonical logical-path validation shared by lowering and verification without exposing physical paths.
- [`decode_verify.rs`](../../crates/ling-bytecode/tests/decode_verify.rs) proves Hello decode/verify/re-encode byte equality, independently built branch and cyclic CFG artifacts, direct calls, transitive Effects, caller limits, invalid names/types/constants/source references, and all 22 registered malformed vectors with exact stable reason tags.
- The deterministic arbitrary-byte suite exercises 512 inputs twice and requires equal bounded results without panic. [`bytecode_bytes.rs`](../../fuzz/fuzz_targets/bytecode_bytes.rs) repeats that invariant under libFuzzer with reviewed valid and corrupt seeds; pinned Linux CI replays the corpus.

Validation completed on Windows against the implementation commit:

- `cargo test -p ling-bytecode --all-targets --all-features --locked --offline` — 23 passed; one VM-1204 differential test remains explicitly ignored.
- `cargo check --manifest-path fuzz/Cargo.toml --bins --locked --offline` — passed for the fuzz target and locked standalone fuzz graph.
- `cargo test --workspace --all-features --locked --offline` — all workspace unit, integration, conformance, governance, and documentation tests passed.
- `cargo fmt --all -- --check`, full workspace Clippy with warnings denied, pinned Rust 1.85 workspace check, workspace documentation, and release build passed offline.
- Governance reported 46 documents, 26 gaps, 21 lifecycle records, 20 protocols, 81 active plus one retired diagnostic code, and 15 diagnostic domains; support, traceability, schema, CI-contract, status, and Seed-reproduction gates passed.

The pinned nightly libFuzzer target built locally, but Windows could not start the ASan-instrumented executable because the Visual Studio ASan runtime DLL is unavailable on this host. This is recorded rather than represented as a passing local fuzz run. The deterministic arbitrary-byte suite is executable on the current host, and the Linux CI job owns the sanitizer-backed corpus replay.

## Specification gaps or conflicts

No unresolved semantic conflict was encountered. Accepted RFC-0014 fixes every verification phase, resource maximum, evaluation metadata invariant, entry requirement, source-map rule, and diagnostic boundary needed by this slice.

`PROTO-BYTECODE` remains planned-public and Future despite the implemented reader, writer, and verifier. RFC-0014 explicitly requires VM-1204 execution and differential evidence before the protocol/backend can be reported as implemented. Registering artifacts as public protocol fixtures before that transition would overstate support and violate the protocol inventory gate.

## Compatibility, dependencies, determinism, and Unicode impact

- Six diagnostic identities are added: `L-BYTECODE-0001` through `L-BYTECODE-0006`. Their bilingual titles and typed fact contracts are registered centrally; existing code meanings and `ling.diagnostic/0.1` framing are unchanged.
- No public schema version, CLI command or exit behavior, source-language rule, Semantic ID, canonical semantic byte, ABI/FFI contract, evaluator behavior, or third-party package version changed.
- `ling-bytecode` adds direct internal dependencies on `ling-diagnostics` and `ling-unicode`; all normal builds and tests remain locked and offline.
- Ordered verification phases, explicit byte offsets, stable reason tags, typed index tuples, canonical ordered collections, and repeat-result tests exclude hash-map order, timing, addresses, allocation layout, physical paths, and Rust debug output from observable results.
- Unicode remains 17.0.0. Package/module/function identifiers use the accepted NFC/XID/security rules, Text is not normalized, and source maps retain original zero-based half-open UTF-8 byte spans.

## Intentionally deferred work

- VM-1204: execute only verifier-produced state; preflight host Capabilities; enforce deterministic step/frame hooks and the heap safety boundary; map Runtime Faults through verified source spans; enable interpreter/VM differential evidence.
- VM-1205 onward: recursion/closures, aggregates/match, mutation/borrow, and later versioned bytecode extensions only after their governing accepted specifications exist.
- CLI bytecode input/output, backend selection, default runtime limits, cache/artifact identity, signing, compression, JIT/debugger records, and public artifact distribution remain outside RFC-0014 or require separately accepted owners.
