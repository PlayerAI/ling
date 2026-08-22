# NIR-3401-OBSERVATION Authority Audit

## Outcome

The bounded child `NIR-3401-OBSERVATION` is authorized by Accepted
`DEC-0129`. It records only a test-local inventory of proposed backend-neutral
Native IR design boundaries. Public `NIR-3401` remains `BlockedSpec`: no IR,
instruction set, ABI, serializer, verifier, debug schema, diagnostic, or
lowering behavior is defined.

## Normative traceability

- The G3 execution package is non-normative and explicitly depends on
  RFC-N304; its checklist cannot define a public IR, binary format, ABI, or
  semantic lowering rule.
- Accepted `DEC-0128` through `DEC-0125` record only Profile/interop/
  collector/object-model vocabulary. `DEC-0012` governs Seed Semantic ID and
  canonical bytes, not a Native IR schema.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and
  `GAP-CRITICAL-PROFILE-001` remain Open. RFC-N304 and its dependent Native,
  memory, ownership, and Profile authorities are not Accepted.
- `DEC-0129` authorizes this child only.

## Current implementation boundary

`native_ir_design_evidence.rs` defines forty-six test-local boundaries, sorts
them by explicit local rank, rejects duplicates, and compares forward/reverse
insertion order. Its evidence tag is opaque and test-only; it is not an IR
node, instruction, ABI record, serializer, verifier, debug location,
diagnostic, public protocol, or lowering contract.

No Native IR crate, instruction set, ABI record, serializer, verifier, debug
schema, diagnostic, or placeholder backend API was added. Accepted Seed
behavior remains unchanged.

## Evidence and deferred work

Focused tests cover the complete boundary inventory, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent remains
blocked until accepted authority defines the NIR version and instruction set,
SSA/phi validity, representations and ownership facts, cleanup/Fault/Effect
edges, ABI/layout/targets, FFI/runtime operations, source/debug mapping,
serialization/versioning, verifier, migration, security, and differential
semantics.
