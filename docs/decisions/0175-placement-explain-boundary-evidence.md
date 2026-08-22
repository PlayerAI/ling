# DEC-0175: Internal placement-explain boundary evidence / 内部 Placement Explain 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: compiler-quality  
> 相关规范/缺口：`DEC-0174` | `DEC-0173` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`PLC-4804-OBSERVATION`. It records provisional explain fields, transport,
privacy, replay/cache, diagnostics, and CLI-boundary vocabulary while
RFC-H405 and Placement/Device authorities remain unresolved.

本决定只授权 `PLC-4804-OBSERVATION` 使用 test-local 的 Placement Explain 边界清单；
在 RFC-H405 与 Placement/Device 权威尚未解决时，只记录临时 explain field、transport、
privacy、replay/cache、diagnostic 与 CLI boundary 词汇。

## Question

PLC-4804 lists candidate devices, rejection reasons, chosen device, transfers,
numeric mode, fallback, cache hit/miss, and record/replay identity. Which
vocabulary can be retained as bounded evidence without creating a command,
machine-readable schema, or public explain protocol?

## Decision

1. `crates/ling-types/tests/placement_explain_evidence.rs` keeps a test-local
   inventory of sixty provisional explain fields, identity/provenance,
   rejection/transfer/numeric/fallback/cache/replay, transport/lifecycle,
   privacy, CLI, fixture, diagnostic, and support-exclusion boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.placement-explain-observation/0`. These bytes are
   evidence only; they are not a CLI command, JSON schema, placement decision,
   cache/replay record, diagnostic, or support claim.
3. No `zero` command, `ling` explain route, schema, dependency, target,
   diagnostic, editor integration, or placeholder API is added. Public
   `PLC-4804` remains `BlockedSpec` and the accepted CLI remains `ling`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:480-493` is
  non-normative; its stale `zero explain placement` heading and field list do
  not define a public command, schema, ordering, privacy, or replay contract.
- `docs/ROADMAP-1.0.md:381-431` requires explicit and explainable Placement
  decisions but does not authorize a CLI or protocol.
- `docs/status/PLC-4804-AUTHORITY-AUDIT.md` records the missing RFC-H405,
  Placement/Device, runtime, cost, fallback, cache, replay, privacy, and
  diagnostic contracts; `DEC-0174` remains prerequisite test-local evidence.

## Conformance plan

- Assert all sixty explain boundaries and local order; compare forward and
  reverse opaque bytes; reject duplicates.
- Defer command/schema/transport behavior, decision and rejection rendering,
  privacy/redaction, cache/replay identity, diagnostics, and editor protocol
  until accepted authority exists.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Only test-local evidence is added; stale `zero` text is not copied
into implementation, fixtures, schemas, or public surfaces.

## Unresolved alternatives

Candidate/rejection/chosen identity and ordering; transfers and numeric mode;
fallback and cache hit/miss; record/replay/decision identity; Semantic IDs,
spans, provenance and profiles; versioned protocol and JSON transport; stable
versus diagnostic-only fields; bilingual rendering, exit/unknown-field/
migration behavior; privacy and host/path/address/timestamp/allocation/
driver/debug/solver exclusions; topology/capability/policy/cost/fallback/
privacy/migration/replay/explain/differential/Unicode/determinism fixtures;
diagnostics, public CLI/protocol lifecycle, and support remain open under
PLC-4804, PLC-4803, PLC-4802, PLC-4801, GAP-KERNEL-DEVICE-001,
GAP-NATIVE-BACKEND-ABI-001, and missing RFC-H405 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
