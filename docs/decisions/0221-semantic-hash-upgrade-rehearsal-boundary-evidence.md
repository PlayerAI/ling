# DEC-0221: Internal Semantic Hash Upgrade Rehearsal boundary evidence / 内部语义哈希升级演练边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: semantic protocol governance
> 相关规范/缺口：`DEC-0012` | `DEC-0220` | `ROADMAP-1.0` | `GAP-SEMANTIC-HASH-LIFECYCLE-001` | `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes bounded evidence for `PROTO-6203-OBSERVATION`. It
freezes the current hash-bearing schema declarations and the required rehearsal
dimensions without defining an old/new algorithm pair, migration edge, dual
reader, cache rewrite, or new Semantic ID.

本决定授权 `PROTO-6203-OBSERVATION` 使用有界证据，固定当前携带哈希方案的
schema 声明与必要演练维度；不定义新旧算法对、迁移边、双读器、缓存重写或新
Semantic ID。

## Question

Which Semantic Hash upgrade facts can be made executable now without silently
changing identity or converting a non-normative G6 checklist into a public
compatibility promise?

## Decision

1. `schemas/registry.toml` remains the concrete source for current hash-bearing
   schema declarations. The two Semantic Graph schemas retain
   `experimental:blake3:` and their respective v1/v2 domain separators; the
   lock schema retains `sha256:`.
2. Those three schemas remain independent current formats with
   `NoPreviousVersion`, empty previous-version and compatibility paths, and no
   migration adapter. The 0.1 and 0.2 Semantic Graph records are not declared
   as an old/new migration edge.
3. `tools/xtask/src/schema.rs` tests the exact markers and hash-scheme lists and
   proves that no registered hash-bearing schema claims a migration edge.
4. `crates/ling-types/tests/semantic_hash_upgrade_rehearsal_evidence.rs`
   records sixty test-local rehearsal, identity, migration, dependency, cache,
   replay/evidence, failure, determinism, and fixture boundaries with explicit
   ordering and duplicate rejection.
5. Opaque bytes tagged
   `ling.semantic-hash-upgrade-rehearsal-observation/0` are test evidence only.
   They are not canonical Semantic bytes, an ID prefix, an algorithm ID, a
   cache key, a schema marker, or a migration manifest.
6. No algorithm, hash prefix, domain separator, reader, writer, migration,
   dependency/lock identity, cache behavior, replay/evidence protocol,
   diagnostic, or public compatibility promise is added. Public `PROTO-6203`
   remains `BlockedSpec`.

## Normative basis

- Accepted `DEC-0012` fixes the current experimental BLAKE3 text form,
  domain-separated versioned canonical bytes, and the rule that algorithm,
  encoding, or normalization changes require an explicit schema or prefix
  upgrade with migration explanation.
- `schemas/registry.toml` explicitly names current hash schemes and distinguishes
  `NoPreviousVersion` from a supported compatibility or migration edge.
- Accepted `DEC-0220` freezes the truthful current-only reader/writer scope and
  forbids inferring N−1 compatibility from first-version records.
- `GAP-SEMANTIC-HASH-LIFECYCLE-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` remain open and reserve the real
  algorithm/version/migration contract for future Accepted authority.
- `docs/status/PROTO-6203-AUTHORITY-AUDIT.md` records the missing dual-reader,
  dependency/lock, cache invalidation, replay/evidence, and diagnostic rules.

## Conformance plan

- Assert all sixty local rehearsal boundaries, their exact order, duplicate
  rejection, and order-independent opaque bytes.
- Assert exact current hash schemes for `ling.semantic/0.1`,
  `ling.semantic/0.2`, and `ling.lock/1`, plus explicit absence of every
  previous-version and migration declaration.
- Run Semantic evidence, schema, protocol, governance, status, deterministic,
  offline, formatting, lint, and workspace test gates.
- Defer any real old/new algorithm, dual reader, explicit migration, cache or
  lock rewrite, and replay/evidence linkage until Accepted authority and
  executable fixtures exist.

## Compatibility impact

Existing Semantic IDs, canonical bytes, schemas, readers/writers, lockfiles,
caches, dependencies, replay/evidence formats, diagnostics, language/runtime
behavior, CLI/LSP, source spans, and Unicode 17.0.0 remain unchanged.

## Unresolved alternatives

Algorithm and schema-version ownership; old/new algorithm identifiers; dual
reader duration; writer cutover; migration inputs, outputs, idempotence, and
rollback; dependency and lock propagation; cache invalidation ordering;
replay/evidence linkage; corruption and mismatch diagnostics; compatibility
fixtures; release policy; and independent review remain open under
`PROTO-6203`, `PROTO-6204`, RFC-0004 or its Accepted replacement, the two
registered semantic lifecycle gaps, and incomplete G1-G5 exits.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
