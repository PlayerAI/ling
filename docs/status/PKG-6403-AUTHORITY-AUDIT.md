# PKG-6403 Authority Audit

- Task: `PKG-6403` — Registry Minimum Implementation or Deferment Strategy
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:239-246`
- Release: G6
- Status: `BlockedSpec`

## Decision

`PKG-6403` is `BlockedSpec`. The G6 checklist offers two product choices:
provide a minimal read/upload registry, or stabilize only package, lock, and
local-source protocols while keeping a registry in Preview. It does not select
one choice, define the 1.0 support claim, or specify the protocol, trust,
service, CLI, compatibility, and offline guarantees needed for either choice.

The current accepted boundary already supplies a deterministic local/offline
project protocol. `RFC-0002` explicitly excludes a public registry,
publisher/domain ownership, package installation, network/Git dependencies,
mirrors, signatures, transparency logs, version ranges, and registry
federation. The support matrix explicitly marks publication and registry
installation unsupported. Therefore no registry implementation or public
registry API is authorized by the plan alone.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:239-246` is a non-normative decision
  checklist. It records alternatives but no accepted selection, protocol
  version, registry coordinates, service availability, or migration policy.
- Accepted `RFC-0002` §1-2, §3, and §7 freezes only graph-local package names,
  local content/graph identities, vendored path dependencies, manifests, and
  `ling.lock/1`. It states that a future registry needs an independently
  authenticated publisher/source coordinate and cannot reinterpret a local
  package name as ownership.
- Accepted `GAP-PACKAGE-NAMESPACE-001` and `GAP-PACKAGE-PROTOCOL-001` resolve
  the local namespace/manifest/lock gaps through RFC-0002; they do not accept a
  global registry or installation protocol.
- `UNSUP-PACKAGES` in `docs/governance/support-matrix.toml` explicitly covers
  CLI project selection, package installation, publication, and registry
  distribution and states that these remain out of scope. The support matrix
  itself is Draft and cannot silently become a 1.0 promise.
- The protocol inventory contains only Experimental local package manifest,
  package identity, Semantic Graph, and lock protocols. It has no registry,
  index, upload, download, authentication, or installation record; adding one
  without Accepted authority would violate the public-protocol governance.
- `ROADMAP-1.0` G1.1 prioritizes local/locked dependency resolution and says
  centralized services are unnecessary at that stage. Its 1.0 rules require
  Accepted authority and executable evidence for every Stable capability and
  defer unsupported package registry services by default.
- Root `AGENTS.md` requires Accepted authority before public-protocol or
  semantic expansion, stable claims only after support gates and fixtures,
  deterministic/offline behavior, bilingual registered diagnostics, preserved
  UTF-8 byte spans, Unicode 17.0.0, and no placeholder APIs.

## Evidence in this repository

The local project implementation and fixtures demonstrate deterministic
manifest decoding, package graph construction, content/graph identities,
exported-module visibility, canonical `ling.lock/1`, and offline local
resolution. They do not establish:

1. a 1.0 policy selecting Stable, Preview, or Unsupported registry behavior;
2. global package coordinates, publisher/source authentication, namespace
   ownership, transfer, collision, revocation, or trust-root rules;
3. registry index, upload/download, installation, yanking, deprecation,
   mirror, cache, retry, availability, rate-limit, or rollback semantics;
4. package archive/artifact, checksum/signature, provenance, SBOM/license,
   transparency, vulnerability, or reproducible-build contracts;
5. `ling` command/exit/JSON behavior, compatibility readers, migration, or
   deterministic/offline and security fixtures for a registry service.

Implementing a registry server/client, adding a registry URL to the manifest,
or exposing a public package command now would contradict the unsupported
matrix and could turn graph-local names or experimental hashes into accidental
global identity commitments.

## Required authority before implementation

An accepted G6 package-support decision must first select and publish one of:

1. **Registry Stable:** define a versioned registry/index, package coordinate,
   authenticated publisher, archive/artifact, checksum/signature, provenance,
   installation, yanking/deprecation, mirror/cache, availability, migration,
   CLI, diagnostics, and compatibility contract, with independent fixtures;
2. **Registry Preview:** define the same minimum protocol and threat boundary
   but mark it Preview, isolate it from Stable package/lock/local-source
   guarantees, and specify opt-in commands, versioning, failure behavior, and
   explicit non-claims; or
3. **Registry deferred:** accept a lock-only/local-source 1.0 policy that keeps
   registry publication and installation Unsupported, guarantees repeatable
   offline locked builds, and records the criteria and lifecycle for reopening
   the decision.

Whichever option is selected must update the protocol inventory, support
matrix, schema/fixture registry, dependency and traceability records, status
registry, and bilingual diagnostic allocations atomically. It must preserve
the RFC-0002 local identity boundary and distinguish package, artifact,
publisher, and Semantic IDs without host paths or map order.

## Compatibility and deferred work

This audit changes no package graph, manifest, lockfile, resolver, identity,
diagnostic, schema, CLI, registry, network client, cache, dependency, or
public API behavior. It preserves `ling`/`.ling` naming, RFC-0002's local and
offline guarantees, Unicode 17.0.0, original UTF-8 spans, checked Typed Core
boundaries, and explicit Experimental/Preview/Future/Unsupported states.

It deliberately adds no registry, index, upload/download service, package
installer, manifest source kind, publisher/authentication API, archive,
signature, provenance, yanking command, mirror/cache, migration tool,
diagnostic, dependency, public protocol, or placeholder. Future work remains
deferred until the 1.0 support choice and its Accepted protocol and executable
security/compatibility/offline evidence exist.
