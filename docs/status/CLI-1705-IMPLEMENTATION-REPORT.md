# CLI-1705 Implementation Report: Semantic query and patch commands

## Result

Implemented the complete bounded CLI-1705 surface authorized by Accepted
RFC-0027:

- `ling query --symbol NAME SOURCE.ling` performs exact Unicode-17.0.0 NFC
  lookup over checked user definitions and emits `ling.semantic-query/0.1`;
- `ling patch TRANSACTION.json SOURCE.ling` validates one full-source proposal
  and emits `ling.semantic-transaction-result/0.1` with `committed: false`;
- both commands share the existing CLI parser/dispatcher, checked compiler,
  canonical Semantic Graph, output policy, diagnostics, and exit catalog.

## Normative clauses covered

- RFC-0027 §§1–5: command selection, exact query projection, transaction input,
  validation order, preservation, nonmutation, capability boundary, and
  compatibility;
- `SEMANTICS.md` §25.2–25.5: checked temporary state, stale rejection,
  preservation, and Proposal/Commit separation;
- DEC-0012: existing Program/Definition/Body IDs are consumed without changing
  canonical identity bytes;
- DEC-0253 and DEC-0254: one dispatcher and the common human/JSON language,
  color, quiet, and verbose policy.

## Implementation

- `crates/ling-cli/src/semantic_commands.rs` owns versioned request/report data,
  exact query projection, bounded decoding, current-snapshot preflight, target
  authorization, and structural semantic diff validation.
- `crates/ling-cli/src/main.rs` owns command grammar, I/O, checked compilation,
  diagnostics, channels, and protocol rendering; patch never opens a writable
  source handle.
- `crates/ling-cli/src/command_catalog.rs` advertises the two implemented roots
  exactly once and retains rejection of all other plan-only/stale names.
- `ling-diagnostics` and `docs/ERROR-CODES.md` add one query and three transaction
  error identities with generated compatibility-lock evidence.
- Three public schema packages and the transaction reader adapter provide
  independent valid, invalid, and corrupt-input checks.

## Tests and verification

Focused evidence executed successfully before the implementation commit:

```text
cargo test -p ling-cli --locked --offline
cargo test -p xtask --locked --offline
cargo xtask schema validate-all
cargo xtask schema corrupt-inputs
cargo xtask governance check-all
cargo xtask support verify
```

The suites cover exact/NFC/Unicode/empty query behavior, repeated-process byte
determinism, stale rejection before candidate compilation, valid authorized
body changes, malformed and oversized requests, definition/type/Effect and
unauthorized-body drift, no-op rejection, candidate compiler failures through
the shared pipeline, truthful help, schema/reader rejection, and byte-for-byte
source/request nonmutation.

Repository-wide tests, Clippy, CI, status, RC0, traceability, rustfmt, and diff
checks are executed again immediately before committing and after status
binding.

## Compatibility impact

- **CLI:** adds two Preview roots and `--symbol`; existing roots/options/exits
  remain unchanged.
- **Diagnostics:** adds `L-QUERY-0001` and `L-TRANSACTION-0001..0003`; no existing
  meaning, severity, or payload type changes.
- **Schemas/protocols:** adds three 0.1 Preview schemas; no predecessor or Stable
  claim exists.
- **Semantic IDs:** no encoding, algorithm, schema, or identity-prefix change.
- **Source spans/runtime:** no span transformation, AST evaluation, runtime,
  bytecode, VM, ABI, cache, network, or filesystem mutation.
- **Determinism/Unicode:** canonical graph order and Unicode 17.0.0 remain the
  only semantic sources; output adds no compiler-derived path, clock,
  environment, random, allocation, or map-order data. Caller provenance is
  preserved as untrusted audit text.

## Specification gaps encountered

The original audit correctly found no accepted public query or transaction
schema. RFC-0027 now closes only the CLI-1705 proposal slice. The broader
`GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` and
`GAP-LSP-TRANSACTION-PROTOCOL-001` remain Open for commit, project/editor,
migration, and Stable-lifecycle work.

## Intentionally deferred

Projects/workspaces/imports, general graph queries, references, fuzzy search,
pagination, source ranges, partial edits, rename, formatter integration,
required tests/benchmarks/proofs/contracts, ABI constraints, delegated
authorization, disk commit/rollback, LSP `WorkspaceEdit`, migration, and Stable
compatibility are not implemented or implied.
