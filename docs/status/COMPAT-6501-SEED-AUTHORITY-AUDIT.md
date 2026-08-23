# COMPAT-6501-SEED Authority Audit

- Task: `COMPAT-6501-SEED` — Seed historical-corpus freeze evidence
- Parent: `COMPAT-6501` — Historical Corpus
- Decision: Accepted `DEC-0230`
- Release: G6
- Status: authorized bounded evidence

## Authority conclusion

Accepted `DEC-0230` authorizes an internal deterministic freeze of the actual
v0.0.1 `tests/conformance` corpus and an exact non-overclaiming classification
of all ten surfaces requested by `COMPAT-6501`. It does not authorize invented
v0.1-v0.5 artifacts, compatibility outcomes, readers, migrations, or Future/
Unsupported protocol fixtures.

Parent `COMPAT-6501` remains `BlockedSpec` pending real cross-release authority
and historical artifacts.

## Authorized implementation

1. Freeze exact case/file counts and a domain-separated SHA-256 over sorted
   canonical relative paths, byte lengths, and original bytes.
2. Reject symlinks, non-directories, extra/missing case files, invalid evidence
   paths, marker drift, surface drift, count drift, digest drift, and report
   drift.
3. Classify the exact ten requested surfaces as `SeedFrozen`, `NotFrozen`,
   `SeparateProtocol`, or `Unavailable` with repository evidence.
4. Add `cargo xtask corpus verify` to the always-on CI contract and register
   governance, lifecycle, report, backlog, and status evidence.

## Explicit exclusions

No historical release directory, copied protocol fixture, compatibility
matrix, migration tool, reader, deprecation, diagnostic, schema, public CLI,
dependency, protocol, support promotion, or v0.1-v0.5 behavior is added.
