# EVD-5801 Authority Audit

- Task: `EVD-5801` — Evidence Bundle Schema
- Plan: `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:518-541`
- Release: G5
- Status: `BlockedSpec`

## Decision

EVD-5801 is `BlockedSpec`. The execution plan proposes a bundle containing a
manifest, source and Semantic IDs, specification versions, dependency/build
identity, Profile/target/toolchain data, Audit Source, test/proof/model/timing/
memory/replay evidence, FFI/Target Package/TCB data, assumptions, AI and human
provenance, and artifact hashes. It does not define a versioned schema,
canonical encoding, trust boundary, verification result, privacy policy, or
migration behavior.

No accepted RFC-K507 or replacement authorizes a public Evidence Bundle. The
repository's `PROTO-EVIDENCE` entry is explicitly Future, unimplemented,
unversioned, non-canonical, schema-free, and fixture-free. Implementing the
bundle or treating existing reports as that protocol would invent release and
Critical claims, and would make unresolved model-check, timing, Contract,
proof, target, and provenance semantics appear settled.

## Normative traceability

- `09-G5-V0.5-CRITICAL.md:518-541` is a non-normative inventory. It does not
  establish required/optional fields, canonical ordering, digest domains,
  signatures, trust roots, source redaction, evidence polarity, or how a bundle
  records an unproved or unavailable result.
- `docs/governance/protocol-inventory.toml:518-537` records `PROTO-EVIDENCE` as
  Planned public/Future with no current version, public schema, canonical form,
  reader/writer policy, unknown-field policy, migration tool, or fixtures. Its
  only authorities are the roadmap and gap register; it is not implementation
  authority.
- `GAP-CRITICAL-PROFILE-001` remains Open and names the Critical evidence
  schema, boundedness, timing/Fault semantics, model-checking claims, and proof
  boundary as unresolved. Its candidate RFC-0012 is not present or Accepted.
- `docs/ROADMAP-1.0.md:437-498` describes a future independently verifiable,
  offline evidence bundle and lists target/Profile/toolchain, provenance, and
  non-claims. The roadmap is Planning authority and does not define a public
  container or verifier.
- Accepted RFC-0014 fixes only the portable bytecode verifier's bounded
  diagnostics, source maps, deterministic writer, and VM resource evidence.
  RFC-0019 fixes an interpreter–VM differential harness, and RFC-0020 fixes
  VM cancellation/fuzz/resource evidence. None creates a cross-feature
  Evidence Bundle or release identity; their evidence remains in their own
  scopes.
- The plan's `zero evidence verify <bundle>` command is not accepted CLI
  authority. The public command is `ling`, and no evidence verification command,
  schema, or process-exit contract may be introduced from this checklist.

## Evidence in this repository

There is no versioned Evidence Bundle schema, canonical manifest, bundle
reader/writer, independent verifier, signature/trust model, migration policy,
or bundle fixture corpus under `crates/`, `tests/`, or `schemas/`. Existing
bytecode/VM fixtures, project manifests, and internal incident reports have
separate accepted or internal scopes and are not a Critical/release bundle.
No `ling` CLI, LSP request, diagnostic, or public protocol claims EVD-5801
support.

## Required authority before implementation

An accepted RFC or replacement decision must define, at minimum:

1. A versioned canonical bundle/container and manifest with required and
   optional fields, deterministic ordering, digest domains, size limits,
   artifact references, unknown-field handling, migration, and malformed or
   corrupt-input outcomes.
2. Typed identities and provenance for Program/Semantic IDs, original source
   and Audit Source, accepted RFC/decision versions, dependencies/lock/build
   graph, Profile/target/toolchain/TCB, generated artifacts, tests, proofs,
   model-check reports, replay, timing/memory, FFI/Target Packages, and AI or
   human review. Host paths, addresses, timestamps, debug output, secrets, and
   private conversation content must not become Ling identity.
3. Evidence polarity and lifecycle: passed, failed, skipped, unavailable,
   assumed, unknown, bounded, and non-claim states; linkage to assumptions,
   exemptions, known limits, counterexamples, source spans, and Semantic IDs;
   no bounded absence, measurement, or Draft claim may be presented as proof.
4. Independent verification and trust boundaries, including canonical/hash
   checks, artifact linkage, proof/test identity, lock/toolchain checks,
   offline operation, signature policy if any, and a strict rule that bundle
   verification never executes bundle code. Define verifier version/TCB and
   fail-closed behavior for unsupported evidence.
5. Privacy/redaction, AI provenance, human signoff, retention, disclosure,
   reproducibility, and authorization rules, with stable bilingual
   `L-<DOMAIN>-<NUMBER>` diagnostics and documented process/fixture outcomes.
6. Offline positive, negative, missing/unknown, corruption, migration,
   cross-reference, Unicode 17.0.0, BOM/CRLF, source-span, repeated-build,
   and deterministic-verification fixtures for each evidence producer and
   result state.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, or
support claim. It preserves the accepted `ling` CLI and `.ling` source
extension, original UTF-8 spans, Unicode 17.0.0, deterministic identity rules,
and the checked Typed Core boundary. It deliberately adds no bundle schema,
manifest, reader/writer, verifier, signature dependency, evidence diagnostic,
CLI command, or placeholder API, and it introduces no stale `zero` names.

EVD-5801 remains deferred until Critical Profile, Node, Contract/Proof,
boundedness, model-check, replay, timing, target/ABI, reproducible-build,
provenance, and evidence authorities are Accepted with executable fixtures.
EVD-5802 must not implement `zero evidence verify` or any `ling` equivalent
until this schema and its trust boundary are accepted.
