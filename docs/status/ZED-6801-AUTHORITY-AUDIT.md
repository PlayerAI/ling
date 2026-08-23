# ZED-6801 Authority Audit

- Task: `ZED-6801` — Compatibility matrix
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:430-440`
- Release: G6
- Status: `BlockedSpec`; the matrix is preparatory editor evidence.

## Decision

`ZED-6801` remains `BlockedSpec`. The repository has a substantial local
Tree-sitter grammar and query corpus plus a Preview `ling lsp --stdio`
lifecycle and Experimental overlay, but it has no Zed extension package,
document-feature protocol, binary release, or Zed compatibility decision. The
matrix records known values and uses `Not established` or `Unsupported` for
everything that cannot be supported by current evidence.

The matrix does not treat Tree-sitter recovery behavior, query captures,
generated CST names, or a local development toolchain as Ling semantics or a
Stable editor API.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:430-440` is a non-normative checklist. It does
  not authorize a Zed extension, LSP version range, download mechanism, or
  compatibility promise.
- `editors/tree-sitter-ling/README.md`, `KNOWN-DIFFERENCES.md`, and the
  TS/ZQ implementation reports explicitly subordinate Tree-sitter to accepted
  Ling authority and defer LSP/Zed packaging.
- `docs/governance/support-matrix.toml` records `UNSUP-LSP-EDITOR` for document
  features and semantic mutation; the protocol inventory records the Preview
  lifecycle, Experimental overlay, and Experimental/Preview Semantic/Audit
  formats rather than Stable editor contracts.
- `docs/ROADMAP-1.0.md` requires accepted protocol/version policy, cross-host
  evidence, deterministic/offline behavior, and release artifacts before a
  Stable support claim.
- `AGENTS.md` requires the `ling`/`.ling` names, Unicode 17.0.0, original
  UTF-8 spans, checked Typed Core boundaries, and no placeholder public APIs or
  stale legacy names.

## Evidence and gaps

The matrix captures the local grammar package version, locked Tree-sitter CLI,
Node requirement, current tracked grammar snapshot, generated Unicode/query
corpus, Preview lifecycle/overlay boundary, and absence of a released
language-server/Zed package. The full locked offline npm verification passed on
Windows on 2026-08-23 after the process was allowed to access its user-cache
lock; it produced no tracked worktree drift.

The missing evidence is an actual extension package, tested Zed versions,
compiler/LSP compatibility range, protocol/schema and position fixtures,
per-OS installation/acquisition policy, signed release artifacts, and known
limitation/migration policy. These require accepted G1-G5 and G6 authorities;
they cannot be filled by guessing from a Tree-sitter parser.

Accepted `DEC-0048` closes only the bounded `ZED-6801-SEED` child: the
internal `cargo xtask zed verify` command protects the ten-surface matrix and
five package evidence files without executing npm or making a Zed support
claim.

Accepted `DEC-0241` additionally closes only the bounded
`ZED-6801-CURRENT-EVIDENCE` child. It corrects the LSP, acquisition, OS, and
grammar-run facts and structurally validates the three JSON package metadata
files. The passing Windows grammar suite is not Zed/cross-host support.

## Compatibility and deferred work

This audit changes no compiler semantics, diagnostics, schemas, Semantic IDs,
CLI commands, package behavior, runtime, editor protocol, dependency, or
public API. It preserves the editor-only grammar boundary, Unicode 17.0.0,
original UTF-8 spans, deterministic generated files, and offline Rust builds.

Zed integration, LSP document features, standalone binary acquisition,
extension metadata, marketplace publication, formatter integration, semantic
mutation, and Stable node compatibility remain deferred and explicitly
unsupported. The bounded
verifier emits only internal `GOV-ZED-MATRIX-*` failures; these are not public
Ling diagnostics.
