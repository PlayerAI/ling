# CLI-1702 Authority Audit: Output and exit behavior

## Outcome

CLI-1702 now has sufficient Accepted authority and a complete bounded
implementation. DEC-0254 defines one output policy for every current non-LSP
command. It accepts language ordering, diagnostic color, quiet/verbose behavior,
channel ownership, JSON isolation, and the unchanged Seed exit classes.

The decision does not authorize output behavior for unimplemented commands or
make Preview human bytes Stable. CLI-1705 query/patch and CLI-1706 completion
remain separate.

## Normative traceability

- DEC-0003 fixes the hand-written parser and `--format human|json` baseline.
- DEC-0013 and DEC-0037 fix exit codes 0, 1, 2, 4, 5, and 6, reserve 3, and
  separate compile errors, program Faults/host failures, internal incidents,
  and snapshot mismatches.
- `docs/SEMANTICS.md` §26 requires registered bilingual diagnostics, stable
  codes, original UTF-8 spans, Facts, and repairs.
- DEC-0253 fixes the exact current command catalog and sole parser/dispatcher.
- DEC-0254 accepts `OutputPolicy`, exact option values and defaults, language
  ordering without language removal, ANSI isolation, quiet/verbose boundaries,
  stdout/stderr ownership, LSP rejection, and exit-code invariance.

## Implementation evidence

- `crates/ling-cli/src/output_policy.rs` owns the immutable format, language,
  color, and verbosity policy and deterministic rendering rules.
- `Options::parse` accepts each policy option at most once, rejects invalid or
  incompatible combinations with exit 2, and rejects every policy option for
  `ling lsp --stdio`.
- Every current human diagnostic uses the bilingual renderer; `zh-CN` and `en`
  change order only. JSON retains `message_zh` and `message_en` and never emits
  ANSI.
- Quiet mode suppresses auxiliary successful summaries only. Program output,
  diagnostics, failures, formatted source, Semantic Graph, Audit Source, REPL
  output, and format-check change reports remain observable.
- Verbose mode emits one deterministic bilingual, path-free policy event after
  parsing and before dispatch.
- Unit and process-level tests cover defaults, parsing, ordering, color,
  machine-output isolation, quiet program output, verbose determinism, and LSP
  purity while retaining all command-specific exit suites.

## Compatibility and gap disposition

No JSON schema, diagnostic allocation, Semantic ID, source span, language
semantic, runtime Fault, artifact, bytecode, VM, ABI, or Unicode 17.0.0 behavior
changes. Human output remains Preview and non-canonical. Machine consumers must
continue to select JSON or Audit output and interpret the existing exit catalog.

The earlier specification blocker is closed by Accepted DEC-0254. Localized
help grammar, locale/environment inference, themes, progress events, tracing,
future commands, and Stable output compatibility remain intentionally deferred.
