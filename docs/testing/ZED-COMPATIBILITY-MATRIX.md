# ZED-6801 Compatibility Matrix

Status: editor compatibility inventory (2026-08-22). This is preparatory
evidence for `ZED-6801`; it is not a Zed release or a Stable editor-support
claim.

## Authority and current boundary

The compiler specifications, accepted RFCs/decisions, conformance fixtures,
and `ling-syntax` are authoritative for Ling validity and semantics. The
Tree-sitter package is an editor-oriented, tolerant parser and cannot make
invalid Ling source valid. The repository implements the Preview
`ling lsp --stdio` lifecycle and Experimental document overlay, but it has no
Zed extension, document diagnostics/queries/edits, accepted semantic-mutation
protocol, or Stable editor-support contract.

Compiler diagnostics and source maps preserve original UTF-8 byte spans;
Tree-sitter CST/query positions remain editor-only projections and are not
Semantic IDs or a public editor protocol.

The G6 checklist asks for a matrix containing the Zed version, compiler/LSP
range, grammar revision, protocol/schema, operating system, binary acquisition
mode, and known limitations. Unknown values are recorded as `Not established`
instead of inferred from a development tool.

## Matrix

| Surface | Current evidence | Compatibility state |
| --- | --- | --- |
| Zed minimum/tested version | No Zed extension package or Zed CI job exists in this repository | Not established; no support claim |
| Ling compiler | CLI is `ling`, source extension is `.ling`, language version is `0.0.1-dev`; compiler conformance runs on the locked Rust workspace | Seed compiler evidence only; no Zed integration |
| LSP executable/version | `ling lsp --stdio`; `ling.lsp.lifecycle/0.1` Preview and `ling.lsp.overlay/0.2` Experimental fixtures; no released Zed language-server artifact | Preview lifecycle/overlay only; no Zed compatibility range |
| Tree-sitter grammar | `editors/tree-sitter-ling`, package `0.0.1-dev`, grammar metadata `source.ling`, `.ling` file type | Editor-only implementation; no Stable node compatibility |
| Grammar revision | Current tracked grammar snapshot; generated parser and Unicode ranges were regenerated without worktree drift before the local suite | Pinned for this evidence snapshot, not a public Zed release tag |
| Tree-sitter CLI / Node | `tree-sitter-cli@0.26.12` in `package-lock.json`; Node `>=20` in `package.json` | Locked development toolchain; no consumer guarantee |
| Protocol/schema | No Zed protocol; `ling.semantic/0.1` is Experimental and `ling.audit/0.1`/`0.2` are Preview; Tree-sitter CST/query output is editor-internal | No Stable editor schema |
| Operating systems | Locked offline grammar verification passed on Windows with Node 24.15.0/npm 12.0.2; Linux/macOS and any Zed extension were not executed here | Windows grammar suite verified locally; no Zed OS support matrix |
| Binary acquisition | The source-built `ling` CLI contains the Preview stdio lifecycle; no standalone language-server download, extension package, URL, checksum, signature, or installer exists | Source-built ling CLI only; no Zed acquisition contract |
| Known limitations | LSP lifecycle, overlay, workspace reload, and bounded formatting exist, but diagnostics/hover/definition/references/rename/completion/code actions/semantic tokens and extension metadata/marketplace packaging do not | Explicitly unavailable; grammar-only development surface |

## Existing grammar evidence

The local package contains the following evidence, all below the compiler's
semantic authority:

- 42 compiler conformance programs are parsed through the differential runner;
  valid and invalid policies are recorded in TS-3108.
- Unicode 17.0.0 identifier ranges are generated from repository-controlled
  data; compiler NFC, security, mixed-script, and original-span checks remain
  authoritative.
- ZQ-3201, ZQ-3202, and ZQ-3203 provide reviewed highlight, bracket, and
  indentation queries with ASCII/Chinese, combining-character, emoji-recovery,
  nested-comment, and layout fixtures.
- The package README and `KNOWN-DIFFERENCES.md` state that recovery nodes,
  captures, query output, and CST names are not Semantic IDs or language
  semantics.

Windows grammar suite passed on 2026-08-23 through
`npm run verify --offline` after granting the Tree-sitter process access to its
user-cache lock. The locked run regenerated without worktree drift and passed
41 grammar corpus cases, scanner/layout checks, 18 Unicode cases, 29 expression
precedence cases, 41 pattern/type cases, 10 recovery cases with 9 incremental
edits and 64 mutations, 42 compiler-conformance programs with 84 edits and 43
stable mappings, 18 highlight captures, 4 bracket pairs, 15 indentation CST
nodes, and the example parse. This is Windows grammar evidence, not a Zed or
cross-platform support result. The repository CI workflow currently checks
generated Unicode parity but does not run the full npm suite.

## Required evidence before ZED-6801 completion

Before claiming a Zed compatibility release, add an actual extension package
and accepted support decision that define:

1. the minimum and tested Zed versions, compiler/LSP version range, and grammar
   revision/tag;
2. the LSP and editor protocol/schema versions, capability negotiation, and
   UTF-8/UTF-16/CRLF/emoji/Chinese position fixtures;
3. per-OS build and installation evidence, binary acquisition policy, and
   checksum/signature verification where downloads exist;
4. known limitations, offline behavior, crash/restart behavior, and migration
   policy; and
5. a release artifact, license/metadata record, and executable compatibility
   suite.

Until those authorities and artifacts exist, this matrix must retain the
Preview/Experimental LSP boundary and remain `Not established`/`Unsupported`
for actual Zed integration and document features. No placeholder command,
download, protocol, backend, schema, or editor promise may be inferred from
the Tree-sitter development package.
