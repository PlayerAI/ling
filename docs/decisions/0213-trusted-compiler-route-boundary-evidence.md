# DEC-0213: Internal Trusted Compiler Route boundary evidence / 内部可信编译路线边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: critical-backend
> 相关规范/缺口：`DEC-0212` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-CRITICAL-PROFILE-001` | `PROTO-ABI`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`CBK-5901-OBSERVATION`. It records provisional route-alternative, Core/target,
IR/ABI, proof/trust, failure, fixture, protocol, and support vocabulary while
RFC-K508 and Native/Critical compiler-route semantics remain unresolved.

本决定只授权 `CBK-5901-OBSERVATION` 使用 test-local 的 route alternative、
Core/target、IR/ABI、proof/trust、failure、fixture、protocol 与 support
边界清单；在 RFC-K508 和 Native/Critical 编译路线语义尚未解决时，只记录临时
词汇，不选择路线，也不声明 Native 或 Critical 支持。

## Question

CBK-5901 proposes choosing among a verified restricted backend, translation
validation, proof-producing lowering, a controlled C subset bridge, and target
machine-code verification for a small Critical Core and limited target. Which
vocabulary can be retained as bounded evidence without selecting a route or
defining Native IR, ABI, proof, target, or support semantics?

## Decision

1. `crates/ling-types/tests/trusted_compiler_route_evidence.rs` keeps a
   test-local inventory of sixty provisional route, Core/target, IR/ABI,
   identity/build, equivalence/proof/trust, failure, fixture, protocol, and
   support boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.trusted-compiler-route-observation/0`. These bytes
   are observation evidence only; they are not a route selection, IR, ABI,
   lowering result, proof certificate, diagnostic, protocol, or support claim.
3. All five proposed route families remain distinct and unselected.
   `CheckedTypedCoreInput` and `DerivedVerifiedRepresentation` preserve the
   existing compiler boundary without authorizing a Native lowering.
4. `Unavailable`, `Unsupported`, and `SupportMatrix` preserve the current
   truthful support state. This decision does not change Native or Critical
   support-matrix entries.
5. No Native backend/IR, ABI/FFI dependency, target package, translation
   validator, proof producer/checker, C bridge, machine-code verifier, route
   selector, CLI/LSP route, diagnostic allocation, public protocol, support
   claim, or placeholder API is added. Public `CBK-5901` remains
   `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:585-597` is a
  non-normative alternatives list. It defines no selection criteria, target,
  trust model, equivalence, TCB, or compatibility consequences.
- `docs/status/CBK-5901-AUTHORITY-AUDIT.md` records the absent RFC-K508,
  Native/ABI/ownership/target/proof authority, diagnostics, and executable
  fixtures.
- `docs/IMPLEMENTATION.md` excludes Native Backend and Critical/proof tooling
  from the Seed target. `GAP-NATIVE-BACKEND-ABI-001` and
  `GAP-CRITICAL-PROFILE-001` remain Open; `PROTO-ABI` is Future.
- The support matrix marks `BACKEND-NATIVE` and Native/Critical profiles or
  targets Unsupported/Unavailable. Those records are constraints, not a route
  decision.
- Accepted RFC-0014 through RFC-0020 authorize the portable bytecode/VM path
  only. The bytecode verifier cannot be relabeled as a Native or Critical
  verifier.
- `DEC-0212` authorizes only test-local AI-provenance vocabulary and defines no
  compiler route, target, proof, or TCB semantics.

## Conformance plan

- Assert all sixty compiler-route categories and local order; compare forward/
  reverse opaque bytes; reject duplicates; retain all five route alternatives,
  checked-Typed-Core input, and support-matrix boundaries together.
- Defer route selection, Native IR/ABI/FFI, target, proof/equivalence/TCB,
  diagnostics, protocols, and support claims until Accepted authority and
  offline cross-target fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, support matrix, and Unicode
17.0.0 remain unchanged. Existing bytecode lowering/verifier behavior is not
reinterpreted as Native/Critical route evidence; only test-local boundary
evidence is added.

## Unresolved alternatives

Route selection and rejected alternatives; initial Critical Core and targets;
Native/backend-neutral IR validity; type/layout/calling convention; ownership/
resource/effect/capability, FFI, Fault/unwinding, thread/reentry, startup/
shutdown, and Target Primitive Package semantics; checked Typed Core versus a
versioned derived representation; target/toolchain/profile/build/artifact
identity; source/binary mapping; semantics-preserving equivalence; proof
obligations and certificates; independent checker, trust, TCB, assumptions,
and optimization boundaries; controlled-C and machine-code boundaries;
fail-closed unsupported/invalid/ABI/target/proof/certificate/verifier/bridge
failures; bilingual stable diagnostics and exits; differential, translation,
proof, reproducibility, Unicode 17.0.0, BOM/CRLF, source-span, ownership/FFI,
and cross-target fixtures; protocol inventory and truthful support remain open
under CBK-5901, CBK-5902, CBK-5903, RFC-K508,
GAP-NATIVE-BACKEND-ABI-001, GAP-OWNERSHIP-MODEL-001,
GAP-KERNEL-DEVICE-001, GAP-CRITICAL-PROFILE-001, PROTO-ABI, and missing
Native/Critical route authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
