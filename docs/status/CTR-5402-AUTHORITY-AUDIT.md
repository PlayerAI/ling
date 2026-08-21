# CTR-5402 authority audit — Contract status model

Status: **BlockedSpec**
Date: 2026-08-22
Owner: Codex
Release: G5

## Outcome

`CTR-5402` proposes a lifecycle for every Contract obligation:
`Proved`, `RuntimeChecked`, `Assumed`, `Unknown`, `Failed`, and
`NotApplicable`. It further requires status propagation into Audit, Semantic
Graph, and Evidence, with UI indicators that cannot hide the textual state.
The checklist does not define state transitions, evidence provenance,
authority/trust levels, identity, versioning, or a public schema.

No accepted RFC-K503, proof, model-checking, or evidence decision defines this
model. The Draft `docs/SEMANTICS.md` uses a different set (`Proved`,
`RuntimeChecked`, `ModelChecked`, `Tested`, `Assumed`, and `Unverified`), while
the plan uses `Unknown`, `Failed`, and `NotApplicable` and omits `ModelChecked`
and `Tested`. This unresolved conflict cannot be resolved by an implementation
or a UI convention. `GAP-CRITICAL-PROFILE-001` remains open and explicitly
blocks `CTR-5401`, `PROOF-5501`, `MC-5601`, and `EVD-5801` over the Contract
proof/runtime boundary and evidence schema.

`CTR-5402` therefore remains `BlockedSpec`. No status enum, graph field,
evidence schema, renderer, diagnostic, CLI/LSP route, or public protocol may
be added while the authority is missing.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:318-328` is a planning
  checklist. It names states and destinations but does not define their
  meanings, legal transitions, precedence, aggregation, or compatibility.
- `docs/SEMANTICS.md:1214-1227` is a Draft status sketch. It treats status as
  evidence metadata and includes `ModelChecked`, `Tested`, and `Unverified`,
  but does not define a versioned lifecycle, transition table, identity, or
  report schema.
- `docs/SEMANTICS.md:1229-1238` says optimization must preserve Contracts and
  expose new assumptions, but it does not authorize a status implementation.
  `docs/SEMANTICS.md:1914-1931` reserves Contract proof for a later release.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` place Contract/Proof and
  Evidence Bundle work in the later Critical roadmap; both remain Draft or
  Planning authorities.
- `GAP-CRITICAL-PROFILE-001` is open for the minimum Critical Core, Contract
  proof/runtime boundary, model-checking claims, and evidence schema. Its
  candidate RFC-0012 is not Accepted and its required evidence includes
  independent checking, counterexamples, boundedness, and reproducibility.
- The schema lifecycle policy and protocol inventory require an accepted
  authority, explicit version, fixtures, and compatibility policy before a
  public status or evidence schema can be published. No Contract status
  protocol is inventoried.
- Accepted Seed RFCs 0014–0020 and DEC-0012 cover bytecode/VM evidence,
  interpreter–VM differential events, host controls, and semantic identity
  inputs. They do not define Contract obligation statuses, proof trust, or
  propagation into Graph/Audit/Evidence.

## Repository evidence

The repository has no Contract obligation type or status enum in the parser,
AST/HIR, Checked Core, Semantic Graph, Audit model, evaluator, bytecode, or VM;
no Contract status JSON schema, evidence bundle, lifecycle registry, or
conformance fixtures; and no public Contract protocol. Existing status and
governance registries describe implementation tasks and accepted Seed
features, not user Contract claims. The diagnostic registry has no allocated
`L-CONTRACT-*` family for state transitions, unknown proof, failed checks, or
evidence mismatch.

The plan leaves these observable questions unanswered:

- exact meanings and legal transitions among proof, runtime check, test,
  model check, assumption, unknown, failure, and inapplicability;
- whether states are mutually exclusive, ordered, composable, or attached to
  claims, obligations, definitions, snapshots, or evidence artifacts;
- aggregation and precedence for multiple proofs, checks, counterexamples,
  timeouts, stale inputs, failed instances, and partial evidence;
- provenance, trusted-computing-base, approver, expiry, tool/version/config,
  bound, replay, and migration fields;
- stable obligation/Contract/Semantic IDs, canonical bytes, source spans, and
  Graph/Audit/Evidence linkage across revisions and renamed source;
- behavior when evidence is missing, corrupt, stale, unverifiable, or produced
  by a solver that is not an independently checked authority;
- optimization/profile gates, UI text and accessibility, deterministic order,
  resource limits, bilingual diagnostics, and CLI/LSP transport behavior.

## Required authority before implementation

An accepted Contract/proof/evidence decision must define, at minimum:

1. A versioned status vocabulary that resolves the Draft/plan conflict,
   includes explicit semantics for every state, and declares whether unknown,
   failed, assumed, tested, model-checked, runtime-checked, and not-applicable
   are terminal, intermediate, or compositional.
2. A transition and aggregation table with precedence, monotonicity or
   invalidation rules, stale-snapshot behavior, downgrade/revocation policy,
   and deterministic ordering across claims and artifacts.
3. Stable obligation, Contract, Semantic Graph, Audit, and Evidence IDs plus
   canonical serialization, source-span/provenance linkage, versioning, and
   migration rules that preserve Unicode 17.0.0 and exclude host details.
4. Evidence provenance and trust rules for proofs, tests, runtime checks,
   model-check bounds, assumptions, solver/checker versions, tool configs,
   reviewers, expiry, counterexamples, and independent verification.
5. Fail-closed handling for missing, unknown, failed, stale, corrupt, or
   unverifiable evidence, including exact Fault/diagnostic projection and the
   rule that status metadata cannot silently change program semantics.
6. Graph/Audit/Evidence/public-schema boundaries, unknown-field and migration
   policy, UI text/accessibility requirements, and any CLI/LSP transport.
7. Offline executable positive, negative, boundary, migration, corruption,
   Unicode, CRLF/BOM, deterministic-order, and cross-tool differential
   fixtures for every transition and aggregation rule.

## Compatibility and deferred work

This audit changes no parser, AST/HIR, Checked Typed Core, Semantic Graph,
Audit model, evaluator, bytecode, VM, diagnostic registry, schema, CLI, LSP,
dependency, or public protocol. It preserves the checked-only evaluation
boundary, Unicode 17.0.0, original UTF-8 byte spans, deterministic ordering,
and the `L-<DOMAIN>-<NUMBER>` diagnostic convention. It adds no status field
or UI route and makes no stability claim for Contract evidence.

Implementation is deferred until RFC-K503 or an accepted replacement, the
Critical/proof/model-check/evidence decisions, and executable fixtures settle
the status vocabulary and lifecycle. Do not add a placeholder status enum,
Graph/Audit/Evidence field, schema, renderer, proof adapter, diagnostic
allocation, CLI/LSP route, public protocol, support claim, or API while those
authorities remain open.
