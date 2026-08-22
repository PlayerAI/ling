# DEC-0184: Internal resource-budget diagnostic boundary evidence / 资源预算诊断边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0183` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `GAP-LSP-TRANSACTION-PROTOCOL-001` | `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` | `GAP-PROJECT-CLI-INTERFACE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`BND-5204-OBSERVATION`. It records provisional resource-budget diagnostic
vocabulary for facts, estimates, proof and provenance, schema/migration,
repairs and transactions, and offline fixtures while BND-5203, RFC-K504, and
the CLI/LSP/Semantic Transaction authorities remain unresolved.

本决定只授权 `BND-5204-OBSERVATION` 使用 test-local 的资源预算诊断边界清单；在 BND-5203、
RFC-K504 与 CLI/LSP/Semantic Transaction 等权威尚未解决时，只记录临时的 fact、estimate、proof、
provenance、schema/migration、repair/transaction 与 offline fixture 词汇。

## Question

BND-5204 proposes a diagnostic view with budget facts, usage, contributors,
path/provenance, assumptions, unknowns, and candidate transformations. Which
vocabulary can be retained as bounded evidence without choosing a budget fact
schema, diagnostic code meanings, proof status, or a semantics-preserving
transaction protocol?

## Decision

1. `crates/ling-types/tests/resource_budget_diagnostics_evidence.rs` keeps a
   test-local inventory of sixty provisional diagnostic facts, provenance and
   proof states, schema/transaction boundaries, repair obligations, and
   fixtures.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.resource-budget-diagnostics-observation/0`.
   These bytes are evidence only; they are not diagnostic facts, an error-code
   allocation, a schema, a code action, a repair, a transaction, a protocol,
   or a support claim.
3. No budget diagnostic code, schema field, `FixPlan`, Workspace Edit,
   Semantic Transaction, CLI/LSP route, dependency, protocol, support claim,
   or placeholder API is added. Public `BND-5204` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:184-195` is
  non-normative; it defines no field types, units, proof/estimate states,
  contributor ordering, provenance identity, assumption model, unknown
  behavior, or transformation transaction.
- `docs/ROADMAP-1.0.md:243-249` requires diagnostics to reuse the compiler's
  model and code actions to use versioned Workspace Edits or Semantic
  Transactions; it does not authorize this budget schema.
- `docs/status/BND-5204-AUTHORITY-AUDIT.md` records missing BND-5203 facts,
  RFC-K504, diagnostic-code allocation, and CLI/LSP/transaction authority.
- DEC-0001/DEC-0002 and the Preview `ling.diagnostic/0.1` container preserve
  existing code/Facts/Repair compatibility; they do not define resource-budget
  meanings or a transformation protocol.

## Conformance plan

- Assert all sixty resource-budget diagnostic boundaries and local order;
  compare forward/reverse opaque bytes; reject duplicates.
- Defer fact production, diagnostic codes/schema, proof and provenance,
  repairs, transactions, CLI/LSP, migration, and protocol behavior until
  accepted authority exists.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing Preview diagnostic Facts/Repairs and VM Runtime Faults are
not reinterpreted as resource-budget semantics; only test-local evidence is
added.

## Unresolved alternatives

Budget/usage/contributor field types and units; proof/estimate/assumption/
unknown/overflow/unsupported/target-mismatch states; target/compiler and
path/provenance identity; contributor ordering and diagnostic size limits;
bilingual code allocation, Facts, severity, localization, schema version and
migration; Preview container compatibility; Repair versus transformation;
Workspace Edit, Semantic Transaction, snapshot/version, stale-result,
cancellation, confirmation, rollback, ownership/effect/resource/source-map
preservation; checked Typed Core and Critical/BND dependencies; positive,
negative, boundary, unknown-assumption, provenance, localization,
target-migration, transaction, repair-equivalence, Unicode, determinism and
differential fixtures; protocol inventory and public status remain open under
BND-5204, BND-5203, GAP-CRITICAL-PROFILE-001,
GAP-LSP-TRANSACTION-PROTOCOL-001, GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001,
GAP-PROJECT-CLI-INTERFACE-001, and missing RFC-K504 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
