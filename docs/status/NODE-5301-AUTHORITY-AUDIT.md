# NODE-5301 Authority Audit — Node Syntax and Semantics

Status: BlockedSpec

Date: 2026-08-22

## Outcome

NODE-5301 proposes a synchronous reactive `node` form with periodic execution,
an input frame, a deadline, persistent state, and explicit output. The plan
requires rules for logical ticks, input sampling, output commit, initialization,
absence/presence, clock domains, deterministic evaluation order, Fault/overrun,
and composition.

The repository has no Accepted RFC-K502 or equivalent Node contract. The
current semantic design gives a conceptual Node section but explicitly limits
v0.0.1 to the Seed Core and lists Node as reserved future functionality. The
plan example therefore cannot authorize a new grammar, Checked Core node, or
runtime: doing so would invent timing, state, Fault, and target semantics.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:199-224` is a
  non-normative task fragment dependent on the absent RFC-K502. It does not
  define grammar, type/effect rules, tick boundaries, input/output ownership,
  clock units, initialization, absence, composition, or Fault transitions.
- `docs/SEMANTICS.md:372-404` defines the Seed Typed Core boundary and states
  that v0.0.1 implements only the first twelve Core forms plus
  `Console.Write`; `NodeStep` is not part of the implemented subset.
- `docs/SEMANTICS.md:1380-1425` sketches Node timing, state, and deadline
  concepts, but it does not provide the accepted syntax/Core/runtime,
  scheduler, overrun, or target evidence contract. `:1914-1931` explicitly
  reserves Node for a future version and rejects silent placeholders.
- `docs/LANGUAGE.md:857-866` is a surface-design example, not an Accepted
  Node specification. Its `every`, `deadline`, and `no_gc` forms have no
  accepted grammar, unit, lowering, or diagnostic authority.
- `docs/ROADMAP-1.0.md:118` and `:433-498` place Node in G5 after G2/G3/G4
  prerequisites and require tick/state/deadline/overrun/Fault semantics,
  bounded allocation, and reproducible evidence. They do not authorize
  implementation before those gates close.
- `GAP-CRITICAL-PROFILE-001` explicitly blocks NODE-5301 and records Node
  timing/Fault semantics, Critical boundaries, boundedness, and evidence as
  Open. The ownership, concurrency/mailbox, Native/ABI, Kernel/Device, and
  numeric/effect gaps also affect a Node contract.
- RFC-K502 is only a lower-authority plan label; no RFC-K502 or Accepted
  replacement is present in the repository. No public Node protocol, runtime,
  or target-support claim is registered.

## Current implementation evidence

- The parser, resolver, type/effect checker, Typed Core, evaluator, bytecode,
  and VM have no accepted `node` declaration, `NodeStep` lowering, logical
  clock, input sampling, output commit, persistent-state store, or Node Fault
  model.
- There is no definition of duration units such as `10.ms`, clock-domain
  conversion, sampling/commit atomicity, absence/presence representation,
  initialization/reinitialization, composition order, or deterministic
  scheduling.
- No scheduler, virtual clock, target WCET model, interrupt/cache/bus
  assumptions, overrun policy, deadline Fault, recovery/fallback behavior, or
  evidence identity exists for Node execution.
- Existing VM step/frame/heap limits are host-safety controls for bytecode and
  do not establish logical ticks, deadlines, or real-time guarantees. Existing
  Actor/Task designs and device/Native plans do not supply a Node boundary.
- No stable bilingual diagnostic or schema fixes unsupported Node syntax,
  missed deadline, overrun, absent input, clock mismatch, state violation,
  composition conflict, or target-evidence mismatch.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. Node grammar and checked representation, including declarations, input and
   output types, `every`/`deadline` units, state declarations, initialization,
   absence/presence, composition, and rejection of unsupported forms.
2. Exact synchronous semantics for logical ticks, sampling, evaluation order,
   output commit, state visibility, startup/restart, missed ticks, and
   composition; specify whether and how Node interacts with effects,
   capabilities, Task, Actor, Kernel, Device, and FFI.
3. Clock and target contracts, including clock domains, conversion and drift,
   scheduler/interrupt assumptions, WCET method, cache/bus model, target and
   compiler identity, and the boundary between evidence and language truth.
4. Deadline, overrun, Fault, cancellation, recovery, fallback, and state
   persistence behavior, with bounded resource and mailbox/ownership rules and
   a fail-closed policy for unknown timing or memory evidence.
5. Critical Profile capability restrictions, allocation/GC/recursion limits,
   deterministic numeric/effect behavior, Node/Actor topology rules, and the
   target-support matrix required before claiming v0.5 behavior.
6. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts
   for invalid Node syntax, unsupported timing, clock mismatch, absent input,
   deadline/overrun, Fault/recovery, state and composition errors, with
   original UTF-8 byte spans and stable Semantic IDs.
7. Offline executable positive/negative, virtual-clock, boundary,
   initialization/restart, absence/presence, composition, deadline/overrun,
   target/compiler, Unicode/CRLF/BOM, migration, determinism, and
   interpreter/VM/Native differential fixtures.

## Evidence and compatibility impact

The eventual implementation must consume checked Typed Core and preserve Seed
behavior until the Node authority is accepted. Timing and WCET facts must be
bound to an explicit target/toolchain/evidence identity; host wall-clock time,
thread scheduling, physical paths, addresses, allocator behavior, and debug
text must not become Ling semantics. Original UTF-8 spans and Semantic IDs
must remain stable across diagnostics and evidence.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, scheduler, Task, Actor, Native backend, memory or
ownership behavior, diagnostics, schemas, Semantic IDs, source spans, CLI,
LSP, dependency lock, target/toolchain support claim, or Unicode 17.0.0
behavior.

## Intentionally deferred

NODE-5301 implementation, Node syntax/Core/runtime, virtual clock and
scheduler, timing/WCET analysis, diagnostics, CLI/LSP/evidence protocols, and
support claims remain deferred until RFC-K502 (or an Accepted replacement),
`GAP-CRITICAL-PROFILE-001`, and the dependent ownership, concurrency,
Native/ABI, Kernel/Device, numeric/effect, and transaction authorities are
resolved with independent offline fixtures. No placeholder `node` parser,
Core variant, runtime, or public API is created.
