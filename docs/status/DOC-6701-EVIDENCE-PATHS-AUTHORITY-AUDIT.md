# DOC-6701-EVIDENCE-PATHS Authority Audit

- Parent: `DOC-6701` — Formal Documentation Set
- Child: `DOC-6701-EVIDENCE-PATHS` — Formal inventory evidence-path gate
- Release: G6
- Decision: `Done` is authorized only for this bounded internal child by
  Accepted `DEC-0238`; the parent remains `BlockedSpec`.

## Authority and gap

Accepted DEC-0045 protects twelve manual names, states, and anti-promotion
phrases. Before this child, the verifier ignored the Current source and
evidence column, so a missing file, wildcard, stale range, or abbreviated path
could remain in the inventory while the gate passed.

DEC-0238 authorizes exact repository-path validation. File existence is
evidence integrity only; it does not establish content completeness, normative
authority, implementation, support, or release readiness.

## Authorized implementation

- Extract backticked references under explicit repository roots from each
  formal-set evidence cell.
- Require at least one exact evidence path per row.
- Reject non-portable, wildcarded, traversing, empty-component, or missing
  paths.
- Replace shorthand ranges with current exact planning, governance, status,
  conformance, implementation, and manual sources.
- Report the total exact-path count alongside the unchanged manual-state
  inventory.

## Explicit exclusions

The gate does not parse Markdown links or anchors, assess prose accuracy,
compare bilingual sections, crawl external URLs, generate API docs, recursively
inventory directories, or promote Future/Unsupported manuals.

No language semantic, diagnostic, schema, Semantic ID, package, dependency,
CLI, editor, runtime, Unicode, protocol, support, or public API changes.
