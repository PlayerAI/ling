# ZED-6801-CURRENT-EVIDENCE Authority Audit

- Parent: `ZED-6801` — Zed Compatibility Matrix
- Child: `ZED-6801-CURRENT-EVIDENCE` — Current LSP/grammar/package evidence
- Release: G6
- Decision: `Done` is authorized only for this bounded internal child by
  Accepted `DEC-0241`; the parent remains `BlockedSpec`.

## Authority and drift

Accepted DEC-0048 protects the ten matrix rows and five evidence files. The
matrix subsequently drifted behind accepted implementation: RFC-0004 and
RFC-0023 now govern a Preview stdio lifecycle and Experimental overlay, while
the matrix still claimed no LSP crate, binary, or fixture. It also retained an
old repository hash and a cache-lock failure even though the locked suite can
run with appropriate cache access.

DEC-0241 authorizes factual correction and stronger internal evidence checks.
Neither the LSP subset nor a successful grammar suite establishes Zed support.

## Authorized implementation

- Correct LSP, OS, acquisition, limitation, and grammar-snapshot evidence.
- Record the actual locked Windows grammar-suite result and reviewed totals.
- Parse and validate the three JSON package metadata files structurally.
- Preserve unavailable Zed/document features and all Stable-support blockers.

## Explicit exclusions

No Zed extension, version range, binary distribution, document-feature claim,
cross-host evidence, marketplace artifact, migration promise, Stable grammar
node contract, or public metadata schema is created.

No language semantic, diagnostic, schema, Semantic ID, package, dependency,
CLI/LSP/editor behavior, runtime, Unicode, protocol, support, or public API
changes.
