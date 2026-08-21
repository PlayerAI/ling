# PKG-6404 Authority Audit

- Task: `PKG-6404` — Supply-Chain Attack Tests
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:248-261`
- Release: G6
- Status: `BlockedSpec`

## Decision

`PKG-6404` is `BlockedSpec`. The G6 checklist names dependency confusion,
namespace spoofing, Unicode-confusable packages, malicious manifests, archive
traversal, decompression bombs, signature/key mismatch, yanked packages,
compromised caches, and build-capability escalation. Several of those attacks
require registry, archive, signature, yanking, cache, or hermetic-build
protocols that are not accepted or implemented. The checklist does not define
the threat model, trust boundary, artifact format, test oracle, or release
claim needed to turn it into conformance tests.

The accepted local package protocol does provide a bounded security slice:
path and symlink escape rejection, graph-local name collision checks, Unicode
17.0.0 identifier/display-name validation, cycle and visibility checks,
canonical content/graph identities, lock corruption rejection, no ambient
network or code execution, and failure-atomic resolution. Those existing
fixtures must remain intact, but they do not authorize inventing attack tests
for absent public package or build protocols.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:248-261` is a non-normative attack-test list.
  It has no accepted input schemas, artifact/archive rules, security levels,
  trust roots, expected diagnostics, or mapping from each attack to a public
  protocol.
- Accepted `RFC-0002` defines only local/offline manifests, vendored path
  dependencies, graph-local package names, path-free SHA-256 identities,
  canonical `ling.lock/1`, bounded traversal, and structured project errors.
  It explicitly excludes registries, publisher ownership, installation,
  network/Git sources, mirrors, signatures, transparency logs, arbitrary build
  scripts, plugins, generated source, binary/artifact metadata, and version
  ranges.
- Accepted RFC-0002 clauses already reject path traversal, root escape,
  invalid logical paths, duplicate/colliding package identities, cycles,
  unknown manifest/lock fields, malformed/corrupt locks, network requests,
  ambient environment lookup, and dependency code execution. These are local
  conformance boundaries, not a global supply-chain security protocol.
- `UNSUP-PACKAGES` explicitly keeps publication, package installation, and
  registry distribution out of scope. `PROTO-PACKAGE-MANIFEST`,
  `PROTO-PACKAGE-IDENTITY`, and `PROTO-LOCKFILE` remain Experimental local
  protocols; `PROTO-BUILD-METADATA` is Future. There is no accepted archive,
  signature, provenance, yanking, registry-cache, or build-capability protocol.
- Accepted `DEC-0022` protects an explicitly disposable internal line-index
  cache with bounded envelopes and checksums. It explicitly does not define a
  public cache directory, shared package cache, migration, or registry cache.
  Accepted `DEC-0019` and `DEC-0021` similarly authorize only internal query
  boundaries and deterministic scheduling, not dependency execution.
- `ROADMAP-1.0` requires security and supply-chain evidence for a 1.0 release,
  but also requires Accepted authority, executable fixtures, and explicit
  support claims; it does not promote its threat list into a protocol.
- Root `AGENTS.md` requires Accepted authority before public-protocol or
  semantic expansion, deterministic/offline tests, bilingual registered
  diagnostics, preserved UTF-8 byte spans, Unicode 17.0.0, checked Typed Core
  inputs, and no placeholder APIs.

## Evidence in this repository

Existing project and Unicode fixtures cover substantial local defenses:

1. manifest size/UTF-8/TOML/unknown-field/resource checks, logical-path
   normalization, root containment, symlink-aware traversal, and bounded
   dependency discovery;
2. graph-local package-name and content-identity collision rules, cycle and
   visibility errors, deterministic traversal, path-free SHA-256 package and
   graph identities, and canonical lock bytes;
3. malformed, truncated, unknown-version, checksum, ordering, dangling-edge,
   and mismatch lock failures with failure-atomic updates;
4. Unicode 17.0.0 XID/NFC/confusable/bidi/hidden-character policy and
   original UTF-8 diagnostic spans; and
5. explicit offline/no-network/no-ambient-environment/no-dependency-code
   execution boundaries in RFC-0002.

They do not establish:

- a globally authenticated namespace or dependency-confusion oracle;
- package archives, decompression limits, symlink/permission preservation,
  or archive traversal semantics;
- signing/key/trust/revocation/rotation, provenance, SBOM, transparency,
  yanked/deprecated package, or registry cache behavior; or
- hermetic build capabilities, sandbox escalation rules, package installation,
  network retry/isolation, or a stable security-diagnostic matrix.

Adding fixtures for these absent surfaces would either test an invented
protocol or encode implementation details as security promises.

## Required authority before implementation

An accepted package-security and test strategy must define, at minimum:

1. The threat model and trust boundaries for local projects, registries,
   publishers, mirrors, caches, installers, archives, build tools, and hosts;
2. canonical package/archive/artifact inputs, decompression and resource
   limits, path/symlink/permission policy, content/build hashes, and the
   deterministic oracle for every identity and output;
3. authenticated namespace/publisher coordinates, signature/key algorithms,
   trust roots, rotation/revocation, provenance/SBOM/license/transparency,
   yanking/deprecation, and dependency-confusion/Unicode-confusable policy;
4. installer, registry, mirror/cache, hermetic-build, network/credential,
   rollback, retry, and failure-atomicity behavior, including offline mode;
5. stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, severity and facts,
   original UTF-8 byte spans, CLI/JSON exit behavior, and compatibility/
   migration rules; and
6. positive and negative fixtures for every listed attack, resource/fuzz and
   cross-process determinism evidence, Unicode 17.0.0, platform/path cases,
   and generated protocol/support/traceability/status drift checks.

Until those authorities exist, tests may extend only the already accepted
local manifest/graph/lock and Unicode boundaries without naming them as full
1.0 supply-chain coverage.

## Compatibility and deferred work

This audit changes no manifest, resolver, package graph, identity, lockfile,
Unicode policy, cache, build, diagnostic, schema, CLI, dependency, or public
API behavior. It preserves RFC-0002's local/offline guarantees, existing
bounded rejection and failure-atomicity evidence, Unicode 17.0.0, original
UTF-8 spans, checked Typed Core boundaries, and explicit
Experimental/Preview/Future/Unsupported states.

It deliberately adds no registry/archive parser, decompression layer,
signature verifier, publisher trust store, yanking state, package cache,
hermetic-build sandbox, security diagnostic, dependency, public protocol,
security claim, or placeholder. Future attack coverage remains deferred until
the governing package, registry, artifact, cache, and build authorities are
Accepted and executable fixtures can state an unambiguous oracle.
