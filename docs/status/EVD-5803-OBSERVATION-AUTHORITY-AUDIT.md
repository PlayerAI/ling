# EVD-5803-OBSERVATION Authority Audit — Reproducible Build Binding Evidence

Status: BlockedSpec
Date: 2026-08-23

## Outcome

Accepted `DEC-0211` permits only test-local reproducible-build-binding
vocabulary. It does not authorize a build manifest, controlled or hermetic
runner, artifact identity/hash, equivalence relation, nondeterminism policy,
diagnostic, protocol, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:557-564` is a
  non-normative rebuild-and-compare checklist.
- `docs/status/EVD-5803-AUTHORITY-AUDIT.md` records missing manifest,
  environment, input-closure, artifact, equivalence, nondeterminism,
  provenance, failure, migration, and fixture authority.
- RFC-K507 is absent; `PROTO-EVIDENCE` and `PROTO-BUILD-METADATA` are Future;
  `GAP-CRITICAL-PROFILE-001` remains open.
- Accepted `DEC-0012`, RFC-0002, and query-determinism decisions have narrower
  identity scopes; `DEC-0210` provides only test-local verifier vocabulary.

## Current implementation evidence

The observation adds one isolated test with sixty explicit manifest/
environment, identity, artifact/provenance, nondeterminism/exclusion,
result/failure, diagnostic, and fixture boundaries. It sorts by explicit local
rank, rejects duplicates, compares opaque bytes for forward/reverse input
order, and keeps source/Semantic IDs, object/binary hashes, hermetic build, and
accepted nondeterminism distinct. No build, artifact, hash, environment,
equivalence, diagnostic, protocol, dependency, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define hermetic input closure and manifest bytes;
toolchain/target/profile/environment/TCB identity; artifact/hash domains and
equivalence; nondeterminism registration and comparison; generated provenance;
offline isolation; fail-closed results, diagnostics, and exits; stable Semantic
IDs and original UTF-8 spans; and executable repeated/cross-host fixtures. Seed
behavior, Semantic IDs, dependencies, and Unicode 17.0.0 remain unchanged.

## Deferred work

EVD-5803 implementation, controlled rebuilds, artifact production and
comparison, nondeterminism policy, diagnostics, protocols, and public support
remain deferred until Accepted authority and executable offline evidence exist.
No placeholder reproducible-build API is created.
