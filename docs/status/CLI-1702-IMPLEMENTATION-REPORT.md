# CLI-1702 implementation report

## Result

CLI-1702 is complete under DEC-0254. Every current non-LSP command receives one
parsed `OutputPolicy` with deterministic format, language, color, and verbosity
behavior. The existing process exit catalog remains unchanged.

## Implemented surface

- `--format human|json`, default `human`;
- `--language bilingual|zh-CN|en`, default `bilingual`;
- `--color auto|always|never`, default `auto`;
- mutually exclusive `--quiet` and `--verbose`, default normal;
- bilingual human diagnostics in selectable order without dropping either
  language;
- ANSI color only for human diagnostics on stderr;
- auxiliary success-summary suppression in quiet mode;
- one deterministic, bilingual, path-free stderr policy event in verbose mode;
- protocol-pure LSP rejection of every output-policy option; and
- unchanged exits 0, 1, 2, 4, 5, and 6, with 3 still reserved.

## Evidence

`crates/ling-cli/src/output_policy.rs` centralizes policy and rendering.
`crates/ling-cli/src/main.rs` parses once and threads the value through current
command handlers. `crates/ling-diagnostics/src/lib.rs` provides exact bilingual,
Chinese-first, and English-first human rendering while preserving codes, Facts,
repairs, and byte spans.

Unit and integration suites verify parser exclusions and invalid combinations,
diagnostic ordering and color, JSON ANSI exclusion, quiet program output,
verbose determinism/path independence, existing schemas, LSP transport purity,
stdout/stderr ownership, and exit behavior. Repository-wide evidence is bound
to the implementation commit in the task registry after all gates pass.

## Compatibility impact

Human diagnostics become bilingual by default and current commands gain four
Preview controls. JSON fields and cardinality are unchanged. There is no change
to Ling semantics, Checked Core, diagnostics identity, Semantic IDs, Audit,
project artifacts, bytecode, VM, ABI, source spans, deterministic machine
output, offline behavior, or Unicode 17.0.0.

## Intentionally deferred

Localized help/parser grammar, locale inference, color themes, progress bars,
timestamps, tracing levels, JSON event streams, retries, future command output,
shell completion, and Stable byte compatibility remain outside CLI-1702.

