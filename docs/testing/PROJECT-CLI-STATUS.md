# Project CLI and API Current-Surface Inventory / 工程 CLI 与 API 当前表面盘点

## Scope

This inventory records the bounded project behavior that is executable today
and the public behavior that still lacks Accepted authority. The full
`PRJ-1107` task remains `BlockedSpec`. Passing this inventory does not turn the
graph-only command into semantic project checking and does not promote any
internal library boundary to a public compatibility promise.

本盘点记录当前可执行的有限工程能力，以及仍缺少 Accepted 权威的公开能力。
完整 `PRJ-1107` 任务继续保持 `BlockedSpec`；本盘点通过不等于公开工程接口完成。

## Current surface matrix

| Project surface | Current evidence | State | Authority / blocker |
| --- | --- | --- | --- |
| Explicit locked graph check CLI | `ling project check --manifest-path PATH --locked`, integration tests, and `ling.project.check/0.1` fixtures | Experimental | Accepted RFC-0024 |
| Read-only locked project snapshot | `LockedProject` and `load_locked_project` with repeatability and nonmutation tests | Internal | Accepted DEC-0058 |
| Locked project semantic snapshot | `CompilerDb::project_semantic_snapshot(&LockedProject)` and cache/path-free tests | Internal | Accepted DEC-0083 |
| Public semantic project check | No public command consumes the checked project snapshot | BlockedSpec | `GAP-PROJECT-CLI-INTERFACE-001` |
| Project run | No accepted project entry, capability, or process contract | BlockedSpec | `GAP-PROJECT-CLI-INTERFACE-001` |
| Project test | No accepted discovery, isolation, result, or exit contract | BlockedSpec | `GAP-PROJECT-CLI-INTERFACE-001` |
| Project build and artifacts | No accepted artifact, target, layout, or reproducibility contract | BlockedSpec | `GAP-PROJECT-CLI-INTERFACE-001` |
| Workspace and member selection | Only an explicit `ling.toml` path is accepted; no discovery or member-selection contract exists | BlockedSpec | `GAP-PROJECT-CLI-INTERFACE-001` |

## Evidence contract

`cargo xtask project verify` validates the exact eight-row matrix, the source
and test markers for all three implemented slices, the implementation reports,
and the status relation: `PRJ-1107` is `BlockedSpec`, while
`PRJ-1107-CHECK`, `PRJ-1107-LOAD`, and `PRJ-1107-SEMANTIC-SNAPSHOT` are `Done`.
Evidence drift fails closed; the command does not execute a user project.

No project `run`, project `test`, project `build`, artifact,
workspace-selection, or implicit-discovery API is implemented by this
inventory. The verifier is deterministic, read-only, and offline. It performs
no network request, filesystem mutation, lock rewrite, compilation of user
input, code execution, artifact production, install, or system change.

No language semantics, diagnostic allocation, schema, Semantic ID, runtime,
bytecode, VM, ABI, or Unicode 17.0.0 behavior changes. Only `ling` and `.ling`
are valid public names.

## Deferred acceptance

The five `BlockedSpec` rows require an Accepted decision that defines their
observable CLI/API contracts and evidence. Until then, implementation must not
infer those contracts from the execution plan, internal Rust APIs, or the
graph-only Preview command.
