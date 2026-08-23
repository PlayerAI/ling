# CLI-1701 Authority Audit: Unified command model

## Outcome

CLI-1701 now has sufficient Accepted authority and a complete bounded
implementation. DEC-0253 accepts the existing `ling` single parser/dispatcher
and composes only commands whose individual behavior is already authorized and
implemented. The FMT-1507 and PRJ-1107 dependencies are Done.

This outcome does not pull CLI-1702 through CLI-1706 into scope. In particular,
`query`, `patch`, shell completion, generalized output policy, plugin commands,
and Stable compatibility remain unimplemented and unadvertised.

## Normative traceability

- DEC-0003 defines the hand-written parser, top-level controls, invalid-usage
  boundary, and original file command baseline.
- DEC-0013 defines checked compilation/execution and tool-versus-program
  failure separation.
- DEC-0036 centralizes current command identity without owning options or
  execution.
- DEC-0028, DEC-0038, and DEC-0039 authorize the current formatter,
  initializer, and standalone test commands.
- RFC-0004 authorizes only the `ling lsp --stdio` launcher and its
  protocol-pure transport.
- RFC-0024 and RFC-0025 authorize graph-only `project check` and the explicit
  locked/offline semantic project commands.
- DEC-0253 fixes the exact current catalog, single selection/parse/dispatch
  path, service-reuse meaning, non-advertisement rule, and compatibility
  boundary.

The lower-authority `zero`/`.zero` spellings remain rejected.

## Implementation evidence

`crates/ling-cli/src/main.rs` provides one path:

```text
run(arguments) -> Command -> Options::parse -> execute
```

`command_catalog.rs` contains each implemented command exactly once. The
dispatcher delegates to existing authoritative boundaries: the checked file
pipeline, locked project pipeline, test runner, formatter, initializer, graph
checker, REPL session, and LSP stdio server. It does not contain a duplicate
compiler, unchecked AST interpreter, formatter, resolver, or transport.

The current roots are `run`, `check`, `semantic`, `audit`, `test`, `build`,
`fmt`, `init`, `repl`, and `lsp`; `project check` remains hierarchical. Parser
and help tests reject planned and stale roots.

## Gap disposition

- `GAP-PROJECT-CLI-INTERFACE-001` is Accepted through RFC-0024/RFC-0025 for
  the composed project forms.
- `GAP-FORMATTER-CLI-PROTOCOL-001` is Accepted through DEC-0028 for `ling fmt`.
- The open LSP/Semantic Transaction gaps do not block the already Accepted
  lifecycle launcher and do not authorize `query` or `patch`; those commands
  remain absent.
- CLI-1702 output policy, CLI-1705 query/patch, and CLI-1706 completion/help
  artifacts remain separate tasks and are not implied by this parent closure.

## Compatibility and deferred work

DEC-0253 adds no command, alias, option, output field, exit code, diagnostic,
schema, Semantic ID, Audit byte, package rule, bytecode, VM behavior, ABI,
source-span change, or Unicode data. `PROTO-CLI` remains Preview at
`0.0.1-dev`; only its authority and evidence are made current.

Generalized language/color/quiet/verbose behavior, Semantic Query/Transaction,
completion scripts, help golden fixtures, explain/replay/evidence/support/
migration commands, plugins, daemons, and Stable compatibility are
intentionally deferred.
