# `ling.lsp.workspace-symbol/0.1` Preview fixture

Accepted RFC-0045 defines the bounded `workspace/symbol` provider:

- initialize advertises the exact standard provider and immutable Experimental
  discovery object;
- each request captures and revalidates one complete snapshot and searches
  resolver-owned user definitions from tracked non-temporary writable
  workspace sources;
- empty, exact, and case-sensitive prefix queries use a fixed total order,
  exact names precede prefixes, and only the first 256 matches are emitted;
- standard `SymbolInformation` carries exact source spelling, resolver-kind
  mapping, module container, workspace URI, and original-span UTF-8/16/32
  projection without internal identities or host paths;
- one disposable exact-snapshot cache is published only after complete success;
  invalid input, compiler failure, staleness, response overflow, and cooperative
  cancellation produce fixed atomic failures without partial results.

Executable evidence:

```text
cargo test -p ling-lsp --test workspace_symbols --locked --offline
cargo test -p ling-lsp --test diagnostic_transcripts --locked --offline
```
