# PROTO-6203-OBSERVATION Authority Audit

- Task: `PROTO-6203-OBSERVATION` — Internal Semantic Hash Upgrade Rehearsal boundary evidence
- Parent: `PROTO-6203` — Semantic Hash Upgrade Rehearsal
- Decision: Accepted `DEC-0221`
- Release: G6
- Status: authorized bounded evidence

## Authority conclusion

Accepted `DEC-0012` authorizes the current experimental BLAKE3 Semantic ID
form and requires an explicit schema or prefix upgrade for any algorithm,
encoding, or normalization change. Accepted `DEC-0221` authorizes only a
test-local inventory of rehearsal concerns and exact verification of the
current hash-bearing schema declarations.

No Accepted authority selects a replacement algorithm, defines old/new reader
or writer behavior, or connects identity migration to dependencies, lockfiles,
caches, replay, or evidence. Those behaviors remain blocked by
`GAP-SEMANTIC-HASH-LIFECYCLE-001` and
`GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`.

## Authorized implementation

1. Add a sixty-category test-local inventory with deterministic ordering,
   duplicate rejection, and opaque bytes that are explicitly outside Semantic
   identity.
2. Assert exact hash-scheme declarations for the two Semantic Graph schemas
   and the lock schema.
3. Assert that each hash-bearing schema remains `NoPreviousVersion` with no
   previous marker, compatibility corpus, or migration adapter.
4. Register the scoped decision, lifecycle transition, implementation report,
   and task traceability.

## Explicit exclusions

This slice does not add or alter an algorithm ID, Semantic ID prefix, canonical
byte domain, schema marker, reader, writer, migration adapter, cache key,
invalidation event, dependency or lock identity, replay/evidence link,
diagnostic, CLI/LSP route, public API, Unicode rule, or source span.

The opaque observation tag is test data only and must never be consumed by the
compiler pipeline or interpreted as compatibility authority. Parent
`PROTO-6203` remains `BlockedSpec`.
