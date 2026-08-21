# BND-5201 Authority Audit — Bound Types and Expressions

Status: BlockedSpec

Date: 2026-08-22

## Outcome

BND-5201 proposes bound forms for compile-time constants, Profile parameters,
range types, collection capacity, loop trip counts, recursion depth,
Task/Actor counts, stack/arena budgets, and message size. The plan requires
RFC-K504, but no RFC-K504 exists or is Accepted.

Bounds affect source typing, termination, resource safety, concurrency,
allocation, numeric overflow, Device behavior, and Critical claims. The plan's
list is not enough to select syntax, Typed-Core nodes, units, arithmetic,
proof/assumption states, or diagnostics. Implementing it would invent
irreversible semantics.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:138-155` is a
  non-normative plan fragment. It names categories but defines no grammar,
  type rules, units, constant evaluation, symbolic relations, overflow,
  profiles, or evidence status.
- `docs/ROADMAP-1.0.md:118` makes G5 depend on replay, resource/ownership, and
  restricted lowering boundaries. Its Critical goals require boundedness and
  reproducible evidence, but do not authorize a Bound language feature.
- `GAP-CRITICAL-PROFILE-001` is Open and explicitly includes boundedness;
  `GAP-ACTOR-MAILBOX-SUPERVISOR-001`, `GAP-OWNERSHIP-MODEL-001`,
  `GAP-KERNEL-DEVICE-001`, `GAP-NATIVE-BACKEND-ABI-001`, and numeric/effect
  gaps leave the dependent resource and execution meanings unresolved.
- `docs/SEMANTICS.md` lists Critical minimum Core as an unresolved RFC issue
  and v0.0.1 does not accept Bound syntax or resource-budget semantics.
  RFC-K504 is only a plan placeholder; no accepted schema or protocol exists.

## Current implementation evidence

- The compiler has no Bound syntax, AST/HIR/Typed-Core representation,
  constraint solver, constant/profile parameter model, range/capacity checker,
  termination/resource integration, or bound fixtures under `crates` or
  `tests`.
- Existing parser/type/bytecode safety limits and the type solver's internal
  recursion-depth guard protect implementation inputs; they are not Ling
  source-level bounds, proof claims, or resource budgets.
- No accepted rule defines inclusive/exclusive ranges, units and conversions,
  Nat/Int arithmetic, overflow/underflow, unknown or symbolic bounds,
  multiplication/addition of capacities, dependent collection lengths,
  recursion/task/message relationships, or target/profile limits.
- No diagnostic allocation or provenance schema fixes whether a bound is
  proved, statically bounded, runtime guarded, assumed, forbidden, or unknown.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. Bound grammar and checked representation, including constant expressions,
   Profile parameters, range/capacity types, units, domains, variance,
   arithmetic, overflow, unknown values, and canonical serialization.
2. Soundness rules for collection, loop, recursion, Task/Actor, stack/arena,
   message, buffer, and device bounds, with explicit relationships to
   ownership, effects/capabilities, scheduler, cancellation, Fault, and
   fallback semantics.
3. Proof and runtime states (for example statically bounded, proved
   terminating, runtime guarded, forbidden/unknown), trust boundaries,
   assumptions, limits, and when a code action may transform a program without
   silently changing semantics.
4. Profile selection and composition rules for allowed bounds, target/runtime
   budgets, configuration precedence, migration, deterministic evaluation,
   and build/cache/replay/evidence identity.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics with source spans and
   structured facts for malformed, inconsistent, overflowing, unprovable,
   exceeded, unavailable, or conflicting bounds.
6. Offline positive/negative, boundary arithmetic, Unicode/CRLF, symbolic and
   unknown bounds, recursion/loop, collection/message/task, profile/target,
   runtime-guard, deterministic, migration, and differential fixtures.

## Evidence and compatibility impact

The eventual implementation must reject unsound claims rather than infer a
bound from host limits or optimizer behavior. It must consume checked Typed
Core, preserve original UTF-8 byte spans and Semantic IDs, keep proof/evidence
metadata distinct from language semantics, and never expose Rust allocation,
paths, timestamps, or debug order as bound identity.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
diagnostics, schemas, Semantic IDs, source spans, CLI, dependency lock,
target/toolchain, support claim, or Unicode 17.0.0 behavior.

## Intentionally deferred

BND-5201 implementation, Bound syntax/types, expressions and solver,
termination/resource/effect integration, diagnostics, CLI/LSP/editor support,
and public protocol claims remain deferred until RFC-K504 (or an Accepted
replacement), `GAP-CRITICAL-PROFILE-001`, concurrency/ownership/Kernel/Native
prerequisites, and executable offline bound evidence are Accepted. No
placeholder bounds crate or public API is created.
