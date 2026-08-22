# DEC-0144: Internal DAP debugger boundary evidence / 内部 DAP 调试器边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: debugger-quality  
> 相关规范/缺口：`DEC-0143` | `DEC-0012` | `DEC-0013` | `ROADMAP-1.0` | `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` | `GAP-LSP-TRANSACTION-PROTOCOL-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`DAP-3601-OBSERVATION` debugger boundary. It records provisional protocol and
debugging vocabulary while DAP, VM/Native metadata, identity, security, and
editor-protocol authorities remain unresolved.

本决定只授权 `DAP-3601-OBSERVATION` 使用 test-local 的拟议调试器边界清单，
在 DAP、VM/Native metadata、identity、security 与 editor protocol 权威尚未解决时，
只记录临时协议和调试词汇。

## Question

DAP-3601 proposes a debugger process on stdio for the Explore VM and later
Native, including lifecycle, source maps, breakpoints, stepping, stacks,
variables, Faults, cancellation, and editor integration. Which planning
vocabulary can be retained as bounded evidence without adding a DAP transport,
debugger command, runtime hook, or public protocol?

DAP-3601 计划为 Explore VM 及后续 Native 提供 stdio 调试器进程，覆盖生命周期、source
map、断点、单步、栈、变量、Fault、取消和编辑器集成。在不添加 DAP transport、调试器
命令、runtime hook 或公共协议的前提下，哪些规划词汇可以作为有界证据保留？

## Decision

1. `crates/ling-types/tests/dap_debugger_boundary_evidence.rs` keeps a
   test-local inventory of sixty provisional boundaries covering protocol
   schema/framing/limits/lifecycle/capabilities/launch/attach/disconnect/
   cancellation/session/security, malformed and unknown messages, version and
   migration, reader/writer boundaries, source maps and UTF-8/LSP positions,
   snapshots and binary identity, breakpoints/conditions/logpoints/continue/
   step/pause, stack/scope/variables/mutation, Fault/exception/Resource/
   Managed/ownership/Actor-task views, capability/profile/target restrictions,
   VM/Native metadata and Typed Core input, host-output exclusions, timeout and
   multi-client security, redaction, diagnostics, Unicode, Semantic IDs,
   fixtures, deterministic evidence, and protocol inventory separation.
2. The test-local inventory sorts boundaries by explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.dap-debugger-observation/0`. These bytes are not DAP messages,
   transport frames, a debugger API, source-map metadata, a diagnostic,
   provenance record, Semantic ID, or public protocol.
3. The child adds no DAP adapter, stdio process, `ling dap` command, extension,
   runtime hook, source-map bridge, dependency, toolchain, diagnostic,
   protocol, or placeholder API. Public `DAP-3601` remains `BlockedSpec`; the
   stale planning spelling `zero dap --stdio` is not carried forward.

## Normative basis

- The G3+ execution package is non-normative and explicitly makes DAP
  contingent on source maps, runtime debug behavior, ProgramSnapshot/binary
  identity, Fault categories, and an Accepted debugger RFC. It cannot define a
  wire schema, lifecycle, or debugger semantics.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` fix the public CLI as `ling` and
  require verified Typed Core inputs, stable spans, and Semantic IDs; they do
  not authorize a debugger protocol for the Seed subset.
- Accepted RFC-0014/RFC-0018/RFC-0019 provide experimental bytecode/VM,
  Fault, source-map, and differential foundations only. They do not define
  DAP messages, stop/step behavior, Native debug metadata, or editor support.
- `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`, `GAP-LSP-TRANSACTION-PROTOCOL-001`,
  and `GAP-NATIVE-BACKEND-ABI-001` remain Open; no debugger protocol is
  registered in `docs/governance/protocol-inventory.toml`.

## Conformance plan

- Assert all sixty provisional DAP boundaries and their test-local order.
- Compare forward and reversed insertion order and require identical opaque
  evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep DAP parsing/framing, lifecycle, capabilities, engine-neutral debug
  semantics, security, source-map metadata, extension integration, and public
  protocol behavior deferred until the required authorities are Accepted.

## Compatibility impact

- Accepted Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-local boundary evidence. No DAP message, debugger command,
  source-map contract, diagnostic, dependency, protocol, extension, or support
  claim is registered.

## Unresolved alternatives

DAP schema/framing and lifecycle; capability negotiation and launch/attach/
disconnect/cancel; session/security/limits; source-map/UTF-8/LSP positions;
snapshot/identity; breakpoint/step/stack/scope/variables/Fault/ownership/
Actor-task semantics; VM/Native metadata; Typed Core and host exclusions;
timeouts, redaction, diagnostics, Unicode, Semantic IDs, fixtures, migration,
and editor integration remain open under DAP-3601, DIFF-3702, the listed gaps,
and missing debugger/Native/protocol authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
