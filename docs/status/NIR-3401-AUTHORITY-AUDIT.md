# NIR-3401 Authority Audit — Backend-Neutral Native IR

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

NIR-3401 is a Native design task, not an authorization to invent an IR or ABI.
The execution plan asks for typed SSA/control flow, Value/Resource
representation, checked borrow provenance or alias facts, explicit cleanup,
function ABI, Fault edges, source/debug mapping, Effect boundaries, and
deterministic serialization. Those fields determine safety, compatibility,
debugging, FFI, and backend legality.

No Native IR crate, instruction set, ABI record, serializer, verifier, debug
schema, diagnostic, or placeholder backend API is added. NIR-3401 remains
`BlockedSpec` until RFC-N304 and the dependent memory, ownership, Managed, FFI,
and Profile contracts are Accepted.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:289-306` is a
  non-normative execution proposal and explicitly depends on RFC-N304. Its
  checklist cannot define a public IR, binary format, ABI, or semantic
  lowering rule.
- RFC-N304 is not present or Accepted. RFC-0001 remains Draft under DEC-0018;
  RFC-0011 is only the candidate Native decision in the open governance gap.
- `GAP-NATIVE-BACKEND-ABI-001` is Open and leaves Native IR validity, layout,
  ABI, Fault/unwinding, thread/reentry, typed FFI, target packages, and target
  tiers unresolved. It names RFC-0011 only as a future action after RFC-0007
  defines memory categories.
- GC-3301 through GC-3304 and the MEM/OWN tasks remain `BlockedSpec`, so the
  planned IR's Value/Managed/Resource, borrow, cleanup, handle, profile, and
  runtime-ABI operands have no accepted meanings. `GAP-OWNERSHIP-MODEL-001`
  and `GAP-CRITICAL-PROFILE-001` remain Open as well.
- Accepted DEC-0009/RFC-0017 authorize the Seed mutable-place slice only.
  Accepted runtime-fault decisions do not define Native Fault edges, unwind
  semantics, or a serialized IR. `docs/SEMANTICS.md` and `docs/LANGUAGE.md`
  describe Native as a future Profile and reserve Native backend behavior.

## Current implementation evidence

- The workspace has no Native IR, ABI, FFI, backend, target-package, or debug
  location crate. `ling-bytecode` is the existing VM bytecode path and is not a
  backend-neutral Native IR or ABI contract.
- The compiler currently produces checked Seed Typed Core and the existing
  interpreter/bytecode/VM paths. It has no Native memory kinds, general
  ownership/borrow facts, Managed handles, Task/Actor runtime ABI, or Native
  Fault edges to lower.
- No public IR/schema/protocol or deterministic serializer is registered.
  Rust layout, addresses, unwinding, allocation, enum discriminants, and map
  iteration remain non-semantic.
- Existing tests provide Seed semantic, diagnostic, bytecode/VM, and source
  span evidence. They do not prove Native IR validity, ABI compatibility,
  backend equivalence, or cross-target reproducibility.

## Required authority before implementation

RFC-N304 and the dependent accepted decisions must define at least:

1. The NIR version and semantic boundary: typed SSA/control flow, block/phi
   invariants, evaluation order, value/resource/Managed representation,
   closure/aggregate layout, and which operations are language semantics versus
   backend strategy.
2. Checked borrow provenance or alias facts, ownership and explicit cleanup,
   Resource Drop, Managed handles/barriers, Task/Actor/suspension operations,
   and the rule that unresolved AST/HIR or Rust ownership never reaches NIR.
3. Function and data ABI: calling convention, layout/alignment/discriminants,
   target/endianness policy, return and aggregate passing, Fault/unwind edges,
   thread/reentry/cancellation behavior, and typed FFI/Target Primitive
   Package boundaries.
4. Effect and capability boundaries, Native/Managed-Island/Profile legality,
   safety/TCB responsibilities, source-span/definition mapping, debug
   variable locations, and versioned migration/compatibility rules.
5. Deterministic serialization and schema ownership, invalid-input rejection,
   no backend-specific unresolved operation, diagnostic/error identity, and
   public versus internal status of every NIR field.
6. Semantic-preservation obligations for Core-to-NIR lowering and the
   interpreter/VM/Native differential evidence needed before a Native profile
   can claim support.

## Evidence and compatibility impact

The eventual implementation needs valid and invalid NIR fixtures for blocks,
phi/SSA, types, ownership/cleanup, Fault/effect edges, source/debug mapping,
Managed handles, Task/Actor ABI, and target/FFI boundaries. It also needs
deterministic serialization round trips, malformed/unknown-version rejection,
security/resource bounds, differential traces against interpreter and VM,
cross-target ABI tests, and Unicode/CRLF/BOM source-span preservation. Host
paths, addresses, allocation order, timing, map order, and backend debug text
must not become semantic inputs.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, diagnostic registry, schema, Semantic ID, source span, runtime,
or Unicode behavior. It allocates no Native/ABI diagnostic and introduces no
public IR or serialization protocol. Existing Seed behavior and offline
build/test behavior remain unchanged.

## Intentionally deferred

NIR instruction/schema design, SSA/phi representation, memory and ownership
operands, cleanup/Fault/effect edges, ABI/layout, FFI and target packages,
source/debug mapping, deterministic serialization, verifier integration, and
all Core-to-NIR or interpreter/VM/Native evidence remain deferred until the
required RFCs and governance protocols are Accepted.
