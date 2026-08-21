# MEM-3104 Authority Audit: Managed Types and Island Boundaries

## Outcome

`MEM-3104` is correctly recorded as `BlockedSpec`. The G3 plan requires
Managed references and graphs, rules for Value/Resource-to-Managed and
Managed-to-Resource edges, island roots, cross-thread/Actor/FFI behavior,
pinning, and borrowed views. Those rules determine aliasing, reachability,
finalization, safety, ABI, and profile behavior and are not authorized without
the accepted memory/ownership and Native/FFI RFCs.

No Managed reference or graph, island root, edge rule, pinning/borrowed-view
type, GC/finalization policy, diagnostic, protocol, or placeholder G3 API was
added.

## Normative traceability

- The G3 execution package is non-normative. Its Managed/island sketch cannot
  authorize garbage collection, reference identity, cross-domain aliasing,
  pinning, FFI, or public Checked Core behavior.
- MEM-3104 depends on MEM-3101 through MEM-3103 and the missing RFC-N301/
  RFC-0007 memory/ownership authority. No accepted memory-model RFC exists;
  RFC-0001 remains a Draft under DEC-0018 and `GAP-OWNERSHIP-MODEL-001` remains
  Open.
- Accepted DEC-0009 limits v0.0.1 to Value semantics and excludes Resource,
  Borrow, `&mut`, implicit references, and Borrow Edges. It does not define
  Managed identity, roots, GC, pinning, borrowed views, or cross-island rules.
- `docs/SEMANTICS.md` sketches Managed as a future category, separates cyclic
  Managed cleanup from deterministic Resource Drop, and states that Managed
  references crossing isolation domains need a sharing policy. It does not fix
  graph reachability, root discovery, collection/finalization observability,
  pinning, borrowed-view lifetime, Actor/thread/FFI boundaries, or OOM/Fault
  behavior; v0.0.1 has no Managed runtime.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require accepted memory,
  ownership/region, Managed runtime, Native, FFI, profile, and resource
  boundaries before v0.3 implementation.
- `GAP-OWNERSHIP-MODEL-001` is Open, blocks MEM-3101 through MEM-3104 and
  related ownership tasks, and leaves Managed roots/finalization, aliasing,
  region, and Profile boundaries unaccepted.

## Current implementation evidence

- The workspace has no Managed type, reference identity, graph or island root,
  collector/finalizer, pinning API, borrowed view, cross-thread/Actor/FFI
  bridge, sharing policy, OOM/Fault boundary, or Managed projection in
  Checked Core, Semantic Graph, or Audit Source. The Seed checker and VM use
  Value semantics only.
- Existing Rust references, `Arc`/GC-like host structures, allocations,
  pointer identity, and finalizers are implementation details and cannot
  define Ling Managed semantics. Seed Capabilities are authorization handles,
  not Managed references.
- No diagnostic or fixture defines illegal Value/Resource/Managed edges,
  island escape, root loss, cycle collection, pinning failure, borrowed-view
  expiry, cross-Actor/thread/FFI transfer, OOM, Unicode/CRLF/BOM spans, or
  interpreter/VM/Native differential behavior. No versioned Managed protocol
  is registered.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. Managed identity, reachability, graph ownership, root discovery and island
   boundaries; Value/Resource-to-Managed and Managed-to-Resource edge rules,
   cycle behavior, sharing/aliasing, equality/hash, serialization, and
   lifetime/collection observability.
2. Collector/finalization model, allocation and OOM Faults, pinning and
   borrowed-view creation/expiry, mutation/concurrency, resource cleanup,
   cancellation/Actor-turn/await boundaries, and explicit separation from
   deterministic Resource Drop.
3. Cross-thread, Actor, Task, FFI, Native ABI, Target Primitive, and Profile
   rules, including transfer/share/borrow modes, thread safety, reentrancy,
   pinning, capability and security limits, and compatibility/migration.
4. Checked Core, Semantic Graph/Audit Source, canonical bytes and Semantic ID
   projection, stable bilingual diagnostics, schema/protocol versioning,
   deterministic output, and Unicode 17.0.0 source-span preservation without
   leaking Rust pointers, allocation, GC timing, or hash order.
5. Executable positive/negative/migration/property/drop-order/profile/OOM and
   interpreter/VM/Native differential fixtures covering roots, cycles, island
   escapes, edge restrictions, pinning, borrowed views, concurrency, FFI,
   serialization, cancellation, and finalization observability.

Until those decisions are Accepted, implementing Managed or island boundaries
would freeze memory safety, aliasing, lifetime, GC, ABI, FFI, profile, and
backend behavior that the language authority intentionally leaves open.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0009, DEC-0010, DEC-0012,
DEC-0013, DEC-0018, RFC-0001,
`docs/ling_execution_plan/07-G3-V0.3-NATIVE.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/authority.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
memory kind, Value layout, Resource, Managed, island, ownership, Native ABI,
diagnostic, schema, Semantic ID, source-span, runtime, or Unicode 17.0.0
behavior changed.

## Intentionally deferred

`MEM-3104` can begin only after MEM-3101 through MEM-3103 and RFC-0007 (or an
accepted replacement) define memory kinds, ownership, Resource Drop, Managed
graphs, collection, pinning, and Native/FFI boundaries. The future
implementation must preserve Seed Value semantics, consume accepted types and
checked Core only, make roots and cross-island transfers explicit, and publish
Managed graph, OOM, pinning, borrowed-view, profile, FFI, and
interpreter/VM/Native evidence before exposing v0.3 Managed behavior.
