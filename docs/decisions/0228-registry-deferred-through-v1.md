# DEC-0228: Registry deferred through Ling 1.0 / Registry 推迟至 Ling 1.0 之后

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: package product governance
> 相关规范/缺口：`RFC-0002` | `ROADMAP-1.0` | `PKG-6403`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision selects the registry-deferred strategy for the Ling 1.0 line.
Package publication, installation, and registry distribution remain
Unsupported. Ling 1.0 work may stabilize only the separately authorized local
manifest, content identity, vendored dependency, and lock boundaries.

本决定为 Ling 1.0 产品线选择 registry-deferred 策略。包发布、安装和 registry
分发继续保持 Unsupported。Ling 1.0 只能稳定已由独立权威授权的本地 manifest、
内容身份、vendored dependency 和 lock 边界。

## Question

Must Ling 1.0 implement a package registry, or may it preserve a complete
local/offline package workflow while deferring registry services?

## Decision

1. Registry publication, installation, and distribution are deferred beyond
   Ling 1.0 and remain explicitly `Unsupported`; they are neither Preview nor
   Stable capabilities.
2. No package-registry protocol record, schema, endpoint, index, source kind,
   manifest field, command, diagnostic, client, server, cache, or placeholder
   API may be added under this decision.
3. RFC-0002 remains the only Accepted package basis: graph-local names,
   content-identified vendored dependencies, deterministic local resolution,
   and canonical `ling.lock/1`. This decision does not promote those currently
   Experimental protocols to Stable.
4. A local package name is never publisher ownership. Future registry work
   must introduce an independently authenticated publisher/source coordinate
   and preserve existing local identities without reinterpretation.
5. Reopening registry work requires a new Accepted RFC defining at minimum:
   authenticated coordinates and ownership lifecycle; archive/artifact and
   signature/provenance formats; index/upload/download/install/yank behavior;
   trust, credential, mirror/cache, availability, and threat policies; CLI,
   diagnostics, schema compatibility, migration, and executable security and
   offline fixtures.
6. `UNSUP-PACKAGES` is the product-facing support record for this deferment.
   The protocol inventory must continue to contain no package-registry entry.
7. `PKG-6403-DEFERMENT` records executable governance evidence. Parent
   `PKG-6403` remains `BlockedSpec` because its declared predecessor
   `PKG-6402` remains blocked and its full registry alternative is deliberately
   unimplemented.

## Normative basis

- Accepted `RFC-0002` defines the deterministic local/offline package boundary
  and explicitly excludes registry, publication, installation, network/Git
  sources, publisher ownership, mirrors, signatures, and federation.
- `ROADMAP-1.0` prioritizes repeatable local locked builds, does not require a
  centralized service, and requires unsupported capabilities to remain
  truthfully reported.
- `docs/status/PKG-6403-AUTHORITY-AUDIT.md` enumerates the three permissible
  strategy choices and the authority required for a registry implementation.

## Conformance plan

- Assert `UNSUP-PACKAGES` exactly covers publication, installation, and
  registry distribution and cites this decision.
- Assert no package-registry protocol exists while the local manifest,
  identity, and lock protocols remain implemented and Experimental.
- Assert the sixty-category deferment inventory, deterministic ordering,
  duplicate rejection, and opaque evidence bytes.
- Run support, protocol, governance, status, workspace, lint, formatting, and
  offline gates.

## Compatibility impact

This decision changes the product roadmap/support classification, not runtime
or source behavior. Manifest, lock, local identity, resolver, compiler,
diagnostics, CLI, Semantic IDs, spans, dependencies, Unicode 17.0.0, and
existing protocol stability remain unchanged. Users receive an explicit 1.0
non-claim instead of an implied future registry promise.

## Unresolved alternatives

A Preview or Stable registry; publisher/source coordinates; ownership and key
lifecycle; archive/artifact/signature/provenance; registry operations;
installation and rollback; yanking/deprecation; mirrors/caches; service
availability; CLI, diagnostics, schemas, migration, and security fixtures are
deferred to a future Accepted RFC.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
