# VM-1205 first-class functions, closures, and recursion implementation report

> Status: **Done**
>
> Completed: 2026-08-21
>
> Final verified implementation commit: `9a54775ae0ee48d9fb0c75ce819989a24df27ed2`
> Verified baseline: `main@dfe2df79bcee020a30e178a568c8921b04aea346`

## Outcome

VM-1205 implements Accepted RFC-0015 as the Experimental `ling.bytecode/1.1` extension. The bytecode model, encoder, decoder, independent verifier, checked-source lowerer, disassembler, and verifier-gated VM now support structural function values with latent Effect rows, captureless and captured closures, strict partial and complete application, immutable lexical capture, top-level recursion, and one verifier-proved local `Self` capture for recursive closures.

Closure construction and partial application allocate atomically against the VM heap limit. Complete direct and indirect calls use the same explicit iterative frame stack, so source recursion is bounded by the declared frame limit rather than the host call stack. The 1.1 reader accepts valid 1.0 artifacts under their original rules, while the 1.0 reader rejects 1.1 before version-specific decoding.

## Normative clauses covered

- Accepted [`RFC-0015`](../RFC-0015.md) §1: exact `ling.bytecode/1.1` boundary, backward-compatible 1.0 reader behavior, and separate 1.1 writer.
- RFC-0015 §§2–§3: structural function type records, latent Effect rows, named/closure-body function kinds, capture-first signatures, deterministic closure labels, and entry/direct-call restrictions.
- RFC-0015 §4: lexical free-binding analysis, declaration-order captures, immutable by-value capture, top-level recursion, one local `Self` capture, and rejection of mutable or mutually recursive capture graphs.
- RFC-0015 §5: `MakeClosure`, `CallClosure`, strict left-to-right arguments, partial application, complete frame entry, and atomic allocation behavior.
- RFC-0015 §§6–§7: capture/type/Effect/Capability verification, indirect call closure, source-map coverage, iterative frames, frame/heap/step Fault behavior, and host-independent determinism.
- Shared RFC-0014 trust-boundary and source-span rules remain in force: only verifier-created state reaches `ling-vm`, and original UTF-8 byte spans are preserved.
- `03-G1-V0.1-LIVING.md` VM-1205: function/closure/recursion implementation with explicit resource Faults and no host-stack recursion.

## Implementation and tests

- [`model.rs`](../../crates/ling-bytecode/src/model.rs), [`format.rs`](../../crates/ling-bytecode/src/format.rs), [`encode.rs`](../../crates/ling-bytecode/src/encode.rs), [`decode.rs`](../../crates/ling-bytecode/src/decode.rs), and [`disassemble.rs`](../../crates/ling-bytecode/src/disassemble.rs) define and serialize the 1.1 model and protocol boundary.
- [`verify.rs`](../../crates/ling-bytecode/src/verify.rs) independently validates topological function types, function shape, captures, `Self`, direct/indirect signatures, Effect closure, Capability authorization, and complete source maps.
- [`lower/v1_1.rs`](../../crates/ling-bytecode/src/lower/v1_1.rs) lowers checked Typed Core deterministically, resolves lexical captures, emits direct/indirect/partial calls, and rejects unsupported mutable, polymorphic, aggregate, and mutually recursive forms without publishing partial bytecode.
- [`ling-effects/src/lib.rs`](../../crates/ling-effects/src/lib.rs) now preserves latent callable provenance through aliases, partial applications, returned closures, and higher-order calls so bytecode Effect metadata is sound.
- [`execute.rs`](../../crates/ling-vm/src/execute.rs) and [`value.rs`](../../crates/ling-vm/src/value.rs) implement closure storage, self materialization, partial application, atomic heap charging, explicit frames, and verifier-gated execution.
- [`decode_verify.rs`](../../crates/ling-bytecode/tests/decode_verify.rs) covers 1.0/1.1 compatibility and bounded malformed closure metadata. [`lowering.rs`](../../crates/ling-bytecode/tests/lowering.rs) covers capture, partial/exact application, returned closures, higher-order parameters, top-level recursion, local self recursion, determinism, and explicit unsupported-feature rejection. [`execution.rs`](../../crates/ling-vm/tests/execution.rs) covers interpreter/VM differentials, frame limits, and atomic closure/partial-allocation heap failures.

Validation executed on Windows with locked offline dependencies:

- `cargo test --workspace --locked --offline` — passed; all workspace tests, including 92 xtask tests, passed.
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings` — passed.
- `cargo fmt --all -- --check` and `git diff --check` — passed.
- `cargo xtask governance check-all` — passed: 47 documents, 26 gaps, 22 lifecycle records, 20 protocols, and 82 diagnostic codes.
- `cargo xtask support verify` — passed: 7 features, 3 profiles, 3 hosts, 1 native target, 6 backends, 1 standard package, 20 protocols, and 9 explicit unsupported records.

## Specification gaps or conflicts

No unresolved semantic conflict was encountered. RFC-0015 is Accepted and supplies the authority for this slice. Lower-authority execution-plan references to `zero`, `.zero`, or a 1.0-only bytecode boundary were not carried into implementation. No new diagnostic code was required; existing registered bilingual bytecode and Runtime Fault contracts are reused.

## Compatibility, schema, determinism, and Unicode impact

- `PROTO-BYTECODE` and the support matrix now identify `ling.bytecode/1.1`; both 1.0 and 1.1 remain public Experimental. This does not claim Stable 1.x compatibility, a CLI artifact command, a default backend, or a general N-1 release promise.
- The 1.0 wire format and exact golden bytes remain unchanged. A 1.1 reader accepts 1.0 and 1.1; a 1.0 reader rejects 1.1. No JSON schema, Semantic ID, canonical semantic-byte, CLI, ABI, or FFI contract changed.
- Function type, function, capture, instruction, source-map, and generated-label order is derived from ordered checked data and bytecode-local metadata, never host paths, hash iteration, allocation identity, or Rust debug output.
- Unicode remains pinned to 17.0.0. Closure labels are ASCII metadata; source identifiers and Fault locations retain the existing normalization/security rules and original zero-based half-open UTF-8 byte spans.

## Intentionally deferred work

- VM-1206: records, ADTs, aggregate values, and match lowering require their own accepted bytecode specification before implementation.
- VM-1207: mutable Places, Cells, and borrow-sensitive lowering remain out of scope; mutable captures are rejected explicitly.
- Mutual recursion, polymorphic function values, aggregate captures, closure equality/ordering/serialization, tail-call guarantees, common cross-backend heap accounting, CLI/backend selection, and native ABI layout remain deferred.
- VM-1208 through VM-1210 retain ownership of broader Effect/Fault surfaces, full differential corpus expansion, cancellation, fuzzing, and resource-model hardening.
