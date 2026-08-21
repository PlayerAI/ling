# BACK-3504 Authority Audit — Native Optimization and Verification

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

BACK-3504 proposes a first Native optimization set: constant folding, dead
block elimination, trivial inlining, proof-backed bounds-check elimination,
copy propagation, and tail calls only where semantics are explicit. Every pass
must have pre/post verification and differential/property tests. These passes
can change evaluation order, allocation, cleanup, Fault visibility, stack
traces, debug locations, ABI behavior, and Resource/Managed lifetimes.

No Native optimizer, pass pipeline, proof representation, verifier hook,
optimization diagnostic, or placeholder backend crate is added. The existing
Seed evaluator/bytecode/VM behavior remains unchanged until the NIR, ABI,
memory, ownership, and Profile contracts are Accepted.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:380-391` is non-normative. It
  cannot authorize an optimization, proof rule, pass ordering, or observable
  debug/stack behavior.
- BACK-3501 through BACK-3503 and NIR-3401 through NIR-3403 are `BlockedSpec`.
  RFC-N304/RFC-N306/RFC-0011 and the dependent memory/ownership/Managed/FFI
  decisions are absent or not Accepted.
- `GAP-NATIVE-BACKEND-ABI-001` leaves NIR validity, layout, ABI,
  Fault/unwinding, thread/reentry, FFI, and target tiers unresolved;
  `GAP-OWNERSHIP-MODEL-001` leaves copy/move, borrow, cleanup, Managed, and
  Profile boundaries unresolved.
- `docs/SEMANTICS.md` requires optimization to preserve observable semantics
  and records unaccepted assumptions in Audit/Semantic views, but it does not
  define Native pass proofs, floating/numeric rules, stack/debug behavior, or
  Resource/Managed legality. Accepted Seed decisions cover only the current
  checked Typed Core/interpreter/VM boundary.

## Current implementation evidence

- The workspace has no Native IR optimizer, pass manager, proof/certificate
  format, pre/post NIR verifier integration, or Native differential/property
  harness. Existing source/type/effect checks are not a Native optimizer.
- No accepted NIR operation semantics, runtime ABI, Resource/Managed cleanup,
  Task/Actor call boundary, target numeric model, or Fault/stack/debug contract
  exists for transformations to preserve.
- Existing Seed tests exercise semantic and interpreter/VM behavior; they do
  not establish optimization equivalence or prove bounds-check/tail-call
  legality. Rust compiler optimizations are not Ling semantics.

## Required authority before implementation

The accepted NIR/Native/memory/ownership decisions must define:

1. Semantic-preservation rules for constant folding (numeric/effect/Fault),
   dead blocks, inlining/closure capture, copy propagation, bounds-check
   elimination proofs, and tail calls (stack/recursion/cleanup/debug effects).
2. Effect/capability, evaluation-order, Resource Drop, Managed root/barrier,
   borrow/alias, Task/Actor, cancellation, FFI, Profile, and ABI constraints
   that make each transformation legal or reject it.
3. Proof/certificate and pre/post-verifier interfaces, pass ordering and
   invalidation, deterministic diagnostics, source/debug/Semantic ID mapping,
   and the boundary between internal optimization assumptions and public
   contracts.
4. Numeric/target/endianness and Fault behavior, reproducibility, resource and
   compilation bounds, security/TCB, versioning, and migration rules for
   optimization metadata.
5. Differential/property corpus requirements comparing unoptimized and
   optimized interpreter/VM/Native results and permitted traces, excluding
   host addresses, timing, allocation order, and debug noise.

## Evidence and compatibility impact

The eventual implementation needs per-pass positive/negative fixtures,
pre/post verifier rejection, proof-backed bounds cases, effect/Fault/cleanup
and Resource/Managed cases, closure/Task/Actor/FFI boundaries, stack/debug
mapping, deterministic pass ordering, optimization-failure diagnostics,
property/fuzz stress, and interpreter/VM/Native differential tests. It must
preserve original UTF-8 spans, stable Semantic IDs, and Unicode 17.0.0 while
making no unsupported performance or target claims.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, diagnostic registry, schema, Semantic ID,
source span, runtime, or Unicode behavior. It adds no optimizer, proof schema,
diagnostic, or public protocol and makes no performance claim.

## Intentionally deferred

Native pass manager, constant/dead-block/inlining/copy/bounds/tail-call passes,
proof certificates, pre/post verifier integration, optimization diagnostics,
debug/stack preservation, property/fuzz corpus, reproducibility, and all
optimized interpreter/VM/Native evidence remain deferred until the required
RFCs and NIR/ABI/runtime contracts are Accepted.
