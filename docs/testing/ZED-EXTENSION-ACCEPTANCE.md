# ZED-6803 Extension Acceptance Matrix

Status: current-evidence editor acceptance inventory (2026-08-23). This matrix separates the
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

The support matrix records LSP document features, Zed extension integration,
formatter editor transactions, and semantic mutation as unsupported. A
source-built Preview LSP lifecycle/full-text overlay, position negotiation,
source-position projection, and formatter CLI exist as narrower prerequisites;
none is a Zed feature or Stable editor contract. Seed features and
Semantic/Audit protocols remain Experimental/Preview rather than Stable.
All current source-position evidence remains on Unicode 17.0.0 and projects
from original UTF-8 spans rather than exposing normalized or host-path offsets.

## Acceptance matrix

| Acceptance area | Current evidence | State | Boundary / required next evidence |
| --- | --- | --- | --- |
| `.ling` recognition | `tree-sitter.json` declares `source.ling` and the `ling` file type; compiler sources use `.ling` | Covered for grammar-only parsing | No Zed language package or installed extension; add one under an Accepted editor decision |
| Tree-sitter highlights | ZQ-3201 query and fixtures cover reviewed Seed captures, ASCII/Chinese roles, combining identifiers, and emoji recovery | Covered for query fixtures | No Stable Zed theme/capture contract or packaged extension |
| Brackets | ZQ-3202 query and 20 positive/negative assertions cover four delimiter pairs, escapes, nested comments, and recovery | Covered for query fixtures | No Stable editor package contract |
| Indentation | ZQ-3203 query and fixtures cover relative layout, delimiters, pipelines, Chinese names, and recovery | Covered for query fixtures | Compiler owns layout validity; no formatter or width policy is implied |
| Outline / textobjects / runnables | No outline, textobject, runnable query, Zed task, or extension action exists | Future / Unsupported | Requires accepted editor behavior, query fixtures, and executable task integration |
| LSP diagnostics, hover, definition, references, rename, completion, code actions, format, semantic tokens | Preview lifecycle/full-text overlay and UTF-8/UTF-16/UTF-32 negotiation/projection are tested; formatter CLI and internal diagnostic/edit primitives exist, but none of the listed public feature methods is implemented | Partial prerequisites; listed features Unsupported | Requires accepted LSP/edit protocols, public adapters, capability advertisement, response/edit fixtures, and Zed integration |
| Task / run / test / Audit | File-mode `ling run`, `check`, `semantic`, and `audit` are tested; no Zed task/runnable integration or project test command exists | Partial | CLI protocols are Preview/Experimental; Zed task schema and project orchestration are future |
| Replay / evidence | Replay and evidence protocols are future/unsupported in the support matrix | Unsupported | Requires G2/G5 semantics, schemas, verifiers, and editor navigation fixtures |
| Chinese / emoji / CRLF / UTF-16 positions | Compiler/grammar evidence plus negotiated UTF-8/UTF-16/UTF-32 lifecycle tests and source projections cover Chinese, emoji, CRLF, BOM preservation, and surrogate-boundary negatives | Partial | No listed document feature consumes these positions and no Zed integration fixture exists |
| Language-server crash / restart | A bounded Preview initialize/shutdown/exit lifecycle exists; no crash injection, editor restart harness, process replacement, or recovered diagnostic fixture exists | Unsupported | Requires restart ownership, snapshot/version restoration, backoff/resource policy, and executable Zed recovery evidence |
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
cargo xtask zed-extension verify
```

The locked Windows `npm run verify --offline` suite passed on 2026-08-23 with
41 grammar cases, 18 Unicode cases, 29 precedence cases, 41 pattern/type cases,
10 recovery cases, 42 conformance programs, 18 highlight captures, 4 bracket
pairs, 15 indentation CST nodes, and example parsing. Regeneration left no
tracked worktree drift. This is local grammar/query evidence, not a Zed or
cross-host result.

The internal `cargo xtask zed-extension verify` command composes the current
Zed-matrix and discovery-boundary gates, checks ten historical/current evidence
files, and validates three position-evidence files. It does not run npm, start
an editor, create a Zed manifest, or claim an extension package.

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
