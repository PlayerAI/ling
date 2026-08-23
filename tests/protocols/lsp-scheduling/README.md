# LSP scheduling protocol fixtures

`fixtures/v1.json` is the exact Preview fixture for
`ling.lsp.scheduling/0.1`. It freezes the initialize discovery object, logical
priority classes, fixed fairness bounds, message-boundary debounce, wire-order
request rule, and stale-analysis supersession marker.

The fixture deliberately contains no duration, deadline, worker count, CPU
count, host load, path, source text, request ID, or serialized queue sequence.
`crates/ling-lsp/tests/scheduling.rs` consumes it directly; scheduler unit tests
cover canonical FIFO/priority behavior and both bounded-starvation limits.
