# STAB-6102 Authority Audit

- Task: `STAB-6102` — Remove False Entry Points
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:51-61`
- Release: G6
- Status: `BlockedSpec`

## Decision

STAB-6102 is `BlockedSpec`. The G6 checklist asks the project to find empty
implementations, successful placeholder APIs, undocumented commands, unusable
Zed completions, unsupported backends/profiles, future defaults, and syntax
aliases without RFC authority. It does not define the complete public-surface
inventory, the distinction between an intentional Unsupported/Unavailable
surface and a false entry, or whether a finding must be deleted, hidden,
deprecated, migrated, or retained as explicit negative evidence. The task also
depends on the unfinished G1-G5 exits and STAB-6101 support-matrix audit.

Deletion or hiding without those decisions could remove valid Seed behavior,
break conformance/editor recovery, or silently change a documented rejection
into a compatibility gap. No destructive cleanup is authorized by the
non-normative checklist alone.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:51-61` is a non-normative search list. It does
  not define false-entry classification, public API ownership, deprecation or
  migration semantics, or the required evidence for removing an entry.
- The root `AGENTS.md` repository rules prohibit placeholder public APIs and
  stale `zero` surfaces, but
  also require preserving accepted `ling`/`.ling` behavior, checked Typed Core
  execution, stable diagnostics, and higher-authority specifications. These
  rules do not authorize deleting accepted parser recovery, negative fixtures,
  or internal implementation placeholders that are not public capabilities.
- `docs/ROADMAP-1.0.md:500-573` makes G6 cleanup conditional on G1-G5 exits,
  requires explicit Preview/Experimental controls, and requires stable
  compatibility and migration evidence. It is planning authority, not a
  deletion manifest.
- `docs/ROADMAP-1.0.md:14-23` and `docs/IMPLEMENTATION.md:17` require that
  unsupported or future capabilities are not implied and that Seed exclusions
  remain honest. They do not turn every future mention, recovery node, or
  negative fixture into a false entry.
- The support matrix deliberately records `Experimental`, `Preview`,
  `Unavailable`, `Unsupported`, and `Future` states. Native/Critical profiles,
  unsupported backends, and future protocols must remain explicit negative
  evidence until accepted decisions change them; deleting those records would
  overclaim support.
- The protocol inventory requires help/version output to describe only
  implemented commands and records Future protocols without placeholder
  writers. The accepted CLI and diagnostic contracts still require bilingual
  stable errors, original UTF-8 spans, and truthful rejection behavior.
- The Tree-sitter grammar map and editor corpus intentionally represent
  post-Seed forms as unavailable or recoverable syntax rather than successful
  features. An editor token, completion, recovery marker, or internal
  `Text.format` placeholder is not automatically a public false entry.

## Evidence in this repository

Targeted searches found no `todo!()`, `unimplemented!()`, or `unreachable!()`
constructs under `crates/`, `editors/`, `tests/`, or `schemas`. The scoped CLI,
editor, schema, and fixture paths contain no stale `zero` command entry. The
remaining occurrences of “placeholder” are legitimate `Text.format` fault
semantics, type-inference temporaries, recovery/editor documentation, or
governance notes describing intentionally Future/Unsupported surfaces. Current
support fixtures explicitly encode Unavailable/Unsupported/Future states, and
the existing CLI/help and grammar behavior provide no audited STAB-6102
deletion set. These observations are evidence for the audit, not permission to
remove arbitrary source or documentation.

## Required authority before implementation

An accepted stabilization decision must define, at minimum:

1. A complete public-surface inventory covering CLI commands/help, library
   APIs, diagnostics, schemas/protocols, profiles/backends, build defaults,
   grammar/highlighting/completion, and documentation; each item needs an
   owner, state, authority, and compatibility status.
2. A deterministic classification for false, implemented, Experimental,
   Preview, Unavailable, Unsupported, Deprecated, and Removed entries,
   including how internal recovery nodes, negative fixtures, examples, and
   future-plan text are excluded from public capability claims.
3. Deletion/hiding/deprecation/migration rules with source and byte-span
   compatibility, diagnostic changes, Semantic ID/canonical-byte impact,
   package/build metadata consequences, and release-note requirements.
4. Accepted RFC/decision links for every remaining syntax alias, command,
   profile/backend, protocol, editor capability, and default. Missing
   authority must produce an explicit stable rejection or omission rather than
   a success path.
5. Offline positive, negative, malformed, help/completion, grammar/editor,
   unsupported-capability, migration, and repeated-build fixtures, with
   bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and deterministic ordering.
   CI must fail on newly introduced false success paths without treating
   intentional negative evidence as a false entry.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, support
claim, editor grammar, or Semantic ID rule. It preserves the accepted `ling`
CLI and `.ling` source extension, Seed parser/recovery behavior, original
UTF-8 spans, Unicode 17.0.0, deterministic identity rules, and explicit
Experimental/Preview/Unavailable/Unsupported states.

It deliberately deletes or hides no file, command, API, grammar symbol,
completion, backend/profile, default, diagnostic, protocol, or fixture, and
introduces no stale `zero` names or placeholder API. STAB-6102 remains deferred
until STAB-6101 and G1-G5 exits establish an authoritative inventory,
classification, migration policy, and executable evidence for each proposed
cleanup.
