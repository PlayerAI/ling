# REL-6601 Authority Audit

- Task: `REL-6601` — Fuzz Coverage Inventory
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:319-338`
- Release: G6
- Status: `BlockedSpec` for the G6 release gate; the Seed inventory and
  available harnesses are recorded as preparatory evidence.

## Decision

The repository can and does maintain deterministic Seed-level fuzz harnesses
for source decoding, lexing, parsing/AST lowering, the compiler-owned Format
IR, Audit schema decoding, project manifests, project locks, and bytecode
decoding/verifying. Those harnesses are added or inventoried without changing
language semantics or public protocols.

`REL-6601` cannot be marked complete as a G6 release task yet. The plan asks
for continuous fuzz coverage across replay/evidence, FFI, device metadata,
LSP/DAP, archives, and editor integrations, while G6 itself is gated by the
G1--G5 exits. Several of those surfaces are Future or Unsupported and have no
accepted decoder, schema, or runtime implementation to fuzz. Claiming complete
coverage would overstate the support matrix and create a false release gate.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:319-338` is a non-normative checklist. It
  names input families and inventory fields but does not authorize missing
  protocols, decoders, fuzz APIs, or release claims.
- `docs/ROADMAP-1.0.md:500-545` places fuzzing in G6.5 and requires G1--G5
  exits before G6; it is planning authority, not an accepted replay, FFI,
  device, LSP/DAP, archive, or editor contract.
- The current `docs/governance/protocol-inventory.toml` and support matrix
  record Future, Preview, Experimental, Unavailable, and Unsupported states.
  They do not authorize a fuzz harness for a protocol that is not implemented.
- Accepted diagnostic, source-position, Audit Source, package/lock, and
  bytecode decisions authorize only their respective bounded surfaces. They
  do not create replay/evidence, archive, FFI, device, LSP/DAP, or editor
  protocol behavior.
- `AGENTS.md` requires deterministic/offline tests, original UTF-8 spans,
  Unicode 17.0.0, bilingual registered diagnostics, checked Typed Core input,
  and no placeholder public APIs. Test-only harnesses must preserve those
  boundaries.

## Evidence in this repository

The checked-in inventory at `docs/testing/FUZZ-COVERAGE.md` records each of the
ten planned entry-point families, current harnesses, corpus counts, dictionary
state, timeout, RSS limit, and crash-triage ownership. The available targets
are:

1. `source_bytes`, `lexer_utf8`, and `parser_utf8` for UTF-8 source, lexical,
   layout, CST, and valid-CST AST paths;
2. `formatter_utf8` for deterministic compiler-CST Format IR projection and
   formatter disposition;
3. `audit_schema_bytes` for the Audit decoder and bounded bilingual
   diagnostic JSON;
4. `manifest_bytes` and `lock_bytes` for project manifest/lock readers; and
5. `bytecode_bytes` for the bounded decoder and verifier.

`cargo check --manifest-path fuzz/Cargo.toml --bins --locked --offline` is the
portable local evidence. The pinned Ubuntu CI job executes each corpus with
`-runs=256`, `-timeout=120`, and `-rss_limit_mb=2048`. The MSVC host note makes
clear that local compilation is not a Windows sanitizer execution result.

No harness is fabricated for archive, replay/evidence, FFI metadata, device
metadata, LSP/DAP, or Zed protocol behavior. Tree-sitter corpus scripts remain
deterministic differential fixtures rather than being mislabeled as fuzzing.

## Required authority before G6 completion

Before this task can be promoted from preparatory evidence to a completed G6
reliability gate, the repository needs:

1. G1--G5 exit evidence and a release-level accepted support inventory;
2. an accepted decoder/schema and implementation owner for every remaining
   planned entry point, including archive, replay/evidence, FFI, device, and
   editor protocols;
3. corpus, dictionary, timeout, memory, crash-retention, triage, and
   cross-process/cross-platform policies for each harness;
4. positive, malformed, negative, deterministic, resource-limit, Unicode
   17.0.0, original-byte-span, and diagnostic fixtures for every authorized
   surface; and
5. offline CI evidence that preserves minimized crash inputs and fails on
   nondeterministic decoding or unbounded diagnostics.

## Compatibility and deferred work

This audit changes no language grammar, parser semantics, resolver, evaluator,
Typed Core, diagnostic allocation, schema version, package publication,
runtime, CLI, editor protocol, dependency, or public API. The new fuzz
targets are test-only consumers of existing APIs and preserve `ling`/`.ling`,
original UTF-8 spans, Unicode 17.0.0, deterministic ordering, and offline
locked builds.

The missing future-surface harnesses remain explicitly deferred. No placeholder
binary, protocol field, support claim, or release-completion assertion is
added.
