# LSP diagnostic transcript fixtures

This directory is the internal Preview corpus defined by Accepted RFC-0035.
Each `*.input.jsonl` and `*.output.jsonl` file contains one compact UTF-8
JSON-RPC object per LF-terminated line without transport headers. The
`ling-lsp` conformance test applies strict RFC-0004 framing, runs the real stdio
host twice, and compares exact output-body bytes and ordering.

The fixtures contain only path-free Ling URIs and cover UTF-16 Chinese,
emoji, BOM, CRLF, incremental recovery, push/pull parity, diagnostic-control
summaries, and invalid initialization. Stale-ticket and cancellation boundaries
that cannot be raced through the sequential stdio host remain in focused Rust
integration tests as required by RFC-0035 §5.
