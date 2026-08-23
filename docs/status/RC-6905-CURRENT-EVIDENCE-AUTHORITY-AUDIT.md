# RC-6905-CURRENT-EVIDENCE Authority Audit

- Parent: `RC-6905` — v1.0 Release Artifacts
- Child: `RC-6905-CURRENT-EVIDENCE` — Current v1 upstream/LSP/protocol evidence
- Release: G6
- Decision: `Done` is authorized only for this bounded internal child by
  Accepted `DEC-0249`; the parent remains `BlockedSpec`.

## Authority and drift

Accepted DEC-0056 protects fourteen non-Stable publication rows and the
immutable-Seed/no-publication boundary. The inventory later drifted behind
Accepted DEC-0242 and the protocol registry: a source-built Preview LSP server
exists, and the inventory contains 27 rather than 21 protocols.

DEC-0249 authorizes correcting those facts and composing the current
RC2→RC3→RC1→RC0 bounded chain. None of that evidence is a signed/downloadable
LSP binary, acquisition contract, Stable protocol, artifact, or publication.

## Authorized implementation

- Compose the current predecessor inventory chain.
- Correct the LSP row while keeping distribution `Unsupported`.
- Correct the protocol total while keeping every protocol non-Stable.
- Require current LSP/protocol/upstream/parent-blocked markers and negative
  drift coverage, preserving all fourteen states.

## Explicit exclusions

No v1 candidate/tag, compiler/runtime bundle, checksum/signature, SBOM,
provenance, package, extension, LSP distribution, migration, Stable support,
security policy, evidence bundle, upload, network request, or system change.

No language semantic, diagnostic, schema, Semantic ID, dependency,
CLI/LSP/DAP/runtime, Unicode, protocol-state, support-state, or public API
change occurs.
