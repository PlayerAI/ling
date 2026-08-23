# `ling.lsp.document-symbol/0.1` Preview fixture

Accepted RFC-0036 defines the bounded `textDocument/documentSymbol` provider:

- initialize validates the standard client capability, advertises
  `documentSymbolProvider`, and reports the immutable hierarchical/flat mode;
- one current path-free snapshot resolves module, record/field, variant/case,
  alias, function/value, Trait/member, and implementation/member structure;
- hierarchical output preserves separate full and selection ranges, while the
  standard flat fallback uses the same tree in stable pre-order with immediate
  containers;
- original UTF-8 spans project through negotiated UTF-8/16/32 without
  clamping; Chinese, emoji, BOM, CRLF, temporary isolation, limits, recovery,
  repeated output, and malformed/failure cases are executable;
- no Semantic ID, host path, compiler debug output, partial truncation, or
  placeholder editor behavior enters the wire response.

Executable evidence:

```text
cargo test -p ling-db --all-targets --locked --offline
cargo test -p ling-lsp --test document_symbols --locked --offline
```
