# DEC-0060: Seed Effect row snapshot / Seed Effect Row 快照

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: effect-system-design  
> Related authority/gap: `DEC-0010`, `GAP-EFFECT-STATE-MASKING-001`, `GAP-EFFECT-HANDLER-001`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only a read-only in-process projection of the existing
v0.0.1 Seed closed Effect row. It is evidence for the Seed boundary and does
not decide the unresolved v0.2 Effect model.

## Question

The checked Seed pipeline already computes a deterministic closed `EffectRow`,
but tooling needs a path-free value containing the canonical identities without
depending on the private `BTreeSet` representation. What observation can be
published without choosing open rows, handlers, or State masking semantics?

## Decision

1. `EffectRow::seed_snapshot` returns an immutable
   `SeedEffectRowSnapshot` containing only the row's canonical effect names.
2. Snapshot names are deduplicated and sorted by canonical identity. The
   projection preserves `Pure` as an empty list and does not include display
   spelling, source paths, host state, allocation details, or Rust type names.
3. The snapshot is an in-process Rust value. It does not serialize a new
   schema, change Semantic IDs, alter capability checking, or enter evaluation,
   bytecode, VM, CLI, LSP, or protocol behavior.
4. The decision does not authorize `EffectId`, open/closed row variables,
   operation signatures, handlers, resume rules, State masking, Task,
   ActorSend, new diagnostics, or v0.2 support claims. Those remain blocked by
   the accepted authority required by `EFF-2101`.

## Conformance plan

- Verify a mixed Seed row produces deterministic canonical names independent of
  insertion order and display spelling.
- Verify duplicate labels remain one entry, pure rows produce an empty
  snapshot, and repeated snapshots compare equal.
- Verify the projection contains no paths, host state, allocator identity,
  future row-variable/handler fields, or new wire/schema behavior.

## Compatibility impact

- Adds only an in-process `ling-effects` value and accessor layered over
  DEC-0010's accepted Seed Effect/State boundary. Existing source syntax,
  diagnostics, Semantic IDs, Audit Source, schemas, CLI, runtime, bytecode,
  VM, protocols, and Unicode 17.0.0 behavior remain unchanged.
- No diagnostic allocation or protocol-inventory entry is required.

## Unresolved alternatives

Effect IDs, open and closed row variables, operation signatures, polymorphic
inference, handler matching/elimination, resume cardinality, State masking,
Task/Actor effects, v0.2 diagnostics, and migration remain governed by the
blocked `EFF-2101` task and the open effect gaps.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
