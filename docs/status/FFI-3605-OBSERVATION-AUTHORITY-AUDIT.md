# FFI-3605-OBSERVATION Authority Audit

## Outcome

The bounded child `FFI-3605-OBSERVATION` is authorized by Accepted
`DEC-0141`. It records only a test-local inventory of proposed FFI fuzz and
sanitizer boundaries. Public `FFI-3605` remains `BlockedSpec`: no fuzz target,
sanitizer configuration, native dependency, unsafe code, corpus, crash
artifact, or security result is added.

## Normative traceability

- The G3 execution package is non-normative; its fuzz/sanitizer checklist cannot
  define harness inputs, mutation policy, safety oracle, sanitizer versions,
  coverage thresholds, crash triage, or cross-target security claims.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` describe future verified FFI,
  target, provenance, and security requirements but do not authorize native fuzz
  execution for the Seed subset.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and
  `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain Open. RFC-N305, RFC-0007,
  RFC-0011, `PROTO-ABI`, and `PROTO-EVIDENCE` are not Accepted authorities.
- `DEC-0141` authorizes this child only; earlier FFI/Native evidence decisions
  do not supply fuzz, sanitizer, or security semantics.

## Current implementation boundary

`ffi_fuzz_sanitizer_evidence.rs` defines sixty test-local boundaries, sorts them
by explicit local rank, rejects duplicates, and compares forward/reverse
insertion order. Its evidence tag is opaque and test-only; it is not a fuzz
target, corpus, sanitizer result, coverage number, security claim, diagnostic,
provenance record, Semantic ID, or public protocol.

No fuzz target, sanitizer configuration, native dependency, unsafe code, target
toolchain, generated corpus, crash artifact, diagnostic, or public API was
added. Seed compiler and VM paths remain unchanged.

## Evidence and deferred work

Focused tests cover the fuzz/sanitizer vocabulary, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent remains
blocked until accepted authority defines harness/corpus/mutation, sanitizer and
toolchain, crash/coverage/resource bounds, security, provenance, cross-target/
compiler, diagnostics, and public protocol behavior.
