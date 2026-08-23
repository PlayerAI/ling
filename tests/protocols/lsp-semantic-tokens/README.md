# `ling.lsp.semantic-tokens/0.1` Preview fixture

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

```text
cargo test -p ling-db --test semantic_tokens --locked --offline
cargo test -p ling-lsp --test semantic_tokens --locked --offline
```
