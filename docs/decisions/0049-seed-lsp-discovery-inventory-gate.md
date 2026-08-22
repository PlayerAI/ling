# DEC-0049: Seed language-server discovery inventory gate / Seed 语言服务器发现盘点门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: editor-integration  
> Related authority/gap: `RFC-0004`, `RFC-0001`, `DEC-0048`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `ZED-6802-SEED` child. It does not
complete language-server discovery/acquisition, authorize a Zed extension,
define an executable identity, create a setting or download URL, or promote
the Preview `ling lsp --stdio` lifecycle to an editor integration contract.
The parent `ZED-6802` remains `BlockedSpec` until those authorities and real
artifacts are Accepted.

## Question

The repository has a Preview LSP lifecycle but no accepted discovery or
acquisition contract. The G6 checklist names user override, PATH lookup,
official release download, and explicit failure/install guidance without
specifying a setting, executable, release provenance, or public error schema.
How can the repository protect this negative boundary without performing
network access, process discovery, installation, or public API design?

## Decision

1. `cargo xtask lsp verify` is an internal governance command. It reads
   `docs/testing/LSP-DISCOVERY-ACQUISITION.md` and validates exactly four
   priority sources with their recorded `Not established` or `Unavailable`
   states and non-empty evidence/authority cells.
2. The verifier checks the future-only security requirements for HTTPS,
   version selection, checksum/signature verification, atomic installation,
   no execution before verification, override precedence, offline behavior,
   redaction, bounded process handling, and the explicit no-placeholder and
   no-stale-name boundaries. It fails closed with internal
   `GOV-LSP-DISCOVERY-*` messages.
3. The command validates inventory text only. It does not search PATH, read a
   user setting, contact a registry, download or execute a binary, install a
   package, allocate a public diagnostic, or define an LSP/Zed protocol.
4. The command is included in the governance-authority CI gate. A future
   acquisition implementation requires an Accepted discovery/provenance
   decision, versioned protocol and diagnostic schemas, verified artifacts,
   offline/security fixtures, and per-platform evidence.

## Conformance plan

- Run `cargo xtask lsp verify` offline and assert four priority sources,
  two `Unavailable` states, and two `Not established` states.
- Mutate a source row, state, evidence/authority cell, security phrase, or
  legacy-name boundary and verify that the gate fails closed.
- Run `cargo xtask ci verify` and the existing locked governance, status, and
  traceability checks without treating this inventory as LSP execution or
  editor compatibility evidence.
- Repeat independent processes and verify that no network request, process,
  source, diagnostic, schema, protocol, cache, or system configuration is
  changed.

## Compatibility impact

- Adds only an internal `cargo xtask` validator, documentation evidence, and
  CI preflight. Ling syntax, semantics, Checked Core, runtime, bytecode,
  diagnostics, schemas, Semantic IDs, dependencies, public protocols, and
  Unicode 17.0.0 behavior are unchanged.
- The existing Preview lifecycle, Tree-sitter grammar, and `UNSUP-LSP-EDITOR`
  support record retain their current boundaries. No discovery setting,
  executable, release URL, checksum/signature record, installer, migration
  promise, or placeholder public API is added.

## Unresolved alternatives

Executable naming and version negotiation, setting and workspace precedence,
PATH and release selection, trust roots and signing formats, archive policy,
atomic installation, offline fallback, redacted bilingual diagnostics,
process/resource limits, Zed packaging, and editor migration remain governed
by the parent `ZED-6802` and later Accepted authorities.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
