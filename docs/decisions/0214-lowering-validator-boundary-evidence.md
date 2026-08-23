# DEC-0214: Internal Lowering Validator boundary evidence / 内部 Lowering Validator 边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: critical-backend
> 相关规范/缺口：`DEC-0213` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-CRITICAL-PROFILE-001` | `PROTO-ABI` | `PROTO-EVIDENCE`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`CBK-5902-OBSERVATION`. It records provisional validation-boundary,
representation, semantic-check, identity, equivalence/trust, failure, and
fixture vocabulary while RFC-K508, a selected compiler route, and Native/
Critical lowering-validation semantics remain unresolved.

本决定只授权 `CBK-5902-OBSERVATION` 使用 test-local 的 validation boundary、
representation、semantic check、identity、equivalence/trust、failure 与
fixture 边界清单；在 RFC-K508、编译路线选择和 Native/Critical lowering
validation 语义尚未解决时，只记录临时词汇，不声明已实现 lowering validator。

## Question

CBK-5902 proposes checking type/layout, control flow, value mapping, Contract
preservation, memory/alias behavior, source/binary correspondence, and
unsupported-construct rejection for every lowering. Which vocabulary can be
retained as bounded evidence without defining the validated representations,
equivalence theorem, target, proof system, or public validator?

## Decision

1. `crates/ling-types/tests/lowering_validator_evidence.rs` keeps a test-local
   inventory of sixty provisional validation-boundary, representation, Core/
   target, semantic-check, identity/correspondence, equivalence/proof/trust,
   failure, diagnostic, and fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.lowering-validator-observation/0`. These bytes are
   observation evidence only; they are not IR, validation input/output,
   correspondence facts, proof certificates, diagnostics, protocols, or
   support claims.
3. Type and layout, control flow, value mapping, Contract preservation,
   memory/alias, source/binary correspondence, and unsupported-construct
   rejection remain distinct local categories. Their presence defines no
   equivalence or soundness rule and validates no lowering.
4. `CheckedTypedCoreInput` preserves the compiler boundary. Existing
   `ling-bytecode` lowering and verification retain their accepted VM-only
   scope and are not reclassified as Native/Critical validation.
5. No Native/backend-neutral IR, lowering or translation validator,
   correspondence schema, Contract or alias proof checker, backend/target
   dependency, CLI/LSP route, diagnostic allocation, public protocol, support
   claim, or placeholder API is added. Public `CBK-5902` remains
   `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:599-609` is a
  non-normative checklist. It defines no source/target representation,
  soundness theorem, proof obligation, accepted transformation, or checker/TCB
  boundary.
- `docs/status/CBK-5902-AUTHORITY-AUDIT.md` records the absent RFC-K508,
  selected route, Native IR/ABI/ownership/Contract/target semantics,
  diagnostics, and executable fixtures.
- `docs/IMPLEMENTATION.md` excludes Native Backend and proof tooling from the
  Seed target. Native/ownership/kernel/Critical gaps remain Open;
  `PROTO-ABI` and `PROTO-EVIDENCE` are Future.
- Accepted RFC-0014 and RFC-0019 authorize only checked Typed Core to portable
  bytecode and Interpreter–VM differential evidence. They do not define Native
  source/binary correspondence or target-code equivalence.
- `DEC-0213` records compiler-route vocabulary but deliberately selects no
  route and authorizes no lowering validator.

## Conformance plan

- Assert all sixty lowering-validator categories and local order; compare
  forward/reverse opaque bytes; reject duplicates; retain checked-Typed-Core,
  type/layout/control/value, Contract/memory/alias, source/binary, unsupported-
  construct, and protocol boundaries together.
- Defer validation implementation, representations, equivalence/soundness,
  proof/trust/TCB, diagnostics, protocols, and support until Accepted authority
  and offline cross-target fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, support matrix, and Unicode
17.0.0 remain unchanged. Existing AST and bytecode lowerings and the bytecode
verifier are not reinterpreted as a Native/Critical lowering validator; only
test-local boundary evidence is added.

## Unresolved alternatives

Validated route and exact source/target/composed boundaries; versioned Typed
Core/backend-neutral IR/Native IR/target-code representations; supported Core,
target and profile; type/layout/control-flow/value/evaluation-order/Effect/
Capability/Contract/memory/alias/ownership/Resource/Fault/thread/FFI/ABI
semantics; source/Semantic/artifact/target/toolchain identity; original UTF-8
span and source/binary mapping; equivalence and soundness theorem; obligations,
certificates, independent checker, trust, TCB, assumptions, optimizations and
resource bounds; fail-closed unsupported/invalid/type-layout/control/value/
preservation/alias/source-map/target/validator failures; bilingual stable
diagnostics and exits; positive, negative, malformed, unsupported, control,
Contract, alias, differential, cross-target, reproducibility, fuzz/property,
Unicode 17.0.0, BOM/CRLF, and source-span fixtures; protocol inventory and
truthful support remain open under CBK-5902, CBK-5901, CBK-5903, RFC-K508,
GAP-NATIVE-BACKEND-ABI-001, GAP-OWNERSHIP-MODEL-001,
GAP-CRITICAL-PROFILE-001, PROTO-ABI, PROTO-EVIDENCE, and missing validator
authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
