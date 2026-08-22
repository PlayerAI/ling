# ZED-6802 Language-Server Discovery and Acquisition

Status: boundary inventory (2026-08-22). This document records what is and is
not implemented for language-server discovery and acquisition. It is not an
installer design, download manifest, or public command contract.

## Current state

The repository contains no dedicated discovery/acquisition implementation,
distributed language-server release artifact, Zed extension package, download
manifest, checksum/signature registry, or accepted discovery protocol.
`UNSUP-LSP-EDITOR` in the support matrix explicitly covers this absence. The
existing Preview `ling lsp --stdio` lifecycle is not an acquisition source,
and the local Tree-sitter grammar is an editor parser only, not a language
server.

Accordingly, all discovery/acquisition rows are `Unavailable` or `Not
established`. No setting key, PATH name, URL, protocol field, diagnostic code,
installer, or fallback executable is invented here.

## Required priority matrix

| Planned source | Current repository evidence | State | Required authority before implementation |
| --- | --- | --- | --- |
| User-configured executable | No accepted setting key or workspace configuration exists | Not established | Accepted Zed/LSP decision defining key, path validation, version check, and error schema |
| PATH lookup | No LSP binary is built or published; the compiler CLI is not an LSP server | Unavailable | Accepted executable identity/version contract and bounded process-start policy |
| Official release download | No official LSP release, platform manifest, URL, checksum, signature, or trust root exists | Unavailable | Release/provenance decision defining HTTPS, version selection, digest/signature verification, and supported targets |
| Explicit failure/install guidance | No public LSP error or installation protocol is registered | Not established | Bilingual registered diagnostics, exit/JSON schema, remediation text, and offline behavior |

The priority order is therefore documentation only. A future implementation must
stop at the first usable, verified source and must never silently fall back to
an unrelated executable or to the compiler CLI.

## Security and operational contract (future work)

Before any network or process implementation is authorized, an Accepted
decision must define all of the following:

- HTTPS-only transport, redirect and host policy, and bounded response size;
- explicit compiler/LSP version selection and compatibility with the grammar
  revision and language version;
- checksum and/or signature verification against a versioned trust root before
  extraction or execution;
- atomic installation into a user-owned location with path, symlink, archive,
  permission, and concurrent-install defenses;
- no arbitrary execution before verification, no shell interpolation, and
  bounded child-process arguments/environment;
- user override precedence, workspace trust rules, cache invalidation, and
  rollback after a failed install;
- offline behavior that uses only a verified local binary and reports a
  deterministic bilingual error when it is absent; and
- redacted diagnostics that preserve original UTF-8 spans where a source span
  exists and never expose credentials, host paths, or downloaded payloads.

These are requirements, not implemented behavior. They must not be represented
as a public API until a specification, schema, implementation, and fixtures
exist.

## Current verification evidence

The following checks establish only the negative boundary:

```text
cargo run -p xtask --locked --offline -- governance check-all
cargo run -p xtask --locked --offline -- status verify
cargo run -p xtask --locked --offline -- traceability verify --release v0.0.1
cargo xtask lsp verify
```

The internal `cargo xtask lsp verify` command checks this inventory only. The
Zed compatibility matrix records the local Tree-sitter package and the Windows
cache-lock limitation of its npm verification. No language-server discovery
test is claimed because there is no acquisition executable or protocol to
test.

## Completion gate

`ZED-6802` can leave `BlockedSpec` only after an Accepted discovery/acquisition
decision, a versioned protocol and diagnostic schema, a real LSP executable or
release artifact, positive/negative/malformed/offline/security fixtures,
per-platform installation evidence, and a Zed integration test prove the
priority and verification rules. Until then the correct user-facing behavior
is an explicit unavailable/installation message documented by the future
protocol, not a guessed fallback.

No stale legacy CLI/source name, placeholder command, download URL, checksum,
signature, binary, backend, or editor API is added by this inventory.
