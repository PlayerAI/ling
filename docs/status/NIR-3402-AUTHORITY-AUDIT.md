# NIR-3402 Authority Audit — Core-to-Native IR Lowering

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

NIR-3402 is a lowering/equivalence task, not an authorization to emit a
Native IR. The execution plan proposes vertical slices from integer/bool calls
through records, ADTs, mutable places, closures, effects, Resource/Drop,
Managed handles, and Task/Actor runtime ABI, with an interpreter/VM/Native
differential test for every step. NIR-3401, the Native ABI, and the memory,
ownership, Managed, FFI, and Task/Actor contracts are not accepted.

No lowering pass, NIR instruction use, native target, ABI adapter, diagnostic,
differential protocol, or placeholder crate is added. The existing accepted
Seed Typed Core and VM bytecode pipeline remains the only executable path.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:308-322` is a
  non-normative proposal. Its slice order cannot define translations,
  evaluation order, representation, or differential equivalence.
- NIR-3401 is `BlockedSpec` by absent RFC-N304/RFC-0011 and dependent memory,
  ownership, Managed, FFI, and Profile authority. Without a settled NIR there
  is no legal target for lowering.
- `GAP-NATIVE-BACKEND-ABI-001` is Open and leaves IR validity, layout, ABI,
  Fault/unwinding, thread/reentry, typed FFI, target packages, and target tiers
  unresolved. `GAP-OWNERSHIP-MODEL-001` and the Task/Actor gaps leave the later
  Resource, Managed, suspension, and runtime-ABI slices unresolved.
- Accepted DEC-0009/RFC-0017 cover only Seed mutable-place lowering into the
  existing checked core/evaluator boundary; they do not authorize a Native
  place representation, code generator, or ABI.
- `docs/SEMANTICS.md`, `docs/LANGUAGE.md`, and the v0.0.1 support boundary
  require evaluation from checked Typed Core and reserve Native backend,
  Resource, Managed, Task, Actor, and Critical features. No plan snapshot can
  override that authority.

## Current implementation evidence

- The workspace has no Native IR, lowering, ABI, backend, FFI, target package,
  or interpreter/VM/Native differential crate. Existing bytecode lowering is
  for the VM and is not a Native lowering contract.
- The current compiler produces checked Seed Typed Core and evaluates it via
  the interpreter/bytecode/VM paths. Future memory kinds, ownership facts,
  Managed handles, Resource cleanup, Task/Actor ABI, and Native Fault edges do
  not exist in the checked input.
- Existing semantic, bytecode/VM, diagnostic, and source-span tests are not
  Native equivalence evidence. Rust layout, allocation, addresses, and host
  unwinding cannot stand in for a lowering specification.

## Required authority before implementation

The accepted NIR and Native decisions must define:

1. A total or explicitly rejecting mapping from each authorized Checked Core
   form to NIR, including integer/bool calls, records/tuples, ADTs/match,
   mutable places, closures, Effect operations, Resource/Drop, Managed handles,
   and Task/Actor operations. Unsupported forms must fail before execution.
2. Evaluation order, value/aggregate/closure representation, memory category,
   borrow/alias provenance, cleanup and Drop, allocation/GC/barrier, effect and
   capability boundaries, Fault/cancellation edges, and source-span/semantic
   identity preservation.
3. ABI, target, profile, FFI, thread/reentry, and runtime-library contracts for
   every emitted operation, including representation choices that are not
   observable language semantics and migration/versioning rules.
4. Semantic-preservation obligations: what is compared between interpreter,
   VM, and Native, how nondeterminism, permitted target differences, host
   faults, metrics, and debug locations are excluded or recorded, and how
   failed lowerings are diagnosed with stable bilingual IDs.
5. Deterministic lowering order and serialization, bounded resource behavior,
   Unicode 17.0.0/source-byte span retention, and the rule that only checked
   Typed Core—not unresolved AST/HIR or guessed forms—can be lowered.

## Evidence and compatibility impact

The eventual implementation needs positive and negative fixtures for each
authorized slice, unsupported-form rejection, effect/Fault/cleanup paths,
source and Semantic ID mapping, deterministic NIR output, malformed/unknown
version handling, and per-slice interpreter/VM/Native differential traces.
Cross-target/ABI, FFI, cancellation/Actor, Resource/Managed, and Profile cases
must be added only after their owning RFCs are Accepted. Tests must not depend
on host paths, allocation addresses/order, wall-clock timing, map iteration, or
backend debug text.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, diagnostic registry, schema, Semantic ID, source span, runtime,
or Unicode behavior. It allocates no lowering/Native diagnostic and adds no
public IR, ABI, or differential protocol. Existing Seed tests and offline
build/test behavior remain unchanged.

## Intentionally deferred

Core-to-NIR instruction mappings, memory and closure lowering, effect/Fault and
cleanup edges, Managed/Resource/Task/Actor operations, target/ABI selection,
unsupported-form diagnostics, differential harnesses, and all Native code
generation remain deferred until NIR-3401 and the dependent RFCs are Accepted.
