# IDE-2309 Implementation Report: Bounded Code Actions

## Outcome

IDE-2309 implements the Accepted RFC-0044 Seed surface:
`textDocument/codeAction` may return exactly one preferred
`source.fixAll.ling.format` action for a changed, valid, open writable document.
The action contains one versioned transactional `documentChanges` edit derived
solely from the accepted compiler-CST formatter boundary. The server never
applies the edit.

## Normative clauses covered

- RFC-0044 §§1-2: capability validation, conditional discovery, request shape,
  kind filtering, notification silence, and opaque client diagnostics.
- RFC-0044 §§3-4: complete immutable snapshot eligibility and a structured
  request-local formatter `FixPlan` built only from `FormatEdit`.
- RFC-0044 §§5-6: exact UTF-8/16/32 whole-document projection, BOM/CRLF policy,
  captured client version, transactional Workspace Edit, freshness, response
  bound, deterministic empty results, and fixed bilingual failures.
- RFC-0026 and DEC-0057: compiler-CST Author Source formatting and the accepted
  one-edit publication boundary.
- DEC-0002, DEC-0029, RFC-0023, RFC-0038, and RFC-0041: original-byte position
  truth, negotiated projection, overlay identity/version, complete snapshots,
  and transactional `documentChanges` capability.

## Implementation evidence

- `crates/ling-lsp/src/code_action.rs` owns capability and request parsing,
  immutable formatter plan construction, exact edit projection, and result
  rendering.
- `crates/ling-lsp/src/lib.rs` owns lifecycle dispatch, negotiated state,
  before/after complete-snapshot comparison, encoded response bounds, and
  fixed JSON-RPC failures.
- `crates/ling-lsp/tests/code_action.rs` contains six integration tests for
  negotiation, malformed clients, exact results, three position encodings,
  Unicode, BOM/CRLF, latest versions, determinism, diagnostic opacity, filters,
  invalid sources, read-only/missing documents, and notifications.
- `tests/protocols/lsp-code-action/README.md` records the executable fixture
  boundary and intentional omissions.

## Specification gaps and conflicts

The non-normative execution plan listed missing-import, confusable-rename,
mutability, match-case, stale-syntax, and formatter actions. Only the formatter
has an Accepted checked edit producer in the Seed repository. RFC-0044 therefore
closes the bounded Seed task with the real formatter plan and explicitly defers
the speculative semantic actions. DEC-0081 repair observations remain metadata,
not edits; no diagnostic message or client-returned repair becomes authority.

No higher-authority conflict was found. RFC-0044 composes existing formatter,
snapshot, position, overlay, and Workspace Edit authorities without creating a
general Semantic Transaction.

## Compatibility impact

- Protocol: adds Public Preview `ling.lsp.code-action/0.1`; incapable clients
  retain the previous initialize result and no provider.
- Diagnostics/schema/Semantic IDs: no error-code allocation, Repair kind, Facts
  schema, diagnostic mapping, Semantic ID, or canonical-byte change.
- Determinism: results depend only on negotiated capabilities/encoding and the
  complete immutable request snapshot; no map order, path, clock, environment,
  filesystem, network, or debug output enters the wire.
- Unicode: continues to use the existing Unicode 17.0.0 source and position
  behavior; no generated table changes.

## Checks

Executed during implementation:

- `cargo check -p ling-lsp --all-targets --locked --offline`
- `cargo test -p ling-lsp --test code_action --locked --offline` (6 passed)
- `cargo clippy -p ling-lsp --all-targets --locked --offline -- -D warnings`
- `cargo xtask governance check-all`
- `cargo test --workspace --all-targets --locked --offline --quiet`
- `cargo clippy --workspace --all-targets --locked --offline -- -D warnings`
- `cargo xtask ci verify`
- `cargo xtask lsp verify`
- `cargo xtask support verify`
- `cargo xtask status verify`
- `cargo xtask rc0 verify`
- `cargo xtask traceability verify --release v0.0.1`
- `cargo fmt --all -- --check`
- `git diff --check`

All listed checks passed on the implementation tree before the implementation
milestone commit.

## Intentionally deferred

Missing-import, confusable-rename, mutability, match-case, stale-syntax,
diagnostic quick-fix, multi-document, resolve, command, annotation,
generated/dependency mutation, asynchronous cancellation, general Semantic
Transaction, and Stable lifecycle behavior remain deferred under RFC-0044.
