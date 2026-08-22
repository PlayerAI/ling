# DEC-0176: Internal device-binary-cache boundary evidence / 内部 Device Binary Cache 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: compiler-quality  
> 相关规范/缺口：`DEC-0175` | `DEC-0022` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-INCREMENTAL-CACHE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`PLC-4805-OBSERVATION`. It records provisional device-binary cache identity,
validation, trust, lifecycle, safe-recompile, privacy, fixture, diagnostic,
and protocol vocabulary while Device IR and backend/cache authorities remain
unresolved.

本决定只授权 `PLC-4805-OBSERVATION` 使用 test-local 的 Device Binary Cache 边界清单；
在 Device IR 与 backend/cache 权威尚未解决时，只记录临时 cache identity、validation、trust、
lifecycle、safe-recompile、privacy、fixture、diagnostic 与 protocol 词汇。

## Question

PLC-4805 proposes cache keys containing Program/Semantic ID, Device IR and
backend versions, target architecture, runtime/driver compatibility, numeric
mode, Profile, and compiler options, with corruption falling back to
recompilation without changing semantics. Which vocabulary can be retained as
bounded evidence without widening DEC-0022 or creating a device-binary cache?

## Decision

1. `crates/ling-types/tests/device_binary_cache_evidence.rs` keeps a test-local
   inventory of sixty provisional identity, artifact, validation, trust,
   permissions, lifecycle, failure/recompile, privacy, fixture, diagnostic,
   and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.device-binary-cache-observation/0`. These bytes
   are evidence only; they are not cache keys, artifacts, signatures,
   namespaces, cache entries, diagnostics, or support.
3. Accepted DEC-0022 is not widened: no Device IR serialization, binary cache,
   backend/target dependency, signing API, migration, locking, eviction,
   diagnostic, or placeholder API is added. Public `PLC-4805` remains
   `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:495-510` is
  non-normative and does not define Device IR/binary bytes, ABI, trust, driver
  compatibility, lifecycle, permissions, or cache protocol.
- `docs/ROADMAP-1.0.md:421-431` makes device-binary cache correctness a G4.6
  goal but does not authorize an artifact or observable cache behavior.
- Accepted `DEC-0022` authorizes only an opt-in disposable internal line-index
  payload and forbids unchecked compiler IR/bytecode deserialization; it does
  not authorize device-binary caching.
- `docs/status/PLC-4805-AUTHORITY-AUDIT.md` records missing authority and
  `DEC-0175` remains prerequisite explain evidence only.

## Conformance plan

- Assert all sixty cache boundaries and local order; compare forward/reverse
  opaque bytes; reject duplicates.
- Defer Device IR/binary serialization, cache key/namespace, validation,
  signing/trust, lifecycle, safe-recompile, diagnostics, and protocol behavior
  until accepted authority exists.

## Compatibility impact

Accepted Seed behavior, DEC-0022 line-index cache behavior, diagnostics,
schemas, Semantic IDs, source spans, CLI/LSP, runtime, bytecode, VM,
dependencies, and Unicode 17.0.0 remain unchanged. Only test-local evidence
is added; no device-binary cache or support claim is registered.

## Unresolved alternatives

Program/Semantic ID and Device IR/backend/target/runtime/driver/numeric/Profile/
compiler-option identity; canonical artifact bytes and validation; capability,
signatures, verification, trust, permissions, path isolation, atomic publish,
concurrent writers, eviction, disk limits, disposable/portable policy;
hit/miss, corruption/unknown/ABI/capability/environment/options failures;
safe recompile, migration and privacy; host/path/address/timestamp/allocation/
driver/debug exclusions; corruption/migration/cross-toolchain/cross-target/
numeric/Profile/security/replay/differential/Unicode/determinism fixtures;
diagnostics, protocol inventory, and public cache status remain open under
PLC-4805, PLC-4804, PLC-4803, PLC-4802, PLC-4801, DEC-0022's boundaries,
GAP-INCREMENTAL-CACHE-001, GAP-KERNEL-DEVICE-001,
GAP-NATIVE-BACKEND-ABI-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
