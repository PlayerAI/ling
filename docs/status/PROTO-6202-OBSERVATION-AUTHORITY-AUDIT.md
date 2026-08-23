# PROTO-6202-OBSERVATION Authority Audit

## Result

Accepted `DEC-0220` authorizes only bounded evidence for the existing
eight-schema corpus. Public `PROTO-6202` remains `BlockedSpec`; no N−1 edge,
migration, or universal compatibility behavior is authorized.

## Current executable evidence

- Eight schemas have current-only writers.
- Three schemas have current readers; five expose no reader.
- All eight explicitly declare `NoPreviousVersion`; zero verified N−1 or
  migration edges exist.
- The corpus contains 12 valid, 25 invalid, and 3 canonical-byte fixtures.
- Deterministic corruptions, required/type/extension checks, canonical byte
  form, false previous-version claims, protocol linkage, and generated registry
  consistency are already verified offline.

## Authorized slice

The child task may lock the exact current reader/writer distribution, record
test-local compatibility vocabulary, and run existing compatibility/corruption
gates. It may not add a reader, migration, version edge, limit, diagnostic,
schema, or protocol.

## Deferred authority

Per-protocol N−1 and migration semantics, future-version behavior, size/depth/
resource/security limits, unavailable protocol schemas, diagnostics, and
release evidence remain unresolved.
