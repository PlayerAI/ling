# LSP-2403-SNAPSHOT-IDENTITY implementation report

## Result

The bounded checked-token snapshot-identity child is implemented under
Accepted DEC-0086. It records existing source identity and VFS revision for
future same-snapshot analysis without inventing transport state.

## Implementation

- `CheckedTokenSourceIndex` exposes the existing `SourceId` and session-local
  `Revision` used to build the immutable observation.
- `CompilerDb::checked_token_source_index` passes the source snapshot revision
  and continues caching by the existing `QueryKey`.
- Repeated unchanged queries reuse the same object; the observation contains no
  document version, result ID, token encoding, or delta state.

## Verification

- `cargo fmt --all -- --check`
- `cargo test -p ling-db --all-targets --locked --offline` — 45 tests passed.
- `cargo clippy -p ling-db --all-targets --locked --offline -- -D warnings`

## Boundaries

This child is not semantic-token generation, a full response, delta encoding,
position projection, version negotiation, cancellation, stale handling,
result-ID storage, or an LSP/JSON-RPC response. Public `LSP-2403` remains
`BlockedSpec`.
