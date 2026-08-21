# NODE-5302 Authority Audit — Node Checked Core

Status: BlockedSpec

Date: 2026-08-22

## Outcome

NODE-5302 proposes a Checked Core representation for Node input/output ports,
state cells, tick transitions, clock/period, deadlines, dependency graphs,
feedback delays, Fault transitions, and Contract hooks. It also requires
instantaneous cyclic dependencies to be rejected unless an accepted fixed-point
semantics and proof are available.

No Accepted RFC-K502 or replacement defines this Core schema, its typing and
effect rules, state-transition atomicity, graph identity, clock units,
feedback/fixed-point semantics, or Fault/Contract boundaries. The documented
`NodeStep` token is outside the v0.0.1 Seed implementation and cannot authorize
a new checked representation. Implementing the pass now would create an
unverifiable language semantics and a new trusted execution surface.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:226-240` is a
  non-normative plan fragment. It lists desired Core fields and a cyclicity
  rule but defines no node type grammar, field types, units, ownership,
  dependency identity, fixed-point theorem, or lowering/evaluation contract.
- `docs/SEMANTICS.md:372-404` lists `NodeStep` among future conceptual Core
  forms but states that v0.0.1 implements only the first twelve forms plus
  `Console.Write`. `docs/SEMANTICS.md:1380-1425` sketches Node timing/state
  concepts without an accepted Checked Core schema.
- `docs/SEMANTICS.md:1914-1931` explicitly reserves Node for a future version
  and forbids silent placeholder syntax. `docs/LANGUAGE.md:857-866` provides a
  surface example, not an accepted Core/lowering authority.
- `docs/ROADMAP-1.0.md:441-466` requires Critical Core, Node tick/state/
  deadline/overrun/Fault semantics, boundedness, and reproducible fixtures;
  it does not authorize a Node IR before those gates close.
- `GAP-CRITICAL-PROFILE-001` leaves the minimum Critical Core, Node timing and
  Fault semantics, boundedness, Contract boundary, and evidence schema Open.
  Ownership, concurrency/mailbox, Native/ABI, Kernel/Device, and numeric/effect
  gaps also constrain a checked Node representation.
- RFC-K502 is only a plan label; no RFC-K502 or Accepted replacement is present
  in the repository, and no Node Checked Core or public protocol is registered.

## Current implementation evidence

- The parser, resolver, type/effect checker, Typed Core, evaluator, bytecode,
  and VM have no Node declaration or checked node representation for ports,
  state cells, tick transitions, clock/period, deadlines, dependency graphs,
  feedback delays, Fault transitions, or Contract hooks.
- There is no graph identity or deterministic ordering rule, state-cell
  ownership/alias model, state commit boundary, input/output port capability,
  or clock/period/duration type in the implemented Seed pipeline.
- No instantaneous-cycle checker, delayed-feedback rule, fixed-point solver,
  proof obligation, or fail-closed diagnostic exists. Existing type/effect and
  bytecode verifiers do not establish Node graph soundness.
- Existing VM step/frame/heap limits are host-safety controls and provide no
  Node tick, deadline, state-transition, or cyclic-dependency semantics.
- No stable bilingual diagnostic or schema fixes invalid ports/state, clock or
  period mismatch, illegal cycle, missing delay, unsatisfied fixed-point proof,
  Fault transition, or Contract-hook incompatibility.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Node Checked Core schema with typed input/output ports, state
   cells, tick transitions, clocks/periods/deadlines, dependency edges,
   feedback delays, Fault transitions, Contract hooks, source spans, and
   stable Semantic IDs.
2. Port/state ownership, mutability, aliasing, initialization, absence/
   presence, capability/effect, and input sampling/output commit rules,
   including state visibility across ticks and restart/cancellation.
3. Deterministic graph identity and evaluation order, rate/clock conversion,
   feedback-delay semantics, instantaneous-cycle rejection, and any fixed-point
   semantics with a soundness/proof and resource bound.
4. Deadline, overrun, Fault, recovery/fallback, Contract status, and bounded
   memory/recursion/concurrency behavior, with explicit target/WCET evidence
   boundaries and fail-closed handling for unknown facts.
5. Critical Profile restrictions and interactions with Task, Actor, Kernel,
   Device, Native/ABI, numeric determinism, and effect/capability rules.
6. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts
   for invalid Core fields, cycles/delays/fixed points, clock mismatch,
   state/port errors, deadline/overrun, Fault, and Contract-hook failures.
7. Offline executable positive/negative, graph/cycle/fixed-point,
   state/initialization, clock/rate, Fault/Contract, target/compiler,
   Unicode/CRLF/BOM, migration, determinism, and interpreter/VM/Native
   differential fixtures.

## Evidence and compatibility impact

The eventual implementation must consume checked Typed Core only after the
Node Core authority is accepted and must reject unknown or unsupported graph
semantics before execution. Graph IDs, facts, and diagnostics must preserve
original UTF-8 spans and Semantic IDs while excluding host paths, addresses,
hash order, scheduler timing, and debug rendering from Ling identity.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, scheduler, Task, Actor, Native backend, memory or
ownership behavior, diagnostics, schemas, Semantic IDs, source spans, CLI,
LSP, dependency lock, target/toolchain support claim, or Unicode 17.0.0
behavior.

## Intentionally deferred

NODE-5302 implementation, Node Checked Core schema/lowering, graph and
fixed-point analysis, diagnostics, CLI/LSP/evidence protocols, and support
claims remain deferred until RFC-K502 (or an Accepted replacement),
`GAP-CRITICAL-PROFILE-001`, and the dependent ownership, concurrency,
Native/ABI, Kernel/Device, numeric/effect, and transaction authorities are
resolved with independent offline fixtures. No placeholder Node Core variant,
cycle solver, or public API is created.
