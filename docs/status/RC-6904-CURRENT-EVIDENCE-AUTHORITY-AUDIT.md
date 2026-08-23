# RC-6904-CURRENT-EVIDENCE Authority Audit

- Parent: `RC-6904` — RC2 / Final Change Control
- Child: `RC-6904-CURRENT-EVIDENCE` — Current RC2 upstream/protocol evidence
- Release: G6
- Decision: `Done` is authorized only for this bounded internal child by
  Accepted `DEC-0248`; the parent remains `BlockedSpec`.

## Authority and drift

Accepted DEC-0055 protects six change-control evidence classes and prohibits
classifying ordinary work as an RC2 blocker fix. Its protocol statement later
drifted from 21 to 27 records, and the gate did not execute the current
RC3→RC1→RC0 bounded inventory chain.

DEC-0248 authorizes correcting and composing that evidence. Passing upstream
inventories do not complete their parents and do not authorize blocker status,
risk acceptance, a candidate, source freeze, or Final decision.

## Authorized implementation

- Compose the current RC3→RC1→RC0 bounded gate chain.
- Correct the protocol inventory total to 27.
- Require upstream-pass, predecessor-blocked, and current-protocol markers.
- Add focused fail-closed drift coverage while preserving all six states.

## Explicit exclusions

No source fix, blocker/P0/P1 disposition, regression baseline, risk approval,
impact manifest, matrix rerun, candidate/tag/artifact, reviewer approval,
Final/Go decision, network request, or system change.

No language semantic, diagnostic, schema, Semantic ID, dependency,
CLI/LSP/DAP/runtime, Unicode, protocol-state, support-state, or public API
change occurs.
