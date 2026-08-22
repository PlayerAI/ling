# CTR-5407-OBSERVATION Authority Audit — Contract LSP/Zed Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0198` permits only test-local Contract editor vocabulary. It
does not authorize an LSP method, JSON schema, Contract projection,
counterexample/evidence link, rename transaction, position conversion,
diagnostic, Zed extension, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:375-382` is a
  non-normative Contract editor checklist.
- `docs/status/CTR-5407-AUTHORITY-AUDIT.md` records missing Contract/proof/
  evidence and LSP/Semantic Transaction authority.
- `GAP-LSP-TRANSACTION-PROTOCOL-001`, `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`,
  and `GAP-CRITICAL-PROFILE-001` remain open; `PROTO-EVIDENCE` is Future.

## Current implementation evidence

The observation adds one isolated test with sixty explicit Contract editor,
protocol, snapshot/transaction, position, data-validity, diagnostic, and
fixture boundaries. It sorts by explicit local rank, rejects duplicates,
compares canonical opaque bytes for forward/reverse input order, and uses an
observation-only tag. No LSP method, schema, Contract field, rename edit,
diagnostic, CLI/LSP action, dependency, Zed extension, or support claim is
introduced.

## Required authority and compatibility

Accepted authority must define versioned LSP capabilities and schemas,
checked Contract/Proof/Evidence source data, stable IDs/provenance/
invalidation, snapshot and Semantic Transaction rules, UTF-8 to negotiated
LSP positions, stale-edit rejection, redaction/privacy, bilingual
`L-<DOMAIN>-<NUMBER>` diagnostics, and offline client fixtures. Seed
behavior, Semantic IDs, UTF-8 spans, dependencies, and Unicode 17.0.0 remain
unchanged.

## Deferred work

CTR-5407 implementation, LSP methods, Contract projection, proof/evidence/
counterexample schemas, rename transactions, diagnostics, Zed integration,
protocols, and support claims remain deferred until accepted authority and
executable offline evidence exist. No placeholder editor API is created.
