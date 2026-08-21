# NIR-3403 Authority Audit — Native IR Verifier

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

NIR-3403 is an independent-validation task, not an authorization to define a
Native IR by writing a verifier first. The execution plan requires checks for
blocks/phi/SSA, type consistency, Resource ownership, cleanup coverage, legal
ABI, source IDs, invalid references, and backend-specific unresolved
operations, with safe rejection and no host UB. NIR-3401, NIR-3402, the Native
ABI, and the memory/ownership/Managed/FFI contracts are not accepted.

No verifier crate, validation schema, malformed-IR diagnostic, IR parser,
public protocol, or placeholder backend API is added. The existing compiler
continues to evaluate only checked Seed Typed Core and does not accept a future
Native IR input.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:324-337` is a
  non-normative checklist. It cannot define the NIR grammar, invariants,
  validation order, error schema, or host-safety boundary.
- NIR-3401 and NIR-3402 are `BlockedSpec`; RFC-N304/RFC-0011 is absent or not
  Accepted. Therefore block/phi/SSA, types, ownership, cleanup, ABI, source
  IDs, and backend-neutral operation sets have no normative definitions.
- `GAP-NATIVE-BACKEND-ABI-001` is Open and leaves IR validity, layout, ABI,
  Fault/unwinding, typed FFI, thread/reentry, target packages, and target tiers
  unresolved. `GAP-OWNERSHIP-MODEL-001` leaves Resource/Managed/borrow/drop
  invariants unresolved.
- Accepted DEC-0009/RFC-0017 only cover the Seed mutable-place boundary; they
  do not define Native IR ownership or verifier behavior. The existing
  diagnostics and semantic schemas are for current source/Typed Core and do
  not authorize arbitrary IR ingestion.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` require checked Typed Core as the
  executable authority and reserve Native backend behavior. A plan statement
  that invalid IR must not trigger host UB does not itself specify a safe
  deserializer or verifier protocol.

## Current implementation evidence

- The workspace has no Native IR parser, verifier, ABI validator, or backend
  crate. Existing parser/AST/HIR/type checks validate source and checked Core,
  not an external or serialized Native IR.
- There is no representation for future ownership/cleanup, Managed handles,
  Task/Actor runtime ABI, Native Fault edges, target legality, or source-ID
  mapping that a verifier could check.
- Existing malformed-input handling belongs to current source, semantic,
  bytecode, cache, and protocol boundaries. It does not establish a bounded
  NIR validation format or a public error schema.
- Rust panics, allocation behavior, pointer layout, and host UB are not
  language semantics; adding unchecked IR construction or debug-only assertions
  would violate the repository boundary.

## Required authority before implementation

The accepted NIR and Native decisions must define:

1. A versioned, bounded NIR grammar and canonical representation, including
   block/edge/phi/SSA rules, type and value forms, reference identity, source
   IDs, and extension/unknown-version handling.
2. Type consistency, effect/capability, Resource ownership and cleanup, Managed
   handle/barrier, borrow/alias, Task/Actor, and Fault/cancellation invariants,
   including validation order and whether every failure is a compile error,
   protocol rejection, or a runtime Fault.
3. Legal ABI/layout and target/profile/FFI operations, the backend-neutral
   operation set, rejection of unresolved backend-specific operations, and
   explicit source/semantic identity mapping.
4. Safety requirements for parsing and verification: size/recursion/resource
   limits, no unchecked AST/HIR/Core/IR execution, panic/host-UB isolation,
   deterministic diagnostics with stable bilingual error IDs, and safe
   behavior for malformed, truncated, cyclic, or adversarial inputs.
5. Schema/protocol ownership, serialization determinism, migration and
   compatibility, security/TCB boundaries, and evidence required to prove
   verifier soundness and semantic preservation against interpreter/VM/Native.

## Evidence and compatibility impact

The eventual implementation needs valid and invalid fixtures for CFG/SSA,
phi/type errors, ownership and cleanup gaps, illegal ABI/target operations,
invalid source IDs/references, malformed/oversized/cyclic input, unknown
versions/extensions, deterministic diagnostic ordering, and no-host-UB/panic
behavior. It also needs fuzz/property/stress evidence with bounded resources,
reproducible seeds, differential comparisons, and Unicode/CRLF/BOM source-span
retention. Verification must never execute unchecked IR.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, diagnostic registry, schema, Semantic ID, source span, runtime,
or Unicode behavior. It allocates no verifier diagnostic and introduces no
public NIR validation or serialization protocol. Existing Seed behavior and
offline build/test behavior remain unchanged.

## Intentionally deferred

NIR grammar and parser, verifier invariants and ordering, ownership/cleanup/ABI
checks, source-ID validation, backend-neutral operation set, malformed-input
handling, diagnostic schema, fuzz/property corpus, and all verifier/differential
evidence remain deferred until NIR-3401/NIR-3402 and the dependent RFCs are
Accepted.
