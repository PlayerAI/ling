# FFI-3604-OBSERVATION Authority Audit

## Outcome

The bounded child `FFI-3604-OBSERVATION` is authorized by Accepted
`DEC-0140`. It records only a test-local inventory of proposed Target Primitive
Package and `lingabi` boundaries. Public `FFI-3604` remains `BlockedSpec`: no
target package, manifest, `lingabi` reader, primitive registry, capability/TCB
checker, target selector, proof verifier, or executable primitive is added.

## Normative traceability

- The G3 execution package is non-normative; its Target Primitive checklist
  cannot define package identity, target selection, `lingabi` fields, primitive
  signatures, capability semantics, proof acceptance, TCB membership, or
  update/revocation behavior.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` describe future Target Primitive
  and trusted FFI requirements but do not authorize a target package or
  executable primitive for the Seed subset.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and
  `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain Open. RFC-N305, RFC-0007,
  RFC-0011, `PROTO-ABI`, and `PROTO-EVIDENCE` are not Accepted authorities.
- `DEC-0140` authorizes this child only; earlier FFI/Native evidence decisions
  do not supply package, trust, capability, or TCB semantics.

## Current implementation boundary

`target_primitive_package_evidence.rs` defines sixty test-local boundaries,
sorts them by explicit local rank, rejects duplicates, and compares
forward/reverse insertion order. Its evidence tag is opaque and test-only; it
is not a target package, `lingabi` schema, primitive signature, capability
grant, trust/TCB decision, proof, artifact, Semantic ID, diagnostic, or public
protocol.

No target directory, package manifest, `lingabi` reader, primitive registry,
capability/TCB checker, target selector, proof verifier, build integration,
dependency, toolchain, diagnostic, or public API was added. Seed compiler and
VM paths remain unchanged.

## Evidence and deferred work

Focused tests cover the Target Primitive vocabulary, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent remains
blocked until accepted authority defines package/`lingabi` schemas, discovery/
locking, target/profile selection, primitive lowering, capability/TCB admission,
proof/test verification, provenance/revocation, diagnostics, migration,
sanitizer/fuzz, and cross-target behavior.
