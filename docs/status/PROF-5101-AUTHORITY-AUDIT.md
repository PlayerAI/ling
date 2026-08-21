# PROF-5101 Authority Audit — Machine-Readable Critical Profile

Status: BlockedSpec

Date: 2026-08-22

## Outcome

PROF-5101 proposes a machine-readable Critical Profile describing language and
specification versions, compiler ranges, standard-library set, target
architecture, scheduler, allowed effects/capabilities, memory and numeric
policies, concurrency, FFI target packages, and verification requirements.

The plan marks RFC-K501 as a dependency, but no RFC-K501 exists or is Accepted.
The current authority instead records the minimum Critical Core/Profile and
evidence boundaries as the open `GAP-CRITICAL-PROFILE-001`, with RFC-0012 only
as a candidate. The profile example is therefore not an implementable schema,
and adding a parser, file extension, CLI option, or public JSON/TOML contract
would fix unresolved safety and compatibility semantics by implementation.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:77-106` is a
  non-normative plan fragment. It lists fields and an illustrative TOML shape,
  but does not define field types, defaults, canonical bytes, versioning,
  compatibility, inheritance, target/package identity, or proof claims.
- `docs/ROADMAP-1.0.md:118` makes G5 depend on G2 replay, G3 resources, and G4
  restricted lowering; its Critical section requires explicit forbidden
  capabilities and reproducible evidence but does not authorize a profile
  protocol.
- `docs/SEMANTICS.md:1973-1987` keeps the minimum verifiable Critical Core as
  an unresolved RFC question. `docs/LANGUAGE.md` describes Critical as a goal
  and a profile boundary, not as v0.0.1 behavior.
- `GAP-CRITICAL-PROFILE-001` is Open, blocks PROF-5101 and the downstream G5
  tasks, and names RFC-0012 as a candidate. The support matrix marks Critical
  unavailable and non-selectable. No RFC-K501 or accepted profile schema is in
  the authority index or protocol inventory.
- The accepted CLI/source names remain `ling` and `.ling`; no profile command,
  file, or public transport may be inferred from the plan's examples.

## Current implementation evidence

- The repository has no Critical Profile reader/writer, schema registration,
  profile identity, inheritance/composition model, version-range evaluator,
  capability/effect policy, memory/numeric/concurrency policy, FFI package
  contract, or verification-requirement checker under `crates` or `tests`.
- The support matrix contains a governance record that Critical is unavailable;
  it is not a selectable runtime or compiler profile and provides no stable
  machine-readable profile payload.
- No accepted rule fixes whether a profile is source-authored, project
  metadata, compiler configuration, or release evidence; how it binds to
  Semantic IDs, bytecode/Native artifacts, targets and standard libraries; or
  how unknown fields, defaults, upgrades, and conflicts are handled.
- No stable diagnostics or fixtures define rejection for unsupported effects,
  memory/GC, numeric/concurrency, FFI, scheduler, target, compiler-range, or
  verification requirements.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Profile schema and lifecycle with canonical serialization,
   field types, required/optional fields, defaults, unknown-field policy,
   migration, and stable identity.
2. The exact binding between a profile, language/spec version, compiler range,
   standard-library set, target architecture, scheduler, effects/capabilities,
   memory model, numeric/concurrency policy, FFI packages, and verification
   obligations.
3. Composition/override rules, conflict precedence, profile selection and
   project/CLI configuration precedence under the accepted `ling` interface,
   plus reproducible build and artifact metadata behavior.
4. Explicit non-claims for Critical: what is forbidden, what is checked versus
   assumed, how Resource/Managed/Native/Device and Fault semantics interact,
   and how replay, timing, bounds, and evidence are represented.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts for
   malformed, unknown, incompatible, unavailable, conflicting, or unverifiable
   profile requirements.
6. Offline positive/negative, Unicode/CRLF, unknown-field, version migration,
   composition/conflict, target/package, effect/memory/numeric/concurrency,
   reproducibility, and independent-verification fixtures.

## Evidence and compatibility impact

The eventual implementation must make a Profile a checked input to the
compiler and verifier, not a cosmetic label or an implicit permission to add
Critical features. It must preserve source spans and Semantic IDs, keep
diagnostic text separate from stable machine fields, and never claim proof or
target support from a profile declaration alone.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, ownership behavior, diagnostics,
schemas, Semantic IDs, source spans, CLI, dependency lock, target/toolchain,
support claim, or Unicode 17.0.0 behavior.

## Intentionally deferred

PROF-5101 implementation, Critical Profile syntax/schema, reader/writer,
selection/composition, diagnostics, CLI/LSP/editor integration, support claims,
and evidence-bundle fields remain deferred until RFC-0012 (or an Accepted
replacement for RFC-K501), `GAP-CRITICAL-PROFILE-001` is resolved, G2/G3/G4
prerequisites are accepted, and executable offline fixtures exist. No
placeholder profile crate or public API is created.
