# DEC-0083: Locked-project semantic snapshot boundary / 锁定项目语义快照边界

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: compiler-query-design
> Related RFC/gaps: `RFC-0002` | `DEC-0058` | `GAP-REGISTER`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision authorizes one internal, read-only compiler-database boundary for
building the already accepted package-aware semantic snapshot from a validated
`LockedProject`. It does not define project selection, a compiler-host
filesystem API, a CLI command, execution, testing, artifact production, or a
wire protocol.

本决定只授权一个内部、只读的编译器数据库边界：从已验证的
`LockedProject` 构建已有定义的、面向 package 的语义快照。不定义项目选择、编译器
主机文件系统 API、CLI 命令、执行、测试、构建产物或线协议。

## Question

`ling-semantic::build_project` and `ling-resolve::resolve_project` already
define the package-aware checked pipeline, while `PRJ-1107` still lacks a
compiler-database adapter that consumes the immutable snapshot retained by
`DEC-0058`. Without a bounded adapter, the accepted project pipeline cannot be
observed from the internal query layer; adding a public host or CLI contract
would exceed the current authority.

`ling-semantic::build_project` 与 `ling-resolve::resolve_project` 已经定义了面向
package 的检查流水线，但 `PRJ-1107` 仍缺少一个消费 `DEC-0058` 不可变快照的编译器
数据库适配器。没有这个边界，内部查询层无法观察已接受的项目流水线；加入公开主机
或 CLI 契约则会超出当前权威范围。

## Decision

1. `ling-db::CompilerDb` may expose an internal
   `project_semantic_snapshot(&LockedProject)` query returning the existing
   `ling.semantic/0.2` `ProjectProgramSnapshot`.
2. The query consumes only the locked graph's retained source bytes. Packages
   and sources are traversed in their canonical graph order; each source uses a
   deterministic `SourceId` and the path-free logical name
   `package:<package-name>/<logical-path>`. Original UTF-8 bytes and compiler
   spans remain unchanged.
3. The pipeline is exactly parse → AST → HIR → `resolve_project` → type check →
   effect check → `build_project`. A failed stage publishes no snapshot. A
   successful result is cached only by the immutable package-graph identity.
4. The boundary must not select a manifest or workspace, read physical paths,
   mutate a lock, access the network, execute or test Ling code, produce build
   artifacts, add diagnostics or public schemas, or serialize a CLI/LSP/DAP
   response. The public `PRJ-1107` task remains `BlockedSpec` for those areas.

1. `ling-db::CompilerDb` 可以提供内部
   `project_semantic_snapshot(&LockedProject)` 查询，返回现有的
   `ling.semantic/0.2` `ProjectProgramSnapshot`。
2. 查询只消费锁定图保留的源字节。package 与 source 按图的规范顺序遍历；每个源使
   用确定性的 `SourceId` 和不含主机路径的逻辑名
   `package:<package-name>/<logical-path>`。原始 UTF-8 字节及编译器跨度保持不变。
3. 流水线固定为 parse → AST → HIR → `resolve_project` → 类型检查 → effect 检查 →
   `build_project`。任一阶段失败都不发布快照；成功结果只按不可变 package 图身份缓存。
4. 此边界不得选择 manifest 或 workspace、读取物理路径、修改 lock、访问网络、执行或
   测试 Ling 代码、产生构建产物、增加诊断或公开 schema，也不得序列化 CLI/LSP/DAP
   响应。公开的 `PRJ-1107` 任务在这些范围内仍保持 `BlockedSpec`。

## Conformance plan

- Build the locked offline project fixture and assert the project schema,
  package-graph identity, cross-package modules, and cached pointer reuse.
- Repeat the same graph through separate database instances and compare JSON
  and Semantic IDs; assert that host roots, drive letters, and fixture names do
  not appear in the snapshot.
- Exercise invalid UTF-8, AST/HIR, resolution, type, effect, and semantic
  failures through the owning pipeline errors and verify that no result is
  published; preserve original UTF-8/BOM/CRLF spans where valid.
- Keep project selection, workspace reload, stale/version/cancellation,
  manifest or lock mutation, CLI exits/JSON, execution, test discovery,
  artifacts, and protocol fixtures deferred to their accepted authorities.

## Compatibility impact

- Adds one internal `ling-db` query, a typed internal error, and a direct
  library dependency on the already existing `ling-project` crate.
- Language semantics, registered diagnostics, source schemas, Semantic ID
  algorithms, CLI behavior, LSP/DAP protocols, runtime, bytecode, VM, ABI,
  dependency versions, and Unicode 17.0.0 data are unchanged.
- No host path, allocation order, hash-map order, or debug representation is
  observable in the returned project snapshot.

## Unresolved alternatives

Project/workspace selection, a general compiler-host lifecycle, incremental
project invalidation, public project semantic-check commands, run/test/build
behavior, artifact formats, process exits, machine output, network policy,
and LSP/JSON-RPC publication remain open under `PRJ-1107`,
`GAP-PROJECT-CLI-INTERFACE-001`, and their higher-authority decisions. This
decision can be revisited only when those contracts are accepted.

项目/workspace 选择、通用 compiler-host 生命周期、项目增量失效、公开项目语义检查命令、
run/test/build 行为、产物格式、进程退出码、机器输出、网络策略以及 LSP/JSON-RPC 发布
仍由 `PRJ-1107`、`GAP-PROJECT-CLI-INTERFACE-001` 及其更高权威决定，当前保持开放。
只有这些契约被接受后，才可重新审视本决定。

## Supersession

- Supersedes: `None`
- Superseded by: `None`
