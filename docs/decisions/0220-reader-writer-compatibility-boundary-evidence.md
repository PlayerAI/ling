# DEC-0220: Internal Reader/Writer Compatibility boundary evidence / 内部读写兼容边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: protocol governance
> 相关规范/缺口：`DEC-0219` | `ROADMAP-1.0` | `GAP-REGISTER` | `SCHEMA-REGISTRY` | `PROTOCOL-INVENTORY`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes bounded evidence for `PROTO-6202-OBSERVATION`. It
freezes the truthful current scope of the eight-schema compatibility corpus
without inventing N−1 edges, migration adapters, or universal protocol limits.

本决定授权 `PROTO-6202-OBSERVATION` 使用有界证据，固定八个 schema 兼容语料
的真实当前范围；不虚构 N−1 边、迁移适配器或通用协议限制。

## Question

Which reader/writer compatibility facts are currently executable and safe to
retain while all registered schemas are first-version formats?

## Decision

1. `schemas/registry.toml` remains the concrete compatibility source for eight
   JSON schemas. Every writer is `CurrentOnly`; three schemas have current
   readers and five are writer-only.
2. All eight schemas remain `NoPreviousVersion`, with empty previous markers,
   compatibility directories, and migration adapters. This is explicit
   absence of an N−1 edge, not proof of N−1 compatibility.
3. `tools/xtask/src/schema.rs` tests the exact 8/3/5 writer/reader distribution
   and the absence of migration claims in addition to existing valid, invalid,
   corruption, canonical, and registry-drift gates.
4. `crates/ling-types/tests/reader_writer_compatibility_evidence.rs` records
   sixty test-local schema, reader/writer, field-policy, corruption, canonical,
   limit, failure, and fixture boundaries with deterministic ordering and
   duplicate rejection.
5. Opaque bytes tagged `ling.reader-writer-compatibility-observation/0` are test
   evidence only; they do not define a compatibility edge, schema, migration,
   diagnostic, or Semantic ID input.
6. No reader, writer, N−1 edge, migration tool, canonical re-encoder, limit,
   schema version, public protocol, or diagnostic is added. Public
   `PROTO-6202` remains `BlockedSpec`.

## Normative basis

- `docs/governance/SCHEMA-LIFECYCLE.md` distinguishes `NoPreviousVersion` from
  supported N−1 reading or migration, though the policy remains Draft.
- `schemas/registry.toml` requires every compatibility edge to have explicit
  executable evidence and forbids implicit version compatibility.
- `docs/status/PROTO-6202-AUTHORITY-AUDIT.md` records missing per-protocol
  version graphs, migrations, diagnostics, limits, and future-protocol schemas.
- Existing accepted protocol decisions and RFCs authorize only their scoped
  readers/writers; they do not create universal G6 compatibility promises.
- `DEC-0219` preserves the single protocol inventory and zero Stable public
  protocol claim.

## Conformance plan

- Assert all sixty local boundaries, explicit order, duplicate rejection, and
  order-independent opaque bytes.
- Assert exactly eight current-only writers, three current readers, five
  writer-only schemas, eight `NoPreviousVersion` records, and zero migrations.
- Run schema validation, compatibility, corrupt-input, protocol, support,
  status, and governance gates without adding compatibility edges.
- Defer protocol-specific N−1, migration, future-version, limit, and diagnostic
  semantics until Accepted authority and executable adapters exist.

## Compatibility impact

Existing schemas, readers, writers, fixtures, protocol versions, canonical
bytes, diagnostics, language/runtime behavior, CLI/LSP, dependencies, Semantic
IDs, source spans, and Unicode 17.0.0 remain unchanged.

## Unresolved alternatives

Per-protocol version graphs and ownership; N−1 readers; migration adapters;
unknown/missing/default/future-version behavior beyond current schemas;
canonical re-encoding and identity upgrades; size/depth/resource/security
limits; ABI/replay/evidence/device/LSP/package schemas; bilingual diagnostics;
cross-process, repeated-build, Unicode, BOM/CRLF, golden/corrupt/migration and
release fixtures remain open under PROTO-6202 through PROTO-6204, incomplete
G1-G5 exits, ROADMAP-1.0, Draft schema policy, and registered gaps.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
