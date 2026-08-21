# PKG-6401 Authority Audit

- Task: `PKG-6401` — Package Publication Protocol
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:210-224`
- Release: G6
- Status: `BlockedSpec`

## Decision

`PKG-6401` is `BlockedSpec`. The G6 checklist names package identity,
semantic/content hashes, namespace, version policy, manifest, lock, artifact,
checksum/signature, provenance, yanked/deprecated state, and mirror/offline
cache, but it does not define a versioned publication protocol, a publisher
identity model, an artifact schema, a registry trust model, or compatible
installation and migration behavior.

Accepted `RFC-0002` deliberately freezes only the first local, deterministic,
offline project boundary. It covers `ling.toml`, graph-local package names,
content and graph identities, local vendored dependencies, exported modules,
and canonical `ling.lock/1`. It explicitly excludes a public registry,
publisher/domain ownership, package installation, network or Git dependencies,
mirrors, signatures, transparency logs, arbitrary artifact metadata, and
version ranges. Those exclusions are normative boundaries, not missing
implementation details that this task may fill in.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:210-224` is a non-normative stabilization
  checklist. It lists publication concerns but supplies no wire format,
  command contract, trust roots, key lifecycle, artifact reproducibility rule,
  cache policy, or compatibility/migration clauses.
- Accepted `RFC-0002` §1-2 and §7 explicitly constrain version 1 to local,
  offline resolution and state that registry, publisher ownership,
  installation, network/Git sources, mirrors, signatures, transparency logs,
  and artifact metadata are out of scope. Its future-registry note requires a
  separate authenticated source/publisher coordinate rather than reusing a
  graph-local package name.
- `GAP-PACKAGE-NAMESPACE-001` and `GAP-PACKAGE-PROTOCOL-001` are resolved by
  `RFC-0002` only for local graph identity, manifests, dependency visibility,
  and locks. Their accepted resolution does not authorize publication or
  global ownership semantics.
- `ROADMAP-1.0` G1.1 prioritizes repeatable local/locked dependency resolution
  and explicitly defers unnecessary centralized services. G6.6 asks for
  release checksums, SBOM, licenses, and provenance as release evidence, but
  does not accept a package registry or package-upload protocol.
- `UNSUP-PACKAGES` in the support matrix explicitly excludes CLI project
  selection, installation, publication, and registry distribution. The matrix
  is Draft and cannot promote these capabilities to a stable public claim.
- The protocol inventory marks `PROTO-PACKAGE-MANIFEST`,
  `PROTO-PACKAGE-IDENTITY`, and `PROTO-LOCKFILE` as Experimental local
  protocols. `PROTO-BUILD-METADATA` and `PROTO-EVIDENCE` are Future with no
  reader, writer, schema, or fixtures; neither is a publication contract.
- Root `AGENTS.md` requires Accepted authority before public-protocol or
  semantic expansion, stable claims only after executable fixtures, exact
  `L-<DOMAIN>-<NUMBER>` diagnostics, deterministic/offline behavior, preserved
  UTF-8 byte spans, Unicode 17.0.0, and no placeholder public APIs.

## Evidence in this repository

The existing project implementation and fixtures provide deterministic local
manifest decoding, package graph traversal, path-free SHA-256 content/graph
identities, exported-module visibility, and canonical `ling.lock/1` bytes.
These are useful foundations for a future publication protocol, but they do
not establish:

1. authenticated publisher/source coordinates, ownership transfer, namespace
   collision or impersonation policy, and key rotation/revocation;
2. a canonical package archive/artifact format, file inclusion rules,
   permission/symlink policy, artifact identity, or reproducible build link;
3. checksum and signature algorithms, trust roots, signature envelopes,
   provenance attestations, SBOM/license requirements, or transparency logs;
4. registry upload/download/index semantics, yanking/deprecation visibility,
   mirrors, offline cache freshness, retry/error behavior, or installation
   transaction and rollback rules;
5. version selection, ranges, multiple versions, compatibility, migration,
   CLI commands, stable diagnostics, or positive/negative security fixtures.

Creating a registry crate, archive schema, signing API, cache directory,
publisher command, or placeholder protocol entry now would invent public
behavior and could make local package names or experimental hashes appear to
be globally authenticated identities.

## Required authority before implementation

An accepted publication and supply-chain RFC/decision must define, at minimum:

1. A versioned package coordinate and authenticated publisher/source identity,
   ownership, namespace, collision, transfer, key creation, rotation,
   revocation, and trust-root rules, with Unicode/display names kept separate
   from canonical identity.
2. The manifest, package archive, source/artifact inclusion, canonical
   serialization, path/symlink/permission policy, semantic/content/build
   hashes, target/profile inputs, and deterministic reproducibility boundary.
3. Version policy, dependency selection, lock integration, yanked/deprecated
   states, install/update/rollback transactions, unknown-field behavior,
   migration and compatibility readers, and all `ling` CLI command/exit/JSON
   behavior.
4. Checksum, signature, provenance, SBOM, license, and transparency formats;
   verification order and failure semantics; offline cache/mirror behavior;
   network, credential, and environment isolation; and supply-chain threat
   responses.
5. Stable bilingual diagnostics with registered error codes and original
   UTF-8 byte spans, plus deterministic/offline positive, negative, corruption,
   collision, replay, signature, revocation, yanking, mirror, cache, migration,
   cross-platform, Unicode 17.0.0, and cross-process fixtures.
6. Protocol-inventory, schema-registry, support-matrix, dependency-lock,
   traceability, and status-generator updates performed atomically with the
   implementation, without exposing host paths, allocation order, map order,
   or unverified toolchain details as Ling semantics.

## Compatibility and deferred work

This audit changes no compiler, resolver, evaluator, VM, manifest, lockfile,
identity algorithm, diagnostic, schema, CLI, package, registry, cache,
signature, provenance, dependency, or public API behavior. It preserves the
accepted `ling`/`.ling` names, RFC-0002 local/offline boundary, Unicode
17.0.0, original UTF-8 spans, checked Typed Core input rule, and explicit
Experimental/Preview/Future/Unsupported states.

It deliberately adds no registry, publisher identity, artifact/archive,
signature or provenance implementation, package installer, mirror/cache,
network client, yanking API, migration tool, diagnostic, dependency, schema,
protocol inventory entry, or placeholder. Future implementation remains
deferred until publication authority is Accepted and its executable security,
compatibility, reproducibility, and offline evidence exists.
