# DEC-0139: Internal FFI shim-generator boundary evidence / 内部 FFI Shim Generator 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: ffi
> 相关规范/缺口：`DEC-0138` | `DEC-0137` | `DEC-0136` | `DEC-0124` | `DEC-0117` | `DEC-0009` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-OWNERSHIP-MODEL-001` | `GAP-OWNERSHIP-PUBLIC-LIFETIME-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`FFI-3603-OBSERVATION` shim-generator boundary. It records vocabulary and
deterministic ordering while declaration, ABI, ownership, target, generator
trust, build, and evidence authorities remain unresolved.

本决定只授权 `FFI-3603-OBSERVATION` 使用 test-local 的拟议 Shim Generator 边界清单。
在 declaration、ABI、ownership、target、generator trust、build 与 evidence 权威尚未
解决时，只记录词汇和确定性顺序。

## Question

FFI-3603 proposes generated layout assertions, bounds/null checks, ownership
conversion, string encoding, callback trampolines, Fault/Capability mapping,
and provenance that participates in a build hash. Which planning vocabulary can
be retained as bounded evidence without generating code, selecting a template
trust boundary, or defining artifact/build-hash semantics?

FFI-3603 计划生成 layout assertion、bounds/null check、ownership conversion、string
encoding、callback trampoline、Fault/Capability mapping，并让 provenance 参与 build hash。
在不生成代码、不选择 template trust boundary、不定义 artifact/build-hash 语义的前提下，
哪些规划词汇可以作为有界证据保留？

## Decision

1. `crates/ling-types/tests/ffi_shim_generator_evidence.rs` keeps a test-local
   inventory of sixty provisional boundaries covering shim input/declaration/
   ABI/target/layout/ownership/capability/encoding/callback/error facts;
   source-span/Semantic-ID views; generator/template trust and version; output
   language/schema/source/artifact/metadata; layout, null/bounds/overflow,
   mutability/encoding, ownership/allocator/drop, callback lifetime/thread/
   reentry/cancellation, Fault/Capability, unsupported and unknown-field rules;
   canonical/deterministic/clean/repeat/offline generation; provenance,
   license/TCB, tamper, build-hash and Semantic-ID separation, host exclusions,
   diagnostics/Unicode, schema migration, target compatibility, sanitizer/fuzz,
   compiler differential evidence, and Seed compatibility.
2. The test-local inventory sorts boundaries by explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged `ling.ffi-shim-observation/0`.
   These bytes are not generated source, a shim, a layout assertion, a safety
   proof, provenance, a build-hash input, a Semantic ID, a diagnostic, or a
   public protocol.
3. The child adds no generator, template, generated source/header, layout or
   pointer check, ownership adapter, callback trampoline, Fault/Capability
   bridge, provenance record, build-hash input, dependency, toolchain,
   diagnostic, protocol, or placeholder API. Public `FFI-3603` remains
   `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its shim checklist cannot define
  generator inputs/outputs, generated-language ABI, trust, ownership
  conversion, failure behavior, or canonical provenance/build-hash bytes.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` require future verified Typed FFI,
  target, provenance, and TCB boundaries but do not authorize generated shims
  for the v0.0.1 Seed subset.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and
  `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain Open. RFC-N305, RFC-0007,
  RFC-0011, `PROTO-ABI`, and `PROTO-EVIDENCE` are not Accepted authorities.
- Accepted `DEC-0138` and `DEC-0137` authorize only test-local vocabulary; they
  do not supply generator, artifact, provenance, or build-hash semantics.

## Conformance plan

- Assert all sixty provisional shim-generator boundaries and their test-local
  order.
- Compare forward and reversed insertion order and require identical opaque
  evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep shim schemas, generated checks/adapters, trust/TCB, provenance/build
  hash, diagnostics, migration, sanitizer/fuzz, and cross-target behavior
  deferred until the required authorities are Accepted.

## Compatibility impact

- Accepted Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-local boundary evidence. No generator, artifact, build-hash,
  provenance, diagnostic, dependency, protocol, or support claim is registered.

## Unresolved alternatives

Shim input/output schemas; generator/template trust and target language;
layout/null/bounds/overflow/mutability/encoding checks; ownership/allocator/drop
conversion; callback lifetime/thread/reentry/cancellation; Fault/Capability;
unknown/unsupported and canonical ordering; deterministic/offline/repeat builds;
provenance/license/TCB/tamper; artifact/cache/release/build-hash/Semantic-ID
relations; diagnostics, Unicode, migration, sanitizer/fuzz, differential,
target compatibility, and public protocol rules remain open under FFI-3603,
FFI-3604, FFI-3605, GAP-NATIVE-BACKEND-ABI-001, GAP-OWNERSHIP-MODEL-001,
GAP-OWNERSHIP-PUBLIC-LIFETIME-001, and missing RFC-N305/RFC-0007/RFC-0011
authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
