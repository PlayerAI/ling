# DEC-0226: Local package and publication-exclusion boundary evidence / 本地包与发布排除边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: package governance
> 相关规范/缺口：`RFC-0002` | `DEC-0007` | `DEC-0012` | `ROADMAP-1.0`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes bounded evidence for `PKG-6401-OBSERVATION`. It
freezes the Accepted local project protocol and its explicit publication
exclusions. It does not define or implement a package publication protocol.

本决定授权 `PKG-6401-OBSERVATION` 使用有界证据，固定已接受的本地工程协议及其
明确的发布排除项，但不定义或实现包发布协议。

## Question

Which package-protocol boundaries can be made executable without contradicting
RFC-0002's exclusion of publication, registry, installation, network, and
supply-chain protocols?

## Decision

1. The exact local markers remain `ling.toml`, manifest version `1`,
   `ling.lock`, and `ling.lock/1`; Chinese display names remain non-identity
   metadata under RFC-0002.
2. Manifest version 1 must reject representative publication-only top-level
   and package fields for registry, publisher, namespace, artifact, checksum,
   signature, provenance, yanking, deprecation, mirrors, and caches.
3. Dependency entries accept only the Accepted local `path` form and reject
   representative version-range, registry, Git, and checksum locators.
4. The project implementation must retain no registry, network, process,
   installation, publication, or signature-verification route.
5. A sixty-category test-local inventory records local protocol, publication,
   supply-chain, compatibility, authority, and fixture boundaries with
   deterministic ordering and duplicate rejection.
6. Opaque bytes tagged `ling.package-publication-boundary-observation/0` are
   test evidence only. They are not a manifest, lock, archive, artifact,
   registry, signature, provenance, installation, or migration protocol.
7. No publisher coordinate, namespace ownership, registry, upload/download,
   archive, signature, provenance, SBOM, mirror/cache, yanking, installation,
   CLI, diagnostic, dependency, public API, or support claim is authorized.
   Public `PKG-6401` remains `BlockedSpec`.

## Normative basis

- Accepted `RFC-0002` sections 1 through 6 define the exact deterministic,
  local, offline manifest, identity, dependency graph, and lock protocols.
- `RFC-0002` status/scope and section 7 explicitly exclude public registries,
  publisher/domain ownership, installation, network/Git dependencies,
  mirrors, signatures, transparency logs, version ranges, binary packages,
  and artifact metadata.
- Accepted `DEC-0007` and `DEC-0012` separate normalized module/Semantic
  identity from host paths and cosmetic source metadata.
- `docs/status/PKG-6401-AUTHORITY-AUDIT.md` records the absent publisher,
  artifact, trust, registry, install, migration, CLI, and security contracts.

## Conformance plan

- Assert exact local protocol markers and representative valid local metadata.
- Assert version-1 rejection of publication fields and external dependency
  locators plus absence of registry/network/process/signing routes.
- Assert all sixty local boundaries, exact ordering, duplicate rejection, and
  order-independent opaque bytes.
- Run project, governance, status, workspace, lint, formatting, deterministic,
  and offline gates.
- Defer every publication and supply-chain behavior until a dedicated Accepted
  RFC defines authenticated coordinates, formats, trust, lifecycle, and
  executable compatibility/security evidence.

## Compatibility impact

Manifest, lock, package identity, graph identity, dependency resolution,
diagnostics, CLI, Semantic IDs, source spans, dependencies, Unicode 17.0.0,
and support claims remain unchanged. The change adds regression and test-local
evidence only.

## Unresolved alternatives

Publisher/source coordinates; namespace ownership and transfer; package
archive and artifact formats; checksums, signatures, provenance, SBOM,
licenses, trust roots, and transparency; registry and installation operations;
yanking/deprecation; mirrors and offline caches; version selection; CLI and
diagnostics; compatibility, migration, and security fixtures remain open under
`PKG-6401`, later package tasks, and future Accepted authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
