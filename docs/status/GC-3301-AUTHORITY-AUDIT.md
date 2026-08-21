# GC-3301 Authority Audit — Minimal Managed Object Model

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

GC-3301 is a G3 design task, not an implementation authorization. The execution
plan asks for an invisible object header, type metadata, a root interface, a
write-barrier interface, weak-reference and finalization policy, an out-of-memory
Fault, and observable pointer-identity rules. None of those contracts is
accepted for Ling v0.0.1 or for a later Profile.

This audit therefore adds no Managed runtime crate, object-header layout,
collector, root/handle API, write barrier, weak reference, finalizer, allocator
policy, OOM diagnostic, schema, or public protocol. The next executable work is
RFC-N303 (with the memory-category and ownership decisions it depends on), not
an implementation chosen from the plan.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:239-254` is a non-normative
  execution proposal and explicitly declares RFC-N303 as the dependency for
  GC-3301. Its checklist cannot define runtime representation or observable
  behavior.
- RFC-N303 is not present or Accepted. RFC-0001 remains Draft under
  `docs/decisions/0018-rfc-0001-lifecycle.md`; its RFC-0007 Value/Managed/
  Resource entry is a roadmap for a future decision, not authority.
- `GAP-OWNERSHIP-MODEL-001` is Open. It explicitly leaves Managed roots,
  finalization, memory categories, and Profile boundaries unresolved and names
  RFC-0007 as the next action. The execution plan's Managed-runtime contract
  additionally requires the missing RFC-N303 decision.
- `docs/SEMANTICS.md` describes Managed identity, graphs/cycles, a
  `Allocate<Managed>` effect, and Explore/Native/Critical Profile sketches, but
  also reserves Managed GC for post-Seed work. It does not specify an object
  layout, root set, tracing algorithm, barrier memory model, weak references,
  finalization, OOM classification, or pointer identity.
- `docs/LANGUAGE.md` gives a design example for `managed type`, permits a
  Managed Island in Native, and excludes general Managed objects from Critical.
  It does not authorize syntax, a runtime ABI, or a collector implementation.
- Accepted DEC-0009 and RFC-0017 define only the Seed mutable-place boundary and
  Seed Place lowering. They do not introduce Managed values, GC roots, or
  pointer identity. Accepted DEC-0013 separates compile, host, internal, and
  runtime-fault handling, but does not decide that allocation failure is a
  public OOM Fault.
- The v0.0.1 support boundary reserves Managed GC and requires an explicit
  unsupported-feature diagnostic; a reserved schema name is not executable
  behavior.

## Current implementation evidence

- The workspace contains source, syntax, AST/HIR, resolver, type/effect,
  semantic, evaluator, bytecode, VM, project, CLI, database, cache, format, and
  diagnostic crates, but no Managed-runtime or collector crate.
- The existing pipeline evaluates checked Typed Core through the current
  interpreter/bytecode/VM paths. Repository search finds no Ling object header,
  root registration, reachability graph, write barrier, weak-reference handle,
  finalization queue, Managed allocator, OOM boundary, or pointer-identity
  operation.
- Existing object-shaped data is internal Rust/JSON representation and is not a
  Ling Managed object. Rust allocation, addresses, pointer layout, drop order,
  and hash-map order remain non-semantic.
- Existing Seed tests and support fixtures cover value semantics, diagnostics,
  and interpreter/VM behavior. They do not establish the future Managed
  runtime contract.

## Required authority before implementation

RFC-N303, coordinated with the accepted resolution of RFC-0007, must specify at
least:

1. The language-visible Managed identity model and the completely private
   object-header/metadata representation, including type identity, versioning,
   alignment, and whether pointer identity can be observed or only a stable
   logical identity can be observed.
2. The root interface and root-lifetime rules for stack frames, globals,
   closures, Tasks, Actors, Native Managed Islands, callbacks, and FFI; the
   treatment of cycles and reachability; and the boundary between Managed
   collection and deterministic Resource Drop.
3. The write-barrier contract, mutation and concurrency ordering, safe points,
   and any collector-specific mechanism that must be honored by generated
   code, without exposing a particular algorithm as Ling semantics.
4. Whether weak references exist in the first release, their liveness and
   clearing rules, and whether finalization is forbidden, bounded, idempotent,
   and effect-free or instead represented by an explicit operation. Finalizers
   must not silently acquire Resource Drop or arbitrary network behavior.
5. The OOM boundary: allocation limits, retry/recovery behavior, cancellation
   and shutdown interaction, stable bilingual diagnostic/error identity, Fault
   payload/schema, and equivalence across interpreter, VM, and Native profiles.
6. Explore, Native Managed Island, and Critical restrictions, including pinning,
   borrowed-view lifetime, cross-island transfer, FFI/ABI, and security/TCB
   consequences. These must be compatible with the ownership and lifetime
   authority rather than inferred from Rust references.
7. Typed Core, Semantic Graph, Audit Source, Semantic ID, public protocol,
   migration, determinism, and Unicode 17.0.0 evidence requirements for any
   exposed Managed construct or runtime diagnostic.

## Evidence and compatibility impact

The eventual implementation needs positive and negative object-graph fixtures,
root-loss and cycle cases, weak-reference/finalization cases (if supported),
bounded OOM and recovery cases, Profile and FFI boundary cases, deterministic
interpreter/VM/Native differential traces, and stress/property evidence. Every
fixture must preserve original UTF-8 byte spans, avoid host addresses and
allocation order, and distinguish language invariants from robustness or host
failures.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, diagnostic registry, schema, Semantic ID, source-span, runtime,
or Unicode behavior. It allocates no diagnostic code and registers no public
Managed protocol. The existing Seed behavior and offline build/test boundary
remain unchanged.

## Intentionally deferred

Object layout, metadata tables, roots, write barriers, collector selection,
weak references, finalization, OOM Faults, pointer identity, Managed Islands,
pinning, FFI handles, runtime metrics, and their tests remain deferred until
the required RFCs and evidence are Accepted and tracked in the governance
registries.
