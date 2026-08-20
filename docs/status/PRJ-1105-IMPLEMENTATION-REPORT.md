# PRJ-1105 canonical project-lock implementation report

> Status: **Done**
>
> Completed: 2026-08-21
>
> Final verified implementation commit: `9ff0fcca0c65b7e9e2fccf3c1df001b4737d3082`
> Verified baseline: `main@9ff0fcca0c65b7e9e2fccf3c1df001b4737d3082`

## Outcome

PRJ-1105 implements the Accepted RFC-0002 `ling.lock/1` protocol at the `ling-project` library boundary. A fully validated local `PackageGraph` can now be projected into an immutable, path-free lock model and encoded as exact compact UTF-8 JSON with canonical field and record order plus one trailing LF.

The reader accepts only the declared format and exact canonical bytes. It rejects malformed JSON, unknown or duplicate fields, invalid identities, duplicate package or dependency names, noncanonical ordering or presentation, missing roots, dangling dependency identities, cycles, unreachable packages, excessive depth/count/size, and incompatible versions with registered bilingual diagnostics.

`resolve_package_graph_with_lock` resolves and validates the complete local graph before reading or writing the lock. `Update` creates a missing lock or replaces a valid stale lock; `Locked` requires an existing byte-valid exact match and never rewrites a mismatch. Corrupt locks are never silently repaired. Version-1 resolution remains intrinsically offline and executes no process or build script.

## Normative clauses covered

- Accepted [`RFC-0002`](../RFC-0002.md) §4–§5: the lock consumes the complete deterministic local graph and its path-free `PackageIdentity` values without introducing another identity scheme.
- RFC-0002 §6: exact filename and format marker; compact JSON plus one LF; ascending object-key order; numeric package-version ordering; `(name, content)` dependency ordering; strict declared-version reading; exact graph comparison; update/locked behavior; and no path, display, target, credential, or environment fields.
- RFC-0002 §7: complete graph validation before publication or lock replacement, preservation of the previous lock on failure, registered bilingual diagnostics, stable structured Facts, bounded hostile-input handling, and no physical path in canonical output or Facts.
- RFC-0002 conformance plan: frozen canonical bytes, parse-render equality, changed-source mismatch, missing/stale/corrupt lock behavior, incompatible format, whitespace/key-order drift, duplicate/dangling records, uppercase digest text, cycles, truncation, resource ceilings, exact filename case, and no network/shell surface.
- [`03-G1-V0.1-LIVING.md`](../ling_execution_plan/03-G1-V0.1-LIVING.md) PRJ-1105: deterministic lock generation, byte-identical writing, locked/offline policy at the library boundary, stable mismatch errors, and corruption tests.

## Implementation

- [`lockfile.rs`](../../crates/ling-project/src/lockfile.rs) owns the validated `LockFile`, `LockedPackage`, and `LockedDependency` domain types; the strict reader; canonical writer; graph comparison; exact-case lock discovery; bounded I/O; and `Update`/`Locked` orchestration.
- The writer declares serialization structs in the RFC key order, sorts packages by `PackageIdentity` (`name`, numeric version tuple, content), sorts dependencies by `(name, content)`, emits compact JSON, and appends exactly one LF.
- The reader decodes into closed Serde models, validates all domain invariants and graph topology, then re-encodes and compares every input byte. This single canonicality check rejects alternate whitespace, escaping, key order, array order, and newline spelling without duplicating a second JSON canonicalizer.
- Lock persistence reserves an adjacent create-new temporary file, writes and synchronizes the complete bytes, closes it, and replaces `ling.lock` with a filesystem rename. A write or replacement error removes the temporary file and leaves any prior lock untouched.
- [`lockfile_fixtures.rs`](../../crates/ling-project/tests/lockfile_fixtures.rs) tests the public API against real multi-package project inputs and frozen lock bytes, including unchanged-lock no-rewrite behavior and transactional mismatch/update behavior.
- [`schemas/lock/1`](../../schemas/lock/1) registers the first public lock schema, valid/canonical corpus, and eleven negative fixtures. The schema gate invokes the production `ling-project` reader, so semantic reader violations cannot pass through structural JSON validation alone.
- Three monotonic diagnostic allocations distinguish lock I/O (`L-IO-0002`), invalid/corrupt bytes (`L-PROJECT-0018`), and a missing or stale lock in locked mode (`L-PROJECT-0019`).
- The implementation follows SRP by keeping lock policy in `ling-project`, DRY by reusing the validated graph/name/version/identity types, KISS by comparing one canonical domain model, and YAGNI by omitting CLI selection, registry services, migration placeholders, and future source kinds.

## Specification gaps or conflicts

- No unresolved Accepted-specification conflict blocks PRJ-1105. RFC-0002 fixes the filename, byte model, identity relationship, reader policy, and update/locked behavior precisely enough for implementation.
- The execution-plan bullet mentions `--locked` and `--offline`, while RFC-0002 explicitly assigns CLI inventory and selection changes to PRJ-1107. PRJ-1105 therefore exposes `LockMode::Locked` and preserves the version-1 offline invariant without adding unadvertised CLI flags.
- RFC-0002 requires atomic replacement after full validation but does not prescribe a temporary filename or expose it as protocol data. The implementation uses an adjacent private temporary file; its process-local sequence is neither semantic nor serialized.
- A remote package service does not exist in Accepted version 1. Cached/offline behavior is satisfied by local vendored dependencies and static no-network/no-process guards; no placeholder service API was added.

## Tests and verification

Executed locally on Windows on 2026-08-21 against implementation commit `9ff0fcca0c65b7e9e2fccf3c1df001b4737d3082` and the completion metadata derived from it:

- `cargo test -p ling-project --all-features --locked --offline` — 16 unit tests, 43 integration tests, and documentation tests pass; seven integration tests are lock-specific.
- The lock integration suite covers frozen writer bytes, exact parse-render round trips, malformed/noncanonical/incompatible inputs, update and locked modes, missing/corrupt/stale locks, unchanged-lock no-rewrite behavior, exact filename case, resource limits, prior-lock preservation, and static network/process exclusion.
- Schema validation reports five schemas, six valid fixtures, nineteen invalid fixtures, and three canonical byte fixtures; compatibility reports five explicit `NoPreviousVersion` records; corrupt-input testing reports 36 deterministic mutations.
- `cargo xtask governance check-all` reports 45 authority documents, 26 gaps, 20 lifecycle records, 20 protocols, and 76 total diagnostic-code records across 14 domains.
- `cargo xtask traceability verify --release v0.0.1` reports seven features, 42 conformance fixtures, and 69 evidence records.
- `cargo xtask support verify`, `ci verify`, `seed reproduce`, and pre-completion `status verify` pass.
- Execution-plan `SHA256SUMS.txt` verifies all 27 entries after the PRJ-1105 backlog transition.
- `cargo test --workspace --all-features --locked --offline`, full workspace Clippy with warnings denied, workspace check on the pinned 1.85 toolchain, documentation build, release build, formatting, Unicode regeneration idempotence, and `git diff --check` pass.

The first full workspace test exposed only three stale governance-count assertions after the intentional schema/protocol/diagnostic additions. Those exact expected counts were updated, the focused 92-test `xtask` suite passed, and the complete workspace gate then passed.

No remote CI result, CLI `--locked`/`--offline` behavior, registry or network-source behavior, crash-consistency guarantee beyond synchronized file contents plus same-directory rename, long-running fuzz/sanitizer campaign, or non-Windows local result is claimed.

## Compatibility, determinism, and Unicode impact

- **Diagnostics and spans:** adds `L-IO-0002`, `L-PROJECT-0018`, and `L-PROJECT-0019` with bilingual messages and registered typed Facts. Existing code meanings and `ling.diagnostic/0.1` fields are unchanged. Lock-local spans use logical `ling.lock` byte coordinates and never leak a physical path.
- **Schemas and protocols:** `PROTO-LOCKFILE` becomes Public/Experimental and implemented at `ling.lock/1`; `SCHEMA-LOCKFILE-JSON` is a first-version, current-only, strict canonical schema with no inferred predecessor or migration.
- **Semantic IDs and canonical bytes:** existing Seed and package-aware Semantic IDs, `ling.semantic/0.1`, `ling.semantic/0.2`, Audit, REPL, and Diagnostic bytes are unchanged. Lock bytes are a separate protocol projection of RFC-0002 package identities.
- **CLI and evaluation:** existing file-oriented CLI commands and exit behavior are unchanged. The lock model is data only; it is not evaluator input, and evaluation still consumes checked Typed Core.
- **Determinism:** canonical output depends only on the validated root identity, sorted package identities, and sorted dependency identities. Host paths, locators, directory order, hash-map order, timestamps, permissions, temporary names, environment state, and Rust debug output are excluded.
- **Unicode:** remains pinned to 17.0.0. Lock technical names, versions, format markers, and SHA-256 text are ASCII; no Unicode table or normalization behavior changed.
- **Dependencies:** no third-party version changed. `serde_json` was promoted from a `ling-project` dev dependency to a production dependency, and `xtask` gained an internal `ling-project` edge so schema fixtures exercise the production reader.

## Intentionally deferred work

- PRJ-1106: the complete named project fixture matrix with expected graph, diagnostic, and lock artifacts.
- PRJ-1107: shared compiler-host project loading, explicit manifest selection, CLI `check/run/test/build` integration, `--locked`, `--offline`, exit codes, JSON output, and file-mode migration evidence.
- PRJ-1108: property tests plus long-running fuzz/sanitizer campaigns for lock round trips, graph ordering, path normalization, cycles, and resource limits.
- Registry, Git, URL, parent/sibling/workspace dependencies, multiple versions of one package name, installation, publication, signatures, credentials, remote cache/service protocols, and arbitrary build scripts remain outside Accepted version 1.
