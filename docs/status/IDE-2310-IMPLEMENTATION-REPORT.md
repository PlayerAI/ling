# IDE-2310 implementation report

## Result

IDE-2310 is complete for the Accepted bounded document-formatting surface.
Implementation commit `0421925f3a8e20f6bc951eff546b00523c3f36ff`
introduced `ling.lsp.formatting/0.1`; this milestone reconciles the later
IDE-2310 backlog entry with that existing implementation and adds direct
preinitialize/post-shutdown conformance evidence.

The plan's range-formatting clause is conditional on accepted boundary
semantics and tests. RFC-0026 leaves that feature out of scope, so no range
provider or placeholder API is claimed.

## Normative clauses covered

- RFC-0026 §1: Experimental marker, capability, request-only method, and
  notification behavior.
- RFC-0026 §2: exact request options, open writable overlay restriction, URI
  validation, and fail-without-mutation behavior.
- RFC-0026 §3: immutable current snapshot, compiler parser and Format IR,
  zero-or-one result, and no server-side application.
- RFC-0026 §4: exact whole-document `TextEdit`, negotiated position encoding,
  BOM preservation, original CRLF projection, and deterministic shape.
- RFC-0026 §5: fixed JSON-RPC failures, bilingual internal failure, and
  versioned migration boundary.
- RFC-0023 overlay rules, DEC-0002/DEC-0029 position rules, and
  DEC-0023/DEC-0057 formatter publication rules.

## Implementation and tests

The existing adapter in `crates/ling-lsp/src/lib.rs` is the smallest complete
vertical slice: it calls the compiler-CST formatter in process and returns a
standard `TextEdit`; it does not duplicate parsing, formatting policy, or edit
application.

`crates/ling-lsp/tests/formatting.rs` verifies exact UTF-8/UTF-16/UTF-32
results, Unicode, BOM, CRLF, latest overlay state, repeat determinism, immutable
server state, unchanged and invalid input, ownership, URI and option failures,
notifications, and lifecycle rejection before initialize and after shutdown.
Formatter suites retain the detailed comment, documentation, literal,
identifier, idempotence, invalid-source, and semantic-equivalence coverage.

## Specification gaps or conflicts

The historical IDE-2310 audit conflicted with newer Accepted RFC-0026 and with
the Done FMT-1507 implementation record. The audit is corrected rather than
creating a second LSP formatter. The lower-authority plan name `ling-fmt` is
reconciled to Accepted `ling fmt`; the LSP server consumes the same formatter
library directly.

Open general transaction, range-formatting, formatter-policy, and Stable
lifecycle gaps remain recorded. None is silently resolved by this task.

## Compatibility and determinism

- Diagnostics and schemas: no `L-*` code or schema change.
- Semantic IDs and source truth: unchanged; all ranges originate from the
  exact immutable UTF-8 snapshot and accepted source map.
- Determinism: response content depends only on negotiated encoding and current
  overlay bytes; no clock, filesystem, environment, or map order participates.
- Unicode: no Unicode table or normalization change; Unicode remains 17.0.0.
- Runtime and packages: no evaluator, VM, bytecode, ABI, package, filesystem,
  or network behavior change.

## Intentionally deferred

Range/on-type formatting, format-on-save, configurable style, minimal diffs,
closed-file and dependency formatting, `WorkspaceEdit`, general Semantic
Transaction semantics, cancellation, asynchronous publication,
multi-document edits, and Stable compatibility remain out of scope.
