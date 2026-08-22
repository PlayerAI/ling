# FFI-3602-OBSERVATION Authority Audit

## Outcome

The bounded child `FFI-3602-OBSERVATION` is authorized by Accepted
`DEC-0138`. It records only a test-local inventory of proposed C ABI
interoperability boundaries. Public `FFI-3602` remains `BlockedSpec`: no C
declaration/import syntax, layout calculator, linker probe, callback runtime,
opaque handle, allocator bridge, or executable ABI boundary is added.

## Normative traceability

- The G3 execution package is non-normative; its C ABI checklist cannot define
  C widths, layout, calling convention, pointer validity, callback lifetime,
  allocator provenance, error transport, or target selection.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` describe future Typed FFI and
  Target Primitive requirements but do not authorize a Seed grammar extension
  or a host C ABI implementation.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and
  `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain Open. RFC-N305, RFC-0007,
  RFC-0011, and the `PROTO-ABI` schema are not Accepted authorities.
- `DEC-0138` authorizes this child only; earlier FFI/Native evidence decisions
  do not supply C layout, linker, or runtime semantics.

## Current implementation boundary

`ffi_c_abi_evidence.rs` defines sixty test-local boundaries, sorts them by
explicit local rank, rejects duplicates, and compares forward/reverse insertion
order. Its evidence tag is opaque and test-only; it is not C syntax, a layout
result, a calling convention, a pointer/handle proof, an ABI schema, a linker
input, a Semantic ID, a diagnostic, or a public protocol.

No C parser/importer, layout calculator, compiler or linker probe, callback
trampoline, handle runtime, allocator bridge, dependency, toolchain, diagnostic,
or public API was added. Seed compiler and VM paths remain unchanged.

## Evidence and deferred work

Focused tests cover the C ABI vocabulary, deterministic ordering, duplicate
rejection, and explicit non-authority boundaries. The parent remains blocked
until accepted authority defines C representation/layout, span/callback/handle/
allocator safety, symbol/linker, ownership/lifetime, Error/Fault, target/profile,
schema, diagnostics, sanitizer/fuzz, and cross-target behavior.
