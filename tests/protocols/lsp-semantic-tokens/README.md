# `ling.lsp.semantic-tokens/0.1` Preview fixtures

Accepted RFC-0048 defines the bounded semantic-token transport:

- initialize validates the standard semantic-token capability, selects the
  RFC-0046 canonical client-supported legend, and advertises full plus optional
  delta support only when relative encoding and a nonempty type legend exist;
- each request captures and revalidates one immutable document/workspace
  snapshot and consumes only RFC-0047 checked or conservative-fallback tokens;
- original UTF-8 spans project through negotiated UTF-8/16/32 units into
  source-ordered, nonoverlapping standard relative integer groups;
- opaque deterministic result IDs use only client-visible protocol data, and a
  32-entry FIFO session history retains no source or compiler identity;
- a valid base produces one canonical middle-replacement delta, while missing,
  expired, or foreign-document bases recover to a full response;
- invalid inputs, cancellation, staleness, projection failures, token/data
  overflow, and frame overflow fail atomically without partial publication.

Executable evidence:

- `fixtures/v1.json` is a deterministic, test-only corpus identified by
  `ling.test.lsp-semantic-tokens/1`. It is not a public schema or compatibility
  surface.
- Each case fixes its negotiated position encoding, URI, document version,
  exact UTF-8 source, and complete JSON-RPC result.
- The corpus covers UTF-16 BOM/CRLF/emoji/Chinese projection; checked scope,
  mutable-field and variant roles; Effect/Capability exclusion; conservative
  error recovery; and an exact canonical delta plus its equivalent full result.
- `crates/ling-lsp/tests/semantic_tokens.rs` is the independent reader. It
  executes every case in fresh sessions, checks exact deterministic results,
  validates order/non-overlap and category-specific invariants, and reapplies
  the frozen delta to its base.

Commands:

```text
cargo test -p ling-db --test semantic_tokens --locked --offline
cargo test -p ling-lsp --test semantic_tokens --locked --offline
```

Incompatible public output requires a new Accepted taxonomy, generation, or
transport marker. The test-only fixture format may be revised independently
when it continues to encode the same Accepted public behavior.
