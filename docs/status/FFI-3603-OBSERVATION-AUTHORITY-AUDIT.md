# FFI-3603-OBSERVATION Authority Audit

## Outcome

The bounded child `FFI-3603-OBSERVATION` is authorized by Accepted
`DEC-0139`. It records only a test-local inventory of proposed FFI shim
generator boundaries. Public `FFI-3603` remains `BlockedSpec`: no generator,
template, generated source/header, layout or pointer check, ownership adapter,
callback trampoline, provenance record, build-hash input, or executable shim is
added.

## Normative traceability

- The G3 execution package is non-normative; its shim checklist cannot define
  generator inputs/outputs, generated-language ABI, trust, ownership
  conversion, failure behavior, or canonical provenance/build-hash bytes.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` describe future Typed FFI,
  provenance, and Target Primitive requirements but do not authorize generated
  shims for the Seed subset.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and
  `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain Open. RFC-N305, RFC-0007,
  RFC-0011, `PROTO-ABI`, and `PROTO-EVIDENCE` are not Accepted authorities.
- `DEC-0139` authorizes this child only; earlier FFI/Native evidence decisions
  do not supply generator, artifact, provenance, or build-hash semantics.

## Current implementation boundary

`ffi_shim_generator_evidence.rs` defines sixty test-local boundaries, sorts them
by explicit local rank, rejects duplicates, and compares forward/reverse
insertion order. Its evidence tag is opaque and test-only; it is not generated
source, a shim, a layout assertion, a safety proof, provenance, a build-hash
input, a Semantic ID, a diagnostic, or a public protocol.

No generator, template, generated source/header, layout or pointer check,
ownership adapter, callback trampoline, Fault/Capability bridge, provenance
record, build-hash input, dependency, toolchain, diagnostic, or public API was
added. Seed compiler and VM paths remain unchanged.

## Evidence and deferred work

Focused tests cover the shim vocabulary, deterministic ordering, duplicate
rejection, and explicit non-authority boundaries. The parent remains blocked
until accepted authority defines shim schemas, generated checks/adapters,
trust/TCB, provenance/build hash, diagnostics, migration, sanitizer/fuzz, and
cross-target behavior.
