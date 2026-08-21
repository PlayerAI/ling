# PKG-6402 Authority Audit

- Task: `PKG-6402` — Hermetic Build
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:226-237`
- Release: G6
- Status: `BlockedSpec`

## Decision

`PKG-6402` is `BlockedSpec`. The G6 checklist requires typed build nodes,
declared inputs and outputs, explicit Capabilities, sandboxing,
determinism, hashing, offline replay, and rejection of undeclared environment
or network access. It does not define the build-graph data model, node/plugin
boundary, execution and resource policy, artifact identity, failure/diagnostic
contract, or a versioned public protocol.

`LANGUAGE.md` and `SEMANTICS.md` describe a hermetic-build design direction,
but both are design drafts and do not supply Accepted authority for a new
post-Seed build system. Their broad statements cannot freeze irreversible
behavior such as process isolation, filesystem/network access, capability
meaning, build scripts, profile selection, artifact hashes, or replay files.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:226-237` is a non-normative checklist. It
  names safety properties but has no typed schema, command/exit behavior,
  sandbox implementation boundary, resource limits, hash version, or migration
  rules.
- `LANGUAGE.md:1249-1274` and `SEMANTICS.md:1789-1827` sketch typed build
  nodes, declared inputs/outputs/Capabilities, dependency identities, and
  typed code generation. They remain design-draft material; `SEMANTICS.md`
  itself requires an Accepted RFC before implementation fixes unresolved
  irreversible semantics.
- Accepted `DEC-0010` defines the source-language `State<T>` Effect and
  Capability model. It does not define build-process capabilities, sandbox
  authority, host access, or a build graph protocol.
- Accepted `DEC-0019` authorizes only an internal in-memory incremental query
  boundary and explicitly forbids a public cache protocol, CLI, schema, or
  persisted query graph. Accepted `DEC-0022` is limited to an opt-in disposable
  line-index cache; it is not a hermetic build executor or artifact contract.
- Accepted `RFC-0002` keeps the project boundary local and offline and excludes
  arbitrary shell build scripts, plugins, generated source, target/profile
  defaults, binary packages, and artifact metadata. It cannot be extended by
  this task without a new accepted decision.
- Open `GAP-INCREMENTAL-CACHE-001` leaves persistent dependent-query
  serialization, migrations, eviction, and corruption policy unresolved.
  Open `GAP-SEMANTIC-HASH-LIFECYCLE-001` blocks stable hash algorithm and
  identity migration rules. Both affect a build graph's reproducibility and
  cache/artifact identity.
- `PROTO-BUILD-METADATA` is `Future` with no version, reader, writer, schema,
  or fixtures. The protocol inventory contains no accepted hermetic-build,
  build-script, artifact, or replay protocol.
- `ROADMAP-1.0` requires typed build steps and offline/reproducible evidence as
  goals, while its G6 and deferred-work sections prohibit treating unsupported
  package services, profiles, targets, or public APIs as stable without
  Accepted authority and executable evidence.
- Root `AGENTS.md` requires Accepted authority before public-protocol or
  semantic expansion, checked Typed Core inputs, deterministic/offline builds,
  preserved UTF-8 byte spans, Unicode 17.0.0, bilingual registered
  diagnostics, and no placeholder APIs.

## Evidence in this repository

The repository currently provides bounded source/package identities, local
manifest and lock handling, internal query/cache slices, and deterministic
compiler/VM evidence. These pieces establish useful inputs for a future
hermetic build design, but they do not establish:

1. a canonical typed node set, graph ordering, dependency-edge semantics,
   declared input/output types, Capability vocabulary, or plugin trust model;
2. sandbox boundaries for processes, filesystem paths, environment variables,
   clocks, randomness, network, credentials, subprocesses, symlinks, and
   resource limits;
3. canonical build-plan, toolchain, target/profile, generated-source,
   artifact, provenance, and reproducible-output identity bytes, including hash
   algorithm/version and migration behavior;
4. build-script admission, installation/build ordering, cancellation,
   failure-atomicity, retry, cache, offline replay, or cross-process semantics;
5. `ling` commands, exit classes, stable bilingual diagnostics, schemas,
   compatibility readers, and security/determinism fixtures for hostile build
   inputs.

Adding a build executor, shell wrapper, sandbox crate, manifest fields,
artifact schema, cache protocol, environment allowlist, or public command now
would guess semantics and could expose host state or untrusted dependency code.

## Required authority before implementation

An accepted hermetic-build RFC/decision must define, at minimum:

1. The typed node vocabulary and versioned graph schema: node kinds, inputs,
   outputs, dependency edges, ordering, generated-source rules, plugin/code
   generator boundary, and checked Typed Core handoff.
2. Capability and sandbox policy: filesystem roots, environment and credential
   access, network, subprocesses, clock/randomness, symlink behavior, resource
   quotas, cancellation, failure isolation, and the exact deny-by-default
   behavior for undeclared access.
3. Canonical build-plan and output identity: compiler/toolchain versions,
   language/Unicode versions, package/lock/profile/target inputs, source and
   generated bytes, artifact/provenance/SBOM references, hash domains,
   determinism, and cross-version migration.
4. Build-script and dependency policy: admission and review, no arbitrary
   shell, reproducible typed transformations, install/build order, cache
   ownership, offline replay, corruption handling, retries, and transaction
   rollback.
5. CLI and diagnostics: command spelling, manifest/lock/profile/target flags,
   exit classes, human/JSON output, stable bilingual `L-<DOMAIN>-<NUMBER>`
   codes, original UTF-8 byte spans, and path/environment redaction.
6. Positive, negative, corruption, hostile-input, capability, sandbox,
   determinism, replay, cross-process, cross-platform, Unicode 17.0.0, and
   migration fixtures, with protocol inventory, schema, support matrix,
   dependency, traceability, and status updates generated atomically.

## Compatibility and deferred work

This audit changes no compiler, resolver, evaluator, VM, package graph,
manifest, lockfile, cache, identity algorithm, diagnostic, schema, CLI,
profile, target, backend, dependency, or public API behavior. It preserves the
accepted local/offline package boundary, internal query/cache limits, checked
Typed Core rule, original UTF-8 spans, Unicode 17.0.0, and explicit
Experimental/Preview/Future/Unsupported states.

It deliberately adds no build graph, executor, shell adapter, sandbox,
manifest field, artifact or replay schema, cache protocol, environment or
network capability, migration tool, diagnostic, dependency, public command, or
placeholder. Future implementation remains deferred until hermetic-build
authority and executable security, reproducibility, offline, and compatibility
evidence are Accepted.
