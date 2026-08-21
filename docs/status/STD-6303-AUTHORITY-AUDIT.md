# STD-6303 Authority Audit

- Task: `STD-6303` — Unicode and Chinese-Programming Stability
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:195-206`
- Release: G6
- Status: `BlockedSpec`

## Decision

STD-6303 is `BlockedSpec`. The G6 checklist asks to freeze Unicode 17.0.0
tables, XID_Start/XID_Continue, NFC, confusable/bidi/hidden-character
diagnostics, Text/Scalar/Byte distinctions, Chinese package/module/symbol
behavior, formatter/LSP/Zed/CLI/Windows-path behavior, and the RFC process for
Unicode upgrades. The repository already implements and tests the core
Unicode-17 identifier boundary, but the checklist reaches across several
public or planned surfaces whose accepted contracts are still absent.

The current language rules pin Unicode 17.0.0 and require an independent
specification change for an upgrade. The open Unicode alias/localization gap
and the open formatter/LSP transaction gaps still leave source display,
identity, migration, and editor/CLI interoperability unresolved. Freezing all
of those surfaces from the plan would turn implementation evidence into an
unaccepted 1.0 protocol.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:195-206` is a non-normative stability
  checklist. It names the data and integration surfaces but does not define
  an upgrade RFC, localized alias grammar, protocol ownership, or migration
  behavior.
- `docs/SEMANTICS.md:76-160` fixes UTF-8 decoding, original byte spans,
  Unicode 17.0.0, XID_Start/XID_Continue, NFC equality, forbidden controls,
  script/confusable checks, and the identifier pipeline. It explicitly says a
  Unicode upgrade requires a migration report; it does not accept a future
  version or localized alias syntax.
- `docs/ROADMAP-1.0.md:69-78` repeats Unicode 17.0.0, source-span,
  bilingual-diagnostic, deterministic, and offline invariants, and requires an
  independent specification change for upgrades. It does not itself provide
  the upgrade RFC or stable editor/tool contracts.
- `GAP-UNICODE-ALIAS-SYNTAX-001` is Open, P1, and blocks TS-3104,
  IDE-2306, and FMT-1501. It requires RFC-0003 to define alias syntax,
  resolution identity, collision, serialization, and localized display.
- `GAP-AUTHOR-SOURCE-LOCALIZATION-001` is Open, P0, and blocks formatter and
  IDE work. It leaves equivalent localized Author Source views, formatting,
  identity, and migration unresolved.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` is Open and the support matrix marks
  LSP, Zed, formatter, and semantic mutation unsupported. Existing byte-span
  conversion evidence does not authorize a stable snapshot/edit protocol.
- `docs/governance/support-matrix.toml` records Unicode feature
  `FTR-SEED-0003` as Implemented but Experimental, with profile-specific
  identifier policies unimplemented. Its `UNSUP-LSP-EDITOR` entry explicitly
  excludes LSP, Zed, formatter, and semantic mutation because no accepted
  integration boundary exists.
- `crates/ling-unicode` and the generated tables are implementation evidence,
  not a public Unicode-version protocol. `editors/tree-sitter-ling` documents
  its parser/query as editor-only and keeps compiler validity, NFC, security,
  and diagnostics authoritative.
- Root `AGENTS.md` requires Unicode 17.0.0, preserved UTF-8 spans, stable
  bilingual diagnostics, deterministic/offline behavior, accepted authority
  before public protocols, and no stale `zero` surfaces.

## Evidence in this repository

`ling-unicode` pins Unicode 17.0.0 at compile time, checks the dependency
versions, exposes generated data checksums, implements XID/NFC/security
metadata, and tests bidi/default-ignorable/confusable behavior. Resolver and
conformance evidence covers Chinese identifiers, mixed-script diagnostics,
normalization, and original source spans. Tree-sitter and Zed query suites
provide bounded editor evidence while explicitly remaining non-semantic.

There is no accepted RFC-0003 or replacement defining localized aliases and
display identity, no stable formatter/LSP/semantic-edit protocol, no
profile-specific identifier policy, and no executable cross-tool migration
corpus for a Unicode version upgrade. Windows path coverage cannot promote
host paths into identifier or Semantic ID semantics.

## Required authority before implementation

An accepted Unicode and localization decision set must define, at minimum:

1. The exact Unicode data release, generated-table manifest/checksums,
   dependency version constraints, reproducible generation command, and an
   upgrade RFC/migration report that enumerates lexical, security, diagnostic,
   formatter, editor, and compatibility changes.
2. XID, NFC, scalar/byte/text boundaries, forbidden controls, bidi/hidden
   characters, confusable skeletons, mixed-script policy, and stable bilingual
   diagnostics with original UTF-8 byte spans and deterministic selection.
3. Chinese package/module/symbol and any localized-alias grammar, resolution
   identity, collision/shadowing, serialization, display, formatter, and
   migration rules; localized spelling must not silently change Semantic IDs.
4. Formatter, LSP, Zed, CLI, and Windows path handling contracts, including
   UTF-8/UTF-16 mapping, snapshot/version preconditions, path display versus
   identity, CRLF/BOM behavior, offline operation, and explicit unsupported
   states for absent integrations.
5. Offline positive, negative, security, normalization, confusable/bidi,
   Unicode-version, Chinese-name, package/module, formatter/editor/CLI,
   Windows-path, cross-process, and migration fixtures, plus generated-table
   and governance report drift checks.

## Compatibility and deferred work

This audit changes no Unicode data, dependency, generated table, lexer,
resolver, diagnostic, formatter, LSP/Zed integration, CLI, path policy,
Semantic ID, package/module rule, or public protocol. It preserves the pinned
Unicode 17.0.0 implementation, XID/NFC/security checks, original UTF-8 spans,
accepted Chinese identifiers, deterministic/offline behavior, and explicit
Experimental/Preview/Unsupported states.

It deliberately adds no Unicode upgrade, alias syntax, localized keyword
view, diagnostic, formatter, LSP/Zed/CLI feature, Windows-path normalization,
migration tool, dependency, public API, or placeholder, and introduces no
stale `zero` names. Future stabilization may proceed only after the Unicode
upgrade/localization and editor/tool authorities are Accepted and their
cross-surface evidence is executable. Implementations must continue to feed
normalized checked names into the existing resolver while retaining original
source byte spans and excluding host paths from Semantic identity.
