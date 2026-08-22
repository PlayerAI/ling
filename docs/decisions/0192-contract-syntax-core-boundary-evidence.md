# DEC-0192: Internal Contract syntax/Core boundary evidence / Contract 语法与 Core 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0191` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `GAP-CONTRACT-PROOF-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`CTR-5401-OBSERVATION`. It records provisional Contract syntax and Core
vocabulary for claim forms, expressions, purity/effects, identity, proof and
runtime status, diagnostics, and fixtures while RFC-K503 and the dependent
Critical, proof, boundedness, effect, ownership, runtime, identity, and
evidence authorities remain unresolved.

本决定只授权 `CTR-5401-OBSERVATION` 使用 test-local 的 Contract 语法与 Core 边界清单；在 RFC-K503 及
Critical、proof、boundedness、effect、ownership、runtime、identity、evidence 等依赖权威尚未解决时，只记录
临时的 claim form、expression、purity/effect、identity、proof/runtime status、diagnostic 与 fixture 词汇。

## Question

CTR-5401 proposes `requires`, `ensures`, `invariant`, `assert`, and a
restricted recorded `assume`, with Contract expressions intended to be pure,
total, or explicitly effect-limited. Which vocabulary can be retained as
bounded evidence without choosing grammar, precedence, AST/Core mapping,
effect restrictions, obligation identity, proof-status lifecycle, or runtime
assertion semantics?

## Decision

1. `crates/ling-types/tests/contract_syntax_core_evidence.rs` keeps a
   test-local inventory of sixty provisional Contract claim, expression,
   purity/effect, identity, status/proof, runtime, diagnostic, and fixture
   boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.contract-syntax-core-observation/0`. These bytes
   are evidence only; they are not a Contract parser, AST/HIR/Core form,
   checker, proof status schema, runtime assertion, diagnostic, protocol, or
   support claim.
3. No Contract parser, AST/Core node, resolver rule, effect restriction,
   diagnostic allocation, CLI/LSP route, protocol, support claim, or
   placeholder API is added. Public `CTR-5401` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:304-316` is a
  non-normative checklist; it defines no grammar, precedence, expression
  logic, lowering, obligation identity, or observable failure contract.
- `docs/status/CTR-5401-AUTHORITY-AUDIT.md` records the missing RFC-K503,
  Contract grammar/Core, purity/effect, identity, status, proof/runtime,
  diagnostics, and evidence authority.
- `docs/SEMANTICS.md:1185-1238` is a Draft Contract sketch and explicitly
  reserves Contract proof in `:1914-1931`; `docs/LANGUAGE.md` likewise keeps
  Contract/Proof checking outside Seed.
- `GAP-CRITICAL-PROFILE-001` and the Contract/proof evidence gap remain open.
  Accepted Seed RFCs 0014–0020 and DEC-0012 do not define source Contract
  syntax, proof obligations, or status semantics.

## Conformance plan

- Assert all sixty Contract syntax/Core categories and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer parser/AST/Core, resolver, effect/purity checking, obligation/status
  schema, proof/runtime checking, diagnostics, CLI/LSP, and protocol behavior
  until accepted authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing verifier/compiler invariants and RFC-0019 “Contract”
wording are not reinterpreted as user Contract semantics; only test-local
evidence is added.

## Unresolved alternatives

Contract grammar/forms/contexts, expression types/precedence/logical and
short-circuit behavior, AST/HIR/Core mapping and malformed recovery;
purity/totality/effects/capabilities/allocation/termination and restricted
assume provenance; obligation/ContractId/DefinitionId identity, canonical
bytes, alpha-normalization and semantic diff; status lifecycle and trust
levels (Proved, RuntimeChecked, ModelChecked, Tested, Assumed, Unknown,
Failed, NotApplicable), timeout and migration; VC/solver/certificate/proof
checker and trusted assumptions; runtime-check order/isolation/Fault;
diagnostics/facts, profiles/optimization, ownership/Node/Task/Actor,
memory/timing, evidence bundles; positive/negative/malformed, Unicode,
migration, determinism and differential fixtures; protocol inventory and
public status remain open under CTR-5401, NODE-5307, RFC-K503,
GAP-CRITICAL-PROFILE-001, GAP-CONTRACT-PROOF-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
