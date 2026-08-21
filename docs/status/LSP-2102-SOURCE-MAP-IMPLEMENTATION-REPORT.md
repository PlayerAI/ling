# LSP-2102-SOURCE-MAP implementation report

Status: In Progress (bounded source projection slice)

## Scope

This slice implements the deterministic source-layer projection authorized by
Accepted `DEC-0029`. It adds no LSP server, JSON-RPC lifecycle, document
version, Workspace Edit, Semantic Transaction, diagnostics publication, or
public editor command.

## Normative authority

- `DEC-0002` keeps original UTF-8 byte offsets and the normalized lexical view
  as the compiler source authority.
- `DEC-0029` defines `utf-8`, `utf-16`, and `utf-32` counting, ordered
  negotiation with UTF-16 fallback, zero-based lexical positions, strict
  boundary validation, and SourceMap round-tripping.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` remains open for snapshot identity,
  version preconditions, failure responses, and Stable versus Experimental
  editor fields.

## Implementation

- `crates/ling-source/src/position.rs` provides `PositionEncoding`,
  `LspPosition`, typed `PositionError`, deterministic negotiation, and
  `SourceFile` byte/position conversion.
- `crates/ling-source/src/lib.rs` exports only this source projection boundary;
  no protocol inventory entry is added.
- Leading BOM, CRLF normalization, Chinese identifiers, emoji, combining
  marks, empty lines, final lines, UTF-8 scalar boundaries, and UTF-16
  surrogate boundaries are covered by unit tests.

## Evidence

Executed checks:

- `cargo test -p ling-source --all-targets --locked --offline`
- `cargo clippy -p ling-source --all-targets --locked --offline -- -D warnings`
- `cargo fmt --all`

The tests verify negotiation order and fallback, all valid lexical boundary
round trips for each encoding, BOM/CRLF projection, strict invalid-boundary
errors, unknown-line rejection, and no-clamping behavior.

## Compatibility and determinism

- No diagnostic code, schema, Semantic ID, CLI behavior, bytecode, runtime,
  source syntax, or public protocol changes.
- Conversion is deterministic and uses only the immutable source text and
  SourceMap; host locale, paths, allocation order, and hash-map order are not
  observable.
- Unicode behavior remains the existing UTF-8/Unicode-scalar behavior; no
  generated Unicode table or version changes.

## Deferred work

The parent `LSP-2102` task remains `BlockedSpec` until an accepted decision
defines document lifecycle, snapshot/version preconditions, stale-result and
failure handling, handler-wide conversion, and public protocol field
stability. Downstream diagnostics, overlays, edits, and Semantic Transaction
work remain deferred with that parent gate.
