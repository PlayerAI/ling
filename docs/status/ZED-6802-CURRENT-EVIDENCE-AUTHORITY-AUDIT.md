# ZED-6802-CURRENT-EVIDENCE Authority Audit

- Parent: `ZED-6802` — Language-server discovery and acquisition
- Child: `ZED-6802-CURRENT-EVIDENCE` — Current Preview server/discovery boundary
- Release: G6
- Decision: `Done` is authorized only for this bounded internal child by
  Accepted `DEC-0242`; the parent remains `BlockedSpec`.

## Authority and drift

Accepted RFC-0004 and the protocol inventory establish the Preview
`ling lsp --stdio` lifecycle. The discovery inventory subsequently preserved an
obsolete statement that no LSP binary was built and the compiler CLI was not
an LSP server. Accepted DEC-0049 protects the negative acquisition boundary,
but it does not require preserving a contradicted implementation claim.

DEC-0242 authorizes factual correction and current-evidence validation. A
source-built server entry point does not establish Zed discovery, distribution,
installation, or compatibility.

## Authorized implementation

- Correct the PATH row to `Not established` and distinguish server availability
  from discovery/acquisition availability.
- Bind the distinction to workspace, manifest, CLI dispatch, process test,
  implementation-crate, and protocol-inventory evidence.
- Parse TOML evidence structurally and fail closed when exact identities drift.
- Preserve the four-source priority model and all future security requirements.

## Explicit exclusions

No Zed extension, setting key, PATH search, standalone server artifact,
download URL, checksum/signature registry, installer, discovery process,
public diagnostic/schema, version policy, platform claim, or Stable support is
created.

No language semantic, source, diagnostic, schema, Semantic ID, dependency,
CLI/LSP runtime, Unicode, protocol, support-state, or public API change.
