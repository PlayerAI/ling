# REL-6603 Authority Audit

- Task: `REL-6603` — Security Audit
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:354-366`
- Release: G6
- Status: `BlockedSpec` for the G6 release gate; current Seed controls are
  recorded without claiming a release security sign-off.

## Decision

`REL-6603` is `BlockedSpec`. The checklist spans current compiler/package/VM
controls and future FFI, remote, replay/evidence, device, and editor binary
systems. The repository can audit and test the implemented boundaries, but no
accepted threat model, trust-boundary contract, sandbox policy, remote
authentication/provenance protocol, replay privacy schema, or binary trust root
exists for the future surfaces.

The audit therefore records facts: Rust unsafe code is denied, current readers
are bounded and canonical, path/symlink and capability checks exist, and lock
files are reproducible. It does not convert those facts into a vulnerability
absence claim or invent security APIs.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:354-366` is a non-normative checklist. It does
  not authorize an FFI ABI, archive sandbox, remote protocol, replay/evidence
  privacy schema, Zed binary verifier, SBOM format, or disclosure process.
- `docs/ROADMAP-1.0.md:540-554` requires a G6 security baseline and response
  process after G1--G5 exits. It is planning authority, not a threat model or
  accepted protocol.
- Accepted Seed decisions cover source spans, diagnostics, package/lock,
  cache, capabilities, bytecode, and VM boundaries only. The governance
  protocol inventory keeps FFI, remote, replay/evidence, device, and editor
  surfaces Future/Unsupported or Experimental.
- Workspace `Cargo.toml` denies Rust `unsafe_code`; the current lockfiles and
  offline builds provide dependency reproducibility but not advisory or license
  completeness.
- `AGENTS.md` requires deterministic/offline behavior, checked Typed Core
  execution, Unicode 17.0.0, original UTF-8 spans, bilingual registered
  diagnostics, and no placeholder public APIs.

## Evidence in this repository

`docs/testing/SECURITY-AUDIT.md` records all nine plan surfaces, current
controls, state, and release evidence. The executable Seed evidence includes
workspace unsafe denial, bounded readers, symlink/path defenses, lock
canonicality/transaction tests, capability enforcement, VM resource limits,
locked offline metadata, and a passing workspace Clippy `-D warnings` gate.

No `unsafe` Rust source, FFI implementation, remote transport, replay/evidence
decoder, device runtime, or Zed binary updater is present. `cargo-audit` and
`cargo-deny` are not available in the current environment, so no advisory or
transitive-license result is claimed; a generated SBOM is likewise not
invented.

## Required authority before G6 completion

Before promotion, an Accepted security package must define:

1. assets, actors, trust boundaries, capabilities, authority escalation, and
   threat assumptions for every supported profile/target;
2. native/FFI and package extraction/build sandbox rules, quotas, symlink and
   archive-bomb handling, and failure diagnostics;
3. remote authentication, provenance, replay/evidence sensitivity and
   redaction, integrity, retention, and offline verification;
4. editor binary signing/checksum/trust-root/update/rollback behavior;
5. deterministic hostile-input, resource, corruption, cross-process,
   cross-platform, Unicode 17.0.0, and original-span fixtures; and
6. advisory/license/SBOM/provenance generation plus incident/disclosure
   ownership and compatibility gates.

## Compatibility and deferred work

This audit changes no language semantics, Typed Core, diagnostic allocation,
schema, package/lock behavior, CLI, editor protocol, dependency version,
runtime, or public API. It preserves `ling`/`.ling`, original UTF-8 spans,
Unicode 17.0.0, deterministic ordering, and offline locked builds.

No FFI, sandbox, remote, replay, evidence, device, Zed updater, SBOM schema,
advisory claim, or placeholder security API is added. The G6 security gate
remains deferred until its threat model, authorities, and release evidence are
Accepted.
