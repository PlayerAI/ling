# DEC-0241: Current Zed-matrix evidence / 当前 Zed 矩阵证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: editor integration
> 相关规范/缺口：`DEC-0048` | `ZED-6801`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes correcting the preparatory Zed matrix to current
repository facts, recording a real locked Windows grammar run, and validating
the editor package's JSON metadata structurally.

本决定授权将预备 Zed 矩阵修正为当前仓库事实，记录真实的 Windows 锁定语法套件运行，并对编辑器包
JSON 元数据进行结构化校验。

## Question

How should the matrix stop preserving obsolete claims that no LSP exists and
that Windows grammar verification is blocked, while avoiding any inference of
Zed or cross-platform support?

## Decision

1. Record the implemented source-built `ling lsp --stdio` Preview lifecycle
   and Experimental overlay, including their registered internal versions.
2. Keep document diagnostics, hover, definition, references, rename,
   completion, code actions, formatting, semantic tokens, Zed packaging, and a
   Zed compatibility range explicitly unavailable.
3. Record binary acquisition truthfully as a source-built Ling CLI only; no
   standalone server/extension download, checksum, signature, installer, or
   acquisition contract exists.
4. Run `npm run verify --offline` with the locked package after permitting its
   Tree-sitter process to access the user-cache lock. Record the passing Windows
   result and exact reviewed corpus totals without claiming Linux/macOS or Zed
   execution.
5. Remove the obsolete repository hash and cache-failure narrative. Describe
   the tracked grammar snapshot and verify regeneration causes no tracked
   worktree drift.
6. Extend `cargo xtask zed verify` to parse `package.json`,
   `package-lock.json`, and `tree-sitter.json` as JSON and validate exact package
   identity/version/private state, locked CLI, Node floor, verify script,
   lockfile version/root metadata, grammar name/scope/file type/query path, and
   grammar metadata version.
7. Bind the corrected LSP claims to the workspace member, `ling-lsp` crate
   manifest, and registered lifecycle/overlay protocol records.
8. Retain the existing ten surfaces, five evidence files, editor-only grammar
   boundary, Unicode 17.0.0, and anti-placeholder policy.
9. Parent `ZED-6801` remains `BlockedSpec` until an accepted Zed support policy,
   extension artifact, cross-host suite, version ranges, and release evidence
   exist.

## Normative basis

- Accepted DEC-0048 authorizes the bounded compatibility inventory and
  fail-closed internal verifier.
- Accepted RFC-0004 and RFC-0023 plus the protocol/support inventories record
  the current LSP lifecycle and overlay boundaries.
- Tree-sitter remains below accepted Ling language authority and cannot create
  semantics or a Zed support claim.

## Conformance plan

- Run the locked offline npm verify script on Windows and retain exact suite
  totals and zero tracked drift.
- Parse all three package JSON files and validate the accepted exact fields.
- Reject missing or changed structured fields in a focused test.
- Validate the corrected ten-surface matrix and five package evidence files.
- Run Zed, LSP, governance, status, workspace, Clippy, formatting,
  deterministic, and offline gates.

## Compatibility impact

Corrects documentation and strengthens internal metadata validation only. Ling
semantics, diagnostics, public schemas, Semantic IDs, packages, dependencies,
CLI/LSP behavior, runtime, Unicode 17.0.0, protocols, support states, and editor
APIs remain unchanged.

## Unresolved alternatives

Zed version ranges; extension/package implementation; document features;
position/edit protocol completion; standalone binary acquisition; Linux/macOS
grammar evidence; per-OS Zed tests; signed releases; marketplace publication;
migration policy; and G6 sign-off remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
