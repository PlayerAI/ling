# DEC-0243: Current Zed acceptance evidence / 当前 Zed 验收证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：editor integration
> 相关 RFC/缺口：RFC-0004 | DEC-0050 | DEC-0241 | DEC-0242 | ZED-6803
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes correcting the Zed acceptance inventory to current
grammar, Preview LSP, and position evidence without claiming a Zed extension.

本决定授权依据当前语法、Preview LSP 与位置证据修正 Zed 验收盘点，但不宣称已经存在 Zed 扩展。

## Question

How should the acceptance inventory incorporate the passing locked Windows
grammar suite, Preview lifecycle/full-text overlay, and tested position
negotiation/projection while keeping every unimplemented public editor feature
and Zed integration explicitly unsupported?

## Decision

1. Amend DEC-0050 Decision item 2 by replacing its historical Windows
   cache-lock marker with the actual passing locked offline grammar/query result
   and exact reviewed totals; retain every other Seed boundary from DEC-0050.
2. Record the source-built Preview lifecycle/full-text overlay, position
   negotiation, and UTF-8/UTF-16/UTF-32 source projection as partial
   prerequisites for editor features, not implementations of diagnostics,
   navigation, completion, formatting, semantic tokens, or Zed behavior.
3. Keep all listed public LSP document features unsupported and retain
   `Partial` for the broader Unicode/CRLF/position row because no Zed fixture or
   public document feature consumes the position primitives.
4. Compose `cargo xtask zed-extension verify` with the current Zed-matrix and
   LSP-discovery gates so stale upstream package, grammar, CLI, lifecycle, or
   acquisition evidence fails the downstream acceptance gate.
5. Validate ten historical/current evidence files and three exact position
   evidence files, including Unicode, surrogate-boundary, BOM/CRLF, negotiation,
   malformed-metadata, overlay-method, and protocol-response markers.
6. The verifier remains read-only and offline. It does not run npm, launch Zed,
   create an extension manifest, advertise a new LSP capability, allocate a
   diagnostic, install a package, or contact a registry.
7. Parent `ZED-6803` remains `BlockedSpec` until all required editor contracts,
   extension artifacts, integration fixtures, and release evidence exist.

## Conformance plan

- Run the locked offline npm suite on Windows and retain exact grammar,
  differential, recovery, query, and example totals with zero tracked drift.
- Validate thirteen exact acceptance areas, four covered, five partial, five
  unsupported, and two future classifications.
- Require both upstream evidence gates and three current position-evidence
  files; reject missing markers with a focused negative test.
- Run position, lifecycle, overlay, CLI process, workspace, CI, governance,
  status, Clippy, formatting, deterministic, and offline gates.

## Compatibility impact

Documentation correction, gate composition, and stronger internal evidence
validation only. Ling semantics, source syntax, diagnostics, public schemas,
Semantic IDs, packages, dependencies, CLI/LSP behavior, runtime, Unicode
17.0.0, protocols, support states, and editor APIs are unchanged. No migration
is required.

No Zed extension, public document feature, capability advertisement, editor
transaction, task/runnable integration, crash/restart policy, marketplace
artifact, network behavior, or Stable support claim is added.

## Unresolved alternatives

Zed manifest/API/version policy; public LSP diagnostics and navigation;
completion/rename/format/edit protocols; tasks/runnables; Replay/Evidence UI;
crash/restart; workspace limits; installation/marketplace provenance;
cross-host/Zed execution; migration policy; and G6 sign-off remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
