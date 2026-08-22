# DEC-0141: Internal FFI fuzz and sanitizer boundary evidence / 内部 FFI Fuzz 与 Sanitizer 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: ffi-quality
> 相关规范/缺口：`DEC-0140` | `DEC-0139` | `DEC-0138` | `DEC-0137` | `DEC-0124` | `DEC-0117` | `DEC-0009` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-OWNERSHIP-MODEL-001` | `GAP-OWNERSHIP-PUBLIC-LIFETIME-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`FFI-3605-OBSERVATION` FFI fuzz and sanitizer boundary. It records vocabulary
and deterministic ordering while FFI, ABI, target, ownership, runtime,
toolchain, and security authorities remain unresolved.

本决定只授权 `FFI-3605-OBSERVATION` 使用 test-local 的拟议 FFI fuzz 与 sanitizer 边界
清单。在 FFI、ABI、target、ownership、runtime、toolchain 与 security 权威尚未解决时，
只记录词汇和确定性顺序。

## Question

FFI-3605 proposes fuzzing declarations, ABI readers, shims, and target packages
with malformed metadata, memory/ownership/callback faults, sanitizer classes,
reproducibility, corpus and coverage gates, and cross-target/compiler evidence.
Which planning vocabulary can be retained as bounded evidence without adding an
unsafe fuzz target, running a native sanitizer, or claiming a security result?

FFI-3605 计划对 declaration、ABI reader、shim 与 target package 进行 fuzz，覆盖 malformed
metadata、memory/ownership/callback fault、sanitizer 类别、reproducibility、corpus 与
coverage gate，以及 cross-target/compiler evidence。在不添加 unsafe fuzz target、不运行
native sanitizer、不声明安全结果的前提下，哪些规划词汇可以作为有界证据保留？

## Decision

1. `crates/ling-types/tests/ffi_fuzz_sanitizer_evidence.rs` keeps a test-local
   inventory of sixty provisional boundaries covering corpus/harness/target,
   declaration/ABI/shim/target loaders, malformed/unknown/truncated/oversized
   input and arithmetic/layout/encoding rejection, pointer/bounds/alignment,
   callback/thread/allocator/ownership/borrow/Fault/cancellation/capability/
   profile failures, linker/tamper/provenance/license/TCB, deterministic and
   reproducible/offline/cross-target/compiler runs, address/undefined/thread/
   memory/leak sanitizers, coverage/crash/corpus/timeout/memory bounds,
   bilingual diagnostics/Unicode, public-protocol and Semantic-ID separation,
   host-output exclusion, and Seed compatibility.
2. The test-local inventory sorts boundaries by explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.ffi-fuzz-sanitizer-observation/0`. These bytes are not a fuzz target,
   corpus, sanitizer result, coverage number, security claim, diagnostic,
   provenance record, Semantic ID, or public protocol.
3. The child adds no fuzz target, sanitizer configuration, native dependency,
   unsafe code, target toolchain, generated corpus, crash artifact, diagnostic,
   protocol, or placeholder API. Public `FFI-3605` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its fuzz/sanitizer checklist cannot
  define harness inputs, mutation policy, safety oracle, sanitizer versions,
  coverage thresholds, crash triage, or cross-target security claims.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` require future verified FFI,
  target, provenance, and security boundaries but do not authorize native fuzz
  execution for the v0.0.1 Seed subset.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and
  `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain Open. RFC-N305, RFC-0007,
  RFC-0011, `PROTO-ABI`, and `PROTO-EVIDENCE` are not Accepted authorities.
- Accepted `DEC-0140` through `DEC-0137` authorize only test-local vocabulary;
  they do not supply fuzz, sanitizer, or security semantics.

## Conformance plan

- Assert all sixty provisional FFI fuzz/sanitizer boundaries and their
  test-local order.
- Compare forward and reversed insertion order and require identical opaque
  evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep harness/corpus/mutation, sanitizer/toolchain, crash/coverage/resource
  bounds, security, provenance, cross-target/compiler, diagnostics, and public
  protocol behavior deferred until the required authorities are Accepted.

## Compatibility impact

- Accepted Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-local boundary evidence. No fuzz target, sanitizer result,
  security, diagnostic, dependency, toolchain, protocol, or support claim is
  registered.

## Unresolved alternatives

Fuzz corpus/harness/target and mutation; declaration/ABI/shim/target loaders;
malformed/unknown/truncated/oversized inputs; arithmetic/layout/encoding and
pointer/bounds/alignment; callback/thread/allocator/ownership/borrow/Fault/
cancellation/capability/profile; linker/tamper/provenance/license/TCB;
deterministic/reproducible/offline/cross-target/compiler runs; sanitizer
versions and classes, leak/coverage/crash/corpus/timeout/memory policy;
diagnostics, Unicode, public protocol, Semantic-ID and host-output exclusions
remain open under FFI-3605, DIFF-3701, DIFF-3702, GAP-NATIVE-BACKEND-ABI-001,
GAP-OWNERSHIP-MODEL-001, GAP-OWNERSHIP-PUBLIC-LIFETIME-001, and missing
RFC-N305/RFC-0007/RFC-0011 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
