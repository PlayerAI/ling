# DEC-0137: Internal FFI declaration boundary evidence / 内部 FFI 声明边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: ffi
> 相关规范/缺口：`DEC-0136` | `DEC-0124` | `DEC-0117` | `DEC-0009` | `DEC-0012` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-OWNERSHIP-MODEL-001` | `GAP-OWNERSHIP-PUBLIC-LIFETIME-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`FFI-3601-OBSERVATION` declaration boundary. It records vocabulary and
deterministic ordering while the declaration grammar, ABI, ownership, target,
runtime, and protocol authorities remain unresolved.

本决定只授权 `FFI-3601-OBSERVATION` 使用 test-local 的拟议 FFI 声明边界清单。
在声明语法、ABI、ownership、target、runtime 与 protocol 权威尚未解决时，只记录
词汇和确定性顺序。

## Question

FFI-3601 lists ABI, symbol, argument/result layout, ownership transfer, borrow
duration, threading, reentrancy, Error/Fault mapping, Capability, profile, and
target constraints, but the execution package is non-normative and RFC-N305 is
not Accepted. Which planning vocabulary can be preserved as bounded evidence
without making a declaration syntax or executable ABI contract observable?

FFI-3601 列出 ABI、symbol、参数/结果 layout、ownership transfer、borrow duration、
threading、reentrancy、Error/Fault mapping、Capability、profile 与 target constraints，
但执行计划是非规范性的，RFC-N305 也未 Accepted。哪些规划词汇可以作为有界证据保留，
而不把声明语法或可执行 ABI 合同变成可观察行为？

## Decision

1. `crates/ling-types/tests/ffi_declaration_evidence.rs` keeps a test-local
   inventory of sixty provisional boundaries covering declaration identity and
   symbol naming; ABI/version, arguments/results, layout and representation;
   target/profile/capability; ownership, borrow, mutability, Resource/Managed,
   allocator, pointer/span/nullability, and encoding; callbacks, threading,
   reentrancy, blocking, cancellation, Error/Fault/unwind, handles, and symbol
   versioning; source spans, Semantic IDs, view separation, grammar/AST/HIR/
   Checked Core/verified lowering; diagnostics, Unicode, unsupported forms,
   deterministic ordering, schema/version/migration, provenance/TCB,
   sanitizer/fuzz, cross-target evidence, and Seed compatibility.
2. The test-local inventory sorts boundaries by explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.ffi-declaration-observation/0`. These bytes are not FFI syntax, an ABI
   schema, a declaration identity, a Semantic ID, a diagnostic, a linker input,
   a target package, a safety proof, or a public protocol.
3. The child adds no parser node, AST/HIR/Checked Core node, resolver, layout
   calculator, foreign symbol lookup, raw-pointer operation, target package,
   dependency, toolchain, diagnostic, protocol, or placeholder API. Public
   `FFI-3601` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its FFI checklist cannot define
  declaration grammar, ABI layout, ownership, lifetime, target selection, or
  foreign-call observability.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` describe a future Typed FFI and
  trusted Target Primitive boundary, but do not accept FFI syntax or an
  executable ABI contract for the v0.0.1 Seed subset.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and
  `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain Open. RFC-N305, RFC-0007, and
  RFC-0011 are not Accepted, and `PROTO-ABI` remains Planned without a schema
  or executable fixtures.
- Accepted `DEC-0136` and earlier Native evidence decisions authorize only
  test-local vocabulary; they do not supply declaration or ABI semantics.

## Conformance plan

- Assert all sixty provisional FFI declaration boundaries and their test-local
  order.
- Compare forward and reversed insertion order and require identical opaque
  evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep grammar, declaration identity, ABI/layout, ownership/lifetime,
  callback/thread/reentry, Error/Fault, target/profile/capability, schema,
  diagnostics, sanitizer/fuzz, and cross-target behavior deferred until the
  required authorities are Accepted.

## Compatibility impact

- Accepted Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-local boundary evidence. No FFI syntax, ABI, target,
  ownership, diagnostic, dependency, protocol, or support claim is registered.

## Unresolved alternatives

Declaration grammar and view projection; ABI/version/layout/endianness and
calling convention; symbol/version/mangling; ownership/borrow/Resource/Managed
and allocator rules; pointer/span/nullability/encoding; callback lifetime,
thread/reentry/blocking/cancellation; Error/Fault/unwind; Capability/Profile/
Target; schema and migration; provenance/TCB; diagnostics, Unicode, security,
offline, sanitizer/fuzz, cross-target, differential, and public protocol rules
remain open under FFI-3601, FFI-3602, FFI-3603, FFI-3604, FFI-3605,
GAP-NATIVE-BACKEND-ABI-001, GAP-OWNERSHIP-MODEL-001,
GAP-OWNERSHIP-PUBLIC-LIFETIME-001, and missing RFC-N305/RFC-0007/RFC-0011
authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
