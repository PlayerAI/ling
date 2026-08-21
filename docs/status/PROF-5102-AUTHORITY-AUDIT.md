# PROF-5102 Authority Audit — Forbidden-Capability Checks

Status: BlockedSpec

Date: 2026-08-22

## Outcome

PROF-5102 proposes stable rejection before lowering for general Managed/GC,
undefined allocation, undeclared Clock/Random/IO/Network/Device, dynamic
loading/reflection/shell steps, unbounded Task/Actor topology or mailbox,
unaudited FFI, nondeterministic numeric/Placement behavior, and unhandled
Fault/fallback.

These are safety-critical language and compiler rules, not implementation-only
lint checks. PROF-5101 has no accepted Profile schema, RFC-K501 is absent, and
`GAP-CRITICAL-PROFILE-001` leaves the minimum Critical Core, forbidden
capabilities, boundedness, Fault, and evidence boundaries Open. Implementing
the list would silently choose which effects, resource models, concurrency,
numeric rules, and device behavior are legal.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:108-120` is a
  non-normative implementation list. It supplies no capability taxonomy,
  effect-to-capability mapping, source/Typed-Core representation, precedence,
  bound definition, profile selection, or diagnostic contract.
- `docs/ROADMAP-1.0.md:118` and its G5 section require a Critical boundary
  with explicit forbidden capabilities and reproducible evidence, but do not
  authorize a checker or define the rejection semantics.
- `docs/SEMANTICS.md:1973-1987` keeps the minimum verifiable Critical Core as
  an unresolved RFC question. `docs/LANGUAGE.md` describes Critical goals and
  examples, while v0.0.1 Seed evaluation does not expose the proposed Native,
  Device, Task/Actor, FFI, or boundedness surface.
- `GAP-CRITICAL-PROFILE-001` is Open and blocks PROF-5102; it names RFC-0012
  only as a candidate. The support matrix marks profile selection/enforcement
  and Critical capabilities unavailable. `GAP-OWNERSHIP-MODEL-001`,
  `GAP-KERNEL-DEVICE-001`, `GAP-NATIVE-BACKEND-ABI-001`, and the concurrency
  gaps also leave the listed categories unresolved.
- No Accepted RFC-K501/RFC-0012 or registered profile/capability protocol
  exists. The plan cannot create stable diagnostics or a new `ling` command.

## Current implementation evidence

- The compiler has no Critical profile context, capability policy, forbidden
  effect table, boundedness checker, pre-lowering rejection pass, or
  source-to-capability explanation fixtures under `crates` or `tests`.
- Existing Seed effect/type checks are not a Critical policy: they cannot be
  reinterpreted as prohibitions on future Managed, Resource, Task/Actor,
  Device, Native, FFI, numeric, or timing constructs.
- No accepted rule defines whether a violation is a parse/type/effect/profile/
  lowering diagnostic, how a declared capability is consumed, how transitive
  calls and target packages are checked, or how unknown/experimental effects
  behave in Explore, Native, and Critical profiles.
- No stable diagnostic allocation, repair schema, source-span mapping, or
  deterministic ordering exists for the proposed rejection categories.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned capability/effect taxonomy and the exact Critical profile
   policy for Managed/GC, allocation, Clock/Random/IO/Network/Device, dynamic
   loading/reflection/shell, Task/Actor topology/mailboxes, FFI, numeric and
   Placement determinism, Fault, and fallback.
2. The checked Typed-Core representation and phase at which each prohibition
   is enforced, including imports, higher-order calls, generated code,
   standard-library and target-package declarations, and transitive summaries.
3. Bound, scheduler, resource, numeric, and device contracts, with explicit
   distinction between forbidden, unavailable, assumed, runtime-checked,
   proved, and experimental behavior across profiles.
4. Profile selection/configuration precedence under the accepted `ling`
   interface, deterministic conflict handling, migration, and proof/evidence
   obligations; no checker may make a profile declaration a proof.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics with structured facts
   for each forbidden capability, undeclared effect, unbounded source, invalid
   FFI/target, nondeterminism, missing Fault/fallback, and unsupported profile.
6. Offline positive/negative, transitive-summary, source-span/Unicode,
   profile-matrix, bound, effect, numeric, Fault/fallback, FFI/target,
   determinism, migration, and differential fixtures.

## Evidence and compatibility impact

The eventual checker must reject only behavior authorized as forbidden by the
accepted profile and must consume checked Typed Core, not guess from syntax or
host availability. It must preserve source spans and Semantic IDs, keep
diagnostic text separate from stable facts, and avoid exposing compiler
ownership, allocation, map order, paths, timestamps, or debug formatting as
language behavior.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
diagnostics, schemas, Semantic IDs, source spans, CLI, dependency lock,
target/toolchain, support claim, or Unicode 17.0.0 behavior.

## Intentionally deferred

PROF-5102 implementation, capability/effect policy, boundedness and topology
checks, profile enforcement, diagnostics, CLI/LSP/editor integration, and
support claims remain deferred until RFC-0012 (or an Accepted RFC-K501
replacement) resolves `GAP-CRITICAL-PROFILE-001` and the ownership,
concurrency, Kernel/Device, and Native/ABI prerequisites, with executable
offline fixtures. No placeholder checker or public API is created.
