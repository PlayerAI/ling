# NIR-3402-OBSERVATION Authority Audit

## Outcome

The bounded child `NIR-3402-OBSERVATION` is authorized by Accepted
`DEC-0130`. It records only a test-local inventory of proposed Core-to-Native
IR lowering slices and preservation boundaries. Public `NIR-3402` remains
`BlockedSpec`: no lowering pass, NIR instruction use, native target, ABI
adapter, diagnostic, differential protocol, or lowering behavior is defined.

## Normative traceability

- The G3 execution package is non-normative; its slice order cannot define
  translations, evaluation order, representation, or differential equivalence.
- Accepted `DEC-0129` defines only test-local NIR design vocabulary; accepted
  `DEC-0128` through `DEC-0125` define only Profile/interop/collector/
  object-model evidence. `DEC-0012` governs Seed Semantic ID/canonical bytes,
  not Native lowering.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and Task/Actor
  gaps remain Open. RFC-N304 and dependent Native, memory, ownership, FFI,
  and Profile authorities are not Accepted.
- `DEC-0130` authorizes this child only.

## Current implementation boundary

`native_ir_lowering_evidence.rs` defines forty-six test-local boundaries, sorts
them by explicit local rank, rejects duplicates, and compares forward/reverse
insertion order. Its evidence tag is opaque and test-only; it is not a
lowering, instruction, ABI adapter, native target, diagnostic, differential
trace, public protocol, or semantic-preservation proof.

No lowering pass, NIR instruction use, native target, ABI adapter, diagnostic,
differential protocol, or placeholder crate was added. The existing accepted
Seed Typed Core and VM bytecode pipeline remains the only executable path.

## Evidence and deferred work

Focused tests cover the complete boundary inventory, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent remains
blocked until accepted authority defines mappings, representation, ownership,
cleanup, Effects/Fault/cancellation, ABI/targets/Profile/FFI/reentry,
unsupported-form diagnostics, deterministic serialization, differential
equivalence, and Native code generation.
