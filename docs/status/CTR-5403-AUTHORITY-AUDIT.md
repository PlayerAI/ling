# CTR-5403 authority audit — Runtime Contract Check

Status: **BlockedSpec**
Date: 2026-08-22
Owner: Codex
Release: G5

## Outcome

`CTR-5403` proposes a reference runtime that checks preconditions at call,
postconditions at return, invariants at declared boundaries, instance values,
source provenance, and a Contract Fault category. It also proposes that a
Profile be able to disable runtime-only checks. These bullets do not define a
Contract Core, assertion evaluation order, side-effect isolation, Fault
projection, profile authority, or an executable reference-runtime protocol.

The task depends on the absent RFC-K503. `docs/SEMANTICS.md` and
`docs/LANGUAGE.md` are Draft, and `GAP-CRITICAL-PROFILE-001` leaves the
Contract proof/runtime boundary, boundedness, and evidence schema open. The
accepted VM RFCs define a different boundary: RFC-0018 and RFC-0020 normalize
host Capability failures and cancellation for already verified Seed bytecode;
they do not execute user Contract claims. Implementing runtime checks now
would invent source semantics and a new Fault/diagnostic contract.

`CTR-5403` therefore remains `BlockedSpec`. No Contract evaluator, runtime
hook, profile switch, Fault category, diagnostic, schema, CLI/LSP route, or
public protocol may be added while the authority is unresolved.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:331-345` is a planning
  checklist. It names check locations and evidence fields but does not define
  Contract expression evaluation, claim ordering, effect isolation, or
  runtime observability.
- `docs/SEMANTICS.md:1185-1238` sketches `requires`, `ensures`, invariants,
  Contract statuses, and a future `ContractViolation` Fault. The document is
  Draft, and `docs/SEMANTICS.md:1914-1931` reserves Contract proof and
  enforcement rather than authorizing Seed execution.
- `docs/LANGUAGE.md` puts Contract/Proof checking in the later Critical
  roadmap and does not fix runtime assertion boundaries, profile controls, or
  a public Fault schema.
- `GAP-CRITICAL-PROFILE-001` is open for the minimum verifiable Core, Node,
  Contract proof/runtime boundary, boundedness, model-checking claims, and
  evidence schema. Its candidate RFC-0012 is not Accepted.
- Accepted RFC-0018 is limited to Effect closure, explicit `Console.Write`
  Capability preflight, host-failure normalization, committed state, and
  source-mapped `L-RUNTIME-0001` for verified Seed bytecode. It explicitly
  excludes new source-level Effects and Capabilities.
- Accepted RFC-0019 compares interpreter and VM events after checked lowering;
  accepted RFC-0020 covers host-owned VM cancellation and robustness. Neither
  defines call/return Contract checks, invariant boundaries, profile gating, or
  ContractViolation semantics.
- DEC-0013 and the existing diagnostic registry define Main/Runtime failure
  boundaries and registered `L-RUNTIME-0001` facts, not a Contract diagnostic
  family. A new runtime Contract Fault would require an accepted allocation and
  compatibility evidence.

## Repository evidence

There is no Contract parser/Core form, obligation evaluator, checked runtime
hook, profile model, Contract Fault kind, `L-CONTRACT-*` diagnostic, Contract
status schema, or conformance fixture in the repository. Existing evaluator
and VM paths consume checked Seed Core and only report internal checked-core
invariants or accepted host Runtime Faults. Existing `ContractViolation` text
appears only in the Draft semantic sketch; it is not executable authority.

The checklist leaves these observable questions unanswered:

- whether preconditions, postconditions, and invariants are expressions,
  declarations, or separate checked regions and how they bind variables;
- exact evaluation order, short-circuiting, purity/totality, Effect/Capability
  restrictions, allocation/termination bounds, and behavior of `assume`;
- whether checks run before/after argument evaluation, body Effects, returns,
  cleanup, suspension, Node ticks, Actor turns, or FFI calls;
- how failed checks isolate or report partially committed observable Effects;
- Fault kind/category, source span, obligation/Contract ID, bilingual facts,
  exit behavior, status transition, and interaction with `Result`/`Fault`;
- profile selection and whether disabling a runtime-only check is legal,
  explicit, auditable, and prevented from weakening a Critical claim;
- instance-value capture, privacy, canonical provenance, deterministic order,
  resource limits, replay, and migration across source/semantic revisions;
- reference-runtime versus VM/Native/target evidence and the differential
  oracle for a check that has no accepted Contract Core.

## Required authority before implementation

An accepted RFC-K503 replacement, coordinated with the Critical and evidence
decisions, must define, at minimum:

1. Contract expression and obligation semantics, binding/scope rules, check
   boundaries, evaluation order, purity/totality, Effect/Capability limits,
   `assume` restrictions, and deterministic claim ordering.
2. A checked Contract Core representation and runtime interface that consumes
   only validated claims, preserves original UTF-8 spans and stable Semantic/
   obligation IDs, and rejects unknown or malformed claims before execution.
3. Exact precondition, postcondition, invariant, and instance-value timing;
   cleanup/suspension/Node/Actor/FFI boundaries; partial-effect and atomicity
   rules; and privacy/size limits for captured values.
4. Fault/status projection, including category, stable bilingual
   `L-<DOMAIN>-<NUMBER>` code, Facts, source/Contract IDs, exit behavior,
   committed-state semantics, and deterministic interaction with existing
   `L-RUNTIME-0001`.
5. Profile and target policy for enabling/disabling runtime checks, with
   Critical non-weakening rules, evidence labels, migration, and explicit
   reference/VM/Native equivalence or allowed differences.
6. Versioned runtime/evidence schema, unknown-field and compatibility policy,
   replay/provenance rules, independent validation, and bounded deterministic
   failure behavior.
7. Offline executable positive, negative, boundary, Unicode, CRLF/BOM,
   malformed-claim, side-effect-isolation, migration, profile, diagnostic,
   replay, and interpreter/VM/Native differential fixtures.

## Compatibility and deferred work

This audit changes no parser, resolver, AST/HIR, Checked Typed Core, evaluator,
bytecode, VM, profile, diagnostic registry, schema, CLI, LSP, dependency, or
public protocol. It preserves the checked-only evaluation boundary, accepted
Seed `L-RUNTIME-0001` host behavior, Unicode 17.0.0, original UTF-8 spans,
deterministic ordering, and the rule that host paths, timing, addresses, and
debug output are not Ling identity.

Implementation is deferred until RFC-K503 or an accepted replacement resolves
the Contract Core/runtime boundary and the related Critical, effect, proof,
profile, Fault, identity, evidence, and fixture decisions. Do not add a
placeholder Contract evaluator, runtime hook, Fault kind, profile toggle,
diagnostic allocation, CLI/LSP route, public protocol, support claim, or API
while those authorities remain open.
