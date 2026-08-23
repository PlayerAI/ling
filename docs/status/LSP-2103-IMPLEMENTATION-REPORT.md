# LSP-2103 implementation report

> Status: Done / 已完成
> Task: `LSP-2103`
> Authority: Accepted `DEC-0002`, `DEC-0019`, `RFC-0004`, `RFC-0023`, and `DEC-0259`

## Scope

This parent milestone accepts the existing RFC-0023 implementation as the
complete execution-plan open-document overlay: deterministic URI-to-file
state, exact editor-byte precedence, monotonic versions, close fallback or
temporary removal, and read-only dependency enforcement. It adds no executable
behavior.

## Normative clauses covered

- `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md` LSP-2103: URI/file
  state plus the four open/change/close/read-only rules.
- `DEC-0019`: immutable VFS publication, overlay precedence, revisions, and
  canonical logical-name ownership.
- `RFC-0023` §§1–9: protocol marker, methods, URI classes, version rules,
  full-text replacement, close behavior, dependency policy, errors, exact
  UTF-8 preservation, and non-leakage.
- `DEC-0259` §§1–8: exact parent composition and separation from incremental
  edits and compiler transactions.

## Implementation and evidence

- `crates/ling-lsp/src/lib.rs` owns session-local URI records, monotonic
  version history, deterministic document views, VFS adaptation, method
  parsing, lifecycle gates, and failure-atomic overlay operations.
- `crates/ling-source/src/vfs.rs` owns immutable disk/overlay snapshots,
  revisions, canonical logical names, and non-reusing temporary removal.
- `crates/ling-lsp/tests/overlay.rs` and protocol fixtures cover workspace,
  dependency, and untitled documents; overlay/disk races; versions; invalid
  URIs/ranges/parameters; read-only changes; size limits; response behavior;
  deterministic ordering; and nonmutation on rejection.

## Compatibility and determinism

- No command, LSP method, field, response, error, protocol marker, stability
  level, diagnostic code, schema, or migration changes.
- No syntax, Checked Core, Semantic ID, span, runtime, bytecode, VM, ABI,
  package, filesystem, or network behavior changes.
- Exact UTF-8 bytes remain authoritative; URI/document ordering and version
  checks use only validated process-local inputs. Unicode remains 17.0.0.

## Verification

The milestone is accepted only after focused LSP tests and the full locked,
offline workspace, CI, governance, support, status, RC0, traceability, Clippy,
formatting, and deterministic-diff gates pass. The exact acceptance commit is
recorded in `docs/status/implementation-status.toml` after it exists.

## Intentionally deferred

Host-path resolution, project discovery, generated files, incremental changes,
compiler snapshots, stale results, diagnostics publication, navigation,
cancellation, Workspace Edits, Semantic Transactions, and Stable editor
compatibility remain owned by later tasks and Accepted authorities.
