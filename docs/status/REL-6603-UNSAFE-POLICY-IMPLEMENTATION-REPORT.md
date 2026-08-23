# REL-6603-UNSAFE-POLICY Implementation Report

## Result

`cargo xtask security verify` now validates the implementation behind the
existing `Rust unsafe` matrix row, not only its prose. It checks that the root
workspace denies unsafe Rust and that every declared workspace member inherits
the workspace lint policy.

The command reports nine security surfaces and twenty-two checked members. The
parent `REL-6603` remains `BlockedSpec`; this result is not a vulnerability-free
claim or release security sign-off.

## Implementation

- `tools/xtask/src/security.rs` parses the root and member manifests, rejects
  missing or non-`deny` policy, and fails closed on unsafe paths, duplicate
  members, unreadable/malformed manifests, or missing lint inheritance.
- Unit tests bind the current member count and reject representative unsafe
  member paths.
- `tools/xtask/src/main.rs` reports the checked member count.
- `docs/testing/SECURITY-AUDIT.md` distinguishes manifest drift prevention from
  compiler enforcement and from unavailable dependency/cross-target audits.

## Acceptance evidence

- Root policy: `workspace.lints.rust.unsafe_code = "deny"`.
- Member policy: all twenty-two current members set `lints.workspace = true`.
- The verifier accepts only explicit relative member paths consisting of normal
  components.
- Existing matrix classification remains three Covered variants, two Partial,
  and four Deferred.
- Focused xtask tests, the security command, full workspace tests, Clippy,
  governance, status, formatting, and offline gates are required before status
  completion is recorded.

## Compatibility and deferrals

No Ling behavior, diagnostic, schema, Semantic ID, package/lock format,
dependency, public command, editor protocol, runtime, or Unicode 17.0.0 behavior
changes. Dependency/generated-code/cross-target/native audits, threat modeling,
advisory/license/SBOM/provenance reports, independent review, and incident
response remain deferred under the parent task.
