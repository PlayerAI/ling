# CTR-5404 authority audit — Verification Condition Generation

Status: **BlockedSpec**
Date: 2026-08-22
Owner: Codex
Release: G5

## Outcome

`CTR-5404` proposes a versioned Proof IR/VC containing SSA and path
conditions, pre/postconditions, loop invariants, arithmetic, memory/alias and
Effect facts, source mappings, and trusted assumptions. These bullets are a
future design checklist, not a definition of a proof language, a VC encoding,
or a soundness boundary. The task depends on RFC-K505, which is absent and has
no Accepted replacement.

The Draft Contract material cannot authorize this work: `SEMANTICS.md` and
`LANGUAGE.md` are below accepted RFC authority, and
`GAP-CRITICAL-PROFILE-001` leaves Contract proof/runtime, boundedness,
model-checking, and evidence claims unresolved. `PROTO-EVIDENCE` is explicitly
Future with no version, schema, fixtures, reader, or writer policy. A VC
generator would freeze arithmetic, alias, Effect, source mapping, assumption,
and trusted-computing-base semantics before those decisions exist.

`CTR-5404` therefore remains `BlockedSpec`. No Proof IR, VC generator,
obligation lowering, proof schema, solver interface, diagnostic, CLI/LSP route,
or public protocol may be added while the authority is unresolved.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:347-358` is a planning
  checklist. It names desired VC ingredients but does not define a grammar,
  version, well-formedness, soundness relation, translation validation, or
  failure/unknown semantics.
- The same plan makes RFC-K505 the Verification Interface dependency and
  separately makes RFC-K506 the model-checking authority. Neither RFC exists
  as an accepted document in the repository.
- `docs/SEMANTICS.md` sketches Contracts, arithmetic, memory/ownership, and
  source identity but is Draft; its v0.0.1 section reserves Contract proof and
  does not authorize a proof-producing compiler path.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` place proof, model checking,
  and evidence in later Critical milestones. Their planning text cannot fix a
  proof calculus or claim soundness.
- `GAP-CRITICAL-PROFILE-001` is open and blocks `CTR-5401`, `PROOF-5501`,
  `MC-5601`, and `EVD-5801`; it explicitly says the Contract proof/runtime
  boundary, boundedness, model-checking claims, and evidence schema are not
  accepted.
- `PROTO-EVIDENCE` in `docs/governance/protocol-inventory.toml` is Future,
  unimplemented, unversioned, and fixture-free. Its writer policy says
  identity, provenance, proof/test linkage, redaction, and verification rules
  require accepted specifications.
- Accepted Seed RFCs 0014–0020 define checked bytecode/VM lowering, verifier
  invariants, host Faults, differential events, and resource/cancellation
  evidence. They do not define Proof IR, VCs, solver obligations, trusted
  assumptions, or Contract proof soundness.

## Repository evidence

The repository has no Proof IR or VC data model, proof-term/certificate
format, VC generator, independent proof checker, assumption registry, proof
diagnostic family, Contract proof schema, or executable proof fixtures. The
existing bytecode verifier and lowerers validate executable Seed artifacts;
their CFG/type/resource invariants are not a source-level proof calculus and
must not be relabeled as one. No production module or test establishes a
mapping from Contract claims to proof obligations.

The checklist leaves these observable questions unanswered:

- the source Contract subset and typed logical language translated to VCs;
- SSA/path construction, branch/loop invariants, recursion/termination and
  boundedness assumptions, and treatment of unreachable or partial paths;
- arithmetic domains, overflow/rounding, floating-point modes, memory/alias,
  ownership, Effect/Capability, Node timing, FFI, and external axioms;
- VC identity, canonical serialization, versioning, source/Semantic-ID maps,
  dependency/profile/target context, and migration compatibility;
- distinction among theorem validity, bounded evidence, testing, assumption,
  solver unknown/timeout, malformed model, and proof-checker failure;
- trusted-computing-base membership, independent validation, resource limits,
  replay/counterexample linkage, privacy, and evidence provenance;
- bilingual diagnostics, stable codes, deterministic ordering, and whether
  failed or unknown obligations can affect optimization or runtime behavior.

## Required authority before implementation

An accepted RFC-K505 replacement, coordinated with RFC-K503, boundedness,
model-checking, and evidence decisions, must define, at minimum:

1. A versioned Proof IR/VC grammar, well-formedness rules, canonical bytes,
   identity, source/Semantic-ID mapping, unknown-field policy, and migration
   compatibility.
2. A precise Contract-to-VC translation for expressions, pre/postconditions,
   invariants, branches, loops, recursion, effects, ownership/aliasing,
   arithmetic, memory, timing, FFI, Node, and external assumptions.
3. Soundness/non-claims, proof obligation scope, bounded versus unbounded
   reasoning, solver candidate versus checked certificate, timeout/unknown,
   invalid model, and fail-closed behavior.
4. Trusted-assumption/TCB registry with provenance, scope, owner/reviewer,
   expiry/version, risk class, affected obligations, redaction, and revocation.
5. Deterministic resource limits, independent checker boundary, diagnostics,
   Fault/status projection, optimization/profile gates, and counterexample/
   replay/evidence linkage that preserve Unicode 17.0.0 and UTF-8 spans.
6. A versioned evidence protocol and offline executable positive, negative,
   boundary, malformed, overflow/rounding, alias/effect, assumption,
   timeout/unknown, migration, corruption, Unicode, CRLF/BOM, and differential
   fixtures for every claim and translation rule.

## Compatibility and deferred work

This audit changes no parser, resolver, AST/HIR, Checked Typed Core, evaluator,
bytecode, VM, verifier, diagnostic registry, schema, CLI, LSP, dependency, or
public protocol. It preserves the checked-only execution boundary, accepted
Seed bytecode/VM semantics, Unicode 17.0.0, original UTF-8 spans, deterministic
ordering, and the exclusion of host paths, timing, addresses, and debug output
from Ling identity. It makes no soundness or certification claim.

Implementation is deferred until RFC-K505 or an accepted replacement, the
Contract/Critical/boundedness/model-check/evidence authorities, and executable
fixtures settle the proof boundary. Do not add a placeholder Proof IR, VC
generator, assumption registry, solver adapter, checker, schema, diagnostic
allocation, CLI/LSP route, public protocol, support claim, or API while those
authorities remain open.
