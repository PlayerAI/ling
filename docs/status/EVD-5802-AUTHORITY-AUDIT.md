# EVD-5802 Authority Audit

- Task: `EVD-5802` — Independent Verifier
- Plan: `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:543-555`
- Release: G5
- Status: `BlockedSpec`

## Decision

EVD-5802 is `BlockedSpec`. The plan sketches a verifier for a bundle command
(`zero evidence verify <bundle>`) that would check schema/version,
canonical/hash values, artifact links, proof certificates, test identity,
lock/toolchain data, missing/unknown fields, offline mode, and no code
execution. It does not define the bundle schema, result semantics, trust roots,
certificate format, verifier TCB, command/exit contract, or migration policy.

No accepted RFC-K507 or replacement authorizes this verifier. EVD-5801 is
blocked and `PROTO-EVIDENCE` is Future without a version, schema, reader/writer,
canonical form, verification rules, or fixtures. Implementing a verifier now
would either invent those semantics or incorrectly reuse the bytecode verifier,
Audit Source parser, or internal reports for a different trust boundary.

## Normative traceability

- `09-G5-V0.5-CRITICAL.md:543-555` is a non-normative checklist. It does not
  define inputs, canonical bytes, hash domains, evidence polarity, trust roots,
  signature or proof-certificate formats, unknown/missing-field behavior,
  process exits, or version migration.
- `docs/governance/protocol-inventory.toml:518-537` records `PROTO-EVIDENCE` as
  Planned public/Future, unimplemented, unversioned, non-canonical,
  schema-free, and fixture-free. Its authorities are only the roadmap and gap
  register, so it cannot authorize an independent verifier.
- `GAP-CRITICAL-PROFILE-001` remains Open and requires an independent checker,
  reproducible-build, boundedness, counterexample, and evidence schema
  decisions before Critical claims. Its candidate RFC-0012 is not present or
  Accepted.
- `docs/ROADMAP-1.0.md:480-498` requires a future bundle to be independently
  verifiable and offline, but the roadmap is Planning authority and supplies no
  verifier protocol or acceptance result model.
- Accepted RFC-0014's independent bytecode verifier checks untrusted bytecode
  structure and VM invariants; it does not verify a release evidence bundle.
  Accepted DEC-0015/`PROTO-AUDIT-SOURCE` isolate Audit Source parsing and
  explicitly do not construct executable Checked Programs. Neither supplies a
  bundle trust model. RFC-0019 and RFC-0020 only verify their bounded VM
  evidence.
- The plan's `zero` command is stale and not an accepted CLI name. The public
  CLI is `ling`; no evidence verification command or exit schema may be
  inferred from the execution plan.

## Evidence in this repository

There is no Evidence Bundle reader, writer, canonical manifest checker,
certificate/signature verifier, trust-root/TCB definition, migration tool, or
bundle verifier fixtures under `crates/`, `tests/`, or `schemas/`. The existing
`ling-bytecode` verifier, `ling.audit/0.1` parser, and governance checks have
distinct inputs and authorities. No `ling` CLI, LSP request, diagnostic, or
public protocol claims EVD-5802 support.

## Required authority before implementation

An accepted RFC or replacement decision must define, at minimum:

1. The versioned Evidence Bundle schema and canonical byte projection, then a
   verifier input model with required/optional fields, hash domains, size
   limits, unknown/missing-field policy, migration, and malformed/corrupt
   outcomes.
2. Typed artifact, source, Semantic ID, build, target/Profile/toolchain,
   dependency, test, proof, model-check, replay, timing/memory, FFI/TCB,
   assumption, AI, and review identities, including the exact linkage and
   provenance that the verifier must recompute. Host paths, addresses,
   timestamps, secrets, and debug output must not become Ling identity.
3. Evidence status/polarity and certificate/signature rules, trust roots,
   verifier version, TCB, key/revocation policy if applicable, and explicit
   non-claims. Bounded absence, measurements, Draft documents, or untrusted
   producer assertions must not be accepted as proof.
4. Independent verification isolation: no bundle-provided code, plugin,
   command, FFI, or deserialization hook may execute; define resource limits,
   network/offline behavior, deterministic ordering, and fail-closed handling
   for unsupported evidence or stale identities.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and process/exit
   semantics for invalid schema, hash/link mismatch, missing/unknown fields,
   invalid certificates, unavailable inputs, migration, trust failure, and
   resource exhaustion.
6. Offline positive, negative, corruption, unknown/missing, migration,
   tampered-link, invalid-certificate, no-code-execution, Unicode 17.0.0,
   BOM/CRLF, source-span, repeated-verification, and deterministic fixture
   suites.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, or
support claim. It preserves the accepted `ling` CLI and `.ling` source
extension, original UTF-8 spans, Unicode 17.0.0, deterministic identity rules,
and the checked Typed Core boundary. It deliberately adds no bundle verifier,
certificate/signature dependency, CLI command, diagnostic, schema, or
placeholder API, and it introduces no stale `zero` names.

EVD-5802 remains deferred until EVD-5801, Critical Profile, Contract/Proof,
boundedness, model-check, replay, timing, target/ABI, reproducible-build,
provenance, and evidence authorities are Accepted with executable fixtures.
Existing bytecode and Audit Source verifiers must not be advertised as the
future Critical evidence verifier.
