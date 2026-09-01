# REP-2501 Authority Audit: Determinism Class

## Outcome

`REP-2501` is `Ready` only for the bounded private baseline authorized by
Accepted DEC-0279. Its implementation dependency is satisfied: SUP-2403 is
Done under Accepted DEC-0278. `SEMANTICS.md` section 22.1 names five category
shapes, while the lower-authority G2 plan proposes four different provisional
labels and public metadata placement. The repository still has no Accepted
classification relation, public claim/inference rules, replay header, privacy,
or migration contract.

Accepted DEC-0279 defines the smallest honest next slice: a private executable
five-case matrix over existing Accepted checked, Task, and Actor execution
routes. It deliberately does not alias the two naming sets, classify programs,
or create build metadata, Semantic Graph fields, or a Replay protocol.

No determinism enum, class inference, build-metadata field, Semantic Graph
field, replay header, diagnostic, protocol, or placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative. Its “minimum recommended” class
  names cannot freeze a public classification, metadata field, or replay ABI;
  the plan itself says final names are subject to an RFC.
- REP-2501's registered dependency SUP-2403 is Done. The plan also requires
  RFC-C205, but no Accepted RFC-C205 or replacement RFC-0010 exists; RFC-0001
  remains a Draft baseline under DEC-0018, and the deterministic replay gap
  blocks REP-2501 through REP-2506.
- `docs/SEMANTICS.md` sketches determinism classes and Actor replay fields, but
  does not fix class ordering/meaning, inference versus declaration, effect and
  scheduler boundaries, equivalence, version migration, privacy, or
  divergence handling. v0.0.1 implements only the Seed Core subset and no
  replay runtime.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require explicit effects,
  deterministic observable behavior, and replay privacy/version boundaries,
  but do not define a stable class schema or user-facing compatibility rule.
- Accepted DEC-0021 defines deterministic scheduling only for independent
  internal compiler queries. Accepted DEC-0267 defines a private seeded Task
  test trace and strict in-process replay; DEC-0268 explicitly leaves
  production Task Effect/Fault order unconstrained; DEC-0274 through DEC-0278
  provide explicit private Actor/Supervisor scripts and reconstruction
  evidence. None defines a public determinism class, Effect Log, replay header,
  or cross-process equivalence.
- `GAP-DETERMINISTIC-REPLAY-001` remains Open, with alternative canonical-log
  versus higher-level-event designs and required positive/negative/migration,
  cross-process, corruption, privacy, and divergence evidence.

## Current implementation evidence

- The workspace has no production determinism-class model, public effect
  recorder/log/header/player, or cross-process comparison tool. `ling-eval`
  does have the Accepted DEC-0267 internal `TaskScheduleTrace`, seeded driver,
  strict replay, and DEC-0268 production scheduler, plus explicit checked Actor
  runtime/Supervisor drivers under DEC-0274 through DEC-0278.
- `ling-semantic` has no accepted determinism-class Semantic Graph node or
  build/replay metadata projection. `ling-effects` computes only the Seed
  closed effect rows and module Capability closure.
- No public protocol inventory entry, schema, diagnostic, or fixture defines
  class claims, unrecorded scheduling, external-effect boundaries, or replay
  divergence. Compiler-query scheduling evidence from DEC-0021 is intentionally
  internal and cannot be reused as runtime replay evidence.

## Accepted implementation boundary

Accepted DEC-0279 supplies only the minimum private evidence boundary:

1. exactly five test-local case families use the category shapes already named
   by `SEMANTICS.md` section 22.1 without freezing a production enum;
2. pure checked execution, DEC-0267 seeded scheduling and strict trace replay,
   explicit DEC-0274/DEC-0276/DEC-0277 Actor scheduling, and the DEC-0268
   nondeterministic production boundary are driven directly;
3. each case fixes exact checked inputs, arguments, limits, host/schedule inputs,
   allowed projections, and forbidden observations;
4. DEC-0104's four provisional plan labels remain separate and no public class,
   build/Semantic Graph/header field, Effect Log, diagnostic, or protocol is
   added; and
5. REP-2501 completes only for the internal Experimental baseline after the
   exact matrix, negative inventory, full gates, commit binding, and status
   synchronization pass.

Public classification still requires an Accepted RFC defining names/parameters,
ordering/composition, inference/declaration, equivalence, Effect/scheduler
boundaries, metadata/versioning, privacy, integrity, diagnostics, migration,
and cross-process evidence.

## Evidence and compatibility

This refreshed audit and Accepted DEC-0279 were checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, DEC-0018,
DEC-0021, DEC-0104, DEC-0267, DEC-0268, DEC-0274 through DEC-0278, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
replay, diagnostic, schema, Semantic ID, source-span, runtime, or Unicode
17.0.0 behavior changed.

## Intentionally deferred

`REP-2501` may proceed only as Accepted DEC-0279's private five-case executable
baseline. Public determinism classification still requires
Accepted RFC-C205/RFC-0010 (or a replacement) to resolve class semantics,
effect/scheduler boundaries, replay identity, privacy, corruption, divergence,
metadata, migration, and cross-process evidence. REP-2502 through REP-2506,
public Replay, and cross-backend claims remain blocked.
