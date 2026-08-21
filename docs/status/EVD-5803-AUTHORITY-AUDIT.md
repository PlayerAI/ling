# EVD-5803 Authority Audit

- Task: `EVD-5803` — Reproducible Build Binding
- Plan: `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:557-564`
- Release: G5
- Status: `BlockedSpec`

## Decision

EVD-5803 is `BlockedSpec`. The execution plan proposes rebuilding from an
Evidence Bundle manifest in a controlled environment and comparing source and
Semantic IDs, object/binary hashes, accepted documented nondeterminism, and
generated source/proof provenance. It does not define the manifest, hermetic
input closure, target/toolchain environment, artifact identity, equivalence
relation, or acceptable nondeterminism.

No accepted specification authorizes a reproducible-build binding or a claim
that two artifacts are equivalent. EVD-5801 and EVD-5802 are blocked, and both
`PROTO-EVIDENCE` and `PROTO-BUILD-METADATA` are Future without schemas or
fixtures. Implementing this task would invent release identity and could turn
host-specific hashes or an incomplete rebuild into a false reproducibility
claim.

## Normative traceability

- `09-G5-V0.5-CRITICAL.md:557-564` is a non-normative checklist. It does not
  define which inputs are in the build closure, how environment/toolchain
  identity is represented, how object/binary bytes are normalized, or how
  allowed nondeterminism is registered and compared.
- `PROTO-BUILD-METADATA` is Planned public/Future with no version, schema,
  canonical encoding, reader/writer, migration policy, or fixtures. It states
  that toolchain identity, target/profile inputs, artifact identity, and cache
  boundaries require accepted specifications.
- `PROTO-EVIDENCE` is also Planned public/Future and leaves identity,
  provenance, checksums, verification, redaction, and migration undefined.
  EVD-5803 cannot establish a build-binding protocol independently of EVD-5801.
- `docs/ROADMAP-1.0.md:480-490` asks for hermetic build identity in a future
  evidence bundle, and `:540-546` calls for build/performance baselines. The
  roadmap is Planning authority and does not define reproducible-build
  semantics.
- Accepted DEC-0012 defines canonical Semantic/Program identity inputs, and
  RFC-0002 defines package/content/graph and lock identities; neither defines
  object/binary artifact identity or a build equivalence relation. RFC-0014
  explicitly excludes build-artifact identity, cache keys, signatures, source
  bundles, linkers, and package distribution, and says bytecode digests are not
  Semantic IDs.
- Accepted DEC-0019/0021/0022 cover internal query determinism and a
  disposable persistent query cache. They exclude persistent compatibility
  claims, cache migration, and release artifact reproducibility; they cannot be
  reused as a build-binding authority.

## Evidence in this repository

There is no hermetic build manifest, reproducible-build runner, target/toolchain
identity schema, artifact normalization rule, object/binary comparison
contract, nondeterminism registry, generated provenance linker, or
reproducibility fixture under `crates/`, `tests/`, or `schemas/`. Existing
Semantic/package/lock identities, bytecode round trips, and compiler query
determinism tests have narrower accepted scopes. No `ling` CLI, LSP request,
diagnostic, or public protocol claims EVD-5803 support.

## Required authority before implementation

An accepted RFC or replacement decision must define, at minimum:

1. A versioned hermetic build manifest and canonical input closure covering
   source/Audit Source, Semantic/Program IDs, dependencies and lockfile,
   compiler/toolchain, target/profile, build flags, generated inputs, native
   tools, environment controls, and TCB.
2. Artifact identity and comparison: object/binary boundaries, canonical hash
   domains, debug/symbol/metadata policy, archive/link order, path and timestamp
   normalization, accepted nondeterminism, equivalence versus byte identity,
   and incompatible-output behavior.
3. Provenance linkage for generated source, proof certificates, model/replay/
   timing reports, FFI/Target Packages, and review records, with original
   UTF-8 spans, stable Semantic IDs, and explicit source/snapshot revisions.
4. Rebuild isolation and determinism rules across supported processes/hosts,
   offline/locked dependency selection, target packages, compiler versions,
   cache state, and filesystem enumeration. Host paths, addresses, timestamps,
   allocator layout, machine-local randomness, and debug text must not become
   Ling identity unless an explicit non-claim records them outside identity.
5. Versioned result, migration, and fail-closed diagnostics for missing inputs,
   lock/toolchain drift, hash mismatch, unknown/unsupported nondeterminism,
   generated-provenance mismatch, stale Semantic IDs, malformed manifests, and
   unavailable targets, using registered bilingual `L-<DOMAIN>-<NUMBER>` codes.
6. Offline positive, negative, cross-process/host, clean/warm cache, target/
   profile variation, migration, corruption, Unicode 17.0.0, BOM/CRLF,
   source-span, generated-provenance, repeated-build, and deterministic fixture
   suites. The evidence must disclose limits and must not claim reproducibility
   for an unspecified environment.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, or
support claim. It preserves the accepted `ling` CLI and `.ling` source
extension, original UTF-8 spans, Unicode 17.0.0, deterministic identity rules,
and the checked Typed Core boundary. It deliberately adds no build manifest,
artifact hash protocol, reproducibility runner, target/toolchain dependency,
diagnostic, CLI command, or placeholder API, and it introduces no stale `zero`
names.

EVD-5803 remains deferred until EVD-5801/EVD-5802, Critical Profile,
target/ABI, reproducible-build, provenance, model-check, replay, timing,
Contract/Proof, and evidence authorities are Accepted with executable fixtures.
