# DEC-0105: Internal replay-schema field evidence / 内部 Replay Schema 字段证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: determinism-design
> 相关规范/缺口：`DEC-0104` | `DEC-0012` | `GAP-DETERMINISTIC-REPLAY-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only field vocabulary and ordering
boundary for the bounded `REP-2502-OBSERVATION` child. It records proposed
replay-envelope fields and checks duplicate-free deterministic ordering. It
does not define a wire schema, payload encoding, checksum, decoder, privacy
policy, or replay behavior.

本决定只授权 test-only 的字段词汇与排序边界，供 `REP-2502-OBSERVATION` 子任务使用。它记录拟议的
replay envelope 字段并检查无重复的确定性排序；不定义 wire schema、payload 编码、checksum、decoder、
privacy policy 或 replay 行为。

## Question

The replay plan lists envelope, event, ordering, identity, integrity,
determinism, toolchain, profile, schema, payload, migration, and privacy
concerns, but Accepted authority does not freeze their wire representation.
What evidence can be retained without creating a protocol?

Replay 计划列出了 envelope、event、ordering、identity、integrity、determinism、toolchain、profile、
schema、payload、migration 和 privacy 关注点，但已接受的权威尚未冻结其 wire 表示。如何在不创建协议的
前提下保留证据？

## Decision

1. `crates/ling-concurrency/tests/replay_schema_evidence.rs` keeps a
   test-local inventory of thirteen proposed fields: canonical envelope, event
   ID/kind, ordering, identity, checksum, determinism class, toolchain,
   profile, schema, payload, migration, and privacy.
2. The test-only inventory sorts fields by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.replay-schema-observation/0`. These bytes are not a wire format.
3. The child adds no replay schema, encoder/decoder, event ID assignment,
   checksum rule, redaction policy, protocol inventory entry, diagnostic,
   Semantic ID, CLI/LSP command, public protocol, or migration rule. Public
   `REP-2502` remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Determinism Class, Effect Log,
  Replay version, and privacy boundaries before replay support.
- `DEC-0012` governs Seed Semantic IDs and canonical bytes, not replay wire
  events or integrity rules.
- `DEC-0104` keeps determinism-class labels test-only while replay authority is
  absent.
- `GAP-DETERMINISTIC-REPLAY-001` remains Open; this decision records field
  vocabulary without resolving the gap.

## Conformance plan

- Assert that all thirteen proposed fields are present in local order.
- Compare forward and reversed field insertion order and require identical
  test-only evidence bytes.
- Reject duplicate field vocabulary.
- Keep payload types, event identity, ordering, checksums, schema versions,
  migration, privacy/redaction, corruption, divergence, cross-process,
  differential, and runtime replay fixtures deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only vocabulary evidence. No public replay or protocol claim
  is registered.

## Unresolved alternatives

Canonical envelope and field types, event IDs, ordering and identity rules,
checksum/integrity, class/toolchain/profile/schema metadata, payload framing,
migration, privacy/redaction, corruption, divergence, diagnostics, resource
limits, and runtime ABI remain open under `GAP-DETERMINISTIC-REPLAY-001` and
`REP-2502`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
