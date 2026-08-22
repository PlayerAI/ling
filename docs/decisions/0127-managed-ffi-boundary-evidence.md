# DEC-0127: Internal Managed/Native/FFI boundary evidence / 内部 Managed/Native/FFI 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: native-interop
> 相关规范/缺口：`DEC-0126` | `DEC-0125` | `DEC-0121` | `DEC-0013` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of the proposed
Managed/Native/FFI boundary contracts for the bounded `GC-3303-OBSERVATION`
child. It checks deterministic, duplicate-free vocabulary. It does not define
pinning, handles, raw-pointer policy, callbacks, thread attachment, foreign
ownership, ABI, FFI schemas, collection during calls, Profile restrictions, or
runtime semantics.

本决定只授权 test-only 的拟议 Managed/Native/FFI 边界契约清单，供
`GC-3303-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不定义 pinning、handle、raw-pointer 策略、
callback、thread attachment、foreign ownership、ABI、FFI schema、调用期间 collection、Profile 限制或运行时语义。

## Question

GC-3303 lists pin/unpin, a handle table, raw-pointer escape prevention,
callback roots, thread attachment, foreign ownership, collection during FFI,
and deterministic cleanup versus finalization. Which boundary vocabulary can be
retained without choosing an ABI or exposing a host pointer contract?

GC-3303 列出 pin/unpin、handle table、raw-pointer escape prevention、callback root、thread attachment、foreign ownership、
FFI 期间 collection，以及 deterministic cleanup 与 finalization 的区分。哪些边界词汇可以保留，而不会选择 ABI 或暴露宿主指针契约？

## Decision

1. `crates/ling-concurrency/tests/managed_ffi_boundary_evidence.rs` keeps a
   test-local inventory of forty-three provisional boundaries: pinning and
   handle-generation safety, raw-pointer and borrowed-view restrictions,
   callback roots and thread attachment, Value/Managed/Resource/opaque
   ownership, transfer/borrow/release/cleanup, finalizer separation,
   collection and callback allocation/blocking/cancellation/fault/reentry,
   Task/Actor invariants, ABI and target rules, Fault/unwind, capability/TCB,
   FFI schema versioning, Profiles, differential evidence, Unicode spans,
   determinism, and sanitizer/security evidence.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.managed-ffi-observation/0`. These bytes are not a handle, pin token,
   pointer, callback registry, thread protocol, ownership transfer, ABI,
   schema, diagnostic, public protocol, or runtime contract.
3. The child adds no Managed handle, pinning API, raw-pointer wrapper,
   callback-root registry, thread-attachment protocol, FFI ownership mode, ABI
   schema, diagnostic, or placeholder Native/FFI crate. Public `GC-3303`
   remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative and cannot define a safe handle
  representation or foreign-call contract.
- Accepted `DEC-0126` and `DEC-0125` record only collector/object-model
  vocabulary; `DEC-0121` preserves suspension vocabulary without resolving
  FFI reentry or ownership.
- Accepted `DEC-0013` preserves compile/host/internal/runtime-fault
  separation and does not create a Native ABI or public raw-pointer facility.
- `GAP-NATIVE-BACKEND-ABI-001` and `GAP-OWNERSHIP-MODEL-001` remain Open;
  RFC-N303/RFC-N304/RFC-N305/RFC-N306 and RFC-0007 are not Accepted. This
  decision records vocabulary without resolving those gaps.

## Conformance plan

- Assert all forty-three provisional Managed/Native/FFI boundaries and their
  test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep handle/pin behavior, raw-pointer policy, callback/thread rules,
  foreign ownership, ABI/target schemas, collection during FFI, cleanup versus
  finalization, Profiles, diagnostics, and differential semantics deferred.

## Compatibility impact

- Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-only boundary evidence. No handle, pointer, ABI, FFI,
  diagnostic, Semantic ID, or public protocol claim is registered.

## Unresolved alternatives

Handle representation and generation lifetime, pinning and movement,
borrowed-view validity, callback roots and thread attachment, foreign
ownership modes, ABI/calling convention/target packages, unwind/reentry and
collection during FFI, Profile/no-GC restrictions, diagnostics, migration,
security, and interpreter/VM/Native differential semantics remain open under
`GC-3303`, `GC-3302`, `GC-3301`, `GAP-NATIVE-BACKEND-ABI-001`,
`GAP-OWNERSHIP-MODEL-001`, and missing RFC-N303/RFC-N304/RFC-N305/RFC-N306/
RFC-0007 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
