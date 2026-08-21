# ZED-6803 Extension Acceptance Matrix

Status: editor acceptance inventory (2026-08-22). This matrix separates the
implemented Tree-sitter development surface from unavailable Zed/LSP features.
It is not a claim that a Zed extension is packaged or ready for a marketplace.

## Authority and status vocabulary

Accepted Ling specifications, decisions, compiler conformance, and
`ling-syntax` remain authoritative. The Tree-sitter parser and queries are
editor aids only. Each row uses one of these states:

- `Covered`: executable repository evidence exists for the stated boundary;
- `Partial`: only a narrower editor/compiler slice exists;
- `Unsupported`: the surface is explicitly unavailable and must not be
  inferred from another component; or
- `Future`: an accepted specification, implementation, or release fixture is
  still required.

The support matrix currently records LSP, Zed extension, formatter, and
semantic mutation as unsupported. Seed features and Semantic/Audit protocols
remain Experimental/Preview rather than Stable.

## Acceptance matrix

| Acceptance area | Current evidence | State | Boundary / required next evidence |
| --- | --- | --- | --- |
| `.ling` recognition | `tree-sitter.json` declares `source.ling` and the `ling` file type; compiler sources use `.ling` | Covered for grammar-only parsing | No Zed language package or installed extension; add one under an Accepted editor decision |
| Tree-sitter highlights | ZQ-3201 query and fixtures cover reviewed Seed captures, ASCII/Chinese roles, combining identifiers, and emoji recovery | Covered for query fixtures | No Stable Zed theme/capture contract or packaged extension |
| Brackets | ZQ-3202 query and 20 positive/negative assertions cover four delimiter pairs, escapes, nested comments, and recovery | Covered for query fixtures | No Stable editor package contract |
| Indentation | ZQ-3203 query and fixtures cover relative layout, delimiters, pipelines, Chinese names, and recovery | Covered for query fixtures | Compiler owns layout validity; no formatter or width policy is implied |
| Outline / textobjects / runnables | No outline, textobject, runnable query, Zed task, or extension action exists | Future / Unsupported | Requires accepted editor behavior, query fixtures, and executable task integration |
| LSP diagnostics, hover, definition, references, rename, completion, code actions, format, semantic tokens | No LSP crate/executable, adapter, position fixture, formatter, or semantic-token protocol exists | Unsupported | Requires accepted LSP/edit protocol, UTF-8/UTF-16 mapping, implementation, and fixtures |
| Task / run / test / Audit | File-mode `ling run`, `check`, `semantic`, and `audit` are tested; no Zed task/runnable integration or project test command exists | Partial | CLI protocols are Preview/Experimental; Zed task schema and project orchestration are future |
| Replay / evidence | Replay and evidence protocols are future/unsupported in the support matrix | Unsupported | Requires G2/G5 semantics, schemas, verifiers, and editor navigation fixtures |
| Chinese / emoji / CRLF / UTF-16 positions | Compiler and grammar cover Chinese, emoji, CRLF, Unicode 17.0.0, and original UTF-8 spans; no LSP UTF-16 adapter exists | Partial | Add negotiated UTF-16 positions, surrogate-boundary negatives, and Zed integration fixtures |
| Language-server crash / restart | No language server exists | Unsupported | Requires bounded lifecycle, restart, snapshot/version, and diagnostic recovery evidence |
| Large file / workspace | Grammar runners bound individual parse/recovery inputs; no extension or workspace benchmark exists | Partial / Future | Define host limits, cancellation, memory/latency evidence, and reproducible workspaces |
| Extension license / metadata / repository | Local grammar metadata is Apache-2.0 and points to `PlayerAI/ling`; no Zed extension manifest/repository package exists | Partial | Add accepted package metadata, license review, provenance, and publication artifact |
| Development install / marketplace package | No Zed extension package or marketplace submission exists | Unsupported | Requires extension artifact, clean install, offline/cache behavior, and marketplace acceptance evidence |

## Existing evidence commands

The following commands verify the implemented grammar/compiler boundaries after
the exact locked dependencies are installed:

```text
cd editors/tree-sitter-ling
npm run verify --offline
npm run test:conformance --offline
npm run test:highlights --offline
npm run test:brackets --offline
npm run test:indents --offline
npm run parse:examples --offline

cd ../..
cargo test -p ling-cli --test conformance --locked --offline
cargo run -p xtask --locked --offline -- governance check-all
cargo run -p xtask --locked --offline -- traceability verify --release v0.0.1
```

The local Windows `npm run verify --offline` attempt on 2026-08-22 was blocked
by Tree-sitter cache-lock access (Windows error 5); it is recorded as a failed
local attempt, not a pass. The Rust/conformance and repository governance
commands remain the executable evidence used by this audit. Existing TS/ZQ
reports record prior grammar/query runs and their exact limitations.

## Completion requirements

ZED-6803 can become complete only after an Accepted editor/LSP contract and an
actual extension package provide executable evidence for every row that is
promoted beyond `Partial`/`Unsupported`: `.ling` activation, grammar queries,
LSP capability negotiation and positions, task/runnable behavior, crash/restart
recovery, large workspace limits, license/provenance, clean development
installation, marketplace packaging, and offline deterministic behavior.

No row may be promoted by copying a planning checklist, treating Tree-sitter
CST/query output as language semantics, or adding an unregistered command,
protocol, backend, schema, migration promise, or stale legacy name.
