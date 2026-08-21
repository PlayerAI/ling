# CTR-5401 authority audit — Contract syntax and AST/Core

Status: **BlockedSpec**
Date: 2026-08-22
Owner: Codex
Release: G5

## Outcome

`CTR-5401` proposes a minimal Contract surface containing `requires`,
`ensures`, `invariant`, `assert`, and a restricted, recorded `assume`. It also
proposes that Contract expressions be pure, total, or limited to effects
allowed by a future specification. The execution plan makes RFC-K503 the
dependency, but no RFC-K503 document or accepted replacement exists.

The current Contract material is a design sketch, not an implementation
authorization. `docs/SEMANTICS.md` and `docs/LANGUAGE.md` are Draft, and
`GAP-CRITICAL-PROFILE-001` explicitly leaves the Contract proof/runtime
boundary, boundedness, and evidence claims unresolved. The plan's status set
(`Proved`, `RuntimeChecked`, `Assumed`, `Unknown`, `Failed`, and
`NotApplicable`) also does not match the Draft semantic sketch's
(`Proved`, `RuntimeChecked`, `ModelChecked`, `Tested`, `Assumed`, and
`Unverified`) set. Choosing one would silently resolve a semantic conflict.

`CTR-5401` therefore remains `BlockedSpec`. No Contract parser, AST/HIR node,
Checked Core form, resolver rule, effect restriction, diagnostic, schema, or
public command may be added until the authority and its evidence contract are
accepted.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:304-316` is a planning
  checklist below the accepted-authority boundary. It names RFC-K503 and lists
  examples, but does not define grammar, precedence, expression logic,
  source-to-Core lowering, obligation identity, or observable failures.
- `docs/SEMANTICS.md:1185-1238` sketches Contract claims and proof-status
  metadata, while `docs/SEMANTICS.md:1914-1931` reserves Contract proof for a
  later release. The document is Draft in `docs/governance/authority.toml`, so
  this sketch cannot authorize v0.0.1 behavior.
- `docs/LANGUAGE.md` places Contract/Proof checking in the long-term language
  design and v0.5 Critical roadmap. It does not provide an accepted Contract
  grammar or a versioned AST/Core representation.
- `docs/ROADMAP-1.0.md` places Contract and evidence in G5 after earlier
  resource, Native, and lowering gates. The roadmap is planning evidence, not
  a semantic decision.
- `GAP-CRITICAL-PROFILE-001` is open and blocks `CTR-5401`, `PROOF-5501`,
  `MC-5601`, and `EVD-5801`. Its observable behavior states that the minimum
  Contract proof/runtime boundary and evidence schema are not accepted; its
  candidate RFC is still RFC-0012.
- Accepted Seed RFCs 0014–0020 define bytecode, VM execution, mutable places,
  effects/capabilities, interpreter–VM differential evidence, and VM host
  controls. They do not define source Contract syntax, proof obligations,
  runtime assertion ordering, or Contract status semantics. RFC-0019's use of
  the word “Contract” concerns interpreter–VM differential evidence, not the
  language Contract feature.
- Accepted DEC-0012 includes Contract-related inputs in identity sketches such
  as `ContractId`, but it does not define the Contract language, checker,
  status lifecycle, or proof/evidence protocol. It cannot be used to invent
  those missing layers.

## Repository evidence

There are no production lexer/parser tokens, AST/HIR/Checked Core forms,
resolver rules, evaluator branches, bytecode opcodes, or conformance fixtures
for `requires`, `ensures`, `invariant`, `assert`, or `assume`. Existing uses of
“invariant” in verifier and internal-error code describe host/compiler
invariants, not user Contract claims. The registry has no allocated
`L-CONTRACT-*` diagnostic family and the protocol inventory has no Contract
syntax, Core, proof, runtime-check, or evidence protocol.

The plan and Draft documents leave these observable questions unanswered:

- Contract grammar, precedence, contexts, expression types, logical operators,
  short-circuit behavior, and the relationship to ordinary Core values;
- whether claims are declarations, expressions, attributes, or separate
  checked regions, and how they preserve original UTF-8 byte spans;
- purity, totality, Effect/Capability restrictions, allocation/termination
  limits, and the exact restricted `assume` rules and provenance;
- stable obligation and `ContractId`/`DefinitionId` identity, canonical bytes,
  alpha-normalization, source mapping, and semantic-diff behavior;
- the complete status lifecycle, trust levels, reviewer/approver metadata,
  unknown/timeout behavior, and migration rules;
- static proof, VC, model-check, runtime-check, and test boundaries, including
  solver output, certificates, trusted assumptions, and fail-closed behavior;
- runtime check boundaries, evaluation order, side-effect isolation, Fault
  category, registered bilingual diagnostic, and partial-effect behavior;
- optimization/profile rules, ownership/borrow interaction, memory/timing
  bounds, Node/Task/Actor interaction, and evidence provenance.

## Required authority before implementation

An accepted RFC-K503 replacement (coordinated with the Critical, verification,
boundedness, model-checking, and evidence decisions) must define, at minimum:

1. A versioned Contract grammar and AST/HIR/Checked Core mapping for every
   claim form, including precedence, contexts, source spans, malformed-input
   recovery, and explicit rejection of unsupported constructs.
2. The Contract expression language, purity/totality and Effect/Capability
   rules, boundedness requirements, restricted `assume` syntax, provenance,
   reviewer/approval policy, and deterministic obligation ordering.
3. Stable obligation, `ContractId`, and `DefinitionId` identity/canonical-byte
   rules that preserve Unicode 17.0.0 and original UTF-8 spans without exposing
   host paths, allocation addresses, or hash-map order.
4. A single status lifecycle resolving the Draft/plan conflict, with explicit
   meanings for proved, runtime-checked, model-checked, tested, assumed,
   unknown, failed, and not-applicable results, including timeout and
   migration behavior.
5. Static-proof, VC, model-check, and runtime-check boundaries, trusted
   assumptions, certificate/checker requirements, deterministic limits, and
   fail-closed behavior for unknown or unverifiable claims.
6. Runtime assertion order and isolation, Fault category and stable bilingual
   `L-<DOMAIN>-<NUMBER>` diagnostics, source/Semantic-ID facts, and the rule
   that failed checks cannot partially perform observable Effects.
7. Optimization, Profile, ownership, Node/Task/Actor, memory/timing, and
   evidence-bundle preservation rules, plus any public schema/protocol version
   and compatibility policy.
8. Offline executable positive, negative, boundary, Unicode, CRLF/BOM,
   migration, deterministic-order, malformed-claim, and differential fixtures;
   proof/runtime evidence must be independently bounded and reproducible.

## Compatibility and deferred work

This audit changes no parser, resolver, AST/HIR, Checked Typed Core, evaluator,
bytecode, VM, diagnostic registry, schema, CLI, LSP, dependency, or public
protocol. It preserves the checked-only evaluation boundary, Unicode 17.0.0,
original UTF-8 byte spans, deterministic ordering, and the stable
`L-<DOMAIN>-<NUMBER>` diagnostic convention. It adds no `ling` command or
`.ling` syntax and does not copy stale planning names into implementation.

Implementation is deferred until RFC-K503 or an accepted replacement resolves
the Contract grammar/Core boundary and the related Critical, proof,
boundedness, model-check, effect, ownership, runtime, diagnostic, identity,
and evidence decisions. Do not add a placeholder Contract parser, AST/Core
node, checker, status schema, proof adapter, runtime hook, diagnostic
allocation, CLI/LSP route, public protocol, support claim, or API while those
authorities remain open.
