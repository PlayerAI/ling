# STD-6302 Authority Audit

- Task: `STD-6302` — Remove Convenience APIs
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:180-193`
- Release: G6
- Status: `BlockedSpec`

## Decision

STD-6302 is `BlockedSpec`. The G6 checklist asks reviewers to examine
implicit I/O/Clock/Random, unbounded collections, a default global runtime,
implicit network retries, dynamic reflection, unclear encoding, FFI helpers
without ownership, and core operations without complexity guarantees. It then
suggests keeping immature APIs in a Preview package rather than `core`. The
checklist does not identify a current public API inventory, select a core or
Preview package boundary, define deprecation/removal compatibility, or
authorize deleting any existing symbol.

Higher-authority Seed decisions already keep the implemented surface small:
DEC-0011 explicitly limits built-ins and requires another RFC for extensions;
DEC-0014 defines only the injected `Ling.Prelude` `Option`/`Result` surface.
Language and semantics documents reject ambient implicit I/O/network and
arbitrary reflection, but those exclusions are not a license to remove or
rename symbols without an accepted API lifecycle. No current Stable standard
library exists from which this task could safely delete convenience APIs.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:180-193` is a non-normative review checklist.
  It names risk categories and a desired Preview/core distinction but does
  not define a removal set, API version, migration, or diagnostic behavior.
- `docs/LANGUAGE.md:1195-1207` rejects ambient global mutable state, untyped
  exceptions, implicit I/O and network, arbitrary reflection, and other old
  mechanisms at the language-design boundary. It does not authorize removing
  an accepted Seed name or changing an existing public signature.
- The accepted semantics model uses explicit Effects and Capabilities for
  host actions and leaves Clock/Random/Network features outside the Seed
  surface. A future operation must be specified before it can be classified
  as a convenience API or a core API.
- Accepted `docs/decisions/0011-seed-builtins.md` fixes the six Seed built-ins
  and their types, Effects, Capability behavior, evaluation, determinism, and
  Runtime Fault boundary. It says extensions require another RFC.
- Accepted `docs/decisions/0014-seed-prelude-option-result.md` fixes the
  logical `Ling.Prelude` `Option`/`Result` definitions, identity, and
  namespace behavior; it does not establish a packaged Preview/core split.
- Accepted DEC-0001 keeps diagnostic code meanings and says deleted features
  retain their codes as deprecated rather than reusing numbers. That policy
  does not define source-symbol deprecation, migration tooling, or API
  deletion authority.
- `docs/governance/support-matrix.toml` records `STD-LING-PRELUDE` as
  `BuiltinOnly`, `Preview`, and un-packaged; the support registry is Draft
  with `stable_basis = false`. There is no Stable `core` package inventory.
- Root `AGENTS.md` requires accepted authority before public API changes,
  stable claims only with ROADMAP gates and executable fixtures, deterministic
  offline behavior, bilingual diagnostics, Unicode 17.0.0, checked Typed
  Core inputs, preserved spans, and no stale `zero` surfaces.

## Evidence in this repository

Current resolver and evaluator evidence covers the deliberately bounded Seed
built-ins and logical Prelude. Governance fixtures classify that surface as
Preview/BuiltinOnly rather than Stable. The repository has no accepted public
symbol inventory marking an existing API as an unsafe convenience, no
versioned core-versus-Preview package contract, and no migration/deprecation
fixtures for removing or renaming symbols.

The language-level negative rules provide evidence against implicit effects
and ambient authority, but they do not identify a concrete API to delete.
Removing a name based only on the plan would risk changing accepted typing,
Effects, Capability propagation, diagnostics, Semantic IDs, or source
compatibility without a governing decision.

## Required authority before implementation

An accepted API-lifecycle and standard-library decision must define, at
minimum:

1. A complete symbol inventory with public/internal status, package and
   profile ownership, current version, stability, and the rationale for
   `core`, `Preview`, `Experimental`, or unsupported classification.
2. The exact removal/deprecation set and process: diagnostic codes and
   bilingual messages, source compatibility window, migration or replacement
   guidance, version transition, lock/package impact, and no silent semantic
   change or name reuse.
3. Per-symbol type, Effect, Capability, Fault, ownership/kind, complexity and
   resource bounds, determinism, termination/panic, Unicode/locale, and host
   authority behavior. Implicit I/O, network, time, randomness, reflection,
   unbounded allocation, and unclear FFI must remain explicit rejected or
   separately specified surfaces.
4. Profile availability and offline distribution rules, including how a
   removed Preview symbol is rejected and how checked Typed Core and Semantic
   identity boundaries remain unchanged.
5. Positive and negative conformance, migration/deprecation, diagnostics,
   capability/effect, resource, determinism, Unicode 17.0.0, and
   cross-process fixtures, plus generated support/protocol/status drift
   checks.

## Compatibility and deferred work

This audit changes no source symbol, built-in, Prelude definition, type,
Effect, Capability, Fault, evaluator, resolver, diagnostic, package, profile,
CLI, protocol, dependency, or public API behavior. It preserves the accepted
Seed exclusions for implicit authority, the six DEC-0011 built-ins,
`Ling.Prelude` injection, `ling`/`.ling` naming, original UTF-8 spans, Unicode
17.0.0, deterministic/offline requirements, and explicit Preview/Future/
Unsupported states.

It deliberately removes no API and adds no replacement, deprecation warning,
core/Preview package, migration tool, diagnostic, dependency, convenience
wrapper, public protocol, or placeholder, and introduces no stale `zero`
names. Future work may classify or remove a symbol only after the API and
standard-library lifecycle authority is Accepted and its migration and
negative evidence are executable. Implementations must continue to consume
checked Typed Core data and must not expose host paths, allocation, locale
defaults, or map order as library semantics.
