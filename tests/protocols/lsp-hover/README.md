# `ling.lsp.hover/0.1` Preview fixture

Accepted RFC-0037 defines the bounded checked `textDocument/hover` provider:

- initialize validates the standard hover capability, selects the first
  supported plaintext or Markdown format, advertises `hoverProvider`, and
  reports the immutable selected format and limits;
- one current path-free snapshot joins exact declaration, binding, parameter,
  and resolver-filtered reference spans to complete checked type, Effect,
  Capability, and concrete Trait-selection facts;
- exact original UTF-8 identifier spans project through negotiated
  UTF-8/16/32 without clamping, while a valid non-target position returns
  `null`;
- output is deterministic, bilingual, bounded, and contains no resolver or
  Semantic IDs, implementation ordinals, host paths, or compiler debug values;
- malformed inputs, incomplete checked programs, unsafe Markdown, overflow,
  stale snapshots, and projection failures produce fixed atomic failures.

Executable evidence:

```text
cargo test -p ling-db --all-targets --locked --offline
cargo test -p ling-lsp --test hover --locked --offline
```
