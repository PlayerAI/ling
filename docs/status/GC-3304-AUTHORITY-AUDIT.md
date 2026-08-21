# GC-3304 Authority Audit — Managed Profile Checks

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

GC-3304 is a profile-policy task, not a license to add profile syntax or a
runtime assertion. The execution plan proposes that Explore allow Managed,
Native allow declared Managed Islands, Critical reject general Managed/GC, and
`no_gc` functions/modules receive static checks and runtime assertions. Those
claims affect source compatibility, allocation bounds, ABI/FFI, diagnostics,
and safety evidence, but no complete profile contract is Accepted.

No profile checker, `no_gc` keyword, Managed capability, Native-island schema,
runtime assertion, profile diagnostic, or placeholder API is added. GC-3304
remains `BlockedSpec` until the Managed/Native decisions and the Critical
Profile authority are Accepted.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:282-287` is a non-normative
  checklist. It does not define profile versioning, feature legality, source
  syntax, diagnostics, or runtime behavior.
- GC-3301 through GC-3303 are `BlockedSpec`; RFC-N303 and the Native/FFI
  authorities are absent. Profile checks cannot be soundly implemented without
  their object, collector, handle, ABI, and allocation contracts.
- `GAP-OWNERSHIP-MODEL-001` remains Open and explicitly leaves Managed and
  Profile boundaries unresolved. `GAP-NATIVE-BACKEND-ABI-001` leaves Native
  layout, target tiers, FFI, and Fault/reentry unresolved.
- `GAP-CRITICAL-PROFILE-001` is Open and says that the minimum Critical Core,
  forbidden capabilities, boundedness, Fault/timing claims, and evidence
  schema are not accepted; it names RFC-0012 as a future decision. The current
  gap list does not enumerate GC-3304, but its observable behavior covers this
  task, so this audit does not silently treat the omission as authorization.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` sketch Explore/Native/Critical
  build models, Managed Islands, no-general-GC and bounded-allocation goals,
  but reserve Critical enforcement and Managed GC outside v0.0.1. Their
  sketches do not settle `no_gc` syntax, transitive effects, imported APIs,
  runtime assertions, profile manifests, or migration rules.
- Accepted DEC-0009/RFC-0017 and the current support matrix authorize only the
  Seed value/mutable-place slice. They do not add a profile legality checker or
  permit reserved Managed/Native/Critical features to run silently.

## Current implementation evidence

- The workspace has no profile-policy crate, profile selection/validation
  pass, `no_gc` AST/Typed Core form, Managed capability, Native Island schema,
  or Critical runtime assertion. Existing compiler and VM paths do not expose
  the future profile feature set.
- No Managed runtime, Native backend, ABI/FFI boundary, allocation budget,
  collector pause, or profile-specific Fault contract exists to check.
- Existing support/governance fixtures describe the current supported surface;
  they are not executable authority for future Managed or Critical behavior.
- Rust allocation, host timing, addresses, and runtime assertions cannot be
  used as profile semantics. Adding a checker that assumes them would make
  unsupported behavior appear stable.

## Required authority before implementation

The accepted Managed, Native, and Critical decisions must define at least:

1. Profile identity, versioning, target/manifest inputs, inheritance and
   compatibility, and the exact feature/capability matrix for Explore, Native,
   Managed Island, and Critical builds.
2. Whether `no_gc` is source syntax, a module/function contract, or an internal
   profile annotation; its transitive effect rules across calls, closures,
   generics, imports, callbacks, Tasks/Actors, FFI, and allocation lowering.
3. Static legality checks and deterministic bilingual diagnostics for Managed
   allocation, collector safepoints, Resource Drop, dynamic code/reflection,
   unbounded allocation/recursion/mailbox behavior, FFI, and forbidden
   capabilities. Diagnostics need stable registered IDs and original byte
   spans.
4. Native Managed-Island boundaries, pin/handle/ABI rules, profile transitions,
   cross-profile calls and data transfer, and the exact runtime assertion or
   Fault behavior for a violated bound. A runtime check cannot replace a
   required compile-time proof.
5. Critical boundedness, timing/Fault, target, security/TCB, evidence, and
   migration contracts, including what remains unspecified and how unsupported
   programs fail before execution.
6. Typed Core, Semantic Graph, Audit Source, Semantic ID, support matrix,
   public protocol/schema, interpreter/VM/Native differential, and Unicode
   17.0.0 requirements for profile decisions.

## Evidence and compatibility impact

The eventual implementation needs positive and negative profile-matrix
fixtures, `no_gc` transitive-call/import/closure cases, Managed allocation and
Native-Island boundary cases, Critical rejection and boundedness cases,
runtime-assertion/Fault cases (if retained), migration and diagnostic repairs,
and deterministic interpreter/VM/Native comparisons. Evidence must be bounded,
offline, and independent of host paths, addresses, wall-clock timing,
allocator order, and map/thread scheduling; it must preserve exact UTF-8 byte
spans and stable diagnostic identities.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, diagnostic registry, schema, Semantic ID, source span, runtime,
or Unicode behavior. It allocates no profile diagnostic and introduces no
`no_gc`, Managed Island, or Critical public protocol. Existing Seed support and
offline build/test behavior remain unchanged.

## Intentionally deferred

Profile syntax and manifests, feature/capability matrices, `no_gc` checking,
Managed allocation legality, Native Island transitions, Critical boundedness,
runtime assertions/Faults, profile diagnostics, migration rules, and all
profile/interpreter/VM/Native evidence remain deferred until the required RFCs
and governance protocols are Accepted.
