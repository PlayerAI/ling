# OWN-3206 Authority Audit: Ownership Diagnostics and Repairs

## Outcome

`OWN-3206` is correctly recorded as `BlockedSpec`. The G3 plan requires
ownership diagnostics to expose resource origin, move/borrow start, conflicting
use, region boundary, persistence of the conflict, and ranked fixes, with JSON
Repair data suitable for LSP code actions rather than editor parsing of human
text. The diagnostic facts and repair schema are only implementable after the
ownership, lifetime, memory, suspension, and FFI semantics are accepted.

No future ownership diagnostic, error-code allocation, repair ranking,
ownership-specific JSON field, LSP code action, protocol, or placeholder G3 API
was added.

## Normative traceability

- The G3 execution package is non-normative. Its diagnostic checklist cannot
  authorize error meaning, repair ranking, source projections, or LSP protocol
  fields.
- OWN-3206 depends on OWN-3201 through OWN-3205 and the missing RFC-N305/
  RFC-0007 ownership/diagnostic authority. No accepted RFC-N305 or RFC-0007
  exists; RFC-0001 remains a Draft under DEC-0018.
- `docs/ERROR-CODES.md` and accepted DEC-0001/DEC-0002 authorize the existing
  bilingual diagnostic registry, stable code meanings, byte spans, Facts, and
  structured Repair candidates. They do not authorize future ownership codes,
  facts, ranking semantics, or LSP code-action behavior without the underlying
  ownership rules.
- Accepted DEC-0009 and RFC-0017 cover only Seed value/mutable-place
  diagnostics and lowering. They do not define move/borrow/region/resource
  origins, conflicts, or repairs.
- `GAP-OWNERSHIP-MODEL-001` and `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain Open,
  leaving Copy/Move, Borrow, Region, Drop, Managed, public lifetime, and Profile
  boundaries unresolved. The relevant effect, Actor/await, Native, and FFI
  authorities are also not Accepted.
- `docs/SEMANTICS.md`, `docs/LANGUAGE.md`, and `docs/ROADMAP-1.0.md` require
  stable bilingual diagnostics and explicit ownership/region/FFI boundaries,
  but do not define future ownership error semantics or a repair ranking.

## Current implementation evidence

- The workspace has a versioned Preview Diagnostic JSON container and generic
  Facts/Repair structures, but no ownership-specific code, resource-origin
  tracking, move/borrow conflict model, region boundary facts, ranked repair
  algorithm, or LSP code-action adapter for v0.3 ownership.
- Existing Seed diagnostics and accepted mutable-place repairs cannot be
  generalized to unaccepted Resource/Borrow/Region semantics. Human wording,
  JSON field order, and editor behavior cannot create language authority.
- No diagnostic or fixture defines first/second move, double drop, alias,
  partial move, region escape, suspension/Actor/Task conflict, FFI transfer,
  repair ranking, Unicode/CRLF/BOM spans, or interpreter/VM/Native differential
  behavior. No new `L-*` code has been allocated.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. Ownership error taxonomy and stable meanings for resource origin,
   move/borrow start, conflicting use, region boundary, persistence, first
   root cause, secondary facts, and deterministic ordering; define which are
   errors versus warnings and how they interact with accepted Seed codes.
2. Structured Repair schema and ranking semantics: source edits, preconditions,
   stale-span/version checks, applicability, alternatives, localization,
   safety, and how suggestions such as copy, shorten borrow, move ownership,
   or split state are proven not to change semantics.
3. Source/CST/AST/HIR/Checked Core/Graph/Audit Source and LSP mappings,
   UTF-8 byte spans to editor positions, Semantic IDs, public diagnostic
   schema/versioning, migration and compatibility, Profile/FFI/Native/
   Task/Actor boundaries, and deterministic output.
4. Stable bilingual templates, registry allocations, host/runtime Fault versus
   compile error boundaries, Unicode 17.0.0 handling, and prohibition on raw
   host/Rust debug text or unchecked-AST execution.
5. Executable positive/negative/migration/property/fuzz/LSP fixtures covering
   moves, aliases, partial moves, branches/loops/match/closures, cancellation,
   Task/Actor/await, FFI transfer, region escape, automatic-borrow ambiguity,
   repair preconditions/ranking, and interpreter/VM/Native parity.

Until those decisions are Accepted, implementing ownership diagnostics or
   repairs would freeze error meanings, source compatibility, editor behavior,
   safety, and public protocol fields that the language authority intentionally
   leaves open.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, `docs/ERROR-CODES.md`, DEC-0001,
DEC-0002, DEC-0009, DEC-0012, DEC-0013, DEC-0018, RFC-0001, RFC-0017,
`docs/ling_execution_plan/07-G3-V0.3-NATIVE.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/authority.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
Seed Place lowering, existing diagnostic registry, future ownership diagnostic,
error-code allocation, schema, Semantic ID, source-span, runtime, or Unicode
17.0.0 behavior changed.

## Intentionally deferred

`OWN-3206` can begin only after OWN-3201 through OWN-3205 and RFC-0007/RFC-N305
(or accepted replacements) define memory kinds, Copy/Move, Resource/Managed,
Borrow/Region, Drop, suspension, Effects/Faults, and FFI boundaries. The future
implementation must preserve existing diagnostic codes and schemas, consume
accepted types and checked Core only, keep repairs deterministic and safe, and
publish diagnostic, LSP, migration, source-span, and interpreter/VM/Native
evidence before exposing v0.3 ownership repairs.
