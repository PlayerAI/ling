# PRJ-1108 project property and fuzz implementation report

> Status: **Done**
>
> Completed: 2026-08-21
>
> Final verified implementation commit: `29f9c4465b58c7eff23c227436563d69409b880e`
> Verified baseline: `main@29f9c4465b58c7eff23c227436563d69409b880e`

## Outcome

PRJ-1108 adds reproducible generated properties for the complete RFC-0002 project pipeline and promotes the existing manifest decoder fuzz target into the pinned offline CI smoke gate.

The property suite uses fixed seeds and a small dependency-free generator. It checks 128 module graphs against an independent cycle oracle, including exactly 64 cyclic and 64 acyclic cases; 64 additional acyclic projects are created in opposite filesystem orders and must produce equal package graphs, identities, canonical locks, and decoded lock models. A further 128 generated canonical logical paths are preserved exactly, while 1,152 noncanonical variants are rejected instead of rewritten.

The manifest fuzz target now decodes every byte input twice with distinct diagnostic source labels. Successful semantic models must compare equal; failures must retain the same stable code and original byte span before both diagnostics are rendered. Four reviewed corpus inputs cover minimal, Unicode, malformed-structure, and traversal cases. CI executes a bounded 256-run smoke with the pinned nightly and cargo-fuzz versions in offline mode.

## Normative clauses covered

- Accepted [`RFC-0002`](../RFC-0002.md) §1–§3: deterministic manifest decoding, source-label-independent semantic models, exact canonical logical paths, no path rewriting, and original UTF-8 error spans.
- RFC-0002 §4: module cycles are rejected, acyclic graphs publish successfully, and filesystem enumeration order is not semantic.
- RFC-0002 §5: equal logical inputs under different physical roots and creation orders produce equal package content and graph identities.
- RFC-0002 §6: graph projection to `ling.lock/1`, strict decode, model equality, graph matching, and canonical parse-render byte equality.
- RFC-0002 §7: arbitrary manifest inputs remain bounded, structured, diagnostic-bearing, and independent of a physical diagnostic source label except for the display file field.
- RFC-0002 conformance plan: cycle, normalization, enumeration-order, identity, canonical lock round-trip, malformed-input, and fuzz evidence.
- [`03-G1-V0.1-LIVING.md`](../ling_execution_plan/03-G1-V0.1-LIVING.md) PRJ-1108: all five requested manifest, graph-cycle, path, lock, and enumeration-order areas are covered.

## Implementation

- [`project_properties.rs`](../../crates/ling-project/tests/project_properties.rs) contains the fixed-seed generator, independent Kahn cycle oracle, temporary-project builder, canonical-path mutations, graph/identity comparisons, and lock round-trip assertions.
- The generator deliberately uses no host entropy, clock, locale, current directory, hash-map order, or new third-party property-test dependency. Every failing case is reproducible from the fixed task seed and case index.
- [`manifest_bytes.rs`](../../fuzz/fuzz_targets/manifest_bytes.rs) now checks semantic and failure determinism across distinct diagnostic source labels in addition to bounded decode and Diagnostic JSON rendering.
- `malformed` and `path-traversal` join the existing `minimal` and `unicode` manifest corpus seeds.
- [`.gitattributes`](../../.gitattributes) treats every fuzz corpus entry as an opaque byte sequence so checkout newline conversion cannot alter reviewed fuzz inputs.
- The separate [`fuzz/Cargo.lock`](../../fuzz/Cargo.lock) was refreshed offline to include the already-required `ling-project` graph/hash dependency closure; no package version was upgraded and no new direct dependency was introduced.
- [CI](../../.github/workflows/ci.yml) now runs `manifest_bytes` beside the Source, Lexer, and Parser/AST smoke targets with the existing pinned nightly, cargo-fuzz version, locked fetch, and offline execution policy.
- The CI contract, implementation guide, protocol inventory, and support matrix now require and cite the property/fuzz evidence.

## Specification gaps or conflicts

- No Accepted-specification conflict blocks PRJ-1108. RFC-0002 defines every equivalence relation and rejection rule asserted here.
- The plan says path “normalization,” while RFC-0002 requires NFC validation and exact canonical path spelling rather than silently rewriting user input. The property therefore preserves valid spelling exactly and rejects noncanonical variants; it does not invent a path-rewriting semantic.
- The engineering guide names `proptest` for later RFC §14.4 type-system properties, but PRJ-1108 does not mandate a framework. A bounded fixed-seed generator is sufficient for this project-only task, keeps normal builds offline, and avoids adding a dependency solely for three focused properties.
- The generated project test uses public production APIs and physical temporary roots. It does not expose a test-only graph constructor or add a placeholder public API.
- PRJ-1107 remains blocked because Accepted documents do not define project `test` discovery or `build` artifact behavior. PRJ-1108 does not infer those CLI semantics.

## Tests and verification

Executed locally on Windows on 2026-08-21 against implementation commit `29f9c4465b58c7eff23c227436563d69409b880e` and completion metadata derived from it:

- Test-first evidence: the new property file was added before any production or fuzz change; all three properties passed immediately, confirming existing production behavior and requiring no semantic repair.
- `cargo test -p ling-project --all-features --locked --offline` — 16 unit and 48 integration tests pass; one maintainer-only PRJ-1106 snapshot writer remains ignored.
- `cargo +1.85 test -p ling-project --test project_properties --locked --offline` — all three generated properties pass on the declared MSRV.
- `cargo check --manifest-path fuzz/Cargo.toml --bins --locked --offline` — all four fuzz targets build from the refreshed independent lock.
- The instrumented Windows `manifest_bytes` binary replayed all four reviewed seeds, then completed 256 deterministic mutation runs with seed `1108` and no crash. It was launched from the installed Visual Studio ASan runtime directory because that DLL directory is not on the host process search path.
- `cargo test --workspace --all-features --locked --offline` and the 92-test `xtask` suite pass.
- `cargo xtask governance check-all` reports 45 authority documents, 26 gaps, 20 lifecycle records, 20 protocols, and 76 diagnostic-code records.
- Schema validation reports five schemas, six valid fixtures, nineteen invalid fixtures, and three canonical byte fixtures; compatibility reports five `NoPreviousVersion` records; corrupt-input testing reports 36 mutations.
- Traceability reports seven features, 42 conformance fixtures, and 69 evidence records; support, CI-contract, Seed-reproduction, and pre-completion status gates pass.
- Execution-plan `SHA256SUMS.txt` verifies all 27 entries after the PRJ-1108 backlog transition; completion status verifies 29 tasks as Done.
- `cargo fmt --all -- --check`, full workspace Clippy with warnings denied, pinned Rust 1.85 workspace check, workspace documentation, release build, Unicode regeneration idempotence, and `git diff --check` pass.

No remote CI result, long-running fuzz campaign, minimized crash artifact, new public schema, new Ling CLI behavior, or non-Windows local result is claimed.

## Compatibility, determinism, and Unicode impact

- **Language behavior:** no production parser, project resolver, type/effect checker, evaluator, Checked Core, or runtime code changed.
- **Diagnostics and spans:** no code, severity, message, Fact, repair, or wire schema changed. The fuzz invariant only verifies existing code/span stability and bilingual JSON rendering.
- **Schemas and protocols:** no public schema or protocol version changed. Existing `ling.manifest/1`, package identity domains, and `ling.lock/1` gain property/fuzz evidence only.
- **Semantic IDs and canonical bytes:** existing Seed and package-aware Semantic IDs are unchanged. The generated tests compare existing package graph identities and canonical lock bytes without creating new vectors or formats.
- **CLI and evaluation:** no Ling command, option, exit code, output shape, evaluator path, or runtime behavior changed. Only the repository CI workflow gains a fuzz step.
- **Determinism:** fixed seeds, case indices, independent cycle classification, opposite creation order, distinct temporary roots, exact lock bytes, and opaque corpus checkout bytes remove host entropy and enumeration order from the evidence.
- **Unicode:** remains pinned to 17.0.0. Existing Unicode corpus and normalization/security behavior are exercised; generated Unicode tables remain byte-identical.
- **Dependencies:** no workspace dependency, direct fuzz dependency, or third-party version changed. The fuzz lock now accurately records dependencies already introduced by the committed `ling-project` implementation.

## Intentionally deferred work

- PRJ-1107: shared compiler-host project API and CLI integration after project `test`/`build` semantics are specified.
- Long-running scheduled fuzz campaigns, corpus minimization after discovered coverage improvements, sanitizer matrices beyond the existing Ubuntu CI job, and remote CI evidence.
- Registry, Git, URL, workspace, publication, installation, signatures, remote caches/services, arbitrary build scripts, item-level visibility, and multiple versions remain outside Accepted manifest version 1.
