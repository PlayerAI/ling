# DEC-0134: Internal Native runtime ABI boundary evidence / 内部 Native Runtime ABI 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: native-backend
> 相关规范/缺口：`DEC-0133` | `DEC-0132` | `DEC-0131` | `DEC-0130` | `DEC-0129` | `DEC-0128` | `DEC-0127` | `DEC-0126` | `DEC-0125` | `DEC-0009` | `DEC-0012` | `DEC-0013` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of the proposed internal
Native runtime ABI boundary for the bounded `BACK-3503-OBSERVATION` child. It
records ABI concerns without freezing representation, calling conventions,
runtime behavior, compatibility, or a public protocol.

本决定只授权 test-only 的拟议 internal Native runtime ABI 边界清单，供
`BACK-3503-OBSERVATION` 子任务使用。它记录 ABI 关注点，但不冻结 representation、calling convention、runtime behavior、compatibility，
也不建立 public protocol。

## Question

BACK-3503 proposes a versionable internal ABI for Value passing, ADT tags,
closure environments, Fault/Result, GC handles, Resource Drop, Task/Actor
calls, and String/Text. Which planning vocabulary can be retained while the
Native ABI, memory/ownership/Managed, concurrency, FFI, and Profile authorities
remain unaccepted?

BACK-3503 提议为 Value passing、ADT tag、closure environment、Fault/Result、GC handle、Resource Drop、Task/Actor call 与 String/Text 建立可版本化
的 internal ABI。在 Native ABI、memory/ownership/Managed、concurrency、FFI 与 Profile 权威尚未 Accepted 时，哪些规划词汇可以保留？

## Decision

1. `crates/ling-types/tests/native_runtime_abi_evidence.rs` keeps a test-local
   inventory of fifty-eight provisional boundaries: primitive/aggregate/
   record/tuple/ADT/closure/string/text/bytes passing; Fault/Result, unwind,
   cancellation, shutdown, thread/reentry; GC identity/root/barrier/pin;
   Resource ownership/Drop, borrow/region, FFI/foreign ownership; Task/Actor,
   mailbox/turn; layout/alignment/endianness/target/calling/runtime library;
   ABI version/compatibility/negotiation/migration/mangling/debug/schema and
   deterministic metadata; unsupported bilingual diagnostics and Unicode
   spans; security/offline, differential/cross-target/sanitizer evidence;
   host/allocation/address/timing/map exclusions; public-ABI exclusion and
   Seed compatibility.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.native-runtime-abi-observation/0`. These bytes are not a layout,
   calling convention, runtime library, handle/drop shim, Task/Actor call
   surface, compatibility record, diagnostic, public ABI, or semantic proof.
3. The child adds no ABI manifest, runtime library, version marker, calling
   convention, handle/drop shim, Task/Actor surface, diagnostic, public
   protocol, dependency, or placeholder crate. Public `BACK-3503` remains
   `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its internal-ABI checklist cannot
  define representation, layout, failure, ownership, concurrency, FFI, or
  compatibility semantics.
- Accepted `DEC-0133`, `DEC-0132`, and `DEC-0131` define only test-local
  codegen/backend-selection/verifier vocabulary. They do not authorize an ABI.
- RFC-N304/RFC-N306 and candidate RFC-0011 are absent or not Accepted.
  `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`,
  `GAP-STRUCTURED-TASK-001`, and `GAP-ACTOR-AWAIT-REENTRY-001` remain Open.
- Accepted Seed decisions `DEC-0009`, `DEC-0012`, and `DEC-0013` govern current
  source/Typed-Core identity and runtime failures; they do not freeze Native
  layout or a cross-compiler/runtime ABI.

## Conformance plan

- Assert all fifty-eight provisional runtime-ABI boundaries and their
  test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep layout/calling/runtime/ownership/GC/Task/Actor/FFI/versioning,
  diagnostics, compatibility, and public-ABI behavior deferred until their
  authorities are Accepted.

## Compatibility impact

- Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-only boundary evidence. No runtime ABI, layout, calling
  convention, diagnostic, Semantic ID, dependency, or public protocol claim
  is registered.

## Unresolved alternatives

Value/aggregate/ADT/closure/text representation; Fault/Result/unwind/
cancellation/shutdown/thread/reentry; GC handles/barriers/pinning; Resource
ownership/Drop/borrow/FFI transfer; Task/Actor mailbox/turn; layout/alignment/
endianness/target/calling/runtime library; versioning/compatibility/
negotiation/migration/mangling/debug/schema; deterministic metadata;
diagnostics; Unicode; security/offline; differential/cross-target/sanitizer;
host/allocation/address/timing/map exclusions; and public ABI semantics remain
open under `BACK-3503`, `BACK-3502`, `BACK-3501`, `NIR-3403`, `NIR-3402`,
`NIR-3401`, `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`,
`GAP-STRUCTURED-TASK-001`, `GAP-ACTOR-AWAIT-REENTRY-001`, and missing
RFC-N306/RFC-N304/RFC-N303/RFC-0007 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
