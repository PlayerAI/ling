# TRAIT-1308-CURRENT-EVIDENCE implementation report

## Result

The bounded internal Trait IDE current-surface gate is implemented under
Accepted DEC-0251. `cargo xtask trait-ide verify` composes the RFC-0022
projection and DEC-0059 lookup evidence while enforcing the six unresolved
public editor surfaces. Parent `TRAIT-1308` remains `BlockedSpec`.

Implementation commit: `bd1a88ced6dddb69e4708f0bc567cdee32963228`.

## Normative clauses covered

- Accepted RFC-0022 §§1–8 govern the optional Experimental
  `x-ling-trait-ide` projection, identities, original byte spans, ordering,
  validation, and observational-only boundary.
- Accepted DEC-0059 authorizes four read-only in-process identity lookups and
  no wire/editor behavior.
- Accepted DEC-0251 authorizes this evidence composition and preserves the
  unresolved editor/protocol boundary.

## Implementation and tests

- `docs/testing/TRAIT-IDE-STATUS.md` records exactly one Experimental, one
  Internal, and six `BlockedSpec` surfaces.
- `tools/xtask/src/trait_ide_status.rs` validates the exact matrix, seven
  evidence files, and three parent/child task states.
- Focused tests reject state drift, parent promotion, missing child evidence,
  and missing implementation markers.
- The command is wired into xtask usage and the always-on
  `governance-authority` CI contract.

## Compatibility, determinism, and Unicode

The gate is deterministic, read-only, path-independent, and offline. It adds
no language behavior, public diagnostic, core schema, Semantic ID, protocol,
dependency, CLI/LSP/DAP/runtime behavior, bytecode, VM, ABI, network behavior,
or Unicode 17.0.0 change. It neither evaluates source nor selects Traits.

## Intentionally deferred

Hover, navigation requests, completion, rename, diagnostics, repairs,
document positions/versions, Workspace Edits, Semantic Transactions,
cross-package editor fixtures, and Stable compatibility remain governed by the
blocked parent and registered gaps.
