# FFI-3601-OBSERVATION Authority Audit

## Outcome

The bounded child `FFI-3601-OBSERVATION` is authorized by Accepted
`DEC-0137`. It records only a test-local inventory of proposed FFI declaration
boundaries. Public `FFI-3601` remains `BlockedSpec`: no declaration syntax, ABI
schema, foreign symbol resolution, ownership behavior, target package, or
executable call boundary is added.

## Normative traceability

- The G3 execution package is non-normative; its FFI declaration checklist
  cannot define grammar, ABI/layout, ownership, lifetime, target, or runtime
  semantics.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` describe future Typed FFI and
  Target Primitive requirements but do not authorize a Seed grammar extension
  or an executable Native/FFI boundary.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and
  `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain Open. RFC-N305, RFC-0007,
  RFC-0011, and the `PROTO-ABI` schema are not Accepted authorities.
- `DEC-0137` authorizes this child only; earlier Native evidence decisions do
  not supply declaration or ABI semantics.

## Current implementation boundary

`ffi_declaration_evidence.rs` defines sixty test-local boundaries, sorts them by
explicit local rank, rejects duplicates, and compares forward/reverse insertion
order. Its evidence tag is opaque and test-only; it is not syntax, a declaration
identity, an ABI schema, a Semantic ID, a diagnostic, a linker input, a target
package, a safety proof, or a public protocol.

No parser node, AST/HIR/Checked Core node, resolver, layout calculator, foreign
symbol lookup, raw-pointer operation, callback runtime, target package,
dependency, toolchain, diagnostic, or public API was added. Seed compiler and
VM paths remain unchanged.

## Evidence and deferred work

Focused tests cover the declaration vocabulary, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent remains
blocked until accepted authority defines grammar, ABI/layout, symbol/version,
ownership/lifetime, callback/thread/reentry, Error/Fault, Capability/Profile/
Target, schema/migration, diagnostics, provenance/TCB, sanitizer/fuzz, and
cross-target behavior.
