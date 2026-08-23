# Project CLI and API Current-Surface Inventory / 工程 CLI 与 API 当前表面盘点

## Scope

Accepted RFC-0025 and the PRJ-1107 implementation complete the v0.1 explicit
locked-project CLI surface. This inventory distinguishes the retained
RFC-0024 graph-only command from semantic project commands and from the two
internal library boundaries they compose.

Accepted RFC-0025 与 PRJ-1107 实现完成了 v0.1 显式锁定工程 CLI 表面。本盘点
区分 RFC-0024 图检查命令、语义工程命令以及它们复用的两个内部库边界。

The full `PRJ-1107` task is `Done` for its accepted v0.1 scope. Rich source
test declarations, multi-member workspaces, implicit discovery, output
replacement/default directories, non-semantic backends, and publication remain
future work; they do not reduce the completeness of RFC-0025's explicit scope.

## Current surface matrix

| Project surface | Current evidence | State | Authority / boundary |
| --- | --- | --- | --- |
| Explicit locked graph check CLI | `ling project check --manifest-path PATH --locked`, integration tests, and `ling.project.check/0.1` fixtures | Experimental | Accepted RFC-0024 |
| Read-only locked project snapshot | `LockedProject` and `load_locked_project` with repeatability and nonmutation tests | Internal | Accepted DEC-0058 |
| Locked project semantic snapshot | `CompilerDb::project_semantic_snapshot(&LockedProject)` and cache/path-free tests | Internal | Accepted DEC-0083 |
| Public semantic project check | `ling check --manifest-path PATH --locked --offline` checks the complete package-aware pipeline | Experimental | Accepted RFC-0025 §§1–4 |
| Project run | `ling run --manifest-path PATH --locked --offline` executes only the checked root entry and captures JSON-mode stdout | Experimental | Accepted RFC-0025 §5 |
| Project test | `ling test --manifest-path PATH --locked --offline` runs one isolated root-entry smoke test | Experimental | Accepted RFC-0025 §6 |
| Project build and artifacts | `ling build ... --profile explore --target semantic --output PATH` publishes canonical create-new `ling.project.artifact/0.1` bytes | Experimental | Accepted RFC-0025 §7 |
| Workspace and member selection | One exact `ling.toml` selects one root plus vendored dependencies; no ambient search or selectable dependency members | Experimental | Accepted RFC-0025 §2 |

## Evidence contract

`cargo xtask project verify` validates the exact eight-row matrix, the source,
integration tests, RFC, protocol evidence, implementation report, and task
state: `PRJ-1107`, `PRJ-1107-CHECK`, `PRJ-1107-LOAD`, and
`PRJ-1107-SEMANTIC-SNAPSHOT` are all `Done`.

The semantic commands reuse `load_locked_project` and
`CompilerDb::project_semantic_snapshot`; no package resolution or unchecked
compiler/evaluator path is duplicated. The checked semantic artifact is not
executable bytecode, native/Wasm output, a publication package, or a Stable
1.0 format. The complete v0.1 workspace rule is explicit single-root
selection; implicit discovery and multi-member manifests are not claimed.

The verifier is deterministic, read-only, and offline. Executing the verifier
does not run a user project, write a lock, publish an artifact, or access the
network. `L-IO-0005` is the only new diagnostic allocation. Existing language
semantics, source spans, package-aware Semantic IDs, bytecode, VM, ABI, and
Unicode 17.0.0 behavior remain unchanged. Only `ling` and `.ling` are valid
public names.

## Deferred evolution

Future source-level tests, test filtering/parallelism, workspace manifest
versions, member selectors, registry/network sources, lock update mode,
default or replacing artifact locations, caches, bytecode/native/Wasm
backends, Native/Critical profiles, publication, and Stable compatibility
require their own Accepted authority and fixtures.
