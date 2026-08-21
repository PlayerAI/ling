# MC-5602 Authority Audit

Task: `MC-5602` — Exploration Engine
Release: G5
Status: `BlockedSpec`

## Outcome

`MC-5602` is not implementable from the current accepted authority. The
execution plan suggests BFS/DFS, later partial-order reduction, state hashing,
bounded depth, counterexample traces, timeout/memory bounds, and a
deterministic search mode. These algorithm choices do not define the finite
state space, transition semantics, state identity, independence relation,
counterexample format, or the distinction between a bounded result and a
proof. In particular, changing traversal order or enabling reduction can
change the first counterexample unless the ordering and reduction soundness
are specified.

The task depends on `MC-5601` and RFC-K506, both unresolved. Node/Task/Actor,
boundedness, Contract/proof, Critical profile, and evidence authorities remain
`BlockedSpec`; `GAP-CRITICAL-PROFILE-001` is open and `PROTO-EVIDENCE` is
Future without a schema or fixtures. No exploration engine, state hash,
counterexample schema, timeout/result protocol, dependency, diagnostic, or
placeholder API should be added.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:446-456` is a
  non-normative algorithm checklist. It does not define the projected model,
  transition ordering, fairness, reduction independence, canonical state
  bytes, resource charging, or result semantics.
- `MC-5601` explicitly depends on absent RFC-K506; RFC-K501/K502/K504 define
  the missing Critical profile, Node, and bounds inputs. RFC-K505 proof and
  RFC-K507 evidence are also absent, so an engine cannot claim proof or
  independently verifiable evidence.
- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:458-469` reserves
  `CounterexampleFound`, `NoCounterexampleWithinBounds`, `Inconclusive`, and
  `InvalidModel` for the next task. This checklist is not an accepted result
  schema and correctly warns that bounded absence is not a safety proof.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` are Draft. Their model-check
  sketches do not define state hashing, traversal, timeout, memory, or
  counterexample identity.
- `docs/ROADMAP-1.0.md` treats bounded model checking and disclosed bounds as a
  future G5 gate, not implementation authorization.
- `docs/governance/gap-register.toml` records open
  `GAP-CRITICAL-PROFILE-001`, leaving Node timing/Fault, boundedness,
  Contract proof/runtime, model-check claims, and evidence schema unaccepted.
- `docs/governance/protocol-inventory.toml` records `PROTO-EVIDENCE` as
  Planned public/Future with no version, schema, canonical form, reader/writer
  policy, migration tool, or fixtures. No exploration-result or
  counterexample protocol is inventoried.
- Accepted RFC-0019 is an interpreter–VM differential harness for finite test
  executions, not concurrent state-space exploration; it excludes host
  addresses, layouts, paths, debug text, and instruction counts from identity.

## Repository evidence

The repository has no model-checker engine, finite-state work queue, state
canonicalizer/hash, deterministic BFS/DFS implementation, partial-order
reduction, counterexample trace schema, or timeout/memory result fixtures.
Existing graph cycle traversals and bytecode differential tests are unrelated
compiler/project checks. No Task/Actor/Node runtime exists from which the
proposed engine could derive transitions.

## Required authority before implementation

An accepted RFC-K506 replacement coordinated with MC-5601, Node/Task/Actor,
boundedness, Contract/proof, and evidence decisions must define at least:

1. The canonical projected model and transition/event order, scheduler
   fairness, mailbox/queue and time semantics, state identity/serialization,
   state-hash algorithm/version, deduplication rules, and source/semantic-ID
   spans. Host paths, addresses, timing, allocation, and debug output must
   remain non-semantic.
2. BFS/DFS ordering, deterministic tie-breaking, partial-order reduction
   independence/soundness conditions, depth/step/state/queue/time/memory
   bounds, cancellation and timeout charging, and the behavior for resource
   exhaustion or incomplete exploration.
3. Versioned result and counterexample/replay schemas, including
   `CounterexampleFound`, bounded no-counterexample, inconclusive, invalid
   model, and unknown/corrupt cases; stable bilingual `L-<DOMAIN>-<NUMBER>`
   diagnostics; evidence/provenance/checksums/redaction; and an explicit
   prohibition on calling bounded absence a proof.
4. Offline positive/negative, interleaving, reduction on/off, hash collision,
   bound-edge, timeout/memory, invalid model, deterministic repeated-run,
   counterexample replay, malformed/corrupt, migration, Unicode 17.0.0,
   source-span, and compiler/model/runtime differential fixtures.

## Compatibility and deferred work

This audit changes no parser, resolver, AST/HIR, Checked Typed Core,
evaluator, bytecode, VM, scheduler, mailbox, diagnostics, schema, CLI, LSP,
dependency, Semantic ID, or public protocol. It preserves checked-only
execution, accepted Seed semantics, original UTF-8 byte spans, Unicode
17.0.0, deterministic ordering, and exclusion of host paths, timing,
addresses, and debug output from Ling identity. It makes no exploration,
counterexample, bounded-safety, proof, or Critical support claim.

Implementation remains deferred until RFC-K506 or an accepted replacement and
the coordinated projection, concurrency, bounds, proof, Critical, and
evidence authorities provide executable fixtures. Do not add an exploration
engine, state hash, reduction pass, result/counterexample schema, diagnostic
allocation, CLI/LSP route, public protocol, support claim, or placeholder API
while those authorities remain unresolved.
