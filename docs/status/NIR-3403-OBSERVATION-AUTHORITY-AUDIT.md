# NIR-3403-OBSERVATION Authority Audit

## Outcome

The bounded child `NIR-3403-OBSERVATION` is authorized by Accepted
`DEC-0131`. It records only a test-local inventory of proposed Native IR
verifier boundaries. Public `NIR-3403` remains `BlockedSpec`: no verifier,
parser, validation schema, diagnostic, backend operation set, execution path,
or host-safety guarantee is defined.

## Normative traceability

- The G3 execution package is non-normative; its verifier checklist cannot
  define NIR grammar, invariants, validation order, error behavior, or safety
  guarantees.
- Accepted `DEC-0130` and `DEC-0129` define only test-local lowering/design
  vocabulary. Accepted memory, ownership, Managed, Profile, and FFI boundary
  decisions do not define Native IR verification semantics.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and Task/Actor and
  Profile gaps remain Open. RFC-N304 and dependent Native, memory, ownership,
  FFI, and Profile authorities are not Accepted.
- `DEC-0131` authorizes this child only.

## Current implementation boundary

`native_ir_verifier_evidence.rs` defines forty-four test-local boundaries,
sorts them by explicit local rank, rejects duplicates, and compares
forward/reverse insertion order. Its evidence tag is opaque and test-only; it
is not a verifier, parser, validation schema, diagnostic, ABI validator,
execution trace, public protocol, or host-safety proof.

No verifier, NIR parser, malformed-input schema, diagnostic, backend operation
set, public protocol, or placeholder crate was added. The existing accepted
Seed Typed Core and VM bytecode pipeline remains the only executable path.

## Evidence and deferred work

Focused tests cover the complete boundary inventory, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent remains
blocked until accepted authority defines NIR parsing and limits,
CFG/SSA/types/ownership/cleanup/ABI invariants, backend-neutral operations,
source and Semantic ID mapping, malformed/unknown-version behavior,
deterministic diagnostics, panic/host-UB isolation, fuzz/property/security
evidence, and differential verification.
