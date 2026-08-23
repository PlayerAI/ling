# EVD-5801-OBSERVATION Authority Audit — Evidence Bundle Schema Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0209` permits only test-local Evidence Bundle vocabulary. It does
not authorize a manifest, schema, container, canonical encoding, reader/writer,
verifier, signature/trust policy, diagnostic, protocol, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:518-541` is a
  non-normative content inventory.
- `docs/status/EVD-5801-AUTHORITY-AUDIT.md` records missing schema, identity,
  polarity, provenance, trust, privacy, verification, failure, and migration
  authority.
- RFC-K507 is absent, `PROTO-EVIDENCE` is Future, and
  `GAP-CRITICAL-PROFILE-001` remains open.
- Accepted `DEC-0208` and predecessors provide only test-local vocabulary, not
  bundle producers or accepted evidence claims.

## Current implementation evidence

The observation adds one isolated test with sixty explicit content, identity,
producer, polarity, provenance, privacy, trust, failure, diagnostic, and
fixture boundaries. It sorts by explicit local rank, rejects duplicates,
compares canonical opaque bytes for forward/reverse input order, and keeps
non-claim, offline-verification, and no-code-execution categories distinct. No
manifest, schema, verifier, signature policy, diagnostic, protocol, dependency,
or support claim is introduced.

## Required authority and compatibility

Accepted authority must define a versioned canonical container/manifest;
identity and artifact linkage; evidence polarity and non-claims; producer and
assumption semantics; privacy/provenance/review; signature/trust and independent
offline verification without code execution; corruption/migration and
fail-closed behavior; stable Semantic IDs and UTF-8 spans; bilingual
`L-<DOMAIN>-<NUMBER>` diagnostics; and offline cross-reference/determinism
fixtures. Seed behavior, Semantic IDs, dependencies, and Unicode 17.0.0 remain
unchanged.

## Deferred work

EVD-5801 implementation, bundle schema/container, manifest, reader/writer,
verifier, trust/signature model, diagnostics, protocols, and public support
remain deferred until Accepted authority and executable offline evidence exist.
No placeholder Evidence Bundle API is created.
