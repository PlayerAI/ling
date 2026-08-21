# ZED-6801 Compatibility Matrix

Status: editor compatibility inventory (2026-08-22). This is preparatory
evidence for `ZED-6801`; it is not a Zed release or a Stable editor-support
claim.

## Authority and current boundary

The compiler specifications, accepted RFCs/decisions, conformance fixtures,
and `ling-syntax` are authoritative for Ling validity and semantics. The
Tree-sitter package is an editor-oriented, tolerant parser and cannot make
invalid Ling source valid. The support matrix currently records LSP, Zed
extension, formatter, and semantic mutation as unsupported because no
corresponding executable, extension package, or accepted edit protocol exists.

The G6 checklist asks for a matrix containing the Zed version, compiler/LSP
range, grammar revision, protocol/schema, operating system, binary acquisition
mode, and known limitations. Unknown values are recorded as `Not established`
instead of inferred from a development tool.

## Matrix

| Surface | Current evidence | Compatibility state |
| --- | --- | --- |
| Zed minimum/tested version | No Zed extension package or Zed CI job exists in this repository | Not established; no support claim |
| Ling compiler | CLI is `ling`, source extension is `.ling`, language version is `0.0.1-dev`; compiler conformance runs on the locked Rust workspace | Seed compiler evidence only; no Zed integration |
| LSP executable/version | No LSP crate, binary, JSON-RPC fixture, or release artifact | Unsupported; no version range |
| Tree-sitter grammar | `editors/tree-sitter-ling`, package `0.0.1-dev`, grammar metadata `source.ling`, `.ling` file type | Editor-only implementation; no Stable node compatibility |
| Grammar revision | Repository revision `a4377450d26374098d95a9bb38520d3e3552dfd7` at this audit; generated parser and Unicode ranges are committed | Pinned for this evidence snapshot, not a public Zed release tag |
| Tree-sitter CLI / Node | `tree-sitter-cli@0.26.12` in `package-lock.json`; Node `>=20` in `package.json` | Locked development toolchain; no consumer guarantee |
| Protocol/schema | No Zed protocol; `ling.semantic/0.1` is Experimental and `ling.audit/0.1` is Preview; Tree-sitter CST/query output is editor-internal | No Stable editor schema |
| Operating systems | Grammar package uses Node and a C compiler; no Zed/extension OS matrix is published | Not established; Windows local run was lock-permission blocked; Linux/macOS not executed here |
| Binary acquisition | No language-server binary, extension package, download URL, checksum, signature, or installer exists | Not applicable; no download or execution path |
| Known limitations | No LSP diagnostics/hover/definition/references/rename/completion/code actions/formatting/semantic tokens; no extension metadata/marketplace package | Explicitly unavailable; grammar-only development surface |

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

The local `npm run verify --offline` attempt on 2026-08-22 reached the locked
Tree-sitter command but failed before tests with Windows error 5 while opening
the user cache lock
`C:\Users\aijun\AppData\Local\tree-sitter\lock\ling-37ee06c7e4ef0571.lock`.
This is environment evidence only; it is not reported as a passing matrix
entry. The repository CI workflow currently checks generated Unicode parity but
does not run the editor package's full npm suite.

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

Until those authorities and artifacts exist, this matrix must remain
`Not established`/`Unsupported` for Zed and LSP rows. No placeholder command,
download, protocol, backend, schema, or editor promise may be inferred from
the Tree-sitter development package.
