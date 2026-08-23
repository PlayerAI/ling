# DEC-0238: Documentation evidence-path gate / 文档证据路径门禁

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: documentation engineering
> 相关规范/缺口：`DEC-0045` | `DOC-6701`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes exact-path validation for every evidence citation in
the twelve-row formal-document inventory. It proves referenced repository
artifacts exist; it does not promote their language or release authority.

本决定授权校验正式文档清单十二行中每个证据引用的精确路径。它证明所引用的仓库产物存在，但不提升这些
产物的语言或发布权威。

## Question

How should `cargo xtask docs verify` prevent shorthand globs, stale ranges, and
missing files from masquerading as documentation evidence?

## Decision

1. Every formal-set row must cite at least one backticked exact repository path
   in its Current source and evidence cell.
2. Recognized evidence roots are `README.md`, `docs/`, `tests/`, `crates/`,
   `tools/`, and `editors/`.
3. Evidence paths must use forward slashes and normal non-empty components.
   Globs, character classes, parent/current traversal, backslashes, colons, and
   other non-exact spellings fail closed.
4. Every recognized path must exist as a checked-in file or directory at
   verification time. Directory evidence is permitted only for an explicitly
   named repository directory; recursive content coverage is not inferred.
5. Replace the inventory's shorthand decision/status ranges with exact paths.
   Future/Unsupported rows cite their planning and registry/audit evidence
   without becoming implemented manuals.
6. The existing twelve manual states and anti-promotion phrases remain
   unchanged. The verifier reports the exact evidence-path count.
7. Parent `DOC-6701` remains `BlockedSpec` for complete accepted manuals,
   generated reference coverage, missing features, cross-platform operational
   evidence, and G6 documentation sign-off.

## Normative basis

- Accepted DEC-0045 authorizes the bounded formal-document inventory and its
  anti-promotion guardrails.
- Repository authority rules require evidence to resolve to the actual current
  specification, implementation, conformance, governance, and status sources.
- Execution-plan shorthand and wildcards are non-normative and cannot prove a
  concrete artifact exists.

## Conformance plan

- Validate all twelve inventory rows and all forty-six exact evidence paths.
- Reject a wildcard path and a syntactically safe but missing path in an
  isolated test.
- Retain four Future/Unsupported rows and all existing state classifications.
- Run focused documentation tests plus governance, status, workspace, Clippy,
  formatting, deterministic, and offline gates.

## Compatibility impact

This changes internal documentation validation and replaces ambiguous prose
citations with exact repository paths. Ling semantics, diagnostics, schemas,
Semantic IDs, packages, dependencies, CLI/editor behavior, runtime, Unicode
17.0.0, protocols, and support states remain unchanged.

## Unresolved alternatives

Content-level completeness, heading/anchor validation, external-link checking,
generated API/reference documentation, bilingual parity, future manual
authority, versioned documentation publication, and G6 sign-off remain
deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
