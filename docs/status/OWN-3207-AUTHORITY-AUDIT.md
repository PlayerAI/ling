# OWN-3207 Authority Audit: Negative Corpus and Property Tests

## Outcome

`OWN-3207` is correctly recorded as `BlockedSpec`. The G3 plan lists negative
and property coverage for use-after-move, double Drop, mutable aliases, partial
moves, match/loop/closure paths, cancellation, Task/Actor boundaries, FFI
transfers, region escapes, and automatic-borrow mistakes. These tests can only
be normative evidence after the ownership, memory, lifetime, cancellation,
Actor, and FFI semantics they assert are accepted.

No future ownership corpus, property generator, fuzz target, expected
diagnostic, error-code allocation, protocol, or placeholder G3 API was added.

## Normative traceability

- The G3 execution package is non-normative. Its coverage list cannot define
  legal/illegal programs, oracle outcomes, diagnostic meanings, or property
  equivalence.
- OWN-3207 depends on OWN-3201 through OWN-3206 and the missing RFC-N306/
  RFC-0007 ownership authority. No accepted RFC-N306 or RFC-0007 exists;
  RFC-0001 remains a Draft under DEC-0018.
- `GAP-OWNERSHIP-MODEL-001` and `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain Open,
  leaving Copy/Move, Borrow, Region, Drop, Managed, public lifetime, and
  Profile boundaries unresolved. `GAP-STRUCTURED-TASK-001`,
  `GAP-ACTOR-AWAIT-REENTRY-001`, and `GAP-NATIVE-BACKEND-ABI-001` likewise
  leave cancellation, Actor, and FFI/native oracles unresolved.
- Accepted DEC-0009 and RFC-0017 provide only Seed Value/mutable-place
  behavior. Existing conformance and VM differential tests cannot be
  repurposed as future Resource/Borrow/Region/Actor/FFI property oracles.
- `docs/SEMANTICS.md`, `docs/LANGUAGE.md`, and `docs/ROADMAP-1.0.md` require
  positive/negative/property/differential evidence and stable diagnostics, but
  expressly defer the underlying v0.3 semantics to RFCs.

## Current implementation evidence

- The workspace has Seed conformance, VM differential, diagnostic, parser,
  and project property/fuzz coverage, but no Resource/Managed/Move/Borrow/
  Region checker or future ownership corpus. It cannot generate valid or
  invalid ownership programs with authoritative outcomes.
- No fuzz target or property oracle defines use-after-move, double Drop,
  mutable alias, partial move, match/loop/closure ownership, cancellation,
  Task/Actor, FFI transfer, region escape, or automatic-borrow misclassification.
  Rust panic/borrow behavior and host VM cancellation are not language oracles.
- No expected future diagnostic, repair ranking, schema, Semantic ID,
  source-span, determinism, Unicode/CRLF/BOM, or interpreter/VM/Native result
  has been registered for these cases.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. The legal/illegal corpus oracle for Value/Managed/Resource, Copy/Move,
   Borrow/Region/Drop, aliases, partial moves, pattern/match/loop/closure,
   cancellation, Task/Actor/await, FFI, automatic borrow, public lifetimes,
   and Profile behavior.
2. Property generators, shrinking and bounds, state-machine/interleaving
   models, failure/cancellation/restart semantics, deterministic seeds,
   resource limits, and a clear separation between language invariants,
   implementation robustness, and host failures.
3. Expected bilingual diagnostics, stable code/Facts/Repairs, UTF-8 byte-span
   and Unicode 17.0.0 behavior, Semantic IDs/Graph/Audit Source, schema and
   protocol versions, migration, interpreter/VM/Native/FFI differential
   equivalence, and no unchecked-AST evaluation.
4. Evidence policy for positive, negative, property, fuzz, cross-package,
   region-escape, drop-order, cancellation, interleaving, FFI, sanitizer,
   hostile-input, and profile cases, including reproducibility and triage of
   shrinking failures.

Until those decisions are Accepted, adding future ownership corpus or property
tests would encode provisional semantics as test oracles and could falsely
certify unsound, nondeterministic, or incompatible behavior.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, `docs/ERROR-CODES.md`, DEC-0009,
DEC-0012, DEC-0013, DEC-0018, RFC-0001, RFC-0017,
`docs/ling_execution_plan/07-G3-V0.3-NATIVE.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/authority.toml`,
`docs/governance/protocol-inventory.toml`, and the current conformance, fuzz,
syntax, AST, HIR, types, effects, evaluator, bytecode, VM, diagnostic, and
schema tests.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
Seed Place lowering, existing test oracle, future ownership corpus, fuzz
target, diagnostic, schema, Semantic ID, source-span, runtime, or Unicode
17.0.0 behavior changed.

## Intentionally deferred

`OWN-3207` can begin only after OWN-3201 through OWN-3206 and RFC-0007/RFC-N306
(or accepted replacements), plus the Task/Actor/Native/FFI authorities, define
all asserted semantics. The future implementation must preserve existing Seed
tests, consume accepted types and checked Core only, make generators and seeds
deterministic, and publish negative, property, fuzz, interleaving, diagnostic,
profile, and interpreter/VM/Native evidence before claiming v0.3 ownership
coverage.
