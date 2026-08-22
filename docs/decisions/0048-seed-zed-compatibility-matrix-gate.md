# DEC-0048: Seed Zed compatibility-matrix drift gate / Seed Zed 兼容矩阵漂移门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: editor-integration  
> Related authority/gap: `RFC-0001`, `RFC-0019`, `DEC-0018`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `ZED-6801-SEED` child. It does not
complete the G6 Zed compatibility gate or authorize a Zed extension, LSP
executable, public editor schema, Stable CST compatibility, binary download,
installer, or release support. The parent `ZED-6801` remains `BlockedSpec`
until those authorities and artifacts are Accepted.

## Question

The repository already records the current editor-only Tree-sitter evidence,
known limitations, unsupported LSP/Zed surfaces, locked Node/CLI metadata, and
an honest Windows cache-lock failure. Without a drift check, a compatibility
state or package marker could change while the editor boundary remains
unsupported in the support matrix. An inventory-only verifier can protect the
record without running npm, contacting a registry, or defining editor
behavior.

## Decision

1. `cargo xtask zed verify` is an internal governance command. It reads
   `docs/testing/ZED-COMPATIBILITY-MATRIX.md` and validates the exact ten
   compatibility surfaces and their recorded states.
2. The verifier checks non-empty evidence cells, policy language for
   `Not established`/`Unsupported`, Experimental/Preview protocol status,
   Unicode 17.0.0 and original UTF-8 byte-span boundaries, the Windows
   cache-lock limitation, and the explicit no-placeholder boundary. It also
   checks the locked package metadata, grammar scope/file type, README, and
   known-difference markers in five repository files. It fails closed with
   internal `GOV-ZED-MATRIX-*` messages.
3. The command validates documentation and package markers only. It does not
   run npm or Tree-sitter, download binaries, define LSP/Zed protocols,
   generate source, allocate public diagnostics, or promote editor evidence to
   Stable support.
4. The command is included in the governance-authority CI gate. A future
   compatibility promotion requires an Accepted editor protocol, executable
   extension/LSP artifacts, per-OS install and position fixtures, acquisition
   integrity evidence, and a release compatibility suite.

## Conformance plan

- Run `cargo xtask zed verify` offline and assert ten surfaces and five package
  evidence files.
- Mutate an isolated surface row/state, policy phrase, or package marker and
  verify the gate fails closed.
- Retain the documented Windows error-5 cache-lock attempt as environment
  evidence; do not report it as a passing npm/editor run.
- Run the existing locked compiler/editor differential and generated-Unicode
  checks without treating this inventory gate as semantic equivalence or
  Stable editor-support evidence.

## Compatibility impact

- Adds only an internal `cargo xtask` validator and CI preflight. Ling syntax,
  semantics, Checked Core, runtime, bytecode, diagnostics, schemas, Semantic
  IDs, dependencies, public protocols, and Unicode 17.0.0 behavior are
  unchanged.
- The Tree-sitter package remains editor-only implementation evidence. No Zed
  extension, LSP server, binary acquisition, download, installer, migration
  promise, or placeholder public API is added.

## Unresolved alternatives

Zed version support, LSP capability negotiation, editor position encoding,
extension packaging, per-OS artifacts, binary acquisition and signing,
consumer protocol stability, and editor migration policy remain governed by
the parent `ZED-6801` and later Accepted authorities.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
