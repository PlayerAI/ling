# ZED-6803 Authority Audit

- Task: `ZED-6803` — Full extension acceptance
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:461-471`
- Release: G6
- Status: `BlockedSpec`; the matrix is preparatory editor evidence.

## Decision

`ZED-6803` remains `BlockedSpec`. The repository has a Tree-sitter development
package with reviewed highlights, brackets, indentation, recovery, Unicode,
and compiler differential fixtures. It also has a source-built Preview LSP
lifecycle/full-text overlay, tested position negotiation/projection, and a
formatter CLI/internal edit primitive. It has no Zed extension package, public
LSP document-feature adapters, editor formatter transaction, task/runnable
integration, Replay/Evidence navigation, crash/restart harness, or marketplace
artifact.

The acceptance matrix records `Covered`, `Partial`, `Unsupported`, and `Future`
rows without promoting the narrower grammar evidence into a full editor
support claim. The existing Preview `ling lsp --stdio` lifecycle is not an
editor-feature or marketplace package.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:461-471` is a non-normative checklist. It does
  not authorize an extension package, LSP behavior, task schema, debugger,
  replay/evidence UI, or marketplace promise.
- `editors/tree-sitter-ling/README.md`, `KNOWN-DIFFERENCES.md`, TS-3101–TS-3108,
  and ZQ-3201–ZQ-3203 explicitly keep Tree-sitter editor behavior below the
  compiler's language authority and defer LSP/Zed packaging.
- `docs/governance/support-matrix.toml` records LSP/Zed/formatter/semantic
  mutation as unsupported, and future Replay/Evidence/Task/Actor surfaces as
  unavailable; the protocol inventory contains no Stable editor protocol.
- `docs/ROADMAP-1.0.md` requires accepted semantics, implementation,
  conformance, support, compatibility, security, and release evidence before
  a Stable 1.0 claim.
- `AGENTS.md` requires `ling`/`.ling`, Unicode 17.0.0, original UTF-8 spans,
  checked Typed Core boundaries, bilingual diagnostics, deterministic/offline
  builds, and no placeholder or stale public names.

## Evidence and gaps

`docs/testing/ZED-EXTENSION-ACCEPTANCE.md` maps every planned acceptance area:
grammar-only `.ling` recognition, highlights, brackets, indentation, missing
outline/textobjects/runnables, all absent LSP features, partial CLI/Audit
coverage, unsupported Replay/Evidence, Unicode/position limits, lifecycle,
workspace scale, metadata/license, and packaging/install.

The locked offline npm suite passed on Windows on 2026-08-23 and regeneration
left no tracked drift. It covers the current grammar/differential/recovery/query
surface only; it is not a Zed, Linux/macOS, cross-host, or Stable extension
result.

Accepted `DEC-0243` closes only the bounded
`ZED-6803-CURRENT-EVIDENCE` child. The downstream verifier composes the current
Zed-matrix and discovery gates and validates three position-evidence files; no
acceptance row becomes a public Zed feature.

## Compatibility and deferred work

This audit changes no compiler semantics, diagnostics, schemas, Semantic IDs,
CLI commands, package behavior, runtime, editor protocol, dependencies, or
public API. It preserves the editor-only grammar boundary, Unicode 17.0.0,
original UTF-8 spans, deterministic generated files, and locked offline Rust
validation.

Public LSP document features, editor formatter transactions, semantic mutation,
task/runnable integration, Replay/Evidence navigation, crash/restart handling,
large-workspace limits, Zed package metadata, clean install, and marketplace
publication remain deferred or explicitly unsupported.

The internal `cargo xtask zed-extension verify` command protects the thirteen
acceptance rows, ten historical/current evidence files, two upstream gates, and
three position-evidence files without running npm, creating a Zed manifest, or
claiming a marketplace package.
