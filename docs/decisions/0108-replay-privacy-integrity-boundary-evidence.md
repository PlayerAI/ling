# DEC-0108: Internal replay privacy and integrity boundary evidence / 内部 Replay 隐私与完整性边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: determinism-design
> 相关规范/缺口：`DEC-0107` | `DEC-0012` | `GAP-DETERMINISTIC-REPLAY-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed replay
privacy, trimming, and corruption boundaries for the bounded
`REP-2505-OBSERVATION` child. It checks deterministic, duplicate-free boundary
vocabulary. It does not classify data, redact values, retain logs, calculate
checksums, decode chunks, report corruption, or implement offline replay.

本决定只授权 test-only 的拟议 Replay 隐私、裁剪与损坏边界清单，供
`REP-2505-OBSERVATION` 子任务使用。它只检查确定性、无重复的边界词汇；不分类数据、不裁剪值、不保留日志、
不计算 checksum、不解码 chunk、不报告损坏，也不实现离线 replay。

## Question

The replay privacy plan requires field-level redaction, secret/PII exclusion,
dependency-preserving trimming, chunk integrity, corruption handling, and
offline operation, but Accepted replay authority does not define their
contracts. What evidence can be retained without implementing a privacy or
integrity policy?

Replay 隐私计划要求字段级 redaction、secret/PII 排除、保持依赖的裁剪、chunk 完整性、损坏处理与离线运行，
但已接受的 Replay 权威尚未定义其契约。如何在不实现隐私或完整性策略的情况下保留证据？

## Decision

1. `crates/ling-concurrency/tests/replay_privacy_evidence.rs` keeps a
   test-local inventory of sixteen provisional boundaries: field sensitivity,
   field redaction, secret/PII exclusion, capability/resource exclusion,
   authorization, key handling, retention, dependency closure, chunk
   boundaries, checksum integrity, truncation, corruption, failure
   diagnostics, unknown fields, offline mode, and migration.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.replay-privacy-observation/0`. These bytes are not a privacy policy,
   replay chunk, checksum, diagnostic, or offline protocol.
3. The child adds no sensitivity classifier, redactor, trimmer, retention
   store, key manager, chunk decoder, checksum implementation, corruption
   diagnostic, offline command, Semantic ID, public protocol, or migration
   rule. Public `REP-2505` remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted replay, privacy, and
  differential contracts before replay data tooling can be exposed.
- `DEC-0012` governs Seed Semantic IDs and canonical bytes only, not replay
  privacy, chunk integrity, retention, or corruption authority.
- `DEC-0107` keeps replay-player boundaries test-only while playback authority
  is absent.
- `GAP-DETERMINISTIC-REPLAY-001` remains Open; this decision records privacy
  and integrity vocabulary without resolving the gap.

## Conformance plan

- Assert all sixteen provisional boundaries and their test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep sensitivity labels, default redaction, secret/PII policy,
  capability/resource handling, authorization, key/retention behavior,
  dependency closure, chunk/checksum bytes, truncation/corruption taxonomy,
  diagnostics, unknown-field handling, offline enforcement, migration,
  cross-process, differential, and runtime fixtures deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No public privacy, integrity, or
  replay protocol claim is registered.

## Unresolved alternatives

Sensitivity taxonomy, redaction representation, default-deny and authorization
rules, key handling, retention/deletion, dependency closure, chunk framing,
checksum scope, truncation/corruption recovery, diagnostics, unknown fields,
offline guarantees, migration, resource limits, runtime ABI, and cross-process
behavior remain open under `GAP-DETERMINISTIC-REPLAY-001` and `REP-2505`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
