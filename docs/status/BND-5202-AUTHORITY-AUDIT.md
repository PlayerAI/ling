# BND-5202 Authority Audit — Loop and Recursion Checks

Status: BlockedSpec

Date: 2026-08-22

## Outcome

BND-5202 proposes four analysis states—`StaticallyBounded`,
`ProvedTerminating`, `RuntimeGuarded`, and `Forbidden/Unknown`—and makes the
allowed states Profile-dependent. It also suggests an explicit code action
that converts recursion to a bounded work queue without silently changing
semantics.

No RFC-K504 or accepted Critical termination calculus defines these states,
their proof obligations, or their relation to loops, recursion, Task/Actor
mailboxes, effects, resources, Faults, or scheduling. A transformation to a
work queue changes stack, allocation, ordering, cancellation, and resource
behavior; it cannot be implemented as an ordinary refactor or inferred from a
diagnostic.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:156-167` is a
  non-normative plan fragment. It does not define a termination logic, ranking
  functions, loop/recursion syntax, soundness theorem, runtime guard, state
  transitions, or code-action transaction.
- `docs/ROADMAP-1.0.md:118` requires G5 boundedness and reproducible evidence
  after G2 concurrency and G3 resources; it does not authorize a termination
  checker or program transformation.
- `GAP-CRITICAL-PROFILE-001` explicitly leaves boundedness and Critical claims
  Open. `GAP-ACTOR-MAILBOX-SUPERVISOR-001`, ownership/resource, effect,
  deterministic-replay, and numeric/Kernel gaps leave dependent liveness and
  resource behavior unresolved.
- Accepted RFC-0015 covers bytecode closure/recursion lowering and bounded VM
  frame/heap runtime Faults only. It explicitly leaves a common logical heap
  formula, Task/Actor, Native, and later Critical rules out of scope; it does
  not prove source termination or authorize a work-queue rewrite.
- No RFC-K504/RFC-0012, proof/checker, code-action, LSP transaction, or profile
  protocol is Accepted for this behavior. Existing LSP transaction gaps keep
  snapshot/version/stale-edit safety unresolved.

## Current implementation evidence

- The compiler has no loop/recursion termination analysis, ranking-function or
  size-change engine, Bound state in Typed Core, runtime guard contract, or
  termination fixtures under `crates` or `tests`.
- Existing recursive bytecode execution uses explicit VM frames and
  `frame_limit` Runtime Faults under RFC-0015; that runtime resource limit is
  not a source-level proof state and does not establish termination.
- No accepted rule defines termination for higher-order calls, mutual
  recursion, exceptions/Faults, effects, generators, data-dependent loops,
  concurrency, mailbox/backpressure, or Device/Native operations.
- No stable diagnostic or transaction schema fixes proof provenance, ranking
  evidence, guard placement, state transitions, transformation preconditions,
  source-map preservation, or user confirmation for a work-queue code action.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A sound termination/boundedness calculus for loops, direct/mutual and
   higher-order recursion, data/size relations, effects, Faults, and
   concurrency, including ranking functions, assumptions, unsupported cases,
   and proof/checker trust boundaries.
2. Exact definitions and transitions for `StaticallyBounded`,
   `ProvedTerminating`, `RuntimeGuarded`, and `Forbidden/Unknown`, including
   Profile policies, runtime guard semantics, limits, failure categories, and
   evidence/provenance.
3. Resource and scheduling relationships for stack/heap/arena, Task/Actor
   counts, mailbox/backpressure, cancellation, ordering, Device/Native
   execution, numeric determinism, and fallback; no host limit may be treated
   as a language proof.
4. A semantics-preserving transformation specification for recursion to a
   work queue, including eligibility, state/ownership/effect equivalence,
   ordering, allocation, cancellation, Fault, source maps, user consent,
   rollback, and canonical output under an accepted `ling` transaction/CLI
   protocol.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts
   for unbounded/unknown/failed proofs, guard insertion, limit exhaustion,
   unsupported recursion, and rejected transformations.
6. Offline positive/negative, proof-counterexample, loop/recursion,
   higher-order, effect/resource, concurrency/mailbox, runtime-guard,
   transformation-equivalence, source-map/Unicode, determinism, migration,
   and differential fixtures.

## Evidence and compatibility impact

The eventual implementation must consume checked Typed Core and fail closed on
unknown or unproved termination; it must never present a runtime frame limit as
proof. Any work-queue action must be an explicitly reviewed semantic
transaction, preserve original UTF-8 spans and Semantic IDs, and prove the
declared equivalence rather than silently changing execution.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
diagnostics, schemas, Semantic IDs, source spans, CLI, LSP, dependency lock,
target/toolchain, support claim, or Unicode 17.0.0 behavior.

## Intentionally deferred

BND-5202 implementation, termination calculus/checker, Profile states, runtime
guards, work-queue transformation, diagnostics, CLI/LSP/code actions, and
public protocol claims remain deferred until RFC-K504 (or an Accepted
replacement), `GAP-CRITICAL-PROFILE-001`, concurrency/ownership/effect,
replay, and transaction authorities are resolved with independent proof,
counterexample, equivalence, and offline fixtures. No placeholder checker or
transformer API is created.
