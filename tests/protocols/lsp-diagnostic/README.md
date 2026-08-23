# `ling.lsp.diagnostic/0.2` Experimental fixture

This fixture records the Accepted RFC-0031 compiler-diagnostic adapter and the
RFC-0032 backward-compatible temporary-source extension:

- every input is an existing registered Ling diagnostic with a primary
  original UTF-8 byte span;
- exact Ling URI/source identities are unique and path-free; 0.2 accepts the
  complete 0.1 non-temporary set plus validated `untitled://ling/...` sources;
- primary and related spans project strictly through UTF-8, UTF-16, or UTF-32
  without clamping scalar, surrogate, BOM, or CRLF boundaries;
- stable codes, bilingual messages, Error/Warning/Note severity, Facts,
  Semantic IDs, repairs, and related-label order map to exact JSON fields;
- results follow DEC-0034 logical-source/span/code/input ordering and serialize
  repeatably;
- an invalid source, primary span, related span, or later input rejects the
  complete call without partial output;
- push publication, pull parity, and diagnostic control are separately covered
  by their current 0.2/0.1 protocol fixtures; Accepted RFC-0035 also records
  exact stdio transcript composition across those boundaries. Workspace Edits
  remain outside this fixture.

Executable evidence:

```text
cargo test -p ling-diagnostics --all-targets --locked --offline
cargo test -p ling-lsp --test diagnostic_adapter --locked --offline
cargo test -p ling-lsp --test diagnostic_transcripts --locked --offline
```
