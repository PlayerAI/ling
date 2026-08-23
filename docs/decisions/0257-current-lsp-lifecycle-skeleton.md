# DEC-0257: Current LSP lifecycle skeleton / 当前 LSP 生命周期骨架

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：ide-protocol-design
> 相关 RFC/缺口：RFC-0004 | DEC-0029 | RFC-0023 | RFC-0026 | LSP-2101
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision accepts the implemented RFC-0004 transport and state machine as
the complete bounded LSP-2101 lifecycle skeleton. It does not promote the full
LSP editing surface, Semantic Graph, Workspace Edit, or Semantic Transaction
contracts.

本决定接受已实现的 RFC-0004 传输与状态机，作为完整且有界的 LSP-2101
生命周期骨架；它不会把完整 LSP 编辑接口、Semantic Graph、Workspace Edit 或
Semantic Transaction 协议提升为已完成能力。

## Question

Now that CLI-1701 is Done and RFC-0004 lifecycle evidence remains executable,
does LSP-2101 require a second lifecycle implementation, or can the parent be
closed by the existing bounded implementation while later LSP methods retain
their independent authorities?

## Decision

1. LSP-2101 means exactly the RFC-0004 Preview substrate: `ling lsp --stdio`,
   CRLF `Content-Length` framing, bounded JSON-RPC 2.0 messages,
   `initialize`/`initialized`/`shutdown`/`exit`, server information, position-
   encoding negotiation, bounded opaque workspace folders, deterministic
   errors, and protocol-pure stdout.
2. `crates/ling-lsp` owns the transport and lifecycle state machine;
   `crates/ling-cli` owns only exact command selection and stdio delegation.
   No second server loop, transport, lifecycle enum, or editor-specific CLI
   parser may be introduced for this parent task.
3. The current initialization capability object composes later Accepted
   authorities. RFC-0026 permits `documentFormattingProvider: true`; RFC-0023
   supplies the independently versioned overlay needed by later document
   methods. Their presence does not broaden LSP-2101 or change RFC-0004 state
   transitions.
4. Later Accepted methods share the lifecycle gate: requests before
   initialization or after shutdown retain RFC-0004 errors, and no method may
   bypass framed stdout. Their request fields, snapshots, diagnostics, edits,
   cancellation, and compatibility remain owned by their own RFCs.
5. Workspace folder URIs remain validated opaque protocol strings. LSP-2101
   performs no URI-to-path conversion, project discovery, watcher setup,
   manifest loading, filesystem mutation, or network access.
6. `ling.lsp.lifecycle/0.1` remains Preview/current-writer-only. Completion of
   LSP-2101 is not a Stable editor-support claim and does not close open
   Semantic Graph/Transaction or Workspace Edit gaps.
7. RFC-0004 fixtures plus current workspace tests are the executable parent
   evidence. A new implementation is neither required nor permitted merely to
   duplicate the already accepted slice.

## Conformance plan

- Retain lifecycle transcripts for position encodings, Unicode workspace
  metadata, bounded/duplicate/invalid folders, pre-initialize and post-shutdown
  requests, duplicate lifecycle messages, early exit, and clean shutdown.
- Retain malformed frame/JSON/ID/batch/size fixtures, deterministic bilingual
  errors, exact response IDs, CRLF framing, and no stdout contamination.
- Retain the real `ling lsp --stdio` process fixture and parser rejection of
  missing `--stdio`, paths, and output-policy flags.
- Verify later formatting/overlay authorities do not change lifecycle gates,
  workspace opacity, exit behavior, or channel ownership.
- Run workspace, CI, governance, support, status, RC0, traceability, Clippy,
  formatting, and deterministic-diff gates.

## Compatibility impact

- **LSP/CLI:** accepts already implemented Preview behavior; no command,
  method, field, frame, error, option, channel, or exit is added or changed.
- **Protocol:** adds parent-level authority evidence to
  `PROTO-LSP-LIFECYCLE`; its version and stability remain unchanged.
- **Language/compiler:** no syntax, type, Effect, Capability, Checked Core,
  diagnostic, Semantic ID, source-span, runtime, bytecode, VM, or ABI change.
- **Filesystem/network:** none; workspace URIs remain opaque and no discovery
  or mutation is added.
- **Determinism/Unicode:** no new input source or ordering; Unicode remains
  17.0.0 and editor positions retain DEC-0029 ownership.
- **Migration:** none because observable protocol bytes are unchanged.

## Unresolved alternatives

Document synchronization beyond accepted overlay slices, incremental edits,
workspace reload/watchers, diagnostics, navigation, completion, code actions,
semantic tokens, cancellation, concurrent scheduling, Workspace Edits,
Semantic Transactions, and Stable editor compatibility remain separately
governed. A general third-party LSP framework is not required for this fixed
lifecycle substrate.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
