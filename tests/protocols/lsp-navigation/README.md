# `ling.lsp.navigation/0.1` Preview fixture

Accepted RFC-0038 defines bounded resolver-backed definition, declaration, and
type-definition navigation:

- initialize validates the three standard client capability objects,
  advertises static providers, and reports the immutable one-target limit;
- definition and declaration consume complete resolution and return the same
  exact Seed definition or binding identifier location;
- type definition consumes a complete checked workspace and returns only a
  direct nominal record/variant type after peeling function result layers;
- tracked workspace and read-only dependency URIs are reused exactly; source-
  less builtin/Prelude targets and unsupported positions return `null`;
- original UTF-8 spans project through negotiated UTF-8/16/32 without
  clamping, while IDs, ordinals, paths, arrays, virtual documents, and
  `LocationLink` fields never enter the response.

Executable evidence:

```text
cargo test -p ling-db --all-targets --locked --offline
cargo test -p ling-lsp --test navigation --locked --offline
```
