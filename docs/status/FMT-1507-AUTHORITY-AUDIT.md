# FMT-1507 Authority Audit: Formatter CLI/LSP Integration

## Outcome

FMT-1507 is correctly recorded as `BlockedSpec`. The bounded CLI preview is
authorized by DEC-0028, and DEC-0057 now supplies an in-process whole-document
edit value. The accepted authority still does not define the public LSP edit,
range protocol, format-on-save hook, or Semantic Transaction behavior required
by the parent task.

The current public CLI is `ling`; the lower-authority plan spelling `zero fmt`
is stale and is not propagated. The existing `PROTO-CLI` inventory entry remains
limited to the accepted command and option surface, so FMT-1507 does not claim a
new public protocol.

## Normative traceability

- `DEC-0023` §9 explicitly adds no formatter CLI/LSP command, range-format
  protocol, JSON schema, or public stability claim. Its compatibility clauses
  also do not imply automatic format-on-save or range edits.
- `DEC-0003` fixes the current M0 CLI parser and command baseline; it does not
  authorize a formatter command or its process contract.
- `DEC-0023` §8 and `DEC-0015` keep Author Source formatting separate from
  canonical Audit Source rendering. The in-process formatter remains available
  only through the accepted library boundary already implemented by FMT-1502
  through FMT-1506.
- `DEC-0057` authorizes only `FormatEdit`/`format_core_edit`: one original
  UTF-8 byte-range replacement or no edit. It does not authorize an LSP
  `TextEdit`, `WorkspaceEdit`, URI, document version, position encoding, or
  transaction.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` fix the public executable name to
  `ling` and the source extension to `.ling`; the execution-plan `zero` spelling
  cannot override them.

## Specification gaps and conflicts

The following gaps are now the explicit blockers rather than implicit plan
assumptions:

- `GAP-FORMATTER-CLI-PROTOCOL-001` covers the command spelling, stdin and
  logical-filename rules, check-mode output and exit status, and JSON report
  schema.
- `GAP-FORMATTER-AUTHOR-SOURCE-001` keeps broader formatter normalization and
  localization policy from becoming an implicit public CLI guarantee.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` covers position encoding, document
  versions, stale-edit preconditions, and Workspace Edit shape for any LSP
  formatter request.
- `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` covers Stable versus Experimental
  transaction fields, reader/writer compatibility, and schema migration.

The lower-authority plan is therefore not sufficient authorization for changing
public CLI/LSP behavior or creating a public protocol. The internal edit
projection is isolated by DEC-0057 and adds no wire or semantic behavior.

## Tests and evidence

The bounded children provide focused tests for the accepted CLI report and
in-process edit value; no public LSP tests were added. The audit itself was
checked against:

- the current `crates/ling-cli/src/main.rs` parser and `PROTO-CLI` inventory;
- `DEC-0003`, `DEC-0015`, and accepted `DEC-0023` clauses;
- accepted `DEC-0028` and `DEC-0057` clauses;
- the `ling`/`.ling` requirements in `docs/SEMANTICS.md` and `docs/LANGUAGE.md`;
- the three registered specification gaps listed above.

The governance and status renderers remain the acceptance mechanisms for this
blocked task; no implementation, diagnostic code, schema version, Semantic ID,
Audit byte, source span, or Unicode table changed.

## Intentionally deferred

FMT-1507 can start only after an accepted formatter CLI decision and the LSP
transaction authority define their externally visible fields and failure
behavior. FMT-1508 now supplies the separate in-process proof that Author
Source formatting does not replace or mutate canonical Audit rendering; it does
not unblock the missing public protocol decisions.
