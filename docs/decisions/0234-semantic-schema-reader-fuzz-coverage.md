# DEC-0234: Semantic-schema reader fuzz coverage / 语义 Schema Reader 模糊测试覆盖

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: compiler reliability
> 相关规范/缺口：`DEC-0012` | `RFC-0002` | `DEC-0041` | `REL-6601`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes a libFuzzer harness for the two already implemented,
isolated Semantic Graph JSON readers. It adds test coverage, not a schema,
reader policy, compatibility edge, or executable deserialization path.

本决定授权为两个已经实现、彼此隔离的 Semantic Graph JSON Reader 增加 libFuzzer
harness。它只增加测试覆盖，不新增 Schema、Reader 政策、兼容边或可执行反序列化路径。

## Question

Should the implemented `ling.semantic/0.1` and `ling.semantic/0.2` readers be
covered by the G6 schema-decoder fuzz inventory?

## Decision

1. Add `semantic_schema_bytes` as a fuzz-only binary consuming bounded UTF-8
   JSON and calling `read_json` and `read_project_json` independently twice.
2. Equal input must produce equal success/error results for each reader. A
   reader must never guess or silently migrate the other reader's schema.
3. The corpus contains one minimal valid Seed graph and one malformed JSON
   input. A normal workspace test proves the valid seed succeeds, the project
   reader rejects it deterministically, and malformed input remains a
   deterministic JSON error.
4. The harness may return only data-only `SemanticGraph` values. It must not
   feed decoded graphs into evaluation, checked Typed Core, project resolution,
   bytecode, or runtime execution.
5. Input is bounded to 1 MiB; the common pinned corpus policy remains 256 runs,
   120 seconds per input, and 2048 MiB RSS. The Ubuntu CI fuzz job replays the
   corpus, while Windows evidence remains compile-only without the optional
   MSVC sanitizer runtime.
6. `cargo xtask fuzz verify` must fail on target, path, corpus-count, or
   inventory-name drift and report nine targets with twenty corpus files.
7. Parent `REL-6601` remains `BlockedSpec` for archive, replay/evidence, FFI,
   device, LSP/DAP, Zed fuzzing, and final G6 release evidence.

## Normative basis

- Accepted DEC-0012 governs Semantic IDs, canonical bytes, the file-mode
  reader, and its data-only boundary.
- Accepted RFC-0002 and the registered package-aware Semantic Graph protocol
  authorize the isolated `ling.semantic/0.2` reader without changing 0.1.
- Accepted DEC-0041 authorizes the deterministic fuzz inventory, pinned tools,
  bounded corpus replay, and explicit future-surface exclusions.
- The protocol inventory records both readers as implemented Experimental
  protocols with exact-version policies and no migration tool.

## Conformance plan

- Compile all fuzz binaries offline and locked.
- Verify the positive and malformed semantic corpus through a normal workspace
  test and both exact-version reader calls through the fuzz harness.
- Verify nine declared targets and twenty corpus files through xtask.
- Replay `semantic_schema_bytes` in the pinned Ubuntu fuzz job.
- Run semantic, fuzz, CI-contract, governance, status, workspace, lint,
  formatting, deterministic, and offline gates.

## Compatibility impact

The change adds a fuzz-only dependency edge from `ling-fuzz` to the existing
`ling-semantic` crate, one harness, two corpus files, and test/governance
evidence. It changes no schema bytes, reader behavior, Semantic ID, diagnostic,
CLI, package, dependency shipped to users, Unicode version, or runtime behavior.

## Unresolved alternatives

Long-run coverage thresholds; dictionaries; resource-limit oracles beyond the
shared baseline; package-aware valid corpus expansion; cross-platform sanitizer
execution; and harnesses for archive, replay/evidence, FFI, device, LSP/DAP,
and Zed remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
