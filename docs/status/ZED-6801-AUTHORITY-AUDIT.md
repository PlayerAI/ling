# ZED-6801 Authority Audit

- Task: `ZED-6801` — Compatibility matrix
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:430-440`
- Release: G6
- Status: `BlockedSpec`; the matrix is preparatory editor evidence.

## Decision

`ZED-6801` remains `BlockedSpec`. The repository has a substantial local
Tree-sitter grammar and query corpus, but it has no Zed extension package, LSP
executable, public editor protocol, binary release, or Zed compatibility
decision. The new matrix records known values and uses `Not established` or
`Unsupported` for everything that cannot be supported by current evidence.

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
- `docs/governance/support-matrix.toml` records `UNSUP-LSP-EDITOR` and the
  LSP/formatter/semantic-mutation gaps; the protocol inventory marks Semantic
  and Audit formats Experimental/Preview rather than Stable editor contracts.
- `docs/ROADMAP-1.0.md` requires accepted protocol/version policy, cross-host
  evidence, deterministic/offline behavior, and release artifacts before a
  Stable support claim.
- `AGENTS.md` requires the `ling`/`.ling` names, Unicode 17.0.0, original
  UTF-8 spans, checked Typed Core boundaries, and no placeholder public APIs or
  stale legacy names.

## Evidence and gaps

The matrix captures the local grammar package version, locked Tree-sitter CLI,
Node requirement, repository grammar revision, generated Unicode/query corpus,
OS/toolchain limitations, and the absence of LSP/Zed binaries and schemas. The
local npm verification was attempted offline but was blocked by a Windows
cache-lock permission error; no pass is claimed.

The missing evidence is an actual extension package, tested Zed versions,
compiler/LSP compatibility range, protocol/schema and position fixtures,
per-OS installation/acquisition policy, signed release artifacts, and known
limitation/migration policy. These require accepted G1-G5 and G6 authorities;
they cannot be filled by guessing from a Tree-sitter parser.

## Compatibility and deferred work

This audit changes no compiler semantics, diagnostics, schemas, Semantic IDs,
CLI commands, package behavior, runtime, editor protocol, dependency, or
public API. It preserves the editor-only grammar boundary, Unicode 17.0.0,
original UTF-8 spans, deterministic generated files, and offline Rust builds.

Zed/LSP support, binary acquisition, extension metadata, marketplace
publication, formatter integration, semantic mutation, and Stable node
compatibility remain deferred and explicitly unsupported.
