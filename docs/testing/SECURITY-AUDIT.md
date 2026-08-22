# Ling Seed security audit

Status: Seed boundary audit (2026-08-22)

This document records security controls that are present in the repository and
the security surfaces that are not implemented. It is not a vulnerability-free
claim, a third-party penetration test, or a release security sign-off.

## Control matrix

| Required audit surface | Current control / evidence | State | Deferred work |
| --- | --- | --- | --- |
| Rust `unsafe` | Workspace lint sets `unsafe_code = "deny"`; repository source has no Rust `unsafe` block. | Covered for current crates | Re-audit every new dependency and target-specific crate. |
| FFI / Target Primitive TCB | No FFI or Target Primitive API is in the Seed workspace. | Deferred | Accepted FFI/TCB contract, isolation, ownership, ABI, and cross-target evidence. |
| Deserializers | Source/manifest/lock/Audit/semantic/bytecode readers have size, depth, schema/version, unknown-field, canonical-byte, and bounded-diagnostic checks. | Covered for implemented schemas | Add every future reader only with an Accepted schema, malformed corpus, and resource limits. |
| Package extraction / build sandbox | Project discovery rejects traversal, host-path forms, symlink escape/cycle/alias, and lock symlinks; package resolution is local and does not execute a build script. | Partial | Archive extraction policy, sandbox/capability model, quota, and hostile archive corpus. |
| Capability enforcement | Effect/type/VM tests reject missing capabilities before effects and preserve source-mapped bilingual faults. | Covered for Seed effects | Extend only with accepted capabilities and profile/target authority. |
| Remote protocol | Project graph resolution has no network or process execution surface. | Deferred | Accepted package/remote protocol, authentication, provenance, retry, and redaction rules. |
| Replay / evidence sensitive data | No replay recorder/player or evidence bundle reader exists. | Deferred | Accepted schemas, sensitivity labels, redaction, retention, keys, integrity, and offline verification. |
| Zed extension binary download / verification | No Zed binary download or extension updater is implemented; Tree-sitter assets are checked in. | Deferred | Accepted editor distribution, signature/checksum, trust root, update, and rollback policy. |
| Dependency / license / SBOM | Workspace and fuzz manifests pin versions through both lockfiles; workspace license is Apache-2.0. `cargo metadata --locked --offline` is reproducible. | Partial | Advisory database scan, transitive license review, generated SBOM/provenance, and release attestation. `cargo-audit` and `cargo-deny` are not installed in this environment. |

## Seed evidence

The following controls are executable and remain within the accepted Seed
surface:

- `cargo test -p ling-project --test discovery_fixtures --locked --offline`
  covers path traversal and symlink escape/cycle/alias rejection;
- `cargo test -p ling-project --test lockfile_fixtures --locked --offline`
  covers lock filename, symlink, corruption, canonicality, and transactional
  replacement behavior;
- `cargo test -p ling-project --test package_graph_fixtures --locked --offline`
  covers dependency path and graph isolation;
- `cargo test -p ling-effects --locked --offline` and
  `cargo test -p ling-vm --all-targets --locked --offline` cover capability,
  cancellation, host-failure, and resource-limit behavior;
- `cargo test --workspace --all-targets --locked --offline` exercises the
  bounded readers, diagnostics, Unicode 17.0.0 data, and deterministic
  canonical protocols together; and
- `cargo metadata --locked --offline --format-version 1` verifies the locked
  dependency graph without network access.
- `cargo clippy --workspace --all-targets --all-features --locked --offline
  -- -D warnings` passes; the HIR sequence enum keeps its intentional inline
  layout under a narrowly scoped lint rationale rather than changing public
  data shape.

These commands do not prove resistance to a vulnerability database entry,
malicious native code, a remote service, an archive bomb, a device, a replay
log, or an editor binary that the Seed implementation does not accept.

## Internal matrix drift check

The repository has one offline inventory check for this document:

```text
cargo xtask security verify
```

The check validates the exact nine audit-surface rows, their current
Covered/Partial/Deferred states, and the release-evidence guardrails. It only
protects the audit inventory from documentation drift; it does not run an
advisory scanner, inspect a remote service, or make a vulnerability-free claim.
The required release evidence remains a threat model and trust-boundary
inventory, accepted security decisions, deterministic hostile-input fixtures,
reproducible advisory, license, SBOM, checksum, and provenance reports, and an
incident/disclosure process. No security API is inferred from this check.

## Required release evidence

Before `REL-6603` can become a completed G6 security gate, the project needs:

1. a threat model and trust-boundary inventory for compiler, VM, package,
   cache, editor, FFI, device, remote, replay, and evidence surfaces;
2. accepted security decisions for capabilities, archive extraction/build
   sandboxing, remote authentication/provenance, replay/evidence privacy, and
   editor binary verification;
3. deterministic hostile-input, resource-limit, symlink/archive, crash,
   protocol, Unicode/span, cross-process, and cross-platform fixtures;
4. reproducible advisory, license, SBOM, checksum, and provenance reports
   generated from locked dependencies; and
5. an incident/disclosure process with ownership, response times, artifact
   retention, and compatibility impact.

Until those conditions exist, all absent surfaces remain explicit
Future/Unsupported states. No security feature is inferred from a crate name,
dependency metadata, or a passing unit test.
