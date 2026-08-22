# DEC-0131: Internal Native IR verifier boundary evidence / 内部 Native IR Verifier 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: native-backend
> 相关规范/缺口：`DEC-0130` | `DEC-0129` | `DEC-0128` | `DEC-0127` | `DEC-0126` | `DEC-0125` | `DEC-0012` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of the proposed Native IR
verifier boundary for the bounded `NIR-3403-OBSERVATION` child. It checks a
deterministic, duplicate-free vocabulary for validation concerns. It does not
define a Native IR grammar, implement a verifier, allocate diagnostics, or
establish a parser, ABI, execution, or public protocol.

本决定只授权 test-only 的拟议 Native IR verifier 边界清单，供
`NIR-3403-OBSERVATION` 子任务使用。它只检查确定且无重复的验证关注点词汇；不定义 Native IR grammar、不实现 verifier、不分配诊断码，
也不建立 parser、ABI、执行或 public protocol。

## Question

NIR-3403 lists block/phi/SSA, type consistency, Resource ownership, cleanup
coverage, legal ABI, source IDs, reference validity, and rejection of
backend-specific operations, with safe rejection and no host UB. Which
planning vocabulary can be retained without deciding the missing NIR and
Native contracts?

NIR-3403 列出 block/phi/SSA、类型一致性、Resource ownership、cleanup coverage、合法 ABI、source ID、引用有效性、拒绝 backend-specific
operation，以及安全拒绝和不触发 host UB。哪些规划词汇可以保留而不会决定尚未解决的 NIR 与 Native 契约？

## Decision

1. `crates/ling-types/tests/native_ir_verifier_evidence.rs` keeps a test-local
   inventory of forty-four provisional verifier boundaries: block/control-flow
   and phi/SSA structure, type and aggregate consistency, Resource/Managed and
   borrow facts, cleanup/Drop/Fault/Effect edges, ABI and calling convention,
   source IDs/spans and definition mappings, reference and backend-neutrality
   checks, version/malformed/duplicate/structural rejection, deterministic
   diagnostics, bounded safe rejection, Unicode spans, Semantic IDs,
   differential evidence, and migration compatibility.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.native-ir-verifier-observation/0`. These bytes are not a verifier,
   parser, IR schema, diagnostic, ABI validator, execution trace, public
   protocol, or proof of host-UB freedom.
3. The child adds no verifier, NIR parser, malformed-input schema, diagnostic,
   backend operation set, public protocol, or placeholder crate. Public
   `NIR-3403` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its verifier checklist cannot
  define NIR grammar, invariants, validation order, error behavior, or safety
  guarantees.
- Accepted `DEC-0130` and `DEC-0129` define only test-local lowering/design
  vocabulary. Accepted memory, ownership, Managed, Profile, and FFI boundary
  decisions do not define Native IR verification semantics.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and Task/Actor and
  Profile gaps remain Open. RFC-N304 and dependent Native, memory, ownership,
  FFI, and Profile authorities are not Accepted.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` keep checked Seed Typed Core as
  the executable authority; they do not authorize arbitrary or unchecked IR
  input.

## Conformance plan

- Assert all forty-four provisional verifier boundaries and their test-local
  order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep NIR parsing, CFG/SSA/type/ownership/cleanup/ABI validation,
  backend-neutral operation rules, diagnostics, fuzzing, and execution
  isolation deferred until their authorities are Accepted.

## Compatibility impact

- Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-only boundary evidence. No verifier, NIR, ABI, diagnostic,
  Semantic ID, execution, or public protocol claim is registered.

## Unresolved alternatives

NIR grammar/versioning and parser limits; CFG/SSA/phi and type rules;
Resource/Managed/borrow/cleanup/Fault/Effect invariants; ABI/layout/target and
backend-neutral operation policy; source-ID and Semantic ID mapping; malformed,
cyclic, oversized, and unknown-version handling; stable diagnostics; fuzz,
property, panic/host-UB, differential, security, migration, and protocol
semantics remain open under `NIR-3403`, `NIR-3402`, `NIR-3401`, `GC-3304`,
`GC-3303`, `GC-3302`, `GC-3301`, `GAP-NATIVE-BACKEND-ABI-001`,
`GAP-OWNERSHIP-MODEL-001`, and missing RFC-N304/RFC-N303/RFC-0007 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
