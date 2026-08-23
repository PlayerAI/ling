# CLI-1705 Authority Audit: Semantic query and patch commands

## Outcome

`CLI-1705` is implementation-ready under Accepted RFC-0027. The RFC resolves
the command's bounded G1 requirement without pretending that the broader
Semantic Transaction lifecycle is complete: `query` is an exact read-only
projection, while `patch` validates a proposal and always reports
`committed: false`.

The open semantic/LSP transaction gaps continue to block project/multi-file
edits, source ranges, `Graph.Commit`, atomic disk publication, LSP
`WorkspaceEdit`, migration, and Stable compatibility. They no longer block the
explicit one-file proposal-only slice accepted for CLI-1705.

## Normative traceability

- `SEMANTICS.md` §25 requires exact stale-base rejection, temporary checked
  state, preserve validation, and Proposal/Commit separation.
- Accepted DEC-0012 fixes Program/Definition/Body identity and canonical bytes.
- Accepted DEC-0253 and DEC-0254 fix command dispatch and output policy.
- Accepted RFC-0027 defines exact command grammar, one-file/no-import scope,
  query matching, request/result schemas, validation order, nonmutation,
  diagnostics, exits, bounds, compatibility, and deferred work.
- `PROTO-SEMANTIC-QUERY`, `PROTO-SEMANTIC-TRANSACTION`, and
  `PROTO-SEMANTIC-TRANSACTION-RESULT` register the three Preview protocols.

Lower-authority references to `zero`, `.zero`, arbitrary query languages,
project selection, or patch commit do not authorize implementation behavior.

## Authorized surface

```text
ling query --symbol NAME [OUTPUT] SOURCE.ling
ling patch [OUTPUT] TRANSACTION.json SOURCE.ling
```

Query validates and NFC-normalizes one Unicode 17.0.0 Ling identifier, then
selects exact checked user definitions in canonical Semantic Graph order. Zero
matches is successful. The response exposes semantic identities and checked
type/Effect/Capability facts, but no path, span, evaluation, or mutation.

Patch compiles the current source first, reads a bounded exact-version request,
checks the base Program ID and target authorization, compiles the replacement
in memory, and compares canonical checked definitions. It requires an unchanged
definition set, types, Effects, and Capabilities; every changed Body ID must be
listed. Success is a deterministic proposal report and never a write.

## Compatibility and safety boundary

- Public failures use registered bilingual `L-QUERY-0001` and
  `L-TRANSACTION-0001` through `0003` diagnostics.
- Existing compiler diagnostics and exit classes retain their meanings.
- Source bytes, transaction bytes, metadata, directories, runtime state,
  Semantic ID algorithms, and Unicode 17.0.0 data remain unchanged.
- JSON schemas reject unknown fields; the transaction schema also exercises the
  real exact-version reader in governance validation.
- No source content or compiler-derived host path appears in a successful
  protocol result; caller provenance remains untrusted audit text.

## Required evidence

- unit and process tests for NFC query, empty results, deterministic output,
  stale precedence, checked candidate compilation, target authorization,
  preserve failures, and exact nonmutation;
- valid/invalid schema fixtures plus deterministic corrupt-input mutations;
- truthful help/catalog and output-policy regression suites;
- repository-wide offline tests, Clippy, CI, governance, support, status, RC0,
  traceability, formatting, and deterministic-diff gates.

## Intentionally deferred

Project/workspace queries, imports, references, fuzzy search, pagination,
general graph expressions, ranges, partial edits, rename, formatter edits,
tests, benchmarks, proofs, contracts, ABI preservation, delegated capability
tokens, `Graph.Commit`, atomic publication/rollback, LSP projection, migration,
and Stable compatibility require later Accepted authority.
