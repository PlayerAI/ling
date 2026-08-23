# DEC-0229: Local supply-chain attack-boundary evidence / 本地供应链攻击边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: package security governance
> 相关规范/缺口：`RFC-0002` | `DEC-0022` | `DEC-0228` | `PKG-6404`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes executable security evidence only for Ling's accepted
local package boundary. It classifies every attack named by `PKG-6404` as
either an RFC-0002 local subset with an executable oracle or an unavailable
protocol explicitly deferred beyond Ling 1.0.

本决定只授权 Ling 已接受本地包边界的可执行安全证据。它将 `PKG-6404` 列出的每种
攻击分类为具有可执行判定标准的 RFC-0002 本地子集，或明确推迟到 Ling 1.0 之后的
不可用协议。

## Question

Which `PKG-6404` supply-chain attack tests can be executed without inventing
registry, archive, signing, package-cache, or build-sandbox protocols?

## Decision

1. Dependency confusion, namespace spoofing, Unicode-confusable packages, and
   malicious manifests receive bounded local-subset evidence only. The oracle
   is RFC-0002's graph-local ASCII technical name, separate Unicode display
   name, content identity, vendored path, closed manifest, bounded traversal,
   canonical lock, and offline/no-code-execution behavior.
2. Local-subset evidence must retain existing graph collision, transitive
   visibility, path/symlink escape, Unicode 17.0.0, bounded manifest, content
   identity, lock corruption, deterministic traversal, and failure-atomicity
   fixtures. Representative spoofed names, hostile fields, external locators,
   and execution routes are tested directly.
3. Archive traversal, decompression bombs, signature/key mismatch, yanked
   packages, compromised *package* caches, and build-capability escalation are
   classified `UnavailableProtocol`. DEC-0228 intentionally leaves their
   prerequisite registry, archive, signing, installation, and package-cache
   protocols Unsupported through Ling 1.0; PKG-6402 has no accepted build
   executor or sandbox.
4. DEC-0022's disposable internal query cache is not a package cache. Its
   checksum, key, version, truncation, and size-envelope rejection may be
   strengthened as internal evidence, but cannot satisfy or redefine the
   compromised-package-cache item.
5. The ten-entry assessment and its opaque deterministic bytes are test-local
   evidence. They are not a public protocol, security-level declaration,
   threat-model schema, support claim, or guarantee of complete 1.0
   supply-chain security.
6. Parent `PKG-6404` remains `BlockedSpec`. A bounded child task records the
   implemented local evidence; unavailable attack surfaces require a future
   Accepted RFC with executable security oracles before implementation.

## Normative basis

- Accepted RFC-0002 defines local/offline manifests, graph-local names,
  vendored path dependencies, content identities, canonical `ling.lock/1`,
  bounded traversal, and explicit exclusions of registry, archive, signature,
  arbitrary build execution, network, and installation behavior.
- Accepted DEC-0022 defines only a disposable, checksummed internal query
  cache with bounded envelopes and safe-miss corruption behavior.
- Accepted DEC-0228 keeps publication, installation, and registry distribution
  Unsupported through Ling 1.0 and requires future Accepted authority before
  archive, signing, yanking, mirror/cache, and security protocols are added.
- `ROADMAP-1.0` requires truthful evidence and support boundaries; it does not
  authorize absent protocols merely by listing future attacks.

## Conformance plan

- Execute representative technical-name, Unicode spoofing, hostile manifest,
  external locator, and unavailable execution-route tests in `ling-project`.
- Preserve existing graph collision, path/symlink escape, lock corruption,
  deterministic, bounded-resource, Unicode, offline, and failure-atomic tests.
- Exercise hostile internal query-cache envelopes as checksum/key/version/
  truncation/size safe misses without calling them package-cache coverage.
- Assert the exact ten-item execution-plan attack inventory, local/unavailable
  classification, deterministic ordering, and duplicate rejection.
- Run project, cache, governance, status, workspace, lint, formatting, and
  offline gates.

## Compatibility impact

This decision adds tests and governance evidence only. It changes no manifest,
lock, package identity, resolver, compiler, cache format, diagnostic, CLI,
Semantic ID, source span, dependency, Unicode version, or public protocol. It
adds no package registry, archive reader, decompressor, signature verifier,
yanking state, shared package cache, build executor, sandbox, security API, or
Stable support claim.

## Unresolved alternatives

Authenticated publisher namespaces; registry and installer threat models;
archive/artifact formats and resource limits; signature, key, trust,
revocation, provenance, and transparency rules; yanking and rollback;
package-cache trust and repair; hermetic build capabilities and sandboxing;
stable security diagnostics; migration; and complete 1.0 supply-chain claims
remain deferred to future Accepted authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
