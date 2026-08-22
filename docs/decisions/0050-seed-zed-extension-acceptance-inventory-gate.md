# DEC-0050: Seed Zed extension acceptance inventory gate / Seed Zed 扩展验收盘点门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: editor-integration  
> Related authority/gap: `RFC-0004`, `RFC-0001`, `DEC-0049`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `ZED-6803-SEED` child. It does not
complete full Zed extension acceptance, authorize an extension manifest,
marketplace package, editor-feature LSP adapter, task schema, formatter,
Replay/Evidence UI, or Stable editor support. The parent `ZED-6803` remains
`BlockedSpec` until those authorities and executable artifacts are Accepted.

## Question

The repository has reviewed Tree-sitter grammar/query evidence and a matrix of
editor acceptance areas, but no complete Zed package or editor-feature LSP
integration. How can the project protect the exact `Covered`, `Partial`,
`Unsupported`, and `Future` classifications and their historical TS/ZQ
evidence without executing npm or promoting parser output to language
semantics?

## Decision

1. `cargo xtask zed-extension verify` is an internal governance command. It
   reads `docs/testing/ZED-EXTENSION-ACCEPTANCE.md` and validates exactly
   thirteen acceptance areas with their recorded states and non-empty
   evidence/boundary cells.
2. The verifier checks the editor-only authority boundary, `ling`/`.ling`,
   Unicode 17.0.0, original UTF-8 spans, the recorded Windows error-5 cache
   limitation, the no-promotion/no-placeholder policy, and nine package/query
   and TS/ZQ evidence files. It fails closed with internal
   `GOV-ZED-ACCEPTANCE-*` messages.
3. The command validates inventory and evidence markers only. It does not run
   npm, generate a parser, contact a registry, create an extension manifest,
   start an LSP process, define Zed behavior, allocate diagnostics, or claim
   marketplace/install support.
4. The command is included in the governance-authority CI gate. A future
   acceptance promotion requires Accepted editor/LSP authority, executable
   package and protocol fixtures, position/lifecycle/task evidence, clean and
   offline installation evidence, and release provenance.

## Conformance plan

- Run `cargo xtask zed-extension verify` offline and assert thirteen areas,
  four covered, four partial, five unsupported, two future classifications,
  and nine evidence files.
- Mutate an acceptance row/state, policy phrase, package marker, TS/ZQ report
  marker, or stale-name boundary and verify the gate fails closed.
- Run `cargo xtask ci verify` and the existing locked grammar/compiler,
  governance, status, and traceability checks without treating this inventory
  as npm execution or Stable editor evidence.
- Repeat independent processes and verify that no source, generated parser,
  diagnostic, schema, protocol, package, cache, network request, or system
  configuration is changed.

## Compatibility impact

- Adds only an internal `cargo xtask` validator, documentation evidence, and
  CI preflight. Ling syntax, semantics, Checked Core, runtime, bytecode,
  diagnostics, schemas, Semantic IDs, dependencies, public protocols, and
  Unicode 17.0.0 behavior are unchanged.
- Existing Tree-sitter grammar/query and TS/ZQ reports remain editor-only
  evidence. No Zed extension package, LSP feature, formatter, task/runnable,
  Replay/Evidence UI, install path, marketplace promise, migration contract,
  or placeholder public API is added.

## Unresolved alternatives

Zed package metadata and manifest shape, LSP capability/position contracts,
task/runnable and formatter behavior, crash/restart and workspace limits,
Replay/Evidence navigation, clean/offline installation, marketplace review,
provenance, and editor migration remain governed by the parent `ZED-6803` and
later Accepted authorities.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
