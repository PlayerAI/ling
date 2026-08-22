# DEC-0138: Internal C ABI interoperability boundary evidence / 内部 C ABI 互操作边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: ffi
> 相关规范/缺口：`DEC-0137` | `DEC-0136` | `DEC-0134` | `DEC-0124` | `DEC-0117` | `DEC-0009` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-OWNERSHIP-MODEL-001` | `GAP-OWNERSHIP-PUBLIC-LIFETIME-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`FFI-3602-OBSERVATION` C ABI boundary. It records vocabulary and deterministic
ordering while the C ABI, target, ownership, runtime, and `PROTO-ABI`
authorities remain unresolved.

本决定只授权 `FFI-3602-OBSERVATION` 使用 test-local 的拟议 C ABI 边界清单。在
C ABI、target、ownership、runtime 与 `PROTO-ABI` 权威尚未解决时，只记录词汇和
确定性顺序。

## Question

FFI-3602 proposes a small C interoperability surface for scalars, records,
spans, callbacks, opaque handles, allocator pairs, and error codes while
rejecting variadics, bitfields, and platform-dependent layouts. Which planning
vocabulary can be retained as bounded evidence without selecting a host C ABI,
layout, pointer lifetime, linker, or executable interoperation contract?

FFI-3602 计划为 scalar、record、span、callback、opaque handle、allocator pair 与 error
code 提供有限的 C 互操作面，同时拒绝 variadic、bitfield 与平台相关 layout。在不选择
宿主 C ABI、layout、pointer lifetime、linker 或可执行互操作合同的前提下，哪些规划词汇
可以作为有界证据保留？

## Decision

1. `crates/ling-types/tests/ffi_c_abi_evidence.rs` keeps a test-local
   inventory of sixty provisional C ABI boundaries covering ABI/version,
   target/calling/scalar representation, endianness/alignment/record/union
   layout and unsupported forms, symbols/header/declaration identity and
   argument/result layout, span pointer/length/nullability/overflow/provenance/
   mutability/encoding, callbacks and their calling/lifetime/thread/reentry/
   cancellation rules, opaque handles, allocator provenance/deallocation,
   ownership/borrow/Resource/Managed, Error/Fault/unwind/blocking, Capability/
   Profile/target rejection, source spans, diagnostics/Unicode, schema/version/
   migration, provenance/TCB, sanitizer/fuzz, cross-target evidence, and Seed
   compatibility.
2. The test-local inventory sorts boundaries by explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.ffi-c-abi-observation/0`. These bytes are not C declarations, a layout
   result, a calling convention, a pointer/handle validity proof, a linker
   input, an ABI schema, a Semantic ID, a diagnostic, or a public protocol.
3. The child adds no C parser/importer, layout calculator, compiler or linker
   probe, callback trampoline, handle runtime, allocator bridge, dependency,
   toolchain, diagnostic, protocol, or placeholder API. Public `FFI-3602`
   remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its C ABI checklist cannot define
  C widths, calling conventions, layout, pointer validity, callback lifetime,
  allocator provenance, error transport, or target selection.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` require a future verified Typed
  FFI and target boundary but do not make a host C ABI Ling semantics for the
  v0.0.1 Seed subset.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and
  `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain Open. RFC-N305, RFC-0007,
  RFC-0011, and the `PROTO-ABI` schema are not Accepted authorities.
- Accepted `DEC-0137`, `DEC-0136`, and `DEC-0134` authorize only test-local
  vocabulary; they do not supply C layout, linker, or runtime semantics.

## Conformance plan

- Assert all sixty provisional C ABI boundaries and their test-local order.
- Compare forward and reversed insertion order and require identical opaque
  evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep C declaration/import, scalar and aggregate layout, span/callback/handle/
  allocator safety, symbol/linker, Error/Fault, target/profile/capability,
  schema, diagnostics, sanitizer/fuzz, and cross-target behavior deferred
  until the required authorities are Accepted.

## Compatibility impact

- Accepted Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-local boundary evidence. No C ABI, target, ownership,
  diagnostic, dependency, protocol, linker, or support claim is registered.

## Unresolved alternatives

C widths and representations; target/calling/endianness/alignment/record/union
layout; rejection of variadics/bitfields/flexible arrays; symbol/header/version
rules; span pointer/length/nullability/provenance/mutability/encoding; callback
lifetime/thread/reentry/cancellation; opaque handle affinity; allocator pairs;
ownership/borrow/Resource/Managed; Error/Fault/unwind/blocking; Capability/
Profile/Target; schema/migration; provenance/TCB; diagnostics, Unicode,
sanitizer/fuzz, cross-target, differential, and public protocol rules remain
open under FFI-3602, FFI-3603, FFI-3604, FFI-3605, GAP-NATIVE-BACKEND-ABI-001,
GAP-OWNERSHIP-MODEL-001, GAP-OWNERSHIP-PUBLIC-LIFETIME-001, and missing
RFC-N305/RFC-0007/RFC-0011 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
