# DEC-0071: Internal workspace-state snapshot / LSP 内部工作区状态快照

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: compiler-architecture  
> 相关 RFC/缺口：`DEC-0019` | `RFC-0004` | `GAP-INCREMENTAL-CACHE-001` | `GAP-LSP-TRANSACTION-PROTOCOL-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes a deterministic, immutable in-process capture of the
visible source files, workspace inputs, and session revision already owned by
the compiler VFS. It is a compiler observation boundary for the bounded
`LSP-2105` child; it is not a filesystem watcher, workspace-folder protocol,
reload notification, or serialized LSP message.

本决定授权编译器 VFS 对可见源文件、工作区输入和会话 revision 进行确定性的、
不可变的进程内捕获，作为有界 `LSP-2105` 子任务的编译器观察边界；它不是文件
系统 watcher、workspace-folder 协议、reload 通知或序列化 LSP 消息。

## Question

`DEC-0019` defines immutable compiler inputs and revision-aware query identity,
but the VFS has no single owned value that captures the complete visible state
for one observation. The bounded child needs such a value without inventing
the unresolved public reload and watcher contract.

## Decision

1. `ling-source` provides an immutable `WorkspaceStateSnapshot` containing the
   VFS session revision, visible `FileSnapshot` values, and present
   `WorkspaceSnapshot` inputs.
2. Files are stored in canonical logical-name order, and workspace inputs in
   the declared `WorkspaceInput` order. The captured collections are owned;
   later VFS mutation cannot change an earlier capture.
3. `CompilerDb::workspace_snapshot` exposes this value only as an internal
   compiler observation boundary. It does not publish a query cache, semantic
   ID, path identity, or cross-process revision.
4. The capture observes the visible layer, including an open overlay, and
   retains exact bytes, source identity, origin, per-input revision, and the
   session high-water revision. No normalization or source-span rewriting is
   permitted.
5. The implementation must not add filesystem access, watcher ownership,
   event coalescing, debounce policy, workspace-folder identity, URI/version
   fields, stale-result handling, JSON-RPC methods, diagnostics publication,
   or protocol-inventory claims. The public `LSP-2105` target remains
   `BlockedSpec`.

## Conformance plan

- Capture files inserted in different orders and assert canonical logical-name
  ordering and canonical workspace-input ordering.
- Capture disk and overlay layers, then mutate the VFS and prove the earlier
  snapshot remains byte-for-byte and revision-for-revision unchanged.
- Capture empty, source-only, input-only, and mixed workspaces; assert lookup
  results, session revision, per-file revision, origin, and input bytes.
- Repeat captures and compare the complete immutable value; exercise Unicode,
  BOM, CRLF, and exact UTF-8 bytes without changing source identity.
- Verify the compiler query identity and clean/incremental behavior remain
  unchanged; keep watcher, reload, stale-result, URI/version, and LSP fixture
  suites deferred to the required public protocol decisions.

## Compatibility impact

- Adds only an internal `ling-source` snapshot value and a `ling-db` accessor.
  Ling syntax, semantics, diagnostics, schemas, Semantic IDs, CLI, LSP wire
  behavior, bytecode, VM, ABI, and Unicode 17.0.0 data are unchanged.
- No protocol inventory entry, Stable feature claim, filesystem dependency, or
  persistent cache format is added.
- No public diagnostic allocation or migration is required.

## Unresolved alternatives

Workspace roots, watcher/event sources, symlink and path policy, reload
coalescing, graph rebuild scope, failure-atomic publication, stale request
results, document versions, and public LSP schemas remain open under
`GAP-LSP-TRANSACTION-PROTOCOL-001`, `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`,
`GAP-PROJECT-CLI-INTERFACE-001`, and `GAP-INCREMENTAL-CACHE-001`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
