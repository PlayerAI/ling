# DOC-6703-SEMANTIC-EQUIVALENCE Implementation Report

## Result

The process-level example test now proves the Chinese-first and idiomatic
English tutorials have equal checked Semantic shapes after normalizing only
their user nominal type spelling.

The parent `DOC-6703` remains `BlockedSpec`. This is bounded Seed test evidence,
not a Stable localization or tutorial-release claim.

## Implementation

- `crates/ling-cli/tests/conformance.rs` collects the two tutorial Semantic
  Graphs already produced by the shared six-case execution loop.
- The private projection retains version/entry facts, module requirements,
  definition and node kind/type/effect/capability shapes, and reference-kind
  topology.
- Localized names, literals, source evidence, scripts, spans, ordering, and
  experimental IDs are intentionally excluded.
- Exactly one ASCII and one Chinese tutorial projection are required and must
  compare equal.
- Existing exact runtime outputs and localized definition-name witnesses remain
  independently asserted before the structural comparison.

## Acceptance evidence

- Both tutorials pass real `ling check`, `ling run`, and `ling semantic`
  processes from the shared manifest.
- The graphs agree on `ling.semantic/0.1`, language and Unicode versions, entry
  module, Capability requirements, four user-definition shapes, eighty node
  shapes, and eleven reference-kind edges after the authorized normalization.
- Runtime stdout remains respectively `存活\n` and `alive\n`; equivalence does
  not erase localized observable text.
- Correct-error conformance and deterministic Audit evidence remain in the
  full workspace gate.
- Focused and full offline repository gates are required before completion is
  recorded.

## Compatibility and deferrals

No Ling behavior or public contract changes. Stable localization/alias policy,
prose translation validation, public equivalence formats, future tutorials,
profile/target guidance, cross-host release samples, and G6 sign-off remain
deferred.
