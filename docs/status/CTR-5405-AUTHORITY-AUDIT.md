# CTR-5405 Authority Audit

Task: `CTR-5405` — Solver/Proof Checker Adapter
Release: G5
Status: `BlockedSpec`

## Outcome

`CTR-5405` is not implementable from the current accepted authority. The
execution plan permits an external solver only as an untrusted candidate
proof generator and lists proof certificates or replayable queries, fixed
solver/version/configuration, timeout and unknown outcomes, an independent
proof checker, and a trusted-computing-base (TCB) inventory. These bullets do
not define a proof/query language, certificate format, checker soundness
boundary, solver trust model, or public evidence protocol.

The task depends on the planned RFC-K505 Verification Interface; that RFC is
absent and no accepted replacement coordinates the related RFC-K503 Contract,
RFC-K506 Model Checking, or RFC-K507 Evidence Bundle boundaries. The open
`GAP-CRITICAL-PROFILE-001` and Future `PROTO-EVIDENCE` leave the Critical
proof, boundedness, model-checking, identity, provenance, and independent
verification claims unresolved. The existing Trait solver and bytecode
verifier are unrelated internal type/invariant checks and cannot authorize an
external proof solver adapter. No solver dependency, checker, certificate or
query schema, TCB registry, diagnostic, CLI/LSP route, public protocol, or
placeholder API should be added.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:360-369` is a
  non-normative implementation checklist. It requires candidate-only solver
  use and the listed certificate, replay, version, timeout/unknown, checker,
  and TCB concepts, but defines none of their schemas or semantics.
- The plan's RFC table names RFC-K505 as the authority for proof obligations,
  solver/checker integration, and trusted assumptions, with RFC-K506 and
  RFC-K507 covering model-check reports and evidence. These documents are not
  present as accepted RFCs.
- `docs/governance/gap-register.toml` records open
  `GAP-CRITICAL-PROFILE-001`; its observable behavior explicitly leaves
  Contract proof/runtime, boundedness, model-checking, and evidence claims
  unaccepted.
- `docs/governance/protocol-inventory.toml` records `PROTO-EVIDENCE` as
  Planned public and Future with no version, public schema, canonical form,
  reader/writer policy, migration tool, or fixtures. Its writer policy also
  requires accepted identity, provenance, checksum, signature, proof/test
  linkage, redaction, and verification rules.
- Accepted Seed RFC-0014 through RFC-0020 define bytecode/VM safety,
  host-failure normalization, interpreter/VM differential behavior, and
  cancellation/resource evidence. They do not define external solver output,
  proof certificates, replayable queries, timeout/unknown proof semantics,
  independent checking, or a TCB claim.
- Draft Contract and proof sketches in `docs/SEMANTICS.md` and
  `docs/LANGUAGE.md` are below accepted authority and cannot establish a
  proof calculus or make solver output trusted.

## Repository evidence

The repository contains no external proof-solver adapter, canonical
proof/query/certificate schema, independent proof checker, assumption/TCB
registry, or executable proof fixtures. `crates/ling-types/src/solver.rs` is
the accepted internal Trait solver, while the bytecode verifier validates
executable Seed invariants; neither is a source-Contract proof checker.
`PROTO-EVIDENCE` has no implemented reader or writer, and there is no stable
diagnostic allocation for solver, checker, timeout, unknown, certificate, or
TCB failures.

## Required authority before implementation

An accepted RFC-K505 replacement, coordinated with RFC-K503, RFC-K506, and
RFC-K507, must define at least:

1. Versioned canonical Proof/Query/Certificate schemas, well-formedness,
   canonical bytes, stable IDs and original UTF-8 spans, identity, unknown
   fields, migration, and compatibility rules.
2. A precise boundary between checked Contract/Proof IR and untrusted solver
   candidates; the replayable query form; fixed solver/version/configuration
   metadata; and the allowed solver result set, including bounded timeout and
   `unknown` semantics.
3. A deterministic, resource-bounded checker with an explicit soundness
   statement, trusted assumptions, TCB membership, independent-verification
   requirements, and fail-closed behavior for malformed, corrupt,
   unverifiable, timed-out, or unknown results. Solver stdout must never be
   trusted implicitly.
4. Evidence linkage for obligations, certificates, counterexamples, tests,
   provenance, checksums/signatures, redaction, reproducible builds, and
   replay; plus stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics.
5. Offline positive/negative, malformed, timeout/unknown, corruption,
   migration, Unicode 17.0.0, source-span, determinism, and
   compiler/solver/checker differential fixtures before any support claim.

## Compatibility and deferred work

This audit changes no parser, resolver, AST/HIR, Checked Typed Core,
evaluator, bytecode, VM, diagnostic registry, schema, CLI, LSP, dependency, or
public protocol. It preserves checked-only execution, accepted Seed
bytecode/VM semantics, Unicode 17.0.0, original UTF-8 byte spans, deterministic
ordering, and exclusion of host paths, timing, addresses, and debug output
from Ling identity. It makes no proof, soundness, certification, or solver
support claim.

Implementation remains deferred until the missing accepted authorities and
executable fixtures establish the proof/checker/evidence boundary. Do not add
a solver dependency, proof checker, certificate/query format, assumption/TCB
registry, diagnostic allocation, CLI/LSP route, public protocol, support
claim, or placeholder API while those authorities remain unresolved.
