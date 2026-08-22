# DEC-0145: Internal Zed debugger registration boundary evidence / 内部 Zed 调试器注册边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: editor-quality  
> 相关规范/缺口：`DEC-0144` | `DEC-0143` | `ROADMAP-1.0` | `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` | `GAP-LSP-TRANSACTION-PROTOCOL-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`DAP-3602-OBSERVATION` Zed debugger-registration boundary. It records
provisional extension and launch-mapping vocabulary while DAP, Native, editor,
security, and protocol authorities remain unresolved.

本决定只授权 `DAP-3602-OBSERVATION` 使用 test-local 的拟议 Zed 调试器注册边界清单，
在 DAP、Native、editor、security 与 protocol 权威尚未解决时，只记录临时扩展与启动映射词汇。

## Question

DAP-3602 proposes a Zed extension package, language configuration, debugger
registration, adapter discovery, and launch/attach/build/run mappings. Which
planning vocabulary can be retained as bounded evidence without adding an
extension package, manifest, debugger button, adapter locator, or editor
protocol?

DAP-3602 计划提供 Zed 扩展包、语言配置、调试器注册、adapter discovery 以及 launch/attach/
build/run 映射。在不添加扩展包、manifest、调试器按钮、adapter locator 或编辑器协议的前提下，
哪些规划词汇可以作为有界证据保留？

## Decision

1. `crates/ling-types/tests/zed_debugger_registration_evidence.rs` keeps a
   test-local inventory of sixty provisional boundaries covering extension
   package/manifest/language configuration/registration/discovery/install and
   Zed/DAP versions, launch/attach/build/run/root/source/working-directory/
   environment/profile/target/capability mappings, source/binary identity,
   session/restart/cancel/timeout/security/trust, malformed configuration and
   missing executables, offline/locked/platform/update behavior, diagnostics,
   UTF-8/source-map/breakpoint/step/stack/scope/variable/Fault/ownership
   mappings, VM/Native choice and Typed Core input, host exclusions, Unicode,
   Semantic IDs, fixtures, smoke/deterministic evidence, and protocol inventory
   separation.
2. The test-local inventory sorts boundaries by explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.zed-debugger-observation/0`. These bytes are not an extension
   manifest, Zed configuration, debugger registration, command, locator,
   diagnostic, provenance record, Semantic ID, or public protocol.
3. The child adds no extension package, manifest, language configuration,
   debugger registration, adapter discovery, build/run task, `ling build`
   contract, dependency, toolchain, diagnostic, protocol, or placeholder API.
   Public `DAP-3602` remains `BlockedSpec`; stale `zero build` text is not
   carried into implementation.

## Normative basis

- The G3+ execution package is non-normative and makes Zed registration
  contingent on DAP-3601, VM/Native source maps, breakpoint/step/stack/
  variables behavior, ProgramSnapshot/binary identity, Fault categories, and
  an Accepted debugger RFC. It cannot define an extension manifest or editor
  integration contract.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` fix the public CLI as `ling` and
  source extension as `.ling`; they do not authorize Zed support for the Seed
  subset.
- Accepted RFC-0014/RFC-0018/RFC-0019 provide experimental VM/source-map,
  Fault, and differential foundations only. They do not define Zed manifests,
  adapter discovery, launch mapping, or Native/editor metadata.
- `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`, `GAP-LSP-TRANSACTION-PROTOCOL-001`,
  and `GAP-NATIVE-BACKEND-ABI-001` remain Open; no Zed or debugger protocol is
  registered in `docs/governance/protocol-inventory.toml`.

## Conformance plan

- Assert all sixty provisional Zed debugger boundaries and their test-local
  order.
- Compare forward and reversed insertion order and require identical opaque
  evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep extension packaging, manifest/config parsing, registration, discovery,
  launch mapping, editor permissions, DAP integration, migration, and public
  support behavior deferred until the required authorities are Accepted.

## Compatibility impact

- Accepted Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-local boundary evidence. No Zed package, manifest, debugger
  registration, command, locator, diagnostic, dependency, protocol, extension,
  or support claim is registered.

## Unresolved alternatives

Extension package/manifest identity and versions; language configuration and
registration; adapter discovery/install/update; Zed/DAP compatibility;
launch/attach/build/run/root/environment/profile/target/capability mapping;
source/binary/session identity; security, permissions, trust, cancellation,
timeouts, errors, platform and offline behavior; VM/Native metadata and
debug mappings; diagnostics, Unicode, Semantic IDs, fixtures, smoke tests,
migration, and protocol inventory remain open under DAP-3602, DAP-3601,
DIFF-3702, the listed gaps, and missing debugger/Native/editor authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
