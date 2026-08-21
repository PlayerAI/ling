# MC-5601 Authority Audit

Task: `MC-5601` — Finite-State Projection
Release: G5
Status: `BlockedSpec`

## Outcome

`MC-5601` is not implementable from the current accepted authority. The
execution plan proposes projecting Task/Actor/Node programs into a finite
model containing state variables, bounded mailboxes/queues, transitions,
scheduler choices, Fault/restart behavior, an abstract time model, a
property, and an explicit bound. It does not define the source constructs,
state-machine semantics, projection relation, scheduler fairness, mailbox
policy, restart identity, time abstraction, property language, or the meaning
of an explored bound.

The task depends on RFC-K506, which is absent and has no accepted replacement.
Node, Task/Actor, boundedness, Contract/proof, Critical profile, and evidence
tasks remain `BlockedSpec`; `GAP-CRITICAL-PROFILE-001` is open and
`PROTO-EVIDENCE` is Future without a schema or fixtures. A model checker
would otherwise invent semantics and could misrepresent bounded exploration
as proof. No finite-state IR, projection, checker, model protocol, diagnostic,
dependency, or placeholder API should be added.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:431-444` is a
  non-normative checklist. It names projection fields but does not define
  Node/Task/Actor grammar, transition semantics, bounds, state identity,
  fairness, time, or soundness/non-claim language.
- The plan assigns finite-state projection and model-checking semantics to
  RFC-K506. RFC-K501/K502/K504 (Critical profile, synchronous Node, and
  boundedness) are also absent as accepted authorities; RFC-K505/K507 remain
  missing for proof/evidence linkage.
- `docs/SEMANTICS.md:1214-1224` is Draft. Its `ModelChecked(model_id,
  bound)` status sketch does not define a model-check protocol and explicitly
  distinguishes evidence metadata from program logic. `docs/LANGUAGE.md` is
  Draft and v0.0.1 Seed excludes Node/Task/Actor behavior.
- `docs/ROADMAP-1.0.md` describes bounded model checking as a later G5 goal,
  including disclosure of bounds and assumptions, but the roadmap cannot
  authorize a state projection or a proof claim.
- `docs/governance/gap-register.toml` records open
  `GAP-CRITICAL-PROFILE-001`, which leaves Node timing/Fault, boundedness,
  Contract proof/runtime, model-check claims, and evidence schema unaccepted.
- `docs/governance/protocol-inventory.toml` records `PROTO-EVIDENCE` as
  Planned public/Future with no version, schema, canonical form, reader/writer
  policy, migration tool, or fixtures. No model report/counterexample protocol
  is inventoried.
- Accepted Seed RFC-0014 through RFC-0020 cover bytecode/VM execution and
  host control only; they do not define finite-state projection, scheduler,
  mailbox, Node, restart, or model-check semantics.

## Repository evidence

The repository has no Task/Actor/Node language implementation, finite-state
projection IR, model-checker engine, state hash/report schema, counterexample
trace format, or model-check fixtures. The internal `ModuleNode` type in
`ling-project` is a deterministic module-discovery graph, not a Ling runtime
Node. Existing Seed interpreter/VM differential tests compare executable
behavior and do not explore concurrent state spaces or establish bounded
model-check claims.

## Required authority before implementation

An accepted RFC-K506 replacement coordinated with RFC-K501/K502/K504,
Contract/proof, and evidence decisions must define at least:

1. The accepted Task/Actor/Node source and Checked Core model, state variables,
   mailbox/queue bounds, transitions, scheduler choices/fairness,
   Fault/restart semantics, time abstraction, external inputs, and ownership
   boundaries.
2. A versioned finite-state projection and property language with canonical
   state identity, stable Semantic IDs/source spans, explicit bound dimensions,
   state hashing, unknown-field/migration policy, and a precise relation to
   source execution. Define whether a result is bounded evidence or proof and
   prohibit overclaiming.
3. Resource limits, deterministic exploration requirements, timeout/memory
   exhaustion/unknown semantics, counterexample/replay linkage, assumptions
   and profile admission, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and
   evidence/provenance/checksum/redaction rules.
4. Offline positive/negative, bound-edge, mailbox/scheduler interleaving,
   Fault/restart/time, counterexample, incomplete/unknown, malformed/corrupt,
   migration, Unicode 17.0.0, source-span, deterministic, and differential
   fixtures before any model-check output or support claim.

## Compatibility and deferred work

This audit changes no parser, resolver, AST/HIR, Checked Typed Core,
evaluator, bytecode, VM, scheduler, mailbox, diagnostics, schema, CLI, LSP,
dependency, Semantic ID, or public protocol. It preserves checked-only
execution, accepted Seed semantics, original UTF-8 byte spans, Unicode
17.0.0, deterministic ordering, and exclusion of host paths, timing,
addresses, and debug output from Ling identity. It makes no model-check,
counterexample, proof, or Critical support claim.

Implementation remains deferred until RFC-K506 or an accepted replacement and
the coordinated Node/Task/Actor, boundedness, Contract/proof, Critical, and
evidence authorities provide executable fixtures. Do not add a projection IR,
model checker, scheduler model, report/counterexample schema, diagnostic
allocation, CLI/LSP route, public protocol, support claim, or placeholder API
while those authorities remain unresolved.
