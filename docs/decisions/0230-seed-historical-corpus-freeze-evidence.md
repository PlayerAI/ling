# DEC-0230: Seed historical-corpus freeze evidence / Seed 历史语料冻结证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: compatibility governance
> 相关规范/缺口：`CONFORMANCE` | `ROADMAP-1.0` | `COMPAT-6501`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes a deterministic, test-only freeze of the actual
v0.0.1 Seed conformance corpus. It does not manufacture v0.1-v0.5 histories,
general compatibility semantics, migrations, or support for unavailable
protocols.

本决定授权对实际 v0.0.1 Seed conformance 语料进行确定性的、仅用于测试的冻结。
它不会虚构 v0.1-v0.5 历史、通用兼容语义、迁移能力或不可用协议支持。

## Question

What historical-corpus evidence can Ling freeze before accepted v0.1-v0.5
release specifications and compatibility rules exist?

## Decision

1. `tests/conformance` is frozen as the v0.0.1 Seed corpus by exact case/file
   counts and a domain-separated SHA-256 over canonical relative paths,
   byte lengths, and original file bytes.
2. Every case directory must be a real directory containing exactly
   `case.ling` and `expect.toml`; symlinks, extra entries, missing files, and
   corpus byte/path drift are rejected. Ordering is canonical and host paths
   never enter the digest.
3. The ten surfaces requested by `COMPAT-6501` are classified exactly:
   source programs and bounded diagnostic expectations are `SeedFrozen`;
   parser trees are `NotFrozen`; Semantic Graph, Audit, bytecode, and
   package/lock retain `SeparateProtocol` authority; Replay, evidence, and
   Zed/LSP fixtures are `Unavailable` for this corpus.
4. `SeparateProtocol` does not copy those artifacts into the Seed corpus or
   create cross-release compatibility. Their existing RFC, schema, fixture,
   lifecycle, and support rules remain independent.
5. `cargo xtask corpus verify` is an internal repository drift gate and is
   required by CI. Its manifest, report, digest, states, and error labels are
   governance evidence, not a public Ling protocol or CLI.
6. Parent `COMPAT-6501` remains `BlockedSpec`. Freezing v0.1-v0.5 requires
   actual Accepted release authorities, original release artifacts, a corpus
   manifest/version transition, and executable compatibility/migration
   oracles; current files must never be relabeled as historical releases.

## Normative basis

- The active `CONFORMANCE` authority and `tests/conformance` define the
  current v0.0.1 executable feature evidence.
- Accepted decisions governing Seed diagnostics, spans, Unicode 17.0.0, and
  deterministic bytes remain authoritative for the frozen inputs.
- `ROADMAP-1.0` requires a historical corpus but explicitly cannot create
  language semantics or compatibility commitments without Accepted authority.
- The protocol and support inventories already distinguish implemented
  versioned surfaces from Future and Unsupported ones.

## Conformance plan

- Verify exact case/file shape, reject symlinks and extra/missing entries, and
  hash sorted canonical relative paths plus original bytes.
- Freeze the current 42 cases, 84 files, and domain-separated digest in one
  machine-readable source and generated report.
- Verify the exact ten-surface classification and all evidence paths.
- Require `cargo xtask corpus verify` in the always-on CI contract.
- Run traceability, Seed reproduction, governance, status, workspace, lint,
  formatting, deterministic, and offline gates.

## Compatibility impact

This decision freezes existing test evidence only. It changes no source,
parser, resolver, Typed Core, evaluator, diagnostic, Semantic ID, schema,
bytecode, package/lock, editor, CLI, dependency, Unicode version, or runtime
behavior. The digest is an internal drift identifier and carries no reader,
migration, deprecation, or 1.x compatibility promise.

## Unresolved alternatives

Accepted v0.1-v0.5 release inventories; full parser/Typed Core/diagnostic
outputs; per-release Semantic Graph, Audit, bytecode, package/lock, Replay,
evidence, Zed/LSP, formatter, VM, backend, and platform artifacts;
compatibility outcomes; readers; migrations; deprecations; provenance; and
rollback remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
