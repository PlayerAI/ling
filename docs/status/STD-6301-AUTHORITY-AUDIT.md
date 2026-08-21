# STD-6301 Authority Audit

- Task: `STD-6301` — Stable Standard Library Audit
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:164-178`
- Release: G6
- Status: `BlockedSpec`

## Decision

STD-6301 is `BlockedSpec`. The G6 checklist asks for a record for every
public symbol covering type, Effect, Capability, Fault, ownership/kind,
complexity, determinism, profile availability, panic/termination, and
Unicode/locale behavior, with examples and tests. It does not define the
complete standard-library symbol set, a versioned package manifest, profile
selection, public stability rules, or how Seed built-ins and the logical
`Ling.Prelude` relate to a packaged standard library.

Accepted DEC-0011 and DEC-0014 close the Seed built-in and Prelude
Option/Result decisions, respectively. They intentionally describe a small
compiler-injected surface. The active support matrix records
`STD-LING-PRELUDE` as `BuiltinOnly`, `Preview`, and not packaged, while the
support-matrix registry itself is Draft and not a stable basis. Treating that
evidence as a frozen 1.0 standard library would invent package, profile,
version, and compatibility commitments.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:164-178` is a non-normative audit checklist.
  It enumerates metadata fields but does not authorize a symbol inventory,
  package version, profile matrix, or stable API meanings.
- Accepted `docs/decisions/0011-seed-builtins.md` defines only the Seed
  built-ins `Console.write`, `Text.format`, `max`, `min`, `map`, and `sum`,
  including their types, Effects, Capability behavior, strict evaluation, and
  Runtime Fault boundary. It explicitly requires another RFC for signature or
  semantic extensions.
- Accepted `docs/decisions/0014-seed-prelude-option-result.md` defines the
  logical injected `Ling.Prelude` types `Option` and `Result` and their
  constructors, identity/origin, namespace reservation, and no-disk-loading
  rule. It does not define a packaged library, version selection, or 1.0
  profile availability.
- `docs/governance/support-matrix.toml` records `STD-LING-PRELUDE` as version
  `0.0.1-dev`, state `BuiltinOnly`, stability `Preview`, implemented but not
  packaged, with manifest installation/version selection and registry
  distribution explicitly unsupported.
- `docs/governance/authority.toml` marks the support matrix as a Draft
  `1.0-draft` registry with `stable_basis = false`; its scope includes
  standard-package stability, profiles, protocol versions, and unsupported
  claims. It cannot authorize a Stable standard-library table.
- The active protocol inventory has no Stable standard-library protocol or
  package manifest for this task. Existing built-in and Prelude records are
  compiler semantic inputs, not a distributable compatibility surface.
- Root `AGENTS.md` requires accepted authority before public APIs and stable
  protocol claims, deterministic/offline behavior, bilingual diagnostics,
  Unicode 17.0.0, preserved spans, checked Typed Core boundaries, and no
  placeholder or stale `zero` surfaces.

## Evidence in this repository

The resolver injects the accepted built-ins and `Ling.Prelude` definitions,
and conformance tests cover their Seed typing, Effects, identity/origin,
evaluation, and fault behavior. Governance fixtures describe the Prelude as
`BuiltinOnly`/`Preview` and un-packaged. They do not provide a complete public
symbol catalog, standard-package version graph, profile availability table,
complexity contract, Unicode/locale policy, or Stable 1.0 migration corpus.

No accepted decision currently specifies a broader standard library, package
installation or version selection, locale behavior, profile-specific symbols,
resource/complexity guarantees, or compatibility policy for removing or
changing a symbol. Existing examples and tests therefore cannot be promoted
to a universal stable-library claim.

## Required authority before implementation

An accepted standard-library and profile decision must define, at minimum:

1. The complete public symbol inventory and package/module/version identity,
   including which compiler built-ins and Prelude definitions are part of the
   library contract and which remain implementation-only.
2. For every symbol, exact type, Effect row, Capability requirement, Fault and
   cancellation behavior, ownership/kind, evaluation order, complexity and
   resource bounds, determinism class, and panic/termination guarantees.
3. Profile and target availability, Unicode 17.0.0 normalization and
   locale behavior, text/encoding rules, offline distribution, dependency and
   lockfile selection, and the no-ambient-filesystem rule for injected
   definitions.
4. Versioning, unknown/missing symbol behavior, migration/deprecation policy,
   semantic identity and canonical-byte boundaries, stable bilingual
   diagnostics, and the distinction between source spans and display metadata.
5. Offline positive, negative, type/effect/capability, fault, complexity,
   determinism, profile, Unicode/locale, malformed-package, migration, and
   cross-process examples/tests, with generated support/protocol/status drift
   checks.

## Compatibility and deferred work

This audit changes no built-in, Prelude, type, Effect, Capability, Fault,
resolver, evaluator, package, profile, diagnostic, protocol, dependency,
CLI, or public API behavior. It preserves the accepted Seed surface,
`Ling.Prelude` logical-module injection, `ling`/`.ling` naming, original
UTF-8 spans, Unicode 17.0.0, deterministic/offline requirements, and the
explicit `BuiltinOnly`/`Preview`/`Future` states.

It deliberately adds no standard-library symbol, convenience wrapper,
package manifest, registry, profile selector, locale API, complexity claim,
version migration, diagnostic, dependency, or placeholder, and introduces no
stale `zero` names. Future stabilization may proceed only after the
standard-library/profile authority and support matrix are Accepted and the
per-symbol evidence is executable. Implementations must continue to consume
checked Typed Core data and must not expose host allocation, paths, locale
defaults, or map order as Ling standard-library semantics.
