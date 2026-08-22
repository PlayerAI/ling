# IDE-2309-REPAIR-INDEX implementation report

## Outcome

The bounded internal child `IDE-2309-REPAIR-INDEX` is implemented. It indexes
existing diagnostic codes, severities, optional primary spans, and structured
`Repair` payloads. The public `IDE-2309` code-action feature remains
`BlockedSpec`.

## Normative clauses covered

- Accepted DEC-0081 §§Decision 1–4 authorizes the immutable structured repair
  observation, message-text independence, read-only lookups, and deterministic
  ordering.
- DEC-0001 governs registered diagnostic codes and structured repair facts;
  DEC-0002 governs preservation of original UTF-8 diagnostic spans.
- DEC-0081 §5 excludes public action, edit, formatter, mutation, version, and
  protocol semantics from this child.

## Implementation

- `crates/ling-diagnostics/src/repair_index.rs` defines
  `DiagnosticRepairIndex` and `DiagnosticRepairObservation`.
- `Diagnostic` and `Repair` expose read-only getters for their existing
  structured fields; localized messages are never parsed.
- Observations retain repair ordinals and sort by registered code, severity,
  source span, repair kind, semantic-change flag, canonical fact JSON, and
  ordinal. Code and kind lookups return only references into the immutable
  index.

## Verification

```text
cargo fmt --all
cargo test -p ling-diagnostics --all-targets --locked --offline
cargo clippy -p ling-diagnostics --all-targets --locked --offline -- -D warnings
```

The focused suite passes with 8 tests, including structured fact retention,
localized-message independence, repeated construction equality, deterministic
lookup behavior, source-span retention, and empty/no-repair input.

## Compatibility and determinism

This is an internal diagnostic observation only. It introduces no `FixPlan`,
action ID, applicability or capability policy, edit, formatter behavior,
position/version handling, snapshot or cancellation state, protocol field,
diagnostic allocation, Semantic ID, runtime, bytecode, VM, ABI, or Unicode
table change.

## Deferred work

Public action kinds and lifecycle, edit overlap/atomicity, positions and
versions, stale/rollback/cancellation/limits, formatter and mutation safety,
diagnostic-to-action policy, and protocol fixtures remain deferred to the
blocked `IDE-2309` parent and its accepted authorities.
