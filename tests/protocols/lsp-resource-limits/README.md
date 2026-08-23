# LSP resource-limit protocol fixtures

`fixtures/v1.json` is the exact Preview fixture for
`ling.lsp.resource-limits/0.1`. It freezes fixed discovery, the registered
`L-LSP-0002` resource failure, integer/string Fact types, and the absence of
URI, path, source, allocator, timing, or request-ID data from the diagnostic.

`crates/ling-lsp/tests/resource_limits.rs` consumes the fixture and proves
individual and aggregate UTF-8 byte boundaries, failure-atomic open/change,
close/retry cleanup, Unicode byte accounting, 128 live request associations,
duplicate precedence, deterministic 129th-request rejection, and response-free
notifications. Existing completion, diagnostic-control, transport, and Trait
solver tests retain their independently accepted bounds.
