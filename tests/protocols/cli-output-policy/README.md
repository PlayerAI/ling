# CLI output policy fixtures

This fixture boundary covers DEC-0254 and CLI-1702.

Executable evidence lives in:

- `crates/ling-cli/src/main.rs` parser unit tests;
- `crates/ling-cli/src/output_policy.rs` renderer unit tests;
- `crates/ling-cli/tests/output_policy.rs` process-level channel, language,
  color, quiet, verbose, and JSON-isolation tests; and
- the existing command-specific integration suites for exit and schema
  compatibility.

Human output is Preview and intentionally has no byte schema. Machine-readable
JSON schemas are unchanged. LSP stdout remains framed protocol output and
rejects every output-policy flag.

