# DIFF-3701 Authority Audit — Interpreter/VM/Native Conformance Harness

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

DIFF-3701 proposes one conformance harness that executes the same corpus on
the Interpreter, VM, and Native engines and compares return values,
stdout/events, Fault category/code, Resource cleanup traces, deterministic
replay, declared floating-point tolerance, and Semantic-ID/source mapping.
The execution-plan item is non-normative and cannot define a Native oracle,
cleanup trace, floating-point tolerance, or permitted backend difference.

No three-engine harness, Native adapter, comparison schema, trace normalizer,
oracle, floating-point policy, replay integration, source-map checker, or
public differential protocol is added. The existing checked Interpreter/VM
differential evidence remains unchanged; Native conformance is deferred until
Native and FFI semantics are Accepted.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:492-510` is non-normative.
  It lists comparison fields but does not define corpus identity, engine
  adapters, event/trace schemas, Resource/Managed cleanup observability,
  replay input, floating-point tolerance, or source-map equivalence.
- `docs/ROADMAP-1.0.md:55-67` requires all engines to consume checked Typed
  Core and to expose their public stability state; `:351-379` makes Native
  lowering, FFI, and Interpreter/VM/Native differential conformance a future
  G3 gate. These clauses do not authorize a Native engine or comparison
  semantics.
- Accepted RFC-0018/RFC-0019 and the `PROTO-BYTECODE` inventory authorize the
  current experimental checked Interpreter–VM differential boundary, including
  logical events, Unit results, stable Fault projections, source spans,
  committed state, and deterministic Program IDs. They do not authorize
  adding Native observations or expanding the bytecode protocol.
- `GAP-NATIVE-BACKEND-ABI-001` remains Open: Native IR, ABI/layout,
  unwinding/Fault, thread/reentry, Typed FFI, Target Primitive, and target
  tiers are unaccepted. `GAP-OWNERSHIP-MODEL-001` and
  `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` leave Resource/Managed, borrow, drop,
  and public lifetime traces unresolved.
- `GAP-SEMANTIC-HASH-LIFECYCLE-001` remains Open, so the cross-engine
  Semantic-ID/source mapping and canonical identity/version boundary cannot
  be frozen by a harness. `PROTO-ABI` and `PROTO-EVIDENCE` remain Planned
  public without schemas, readers, migration rules, or fixtures.
- RFC-N304, RFC-N305, RFC-N306, RFC-0007, and RFC-0011 are not Accepted
  authorities in this repository; RFC-0001 remains Draft under DEC-0018.

## Current implementation evidence

- The workspace has an accepted experimental Interpreter/VM differential path
  for checked bytecode, but no Native backend, Native execution adapter,
  three-way corpus, or Native trace/result schema. VM traces cannot stand in
  for Native ABI, Resource cleanup, FFI, or target behavior.
- No accepted rule says which stdout/events, scheduling, allocation, GC,
  address, debug, or cleanup observations are semantic; no floating-point
  tolerance or target-specific NaN policy is fixed for Native; and no
  machine-readable allowed-difference registry exists for DIFF-3702.
- No Native toolchain, target, FFI dependency, differential harness, diagnostic
  allocation, or public protocol implementation is required for this audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A canonical conformance input and program identity covering checked Typed
   Core, bytecode/Native derived artifacts, dependencies, capabilities,
   Profile, target, Unicode version, and deterministic seed/replay inputs;
   each engine must consume verified representations rather than reinterpreting
   unchecked AST.
2. Versioned engine adapters and comparison projections for values,
   stdout/events, Error/Fault categories/codes/spans, committed state,
   Resource/Managed cleanup and cancellation, replay effects, floating-point
   values/tolerance/NaN, and Semantic-ID/source maps. Host addresses, timing,
   allocation order, debug noise, and permitted scheduling differences must be
   explicitly excluded or registered.
3. Native IR/ABI/runtime/FFI/ownership/target contracts that make the Native
   result comparable, including error/unwind, thread/reentry, allocator,
   Capability, target/profile, and Resource/Managed behavior, plus exact
   rejection behavior for unsupported programs.
4. A machine-readable allowed-difference registry (DIFF-3702) with ownership,
   rationale, scope, versioning, negative tests, migration, and independent
   validation; a harness may not add backend-name conditionals as silent
   exemptions.
5. Versioned differential-result/evidence schemas, provenance and toolchain
   identity, deterministic ordering, offline/reproducibility requirements,
   bilingual stable diagnostics, and conformance/property/fuzz/cross-target
   fixtures sufficient to distinguish language defects from harness, backend,
   or host failures.

## Evidence and compatibility impact

The eventual implementation needs all v0.0.1 and later accepted conformance
corpora; positive/negative checked-Core, bytecode, Native, FFI, Resource,
Fault, Effect/Capability, replay, and source-map cases; independent result
normalization; exact return/event/Fault/cleanup comparisons; declared numeric
tolerance tests; deterministic repeated and cross-process runs; allowed-
difference negative tests; property/fuzz and cross-target evidence; and
provenance/schema migration checks. It must preserve original UTF-8 byte
spans, stable Semantic IDs, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and
Unicode 17.0.0 behavior without comparing host paths, addresses, hash order,
timing, or debug text as Ling semantics.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, memory category, Managed/Resource/ownership
behavior, diagnostic registry, schema, Semantic ID, source span, runtime,
support matrix, target, or Unicode behavior. It adds no Native adapter,
three-way harness, comparison schema, allowed-difference entry, dependency,
toolchain, diagnostic, public protocol implementation, or placeholder API.

## Intentionally deferred

Native execution and adapter support, three-engine corpus and comparison
projections, Resource/Managed cleanup/replay/Fault/float/source-map rules,
allowed-difference registry, differential result/evidence schemas,
cross-target/property/fuzz runs, provenance and reproducibility, and all
Interpreter/VM/Native conformance claims remain deferred until the Native,
ownership, FFI, Semantic-ID lifecycle, `PROTO-ABI`, `PROTO-EVIDENCE`, and
DIFF-3702 authorities are Accepted.
