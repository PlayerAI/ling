# Ling schema lifecycle policy / Ling Schema 生命周期策略

> Status: **Draft engineering policy**
> Version: `1`
> Scope: existing implemented public machine-readable schemas
> Machine inventory: `schemas/registry.toml` (delivered by GOV-0106 slice B)

This policy defines how Ling records and tests schema compatibility. It does not change a public wire format, accept a new language semantic, or promote any protocol to Stable. Accepted RFCs/decisions and the public protocol inventory remain authoritative when they conflict with this document.

本文规定 Ling 如何记录并测试 Schema 兼容性。它不改变公开 wire format、不接受新语言语义，也不把任何协议提升为 Stable。若发生冲突，以 Accepted RFC/decision 和公开协议清单为准。

## 1. Scope / 范围

The first corpus covers the three implemented public JSON schemas recorded by GOV-0104:

| Protocol | Current marker | Reader today | Canonical today |
| --- | --- | --- | --- |
| Diagnostic JSON | `ling.diagnostic/0.1` | no public reader | no |
| Semantic Graph JSON | `ling.semantic/0.1` | isolated exact-version reader | yes |
| REPL event JSON | `ling.repl/0.1` | no standalone reader | no |

Audit Source is an implemented canonical text protocol and Semantic IDs use canonical binary projections, but neither is JSON. They retain their accepted parser/hash tests and are explicitly outside this first JSON-schema corpus. Future package, bytecode, replay, ABI, transaction, and evidence formats have no accepted schema and must not receive placeholder packages.

首批 corpus 只覆盖三个已实现的公开 JSON Schema。Audit Source 与 Semantic ID 不是 JSON；未来协议尚无 Accepted Schema，因此不得建立暗示已实现的占位包。

## 2. Version meaning / 版本含义

### 2.1 Major/minor markers

A marker such as `ling.semantic/0.1` identifies one concrete schema. A different major is incompatible unless an explicit migration edge says otherwise. A different minor is not automatically compatible: compatibility exists only when the registry names the reader range, policy, fixtures, and passing evidence. In particular, `0.x` does not imply a Semantic Versioning promise.

`ling.semantic/0.1` 之类 marker 只标识一个具体 Schema。major 不同默认不兼容；minor 不同也不自动兼容。只有 registry 明确登记 reader 范围、策略、fixtures 与通过证据时，兼容性才成立；`0.x` 不构成 SemVer 承诺。

### 2.2 Integer schema versions

An integer version identifies a revision of an internal or protocol-specific schema. Incrementing the integer means “different revision,” not “compatible revision.” A compatibility or migration edge must still be declared and tested. Public formats must also carry their public protocol marker; an internal integer cannot replace it.

整数版本表示内部或特定协议 Schema 的 revision；递增不自动表示兼容。仍须声明并测试兼容/迁移边。公开格式还必须携带公开协议 marker，内部整数不能替代它。

### 2.3 Stability is separate

`Experimental`, `Preview`, and `Stable` describe commitment level, not parse compatibility. A Preview schema can have no previous reader, and a Stable schema cannot claim N-1 support without executable evidence.

稳定级别与解析兼容性相互独立。Preview 可以没有旧版 reader；Stable 也不能在缺少可执行证据时宣称 N-1。

## 3. Writer and reader rules / Writer 与 reader 规则

- A writer emits exactly the registry’s current version. It never emits an older version opportunistically.
- A reader accepts only listed versions. “Current only,” “no public reader,” and an explicit version range are distinct states.
- N-1 is `NoPreviousVersion` for a first release. This is not equivalent to a passing compatibility reader.
- When a previous version exists, the compatibility edge must name fixtures, expected reader/migration behavior, and the implementation evidence that enforces it.
- A migration must create a new artifact; it must not silently reinterpret or overwrite the source bytes.

- Writer 只输出 registry 的 current version，不机会性输出旧版本。
- Reader 只接受明确列出的版本；“仅当前版本”“无公开 reader”“显式版本范围”是不同状态。
- 首版的 N-1 状态是 `NoPreviousVersion`，不等同于兼容性测试通过。
- 出现旧版本后，兼容边必须列出 fixtures、预期 reader/migration 行为与实现证据。
- Migration 必须产生新产物，不能静默重解释或覆盖源 bytes。

## 4. Fields / 字段策略

Each schema record declares both policies; no repository-wide default is inferred.

| Dimension | Allowed declaration | Meaning |
| --- | --- | --- |
| Unknown fields | `Reject`, `NamespacedExtensions`, `Unspecified` | reject all unknowns; accept only registered extension namespace; or make no compatibility promise |
| Missing fields | `RejectRequired`, `ReaderDefault`, `Unspecified` | reject required omission; apply a named reader default; or make no compatibility promise |

`ReaderDefault` must identify the field and default in executable fixtures. A writer omitting an optional field does not by itself promise that every reader accepts the omission. Misspelled core fields must never be treated as extensions.

`ReaderDefault` 必须通过可执行 fixtures 指明字段与默认值。Writer 省略 optional field 并不自动承诺所有 reader 都接受；拼错的 core field 不得被当作扩展。

## 5. Canonical encoding and identity / Canonical encoding 与身份

- `canonical = false` means JSON member order and insignificant whitespace are not compatibility or identity inputs.
- `canonical = true` requires byte golden files, exactly one declared newline policy, deterministic collection ordering, and an actual reader/writer round trip.
- Ordinary JSON serialization order must never be used directly as a Semantic ID input.
- Semantic IDs continue to use the Accepted DEC-0012 domain-separated, length-prefixed canonical byte encodings and explicit hash-scheme identifiers.
- A schema, canonicalization, hash domain, normalization rule, or hash prefix change requires a new identifier/version and migration explanation; the old identifier must not be silently reused.

- `canonical = false` 表示 JSON member 顺序与无意义空白不是兼容或身份输入。
- `canonical = true` 必须具备 byte golden、唯一 newline 策略、确定性集合顺序以及真实 reader/writer round trip。
- 普通 JSON 序列化顺序绝不能直接作为 Semantic ID 输入。
- Semantic ID 继续使用 Accepted DEC-0012 的 domain-separated、length-prefixed canonical bytes 与显式 hash scheme ID。

## 6. Corpus contract / Corpus 契约

Each JSON schema package uses this layout:

```text
schemas/<name>/<version>/schema.json
schemas/<name>/<version>/valid/*.json
schemas/<name>/<version>/invalid/*.json
schemas/<name>/<version>/canonical/*.bin
```

- `schema.json` is a Draft 2020-12 JSON Schema constrained to the keyword subset enforced by the offline checker.
- `valid/` inputs must parse, satisfy the declared shape, carry the exact protocol marker, and pass an implementation reader when one exists.
- `invalid/` inputs must be rejected either as malformed JSON, schema-invalid data, or reader-invalid data. An expectation sidecar records which rejection class is intended.
- `canonical/` contains exact bytes only for protocols already declared canonical by GOV-0104.
- Fixture discovery is recursive only inside the declared package directories; registry text is never executed.

`schema.json` 使用 Draft 2020-12，并限制在离线 checker 实现的 keyword 子集。`valid/` 必须通过 shape 与现有 reader；`invalid/` 必须按 sidecar 指定类别被拒绝；只有 GOV-0104 已声明 canonical 的协议才有 exact-byte golden。

## 7. Required gates / 必需门禁

GOV-0106 is complete only when these locked/offline commands exist and pass:

```text
cargo xtask schema validate-all
cargo xtask schema compatibility --from N-1 --to N
cargo xtask schema corrupt-inputs
```

`validate-all` checks registry/protocol parity, schema dialect, fixtures, implementation readers, and canonical bytes. `compatibility` verifies explicit edges and reports first-version `NoPreviousVersion` records without relabeling them as N-1 support. `corrupt-inputs` applies deterministic in-memory truncation, trailing-data, version, required-field, unknown-field, and canonical-byte mutations appropriate to each policy.

## 8. Change checklist / 变更清单

Any schema change must update together:

1. Accepted authority when observable behavior changes;
2. protocol inventory version/stability/reader/writer/field policies;
3. schema registry and `schema.json`;
4. valid, invalid, compatibility, and canonical fixtures as applicable;
5. actual reader/writer tests;
6. migration notes and Semantic ID/hash impact;
7. support matrix, feature status, traceability, and release evidence.

## 9. Non-goals / 非目标

This policy does not define package, bytecode, replay, ABI, evidence, Semantic Transaction, Profile, Native, or device schemas. It does not add a general JSON-to-Typed-Core path, and schema validation output must never enter evaluation. It does not promote any current protocol, invent an N-1 reader, or change Unicode 17.0.0 and UTF-8 byte-span behavior.

本策略不定义未来 package、bytecode、replay、ABI、evidence、Semantic Transaction、Profile、Native 或 device Schema；验证后的 JSON 不得进入求值；也不提升稳定级别、不虚构 N-1 reader、不改变 Unicode 17.0.0 或 UTF-8 byte span。
