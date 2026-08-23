# IDE-2310 Authority Audit: Formatting

## Outcome

`IDE-2310` is authorized and implemented for the bounded whole-document slice.
Accepted RFC-0026 defines `ling.lsp.formatting/0.1`, and implementation commit
`0421925f3a8e20f6bc951eff546b00523c3f36ff` already supplies the advertised
`textDocument/formatting` method through the compiler-CST formatter.

The earlier BlockedSpec conclusion predated RFC-0026 and the completed
FMT-1507 integration. It is no longer accurate. The execution plan requires
document formatting and permits range formatting only after its boundary
semantics and tests exist. RFC-0026 deliberately defers range formatting, so
that deferred feature is not part of IDE-2310 completion.

## Authority reconciliation

- RFC-0026 §§1–5 is Accepted and defines the exact Experimental capability,
  request, snapshot, formatter, `TextEdit`, failure, and compatibility rules.
- RFC-0023 supplies the open-overlay URI, version, ownership, and writability
  contract used by the handler.
- DEC-0002 and DEC-0029 govern original UTF-8 spans and negotiated UTF-8,
  UTF-16, or UTF-32 projection.
- DEC-0023 defines the Author Source preservation and fail-closed publication
  boundary; DEC-0057 exposes its deterministic whole-document `FormatEdit`.
- Accepted DEC-0028 names the public formatter CLI `ling fmt`. The lower-
  authority plan's `ling-fmt` spelling does not override that command name.
  The IDE adapter correctly reuses the formatter library in process instead of
  spawning a second parser or a subprocess.
- FMT-1507 and LSP-2102, the two recorded dependencies of IDE-2310, are Done.

## Current implementation evidence

- `crates/ling-lsp/src/lib.rs` advertises
  `documentFormattingProvider: true`, dispatches
  `textDocument/formatting`, validates the exact fixed options, and formats
  only a current open writable workspace or untitled overlay.
- The adapter constructs `SourceFile`, parses with `ling-syntax`, builds the
  existing `FormatDocument`, and calls `format_core_edit`. It has no regex,
  Tree-sitter, unchecked AST, alternate style engine, or filesystem fallback.
- Changed valid source returns exactly one whole-document `TextEdit`; unchanged,
  invalid, or recovery source returns an empty array. The server never applies
  the edit or changes text, version, VFS, disk, project, Semantic Graph, Audit,
  or runtime state.
- End positions are projected from the exact original snapshot in the
  negotiated encoding. BOM is preserved outside LSP position zero, and CRLF
  ranges are computed from original bytes while accepted formatter output uses
  LF.
- `PROTO-LSP-FORMATTING` registers the implemented Experimental
  `ling.lsp.formatting/0.1` protocol and its executable fixtures.

## Conformance evidence

`crates/ling-lsp/tests/formatting.rs` directly covers:

1. exact capability and protocol marker discovery;
2. hard-coded UTF-8, UTF-16, and UTF-32 ranges over Unicode source;
3. BOM, CRLF, latest-overlay version, repeat determinism, and zero mutation;
4. changed, unchanged, invalid, missing, closed, read-only, invalid-URI, and
   malformed-option behavior; and
5. notification, preinitialize, and post-shutdown fail-closed behavior.

Formatter suites separately cover CST completeness, comments/documentation,
literal and identifier preservation, invalid-source publication, idempotence,
and semantic/Audit separation. Repository gates validate the protocol and
support inventories.

## Remaining gaps and compatibility

`GAP-FORMATTER-AUTHOR-SOURCE-001`,
`GAP-LSP-TRANSACTION-PROTOCOL-001`, and
`GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` still govern broader formatting and editor
work. They do not invalidate the explicitly bounded Accepted RFC-0026 slice.

No compiler syntax or semantics, Typed Core evaluation, interpreter, VM,
bytecode, ABI, diagnostic allocation, schema, Semantic ID, source-span unit,
package behavior, runtime behavior, or Unicode 17.0.0 data changes as part of
this reconciliation.

## Intentionally deferred

Range/on-type formatting, format-on-save, configurable style, minimal diffs,
closed-file or dependency formatting, filesystem mutation, `WorkspaceEdit`,
general Semantic Transaction behavior, cancellation, asynchronous publication,
multi-document edits, and Stable compatibility remain unimplemented and
unclaimed. Each requires separate Accepted authority and executable evidence.
