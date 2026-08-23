# STAB-6103-OBSERVATION Authority Audit

## Result

Accepted `DEC-0218` authorizes only internal/test feature-state boundary
evidence. Public `STAB-6103` remains `BlockedSpec` because no Accepted public
schema, lifecycle, compatibility policy, or CLI/build/package/LSP/Zed consumer
contract exists.

## Existing authoritative observations

- The internal status registry separates implementation state from stability.
- Status validation cross-checks feature identity, current state, stability,
  blockers, profiles, targets, and verification commit against traceability and
  the draft support matrix.
- The generated fixture declares an internal `ling.governance.*` schema,
  `implemented: false`, and `public_contract: false`.
- `ling support --format json` and `ling features` are not accepted CLI
  commands; `DEC-0217` provides black-box rejection evidence.

## Authorized slice

The child task may add deterministic test-local vocabulary and validate the
exact separation between `Unavailable/Partial/Implemented` and
`Experimental/Preview/Stable/Deprecated/Removed`. It may not publish a schema,
command, metadata field, transition, diagnostic, or cross-tool consumer.

## Deferred authority

Feature/profile/target identities, lifecycle transitions, ownership, public
schema and versioning, unknown/missing/conflicting behavior, compatibility and
migration, all named consumers, diagnostics, and release evidence remain open.
