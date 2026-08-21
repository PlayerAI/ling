# PROOF-5503 Authority Audit

Task: `PROOF-5503` — Assumption Registry
Release: G5
Status: `BlockedSpec`

## Outcome

`PROOF-5503` is not implementable from the current accepted authority. The
execution plan requires every unproved external fact to be listed with an
assumption ID, description, source, scope, owner/reviewer, expiry/version,
risk class, and affected obligations, and requires the Evidence Bundle to
expose the entries instead of hiding them in comments. This is a useful
accountability checklist, but it does not define the assumption language,
identity, approval lifecycle, proof effect, TCB meaning, or evidence schema.

RFC-K505 is absent, and RFC-K507 Evidence Bundle plus RFC-K501 Critical Profile
and RFC-K503 Contract authorities are not accepted. `PROOF-5501` and
`PROOF-5502` therefore cannot provide a proof/checker boundary. The open
`GAP-CRITICAL-PROFILE-001` leaves proof, boundedness, model-checking, and
evidence claims unresolved, while `PROTO-EVIDENCE` is Future without a schema
or fixtures. No assumption registry, schema, reviewer workflow, diagnostic,
TCB claim, public protocol, or placeholder API should be added.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:412-427` is a
  non-normative checklist. It names fields but does not define identifier
  generation, canonical encoding, scope semantics, approval/revocation,
  expiry, risk taxonomy, or proof/evidence consumption.
- The plan assigns proof/checker/trusted assumptions to RFC-K505 and evidence
  publication/independent verification to RFC-K507. Neither RFC exists as an
  accepted repository authority; RFC-K501 Critical Profile and RFC-K506 Model
  Checking are also absent.
- `docs/governance/gap-register.toml` records open
  `GAP-CRITICAL-PROFILE-001`; it explicitly leaves the Contract proof/runtime
  boundary, boundedness, model-check claims, and evidence schema unaccepted
  and requires independent-checker, counterexample, and reproducible-build
  evidence.
- `docs/governance/protocol-inventory.toml` records `PROTO-EVIDENCE` as
  Planned public/Future with no version, schema, canonical form, reader/writer
  policy, migration tool, or fixtures. It cannot carry an assumption registry.
- `docs/SEMANTICS.md` is Draft. Its TCB list and Contract `Assumed` status
  sketch do not define approval authority, risk classes, expiry, or the
  difference between an assumption, a model-check bound, a runtime fact, and
  a proof hypothesis. `docs/LANGUAGE.md` is Draft as well.
- Accepted DEC-0012 fixes semantic identity and canonical-byte boundaries, and
  DEC-0013 fixes main/runtime failure handling; neither authorizes an
  assumption identity, registry lifecycle, or evidence publication protocol.
- Existing `PROTO-DIAGNOSTIC-JSON`, lifecycle records, and governance gap
  records are unrelated project/compiler records. They must not be repurposed
  as proof assumptions or TCB evidence.

## Repository evidence

The repository has no assumption registry, assumption schema, reviewer/owner
workflow, expiry/revocation checker, risk-class taxonomy, proof-obligation
linker, or executable assumption/evidence fixtures. Current governance TOML
tracks specification gaps and lifecycle records, not external facts consumed
by a proof checker. The internal Trait solver, bytecode verifier, VM evidence,
and compiler provenance fields do not provide a source Contract assumption
boundary.

## Required authority before implementation

An accepted RFC-K505/RFC-K507 replacement coordinated with Contract,
boundedness, model-checking, and Critical Profile decisions must define at
least:

1. A versioned canonical assumption record with stable ID, description,
   authoritative source and digest, scope, affected Proof/Contract IDs and
   original UTF-8 spans, owner/reviewer identities, risk taxonomy, expiry and
   version constraints, status, approval/revocation transitions, and unknown
   field/migration rules.
2. The distinction between assumptions, hypotheses, axioms, runtime checks,
   tests, bounded model-check results, solver candidates, and proved facts;
   define whether any assumption can discharge an obligation or affect
   optimizer/profile admission, and fail closed on expired, revoked, missing,
   stale, corrupt, or unreviewed entries.
3. TCB membership, independent review and checker requirements, provenance,
   checksums/signatures, evidence-bundle linkage, redaction, reproducible
   build/test identity, deterministic ordering, resource limits, and no host
   paths/timing/addresses/debug output in Ling identity.
4. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and machine-readable
   results for unknown, expired, revoked, out-of-scope, conflicting,
   unverifiable, or policy-rejected assumptions, with public protocol
   inventory and compatibility policy.
5. Offline positive/negative, missing/duplicate/conflicting, expired/revoked,
   scope/version mismatch, malformed/corrupt, migration, Unicode 17.0.0,
   source-span, deterministic, proof/checker differential, and evidence
   redaction fixtures before a registry is exposed or trusted.

## Compatibility and deferred work

This audit changes no parser, resolver, AST/HIR, Checked Typed Core,
evaluator, bytecode, VM, Trait solver, diagnostics, schema, CLI, LSP,
dependency, Semantic ID, governance lifecycle, or public protocol. It
preserves checked-only execution, accepted Seed semantics, original UTF-8 byte
spans, Unicode 17.0.0, deterministic ordering, and exclusion of host details
from Ling identity. It makes no assumption, proof, TCB, certification, or
Critical support claim.

Implementation remains deferred until RFC-K505/RFC-K507 or accepted
replacements and executable proof/evidence fixtures define the assumption
boundary. Do not add an assumption registry, schema, reviewer/expiry
workflow, TCB field, diagnostic allocation, CLI/LSP route, public protocol,
support claim, or placeholder API while those authorities remain unresolved.
