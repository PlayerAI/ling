# `ling.lsp.diagnostic/0.1` Experimental fixture

This fixture records the Accepted RFC-0031 compiler-diagnostic adapter:

- every input is an existing registered Ling diagnostic with a primary
  original UTF-8 byte span;
- exact non-temporary Ling URI/source identities are unique and path-free;
- primary and related spans project strictly through UTF-8, UTF-16, or UTF-32
  without clamping scalar, surrogate, BOM, or CRLF boundaries;
- stable codes, bilingual messages, Error/Warning/Note severity, Facts,
  Semantic IDs, repairs, and related-label order map to exact JSON fields;
- results follow DEC-0034 logical-source/span/code/input ordering and serialize
  repeatably;
- an invalid source, primary span, related span, or later input rejects the
  complete call without partial output;
- publication, document versions, debounce, stale-result replacement,
  suppression, caps, and Workspace Edits remain outside this fixture.

Executable evidence:

```text
cargo test -p ling-diagnostics --all-targets --locked --offline
cargo test -p ling-lsp --test diagnostic_adapter --locked --offline
```
