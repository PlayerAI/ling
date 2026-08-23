# DEC-0227: Hermetic-build exclusion boundary evidence / Hermetic Build 排除边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: build governance
> 相关规范/缺口：`RFC-0002` | `DEC-0010` | `DEC-0019` | `DEC-0022` | `ROADMAP-1.0`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes bounded evidence for `PKG-6402-OBSERVATION`. It
freezes the absence of build-system authority from RFC-0002 manifest version 1
and the local project implementation. It does not define a hermetic build
graph, executor, sandbox, artifact, cache, or replay protocol.

本决定授权 `PKG-6402-OBSERVATION` 使用有界证据，固定 RFC-0002 manifest
version 1 和本地工程实现中不存在构建系统授权这一边界，但不定义 hermetic build
图、执行器、沙箱、产物、缓存或重放协议。

## Question

Which negative hermetic-build boundaries can be made executable without
inventing the unresolved build graph, Capability, sandbox, and artifact
semantics?

## Decision

1. Manifest version 1 must reject representative top-level and package fields
   for build graphs/scripts, generators, generated source, inputs/outputs,
   Capabilities, sandbox, environment, network, artifacts, and build caches.
2. Local dependency declarations must not gain build execution or host
   authority through build-script, generator, Capability, environment,
   network, shell, command, artifact, or cache fields.
3. `ling-project` must retain no build executor, build-script, shell adapter,
   subprocess, or network route. Existing locked local project loading remains
   input evidence only, not a public build command or artifact producer.
4. A sixty-category test-local inventory records build graph, execution,
   sandbox, identity, cache, authority, and fixture boundaries with
   deterministic ordering and duplicate rejection.
5. Opaque bytes tagged `ling.hermetic-build-boundary-observation/0` are test
   evidence only. They are not a graph, plan, artifact, cache, replay,
   Capability, sandbox, command, or migration protocol.
6. No build node, executor, script/plugin, generator, sandbox, Capability,
   manifest field, artifact, cache/replay, CLI, diagnostic, dependency, public
   API, or support claim is authorized. Public `PKG-6402` remains
   `BlockedSpec`.

## Normative basis

- Accepted `RFC-0002` defines only deterministic local/offline project and
  lock behavior and explicitly excludes arbitrary build scripts, plugins,
  generated source, target/profile defaults, binary packages, and artifact
  metadata.
- Accepted `DEC-0010` governs language Effect/Capability behavior, not build
  process or sandbox authority.
- Accepted `DEC-0019` and `DEC-0022` authorize bounded internal query/cache
  behavior, not a build graph, artifact identity, shared cache, or replay
  protocol.
- `docs/status/PKG-6402-AUTHORITY-AUDIT.md` records the absent node, sandbox,
  Capability, identity, executor, CLI, diagnostic, migration, and security
  contracts.

## Conformance plan

- Assert manifest and local-dependency rejection of representative build and
  host-authority fields.
- Assert absence of build executor, script, shell, subprocess, and network
  routes from the local project implementation.
- Assert all sixty local boundaries, exact ordering, duplicate rejection, and
  order-independent opaque bytes.
- Run project, governance, status, workspace, lint, formatting, deterministic,
  and offline gates.
- Defer every build execution, sandbox, artifact, and replay behavior until a
  dedicated Accepted RFC defines the public contracts and executable security
  and reproducibility evidence.

## Compatibility impact

Manifest, lock, package identity, dependency resolution, compiler/query/cache
behavior, diagnostics, CLI, profiles, targets, artifacts, Semantic IDs, source
spans, dependencies, Unicode 17.0.0, and support claims remain unchanged. The
change adds regression and test-local evidence only.

## Unresolved alternatives

Typed build nodes and graphs; inputs/outputs and Capabilities; sandbox and
resource policies; plugins/generators; environment/network/filesystem/process
authority; build-plan and artifact identities; cache/replay/corruption;
profiles/targets/toolchains; CLI and diagnostics; compatibility, migration,
cross-process, cross-platform, and security fixtures remain open under
`PKG-6402`, later package tasks, and future Accepted authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
