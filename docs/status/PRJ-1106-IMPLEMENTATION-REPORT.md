# PRJ-1106 end-to-end project-fixture implementation report

> Status: **Done**
>
> Completed: 2026-08-21
>
> Final verified implementation commit: `0e9c5800411a6f1acd1441068e6ce2fd58f29816`
> Verified baseline: `main@0e9c5800411a6f1acd1441068e6ce2fd58f29816`

## Outcome

PRJ-1106 adds the seven exact end-to-end project fixture families required by the execution plan: `single-package`, `multi-module`, `path-dependency`, `cycle`, `visibility`, `offline-lock`, and `unicode-names`.

Every fixture has a checked `expect.toml` contract and exact `expected-diagnostics.json`. The five successful cases also freeze a path-free package-graph snapshot and canonical `ling.lock/1` bytes. The two negative cases explicitly declare graph and lock as absent and freeze the complete ordered bilingual Diagnostic JSON for a package cycle and an unexported dependency module.

The fixture harness copies each tree to a distinct temporary physical root, invokes only the production `ling-project` manifest/graph/lock APIs, and compares the resulting diagnostics, graph projection, and lock bytes. It never mutates checked-in source projects during normal tests. The checked-in offline case runs in `Locked` mode, and the path-dependency lock is byte-equal to the independently maintained schema golden.

## Normative clauses covered

- Accepted [`RFC-0002`](../RFC-0002.md) §1–§4: exact manifest selection, deterministic source discovery, multi-module imports, recursive local path dependencies, cycle rejection, direct dependency namespaces, exported-module visibility, and no ambient lookup.
- RFC-0002 §5: frozen content and graph identities exclude physical roots and locator spelling while retaining exact source-byte sensitivity.
- RFC-0002 §6: update and locked behavior, byte-exact `ling.lock/1`, graph/lock agreement, and successful local resolution without network access.
- RFC-0002 §7: complete graph validation before publication, exact logical diagnostic spans, bilingual stable diagnostics, and absent graph/lock results after failure.
- RFC-0002 conformance plan: single/multi-package behavior, path dependencies, graph cycles, visibility, locked offline resolution, Unicode modules/display text, path-free determinism, and canonical lock bytes.
- [`03-G1-V0.1-LIVING.md`](../ling_execution_plan/03-G1-V0.1-LIVING.md) PRJ-1106: all seven named directories exist and each carries expected diagnostic, graph, and lock evidence.

## Implementation

- [`project_fixtures.rs`](../../crates/ling-project/tests/project_fixtures.rs) defines the closed seven-case matrix, strict expectation decoder, temporary-root runner, deterministic graph projection, exact Diagnostic JSON projection, lock comparison, and an explicit ignored maintainer-only snapshot writer.
- [`tests/projects/README.md`](../../tests/projects/README.md) documents the fixture contract, normal verification command, snapshot-review workflow, and the boundary between test-only graph evidence and public protocols.
- `single-package` freezes the minimal manifest, one implicit `Main` module, a one-package graph, and its lock.
- `multi-module` freezes explicit module declarations, a local import edge with original byte span, exports, sources, graph identity, and lock.
- `path-dependency` freezes a valid exported dependency import, two package identities, one dependency edge, and the same canonical lock vector as `schemas/lock/1/canonical/basic.bin`.
- `cycle` freezes `L-PROJECT-0014`, canonical cycle witness `alpha -> beta -> alpha`, original manifest bytes `155..174`, and absence of graph/lock publication.
- `visibility` freezes `L-PROJECT-0017`, exact dependency/importer/module/package Facts, original source bytes `13..33`, and absence of graph/lock publication.
- `offline-lock` carries a checked-in canonical lock and resolves the complete vendored project in `Locked` mode without update, network, registry, cache, environment, or process behavior.
- `unicode-names` freezes a Chinese display name, Unicode module and logical path, a Unicode import span in original UTF-8 bytes, emoji-bearing source content identity, graph, and lock.
- [`.gitattributes`](../../.gitattributes) pins project fixture manifests, Ling sources, expectations, and graph snapshots to LF while treating lock artifacts as raw bytes. This prevents `core.autocrlf` from changing source identities or canonical locks on a fresh Windows checkout.
- Protocol and support registries now cite the integrated fixture matrix; no new public reader, schema, command, or semantic surface was created.

## Specification gaps or conflicts

- No unresolved Accepted-specification conflict blocks PRJ-1106. RFC-0002 already defines every language and project behavior exercised by the seven fixtures.
- The plan requests expected graph artifacts but does not define a new public package-graph JSON protocol. `expected-graph.json` is therefore deliberately test-only and documented as such; it does not masquerade as `ling.semantic/0.2` or create a compatibility promise.
- The plan requests expected graph and lock outcomes for failing fixtures. Their `expect.toml` contracts explicitly declare both as `absent` instead of inventing invalid placeholder graph/lock files.
- Git checkout newline conversion would contradict RFC-0002 source-byte identity and exact lock evidence. Repository attributes now preserve the fixture bytes across hosts without changing language newline semantics.
- PRJ-1107 names `check/run/test/build`, but Accepted documents do not yet define project `test` discovery or `build` artifact behavior. PRJ-1106 does not invent those CLI semantics. PRJ-1108 can proceed independently against the completed library interfaces.

## Tests and verification

Executed locally on Windows on 2026-08-21 against implementation commit `0e9c5800411a6f1acd1441068e6ce2fd58f29816` and completion metadata derived from it:

- Test-first evidence: the new contract test initially failed on the missing `tests/projects/single-package` directory before any project tree was added.
- `cargo test -p ling-project --test project_fixtures --locked --offline` — two verification tests pass; the explicit snapshot writer remains ignored.
- The snapshot writer was run once explicitly and passed for all seven cases; the normal test then reproduced all bytes without changes.
- `cargo test -p ling-project --all-features --locked --offline` — 16 unit tests and 45 integration tests pass; one maintainer-only snapshot writer is ignored.
- `cargo test --workspace --all-features --locked --offline` — all unit, integration, conformance, governance, and documentation tests pass.
- `cargo xtask governance check-all` reports 45 authority documents, 26 gaps, 20 lifecycle records, 20 protocols, and 76 diagnostic-code records.
- Schema validation reports five schemas, six valid fixtures, nineteen invalid fixtures, and three canonical byte fixtures; compatibility reports five `NoPreviousVersion` records; corrupt-input testing reports 36 mutations.
- Traceability reports seven features, 42 conformance fixtures, and 69 evidence records; support, CI-contract, Seed-reproduction, and pre-completion status gates pass.
- `cargo fmt --all -- --check`, full workspace Clippy with warnings denied, pinned Rust 1.85 workspace check, workspace documentation, release build, Unicode regeneration idempotence, and `git diff --check` pass.
- Execution-plan `SHA256SUMS.txt` verifies all 27 entries after the PRJ-1106 backlog transition.

No remote CI result, public package-graph JSON schema, CLI project execution, project test discovery, build artifact, registry/network behavior, long-running fuzz/sanitizer campaign, or non-Windows local result is claimed.

## Compatibility, determinism, and Unicode impact

- **Language behavior:** no parser, resolver, type/effect checker, evaluator, or Checked Core code changed. The fixtures exercise only already Accepted behavior.
- **Diagnostics and spans:** no code, meaning, severity, template, or payload schema changed. Exact existing `L-PROJECT-0014` and `L-PROJECT-0017` outputs are newly frozen, including bilingual messages and original UTF-8 spans.
- **Schemas and protocols:** no public schema/version changed. Test-only graph JSON has no protocol marker or public reader. Existing `ling.manifest/1`, package identity, and `ling.lock/1` registries only gain fixture evidence.
- **Semantic IDs and canonical bytes:** existing Seed/package-aware Semantic IDs and protocol bytes are unchanged. New package graph/content IDs and locks are fixture vectors produced under the existing RFC-0002 domains.
- **CLI and evaluation:** no command, option, exit code, output shape, evaluation path, or runtime behavior changed.
- **Determinism:** every case is reproduced from a different temporary root and compared byte-for-byte. LF/raw-byte Git attributes prevent host checkout policy from changing hashed sources or locks; physical paths, temporary names, directory order, and host separators never enter snapshots.
- **Unicode:** remains pinned to 17.0.0. The Unicode fixture exercises existing NFC/XID/security validation and original byte spans; no table changed, and generator output remained byte-identical.
- **Dependencies:** no Cargo dependency edge or third-party version changed.

## Intentionally deferred work

- PRJ-1107: a shared compiler-host project API and CLI integration after the project `test`/`build` command contract is specified; explicit manifest selection, locked/offline flags, exit mapping, JSON output, and migration evidence remain in that task.
- PRJ-1108: manifest decoder fuzzing, generated graph/cycle/path properties, lock round trips, and enumeration-order invariance.
- Registry, Git, URL, workspace, publication, installation, signatures, remote caches/services, arbitrary build scripts, item-level visibility, and multiple versions remain outside Accepted version 1.
