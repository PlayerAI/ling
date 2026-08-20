# DEC-0018: Keep RFC-0001 as a Draft design baseline / 保留 RFC-0001 的 Draft 设计基线状态

> 状态：Accepted
> 提出日期：2026-08-21
> 决定日期：2026-08-21
> Owner role：language-governance
> 相关 RFC/缺口：RFC-0001 | GAP-GOV-RFC-STATUS-001
> 生命周期记录：`docs/governance/lifecycle.toml`

## Question

RFC-0001 declares itself `Draft`, while historical repository text associated its implemented Seed subset and the published `v0.0.1` tag with acceptance. The repository needs one auditable answer to whether the whole RFC is an Accepted implementation basis before post-Seed tasks can cite it.

The decision is intentionally limited to lifecycle authority. It does not reconsider an individual Seed rule already governed by an Accepted decision, and it does not accept any post-Seed feature described by RFC-0001.

## Decision

RFC-0001 remains `Draft`, is not a Stable implementation basis, and is not superseded. A release tag, implementation, conformance result, status report, or reference to an RFC acceptance section does not perform a lifecycle transition.

The implemented Seed subset continues to be described by the repository authority order and by the Accepted decisions that govern bounded questions. References to RFC-0001 MUST identify it as Draft whenever lifecycle status is material. They MAY cite it for provenance, design context, or a test inventory, but MUST NOT use it alone to authorize new language behavior or a public protocol.

Post-Seed work MUST use a dedicated Accepted RFC or Accepted decision for every semantic or public-protocol expansion required by `AGENTS.md`. An RFC derived from RFC-0001 does not inherit acceptance; it has its own `Open → Draft → Proposed → Accepted` record and compatibility review.

`GAP-GOV-RFC-STATUS-001` is resolved by this decision. Keeping RFC-0001 Draft is deliberate and no longer blocks a task that has a separate Accepted authority for all behavior it changes. A task that still depends only on RFC-0001 remains blocked.

## Conformance plan

- Keep RFC-0001 `Draft` and `stable_basis = false` in both authority and lifecycle registries.
- Keep all current references that discuss lifecycle explicit about the Draft status; a repository audit must find no claim that RFC-0001 itself is Accepted.
- Require every newly discovered RFC or decision to have matching source, authority, and lifecycle states, and retain negative checker coverage for Draft-as-Stable, invalid transitions, missing records, and unapproved legacy-format exemptions.
- Verify the gap registry resolves `GAP-GOV-RFC-STATUS-001` only through this Accepted decision and preserves the original discovery evidence.
- Verify that source, diagnostics, schemas, Semantic IDs, generated artifacts, and runtime outputs are byte-identical before and after this governance-only decision.

## Compatibility impact

- **Source and runtime:** none; no Ling program changes validity, typing, effects, evaluation, or output.
- **CLI and diagnostics:** none; no command, exit class, error code, message, Fact, Repair, or UTF-8 span changes.
- **Schemas, protocols, and Semantic IDs:** none; the decision changes specification authority metadata only.
- **Migration:** documentation that treated RFC-0001 as Accepted must say Draft. Implementations needing post-Seed authority migrate to a dedicated Accepted RFC or decision instead of relying on RFC-0001.
- **Determinism and Unicode:** no canonical data or Unicode table changes; Unicode remains 17.0.0.

## Unresolved alternatives

- Accepting RFC-0001 wholesale was rejected because it would freeze syntax, runtime, package, Trait, Effect, ownership, concurrency, heterogeneous-compute, and tooling proposals without their required focused compatibility evidence.
- Superseding RFC-0001 was rejected because it remains useful design and Seed provenance; no single Accepted replacement covers its full scope.
- Reopening acceptance remains possible only through a later reviewed lifecycle proposal that reconciles every still-Draft clause. Individual successor RFCs remain the preferred route.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
