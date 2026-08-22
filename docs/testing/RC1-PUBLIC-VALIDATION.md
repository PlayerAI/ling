# RC-6902 RC1 Public Validation

Status: `BlockedSpec` (2026-08-22). This is a public-validation readiness
matrix, not a public release, download page, installation protocol, or
compatibility promise.

## Boundary

The G6 RC1 checklist requires multi-platform artifacts, checksums/SBOM/
provenance, clean installation, an offline locked build, a Zed extension,
sample projects, migration tests, issue templates, and a rule that schema
changes require a new release candidate. These checks are downstream of the
RC0 and G1-G5 exits. Existing Seed evidence is recorded as Seed evidence only;
it is not promoted to RC1 or Stable 1.0 support.

## Public-validation matrix

| RC1 criterion | Current evidence | State | Required before RC1 |
| --- | --- | --- | --- |
| Multi-platform artifacts | The support report records Linux, macOS, and Windows Tier2 build/test hosts, all with `release_artifacts = false`; no v1.0 artifact set exists. | BlockedSpec | Per-host artifacts, target identity, reproducible build inputs, and a candidate manifest. |
| Checksums, SBOM, provenance | The security audit records no completed advisory/license scan, SBOM, provenance format, or signing root. | BlockedSpec | Versioned checksum/signature, SBOM, license, provenance, and verification fixtures. |
| Clean install | `UNSUP-PACKAGES` keeps publication, registry installation, and CLI project installation outside the supported surface; no installer or package archive exists. | Unsupported | Accepted acquisition/install contract, clean-root fixture, rollback, and failure diagnostics. |
| Offline locked build | The v0.0.1 Seed report and current locked offline gates provide compiler evidence; they do not build a public RC1 artifact from a candidate manifest. | Partial Seed evidence | Candidate-tag locked build, dependency/cache policy, artifact verification, and repeatable clean environment. |
| Zed extension | ZED-6801 through ZED-6804 document grammar-only/editor evidence and no LSP executable, extension package, or debugger integration. | Unsupported | Accepted editor protocol, extension artifact, package metadata, install test, and compatibility suite. |
| Sample projects | Existing examples and project fixtures exercise the Seed subset, including bilingual tutorials; no 1.0 profile/support matrix admits them as release samples. | Partial Seed evidence | Versioned sample manifest, expected outputs/diagnostics, supported-host matrix, and clean/offline runs. |
| Migration test | COMPAT-6502 through COMPAT-6504 remain blocked; RFC-0002 covers only bounded manifest/lock evolution, not a general 1.0 source/protocol migration. | BlockedSpec | Accepted source/schema versions, migration tool/report, rollback, diagnostics, and positive/negative fixtures. |
| Issue templates | `.github/pull_request_template.md` exists, but no repository issue-template set or RC1 support/security/migration intake contract exists. | Partial repository hygiene | Versioned issue forms/templates, required reproduction/security fields, ownership, and disclosure routing. |
| Schema-change reset rule | Protocol inventory records current Experimental/Preview/Future states; no RC candidate identity or release-reset workflow is implemented. | BlockedSpec | Candidate manifest binding every schema/protocol change to a new candidate, migration impact, and revalidation record. |

## Verification boundary

These offline commands validate repository registries and Seed traceability;
they do not publish artifacts or establish RC1:

```text
cargo xtask rc1 verify
cargo run -p xtask --locked --offline -- status verify
cargo run -p xtask --locked --offline -- governance check-all
cargo run -p xtask --locked --offline -- support verify
cargo run -p xtask --locked --offline -- traceability verify --release v0.0.1
```

`cargo xtask rc1 verify` deterministically checks the exact nine criteria,
their `BlockedSpec`, `Unsupported`, and partial Seed/repository states, the
no-publication boundary, and eight linked audit-marker files. It is an
inventory check only; it does not publish, download, install, sign, migrate,
or otherwise establish public RC1 support.

No package registry, download URL, installer, Zed package, issue tracker,
signing service, network request, or system configuration was exercised. The
existing Windows Tree-sitter cache-lock limitation remains editor-tooling
evidence only and is not converted into an RC1 pass.

## Promotion rules

RC1 may leave `BlockedSpec` only after RC0 is complete, the G1-G5 exits are
closed, and every row has an Accepted authority, a deterministic fixture, a
candidate-bound artifact or report, and an owner for public support. A schema
or protocol change after candidate creation must produce a new candidate
identity and repeat all affected checks. No current Experimental, Preview,
Future, or Unsupported capability may be advertised as Stable through this
matrix.

No placeholder command, download, package, schema, protocol, signature,
migration promise, or stale legacy name is added here.
