# Ling formal documentation inventory

Status: Seed documentation inventory (2026-08-22)

This inventory is the evidence slice for `DOC-6701`. It separates documents
that describe an implemented/accepted Seed boundary from planning material for
future releases. A plan mention is never treated as a feature, protocol, or
support promise.

## Authority and maintenance rules

The repository's public language authority is, in order, accepted RFCs and
decisions, `docs/SEMANTICS.md`, `docs/LANGUAGE.md`, conformance fixtures, and
implementation. `docs/IMPLEMENTATION.md`, `docs/ROADMAP-1.0.md`, and the
`docs/ling_execution_plan/` package are engineering/planning references below
that semantic authority. `docs/governance/authority.toml`, lifecycle, gap,
protocol, support, diagnostic, schema, status, and traceability registries
must agree with the documents they index.

## Formal set

| Planned manual | Current source and evidence | State | Boundary / missing work |
| --- | --- | --- | --- |
| Language Reference | `docs/LANGUAGE.md`, `docs/SEMANTICS.md`, `docs/ERROR-CODES.md`, `tests/conformance/`, `docs/SEED-RELEASE-REPORT.md` | Seed | Covers v0.0.1; later syntax remains RFC-gated. |
| Semantics Reference | `docs/SEMANTICS.md`, accepted decisions, `docs/SEED-TRACEABILITY.md` | Seed | Typed Core, effects, capabilities, spans, Unicode 17.0.0, and deterministic identity are authoritative; future semantics are excluded. |
| CLI / Tooling | `README.md`, `docs/IMPLEMENTATION.md`, `docs/decisions/0003-m0-tooling.md`, `crates/ling-cli`, `tools/xtask` | Seed / Preview | Current `ling` file-mode and internal `xtask` commands are documented; broader command registry, project CLI, formatter/LSP, and stable output remain blocked. |
| Project / Package | `docs/RFC-0002.md`, package/lock implementation reports, protocol/support registries | Seed library slice | Manifest/lock and local graph behavior are documented; publication, registry, archive, remote, and package ecosystem guides remain future. |
| Effect / Capability | `docs/decisions/0010-state-and-capability-model.md`, `0011-seed-builtins.md`, `SEMANTICS.md`, effect/VM tests | Seed | Only accepted Seed effects/capabilities are described; profile/target and future capability manuals remain blocked. |
| Task / Actor / Replay | G2 planning files, `docs/status/TASK-*.md`, `REP-*.md`, protocol inventory | Future / Unsupported | No Task, Actor, replay recorder/player, or replay schema exists to document as implemented. |
| Native / Ownership / FFI | G3 planning files, `docs/status/OWN-*.md`, `FFI-*.md`, support matrix | Future / Unsupported | No Native backend, FFI ABI, ownership runtime, or target primitive contract is accepted. |
| Kernel / Device | G4 planning files, `docs/status/CPU-*.md`, `GPU-*.md`, `DIR-*.md`, `DBUF-*.md` | Future / Unsupported | No kernel/device IR, placement, ABI, or hardware support exists. |
| Critical / Node / Contract / Evidence | G5 planning files, `docs/status/NODE-*.md`, `CTR-*.md`, `PROOF-*.md`, `EVD-*.md` | Future / Unsupported | No Critical runtime, model checker, proof/evidence bundle, or stable schema exists. |
| LSP / Zed | `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`, `05-ZED-EXTENSION.md`, `editors/tree-sitter-ling/README.md`, differential corpora | Zed grammar only; LSP future | Tree-sitter correctness assets are documented; LSP server, editor transaction, binary acquisition, and stable compatibility are not implemented. |
| Migration / Compatibility | `docs/RFC-0002.md`, `docs/ROADMAP-1.0.md`, `docs/status/COMPAT-6501..6504-*`, `ERROR-CODES.md` | Partial Seed | Manifest/lock migration and diagnostic retirement are bounded; general source migration, 1.x compatibility, deprecation, and schema N-1 remain blocked. |
| Security / Disclosure | `docs/testing/SECURITY-AUDIT.md`, `docs/status/REL-6603-AUTHORITY-AUDIT.md`, `docs/DEPENDENCIES.md` | Seed audit only | Controls are documented; threat model, advisory/SBOM, provenance, disclosure, and response policy are not release authority. |

## Required form for a future manual

Every new stable manual must link its Accepted RFC/decision and normative
clauses, implementation symbols, positive and negative conformance fixtures,
diagnostic/schema/protocol IDs, compatibility and migration behavior,
profile/target limitations, deterministic/offline evidence, and release state.
It must state what is intentionally unsupported and must not copy stale legacy
CLI or source names into examples, schemas, or editor integration.

## Validation

The documentation inventory is checked together with the repository registries:

```text
cargo xtask governance check-all
cargo run -p xtask --locked --offline -- status verify
cargo xtask traceability verify --release v0.0.1
```

These gates validate links, lifecycle/status/protocol/schema/diagnostic drift,
and Seed traceability. They do not turn future manuals into implemented
features.
