# REL-6603-UNSAFE-POLICY Authority Audit

- Parent: `REL-6603` — Security Audit
- Child: `REL-6603-UNSAFE-POLICY` — Workspace unsafe-policy drift gate
- Release: G6
- Decision: `Done` is authorized only for this bounded internal child by
  Accepted `DEC-0236`; the parent remains `BlockedSpec`.

## Authority and current fact

The root workspace manifest declares `workspace.lints.rust.unsafe_code =
"deny"`, and all twenty-two current workspace member manifests set
`lints.workspace = true`. Accepted DEC-0043 already records Rust unsafe as
Covered for current crates, but its verifier previously checked only the
Markdown row and could not detect a new member opting out.

Accepted DEC-0236 authorizes a repository-aware drift gate over that existing
fact. It does not authorize new language behavior, an FFI boundary, a public
security feature, or a G6 security conclusion.

## Authorized implementation

- Parse the checked-in root and member Cargo manifests using the already pinned
  TOML dependency of `xtask`.
- Require the exact root deny policy and inheritance by every explicit member.
- Reject unsafe, wildcard, duplicate, unreadable, or malformed member entries.
- Report internal `GOV-SECURITY-*` validation failures and the verified member
  count.

## Explicit exclusions

The gate does not inspect transitive dependency implementation, proc-macro
expansion, generated source outside the declared workspace, target-specific
code that was not compiled, native libraries, FFI, advisories, licenses, SBOM,
provenance, or vulnerabilities. Compiler runs remain the enforcement evidence
for compiled Rust source.

No diagnostic registry entry, Semantic ID, schema, package format, CLI command,
runtime API, dependency version, Unicode table, or public protocol changes.
