# REL-6604-ARTIFACT Authority Audit

- Parent: `REL-6604` — Performance Baseline
- Child: `REL-6604-ARTIFACT` — Performance-baseline artifact integrity gate
- Release: G6
- Decision: `Done` is authorized only for this bounded internal child by
  Accepted `DEC-0237`; the parent remains `BlockedSpec`.

## Authority and gap

INC-1410 already produced an eight-scenario, three-sample internal JSON
artifact under accepted query/invalidation authority. DEC-0044 protects the
twelve-row Markdown coverage matrix, but its verifier intentionally did not
read that artifact. Consequently malformed JSON, scenario drift, truncated
sample arrays, or changed observable query work could pass the matrix gate.

Accepted DEC-0237 authorizes structural and deterministic-work validation of
the existing artifact. It does not authorize elapsed-time comparisons,
thresholds, new measurements, artifact replacement, or a public schema.

## Authorized checks

- Exact internal schema, sample count, synthetic fixture size, and timed-region
  exclusion.
- Exact scenario order shared with the harness and strict unknown-field
  rejection.
- Sample-array cardinality and non-zero recorded durations without threshold
  comparison.
- Existing trace/miss/hit observations and completed-work invariants.
- Isolated negative evidence for schema and work-count drift.

## Explicit exclusions

No timing harness execution, current-host measurement, memory/IO observation,
statistical comparison, regression tolerance, hardware tier, cross-platform
claim, release threshold, or missing LSP/Native/Actor/Replay/device/Kernel/Zed
surface is authorized.

No Ling semantic, diagnostic, schema, Semantic ID, package/cache format,
dependency, CLI, runtime, editor protocol, Unicode, or public API changes.
