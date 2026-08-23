# IDE-2308 Implementation Report: Snapshot-Bound Completion Resolve

## Outcome

IDE-2308 implements the Accepted RFC-0043 Preview as capability-negotiated
standard `completionItem/resolve`. Clients that advertise lazy support for both
`detail` and `documentation` receive `ling.lsp.completion/0.2` items with opaque
`ling.lsp.completion-resolve/0.1` handles. Other clients retain the exact
RFC-0042 `ling.lsp.completion/0.1` discovery and item shape.

Resolution is bound to the complete immutable request snapshot. It preserves
the original RFC-0042 item and edit, then may add a bounded checked signature
and bilingual documentation from DEC-0080 type, Effect, and Capability facts
plus directly attached `///` Author Source text. Candidates without authorized
metadata resolve as an exact no-op; no documentation, identity, provenance, or
insertion behavior is fabricated.

## Normative clauses covered

- RFC-0043 §1: exact capability negotiation, plaintext/Markdown selection,
  resolve provider discovery, malformed-known-field rejection, and byte-for-
  field-compatible RFC-0042 fallback.
- RFC-0043 §2: versioned opaque BLAKE3 handles, deterministic length-delimited
  inputs, retained immutable snapshots and items, 1,024-handle bound, collision
  rejection, batch expiry, and absence of serialized internal facts.
- RFC-0043 §3: Ready/request-only dispatch, exact required item validation,
  malformed/modified/missing/expired handling, complete pre/post snapshot
  freshness, and synchronous bounded processing.
- RFC-0043 §4: exact DEC-0080 resolver-identity selection from a newly rebuilt
  checked metadata index, name/identity validation, and exact no-op behavior for
  unsupported candidate kinds or absent authorized metadata.
- RFC-0043 §5-§6: deterministic display-only full type, canonical Effect and
  Capability components, checked CST/Format IR attachment of contiguous `///`
  text, bilingual factual/Author Source documentation, safe negotiated
  MarkupContent, and detail/documentation byte bounds.
- RFC-0043 §7: exact preservation of label, kind, filtering/ranking,
  PlainText insertion format, original text edit, Unicode 17.0.0 spelling,
  SourceMap position projection, formatter/comment bytes, and source state.
- RFC-0043 §8: exact Completion Item result, fixed bilingual
  InvalidParams/RequestFailed errors, response-size failure, no null/partial
  result, and explicit compatibility boundary.
- DEC-0002/DEC-0029: compiler spans remain original UTF-8 bytes and the retained
  edit remains projected in the negotiated UTF-8/UTF-16/UTF-32 encoding.
- DEC-0019/DEC-0071: all analysis uses owned request snapshots and fresh
  compiler instances without persistent cache promises.
- DEC-0079/DEC-0080: resolver identities remain internal; only existing checked
  metadata values are selected, never inferred or serialized as identity.

## Implementation evidence

- `crates/ling-db/src/checked_completion_catalog.rs` retains an optional
  existing DEC-0080 identity for source-backed user definitions and bindings;
  all other candidate classes explicitly retain no presentation identity.
- `crates/ling-lsp/src/completion.rs` emits negotiated data only after the same
  checked candidate validation, creates deterministic handles, retains exact
  item/snapshot facts, and publishes the batch only after response and freshness
  checks succeed.
- `crates/ling-lsp/src/completion_resolve.rs` owns capability parsing, bounded
  handle state, exact request validation, stale checks, checked metadata lookup,
  formatter-backed documentation attachment, deterministic rendering/Markdown
  escaping, no-op policy, fixed errors, and bounds.
- `crates/ling-lsp/src/lib.rs` dispatches the standard method and advertises
  exact fallback or resolve-enabled provider/discovery values.
- `crates/ling-lsp/tests/completion_resolve.rs` drives the real JSON-RPC handler
  across negotiation, exact discovery, checked presentation, repeatability,
  attached Unicode documentation, Markdown escaping, Unicode/CRLF/BOM edit
  preservation, no-op resolution, malformed/modified/missing/stale errors,
  notification silence, and fallback behavior.
- `tests/protocols/lsp-completion-resolve/README.md` records the executable
  fixture boundary and the absence of filesystem/network/AI/documentation
  synthesis.

## Checks executed before status binding

- `cargo check -p ling-lsp --all-targets --locked --offline`
- `cargo test -p ling-lsp --test completion_resolve --locked --offline`
- `cargo test -p ling-lsp --all-targets --locked --offline`
- `cargo clippy -p ling-lsp --all-targets --locked --offline -- -D warnings`

The implementation tree is also required to pass the locked/offline workspace
test suite, strict workspace Clippy, CI, governance, LSP, support, status, RC0,
v0.0.1 traceability, formatting, and diff gates immediately before its
implementation commit. Those results are local command evidence only; no
hosted CI result is claimed. The same complete gate set is repeated after
status binding.

## Compatibility impact

- Protocol: advances Public Preview completion to negotiated
  `ling.lsp.completion/0.2` and `ling.lsp.completion-resolve/0.1`; clients
  lacking both lazy properties retain exact `ling.lsp.completion/0.1` behavior.
- Compiler API: adds one optional existing metadata identity to the internal
  candidate catalog. The identity is not serialized or made persistent.
- Server state: adds a bounded 1,024-entry in-memory handle registry containing
  shared immutable snapshots and exact items. It is session-local and has no
  cache compatibility promise.
- Diagnostics: no registered diagnostic allocation or compiler diagnostic
  change; protocol errors remain fixed JSON-RPC InvalidParams/RequestFailed.
- Identity/schema: handles are opaque correlation digests, not resolver or
  Semantic IDs; no Semantic ID, diagnostic, package, or cache schema changes.
- Language/runtime: no syntax, language semantics, unresolved-AST evaluation,
  Typed Core evaluation, interpreter, bytecode, VM, ABI, filesystem/network,
  or package behavior changes.
- Unicode: original spelling and edits remain governed by Unicode 17.0.0 and
  RFC-0042; no table, XID, normalization, or confusable-policy change.

## Determinism and safety

Handle input is explicit and length-delimited. A domain-separated digest covers
the complete request snapshot, including client versions and equal-byte state
changes; the final handle covers that digest, protocol marker, request URI,
replacement byte span, exact cursor byte offset, final item ordinal, stable
candidate kind, and existing internal candidate identity. Records use ordered
maps, snapshots are shared immutable values, and
checked metadata uses deterministic identity lookup and canonical
Effect/Capability ordering.

Resolution compares all insertion-critical fields with the retained item and
compares the complete current snapshot both before and after compiler work. It
does not echo unknown client fields, expose handle inputs, read host paths,
interpret unchecked AST, mutate source, run a formatter, access a network, or
invoke AI. Any mismatch, unsafe Markdown, missing checked fact, bound, stale
state, or oversized response fails atomically.

## Specification gaps and conflicts

RFC-0043 supplies the previously missing bounded completion-item lifecycle,
snapshot, presentation, insertion-preservation, error, limit, and migration
authority. It narrows rather than resolves the general Semantic Transaction,
formatter, Alias, localization, and Stable protocol gaps. No higher-authority
conflict was encountered.

SEMANTICS §3.8 accepts `///` documentation comments and permits a separate
`DocumentationId`, while the repository had no accepted IDE attachment,
normalization, localization, or wire contract. RFC-0043 now defines a bounded
snapshot-only CST/Format IR attachment and normalization rule without creating
or serializing `DocumentationId`. It still forbids inferred prose, detached
comments, translation, and fabricated documentation.

## Intentionally deferred

Documentation identities/localization, detached source excerpts, generated/
dependency/builtin/Prelude/field metadata, provenance fields, snippets,
additional edits, auto-imports, formatter rewriting, persistent or cross-session handles,
asynchronous cancellation, dynamic registration, AI assistance, and Stable
lifecycle remain deferred. IDE-2309 and later tasks may not infer edit or
transaction authority from completion resolve.
