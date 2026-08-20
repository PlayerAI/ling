# VM-1202 deterministic bytecode lowering implementation report

> Status: **Done**
>
> Completed: 2026-08-21
>
> Final verified implementation commit: `4fb3f2dc0046cfaa52da6b6db94573044d5ee183`
> Verified baseline: `main@4fb3f2dc0046cfaa52da6b6db94573044d5ee183`

## Outcome

VM-1202 implements the first checked vertical slice from `ProgramSnapshot` to a canonical `LoweredProgramV1`, deterministic `ling.bytecode/1.0` bytes, and a stable review-oriented debug disassembly. The supported slice is exactly `Unit`, `Bool`, arbitrary-precision `Int`, `Text`, monomorphic direct calls, immutable local bindings as register aliases, `Console.write`, and return.

The boundary remains non-executable. `LoweredProgramV1` exposes only the unverified model; there is no decoder, verifier-created `VerifiedProgramV1`, VM, or CLI backend. Unsupported checked constructs fail atomically with a structured error and the original UTF-8 source span instead of being silently reinterpreted.

## Normative clauses covered

- Accepted [`RFC-0014`](../RFC-0014.md) §1: lowering consumes a completed checked snapshot and does not execute or reinterpret unchecked AST.
- RFC-0014 §2–§3: exact 40-byte header, little-endian scalar framing, sorted tables, canonical scalar constants, path-free source metadata, and exact source SHA-256/length.
- RFC-0014 §4–§5: one-block SSA register functions, immutable local aliases, `Const`, direct `Call`, `ConsoleWrite`, and `Return` with explicit tags and record framing.
- RFC-0014 §6: left-to-right argument lowering, checked Effect/Capability projection, complete executable-location source maps, and original half-open UTF-8 byte ranges.
- RFC-0014 §7–§8: hard/caller artifact limits, deterministic output, zero reserved bytes, no host paths or Rust layout/debug text, and no verifier bypass.
- [`03-G1-V0.1-LIVING.md`](../ling_execution_plan/03-G1-V0.1-LIVING.md) VM-1202: all four scalar types, function call, local binding, `Console.Write`, return, debug disassembly, and a byte-exact Hello golden.

## Implementation and tests

- [`lower.rs`](../../crates/ling-bytecode/src/lower.rs) builds canonical tables and functions from checked resolver/type/Effect data, rejects unsupported features and invalid source identities, enforces RFC limits, and retains original source-map provenance.
- [`encode.rs`](../../crates/ling-bytecode/src/encode.rs) writes the exact version-1.0 envelope and records through a checked bounded writer; the public writer accepts only `LoweredProgramV1`.
- [`disassemble.rs`](../../crates/ling-bytecode/src/disassemble.rs) renders a deterministic non-contract view using explicit names and tags rather than Rust `Debug` output.
- [`lowering.rs`](../../crates/ling-bytecode/tests/lowering.rs) covers exact Hello bytes/disassembly, physical-path independence, direct calls and local aliases, all scalar types and integers beyond 128 bits, unsupported-feature spans, canonical logical paths, source identity, caller limits, and exact BOM/CRLF/Unicode/emoji provenance.
- [`hello.lbc.hex`](../../tests/bytecode/v1/golden/hello.lbc.hex) and [`hello.dis`](../../tests/bytecode/v1/golden/hello.dis) freeze the first reviewed writer output. Repository attributes fix the reviewed text corpus to LF; a separate test exercises original BOM/CRLF input.

Validation completed on Windows against the implementation commit:

- `cargo test -p ling-bytecode --all-features --locked --offline` — 15 passed; one verifier-gated VM differential test remains explicitly ignored until VM-1204.
- `cargo test --workspace --all-features --locked --offline` — all unit, integration, conformance, governance, and documentation tests passed.
- `cargo fmt --all -- --check`, full workspace Clippy with warnings denied, pinned Rust 1.85 workspace check, workspace documentation, and release build passed.
- Governance reported 46 documents, 26 gaps, 21 lifecycle records, 20 protocols, and 76 diagnostic codes; support, traceability, schema, CI-contract, status, and Seed-reproduction gates passed.
- The execution-plan checksum manifest verified all 27 entries, and the completion registry verified 32 tasks as Done after this transition.

## Specification gaps or conflicts

No unresolved semantic conflict was encountered. RFC-0014 already fixed the model and instruction behavior but left the byte width of Effect/Capability tags and block-vector element framing implicit. The RFC now states those widths and framing explicitly before any bytecode artifact is published; therefore there is no prior byte stream or compatibility promise to migrate.

The version-1.0 type table has no type-variable or generic-instantiation representation, and no Accepted monomorphization contract exists. VM-1202 therefore demonstrates a monomorphic direct call and rejects polymorphic functions explicitly rather than inventing backend semantics.

## Compatibility, dependencies, determinism, and Unicode impact

- `PROTO-BYTECODE` remains planned-public and Future; `BACKEND-VM` remains unavailable. The exact golden is implementation evidence, not an executable trust claim or public CLI artifact surface.
- No `L-BYTECODE-*` code or public emitter is added. Lowering and encoding expose typed Rust errors only; VM-1203 must register bilingual diagnostics before exposing malformed-byte input failures.
- Source language behavior, Diagnostic/Semantic/Audit schemas, CLI commands and exits, Semantic IDs, ABI/FFI, and evaluator behavior are unchanged.
- `ling-bytecode` now directly uses existing locked compiler crates plus `num-bigint`, `sha2`, and `unicode-normalization`. No third-party package version changed; purposes, licenses, maintenance, and feature scope are recorded in `docs/DEPENDENCIES.md`.
- Canonical ordering uses ordered collections and explicit byte keys. Tests prove independence from physical checkout roots, and the writer emits no timestamp, filesystem metadata, host endianness, capability handle, address, or Rust debug text.
- Unicode remains 17.0.0. Logical names enforce the accepted NFC relative-path rules, while source hashes, lengths, and maps preserve exact original UTF-8 bytes including BOM, CRLF, Chinese text, and emoji.

## Intentionally deferred work

- VM-1203: independent bounded decoder/verifier, `VerifiedProgramV1`, registered bilingual `L-BYTECODE-*` diagnostics, valid/corrupt binary fixtures, round-trip evidence, and arbitrary-byte fuzzing.
- VM-1204: verifier-gated execution, explicit host capabilities and limits, Runtime Fault mapping, and interpreter/VM differential evidence.
- Later VM slices: generic monomorphization policy, recursion/closures, branches and scalar operators/intrinsics, aggregates/match, mutation/borrow, full Effect/Fault coverage, and broader resource fuzzing.
