# PROOF-5501 Authority Audit

Task: `PROOF-5501` — Proof IR
Release: G5
Status: `BlockedSpec`

## Outcome

`PROOF-5501` is not implementable from the current accepted authority. The
execution plan proposes a small, versioned proof/verification intermediate
layer containing sorts and types, terms, hypotheses, theorems,
arithmetic/memory axioms, proof-step or certificate references, and source
provenance. It does not define a proof language, well-formedness rules,
inference kernel, soundness theorem, canonical encoding, trust boundary, or
the relation between a Contract claim and a verified proof.

The task depends on RFC-K505, which is absent and has no Accepted replacement.
`CTR-5401` through `CTR-5407`, `PROF-5101` through `PROF-5104`, and the
Critical profile/evidence work remain `BlockedSpec`; `GAP-CRITICAL-PROFILE-001`
is open and `PROTO-EVIDENCE` is Future without a schema or fixtures. Creating
an IR now would freeze unresolved arithmetic, memory, alias, Effect,
boundedness, timing, Node, FFI, and external-assumption semantics. No proof IR,
certificate format, parser, checker dependency, diagnostic, public protocol,
or placeholder API should be added.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:386-398` is a
  non-normative checklist. Its field bullets do not define grammar, typing,
  proof rules, canonical bytes, source mapping, validation, or soundness.
- The plan's specification gate assigns the Verification Interface to
  RFC-K505 and separates Model Checking (RFC-K506) and Evidence Bundle
  (RFC-K507). None of these RFCs is present as an accepted repository
  authority.
- `docs/SEMANTICS.md:1214-1238,1857-1866` is Draft. Its Contract status
  sketch and TCB list cannot authorize a proof calculus or proof-kernel claim.
  `docs/LANGUAGE.md` likewise remains Draft and reserves later proof/profile
  behavior.
- `docs/governance/gap-register.toml` records open
  `GAP-CRITICAL-PROFILE-001`; it explicitly leaves Critical Core, Contract
  proof/runtime, boundedness, model-check claims, and evidence boundaries
  unaccepted, and requires independent-checker and reproducible-build
  evidence.
- `docs/governance/protocol-inventory.toml` records `PROTO-EVIDENCE` as
  Planned public/Future, with no current version, public schema, canonical
  form, reader/writer policy, migration tool, or fixtures. It cannot be used
  as a proof IR or certificate schema.
- Accepted RFC-0014 through RFC-0020 define the Seed bytecode formats,
  verifier-gated VM, effect/capability host boundary, differential execution,
  and cancellation/resource evidence. The bytecode verifier and the internal
  Trait solver are not source-level proof kernels and do not authorize a new
  proof representation.
- Accepted DEC-0012 fixes semantic identity/canonical-byte boundaries and
  DEC-0002 fixes original UTF-8 spans; neither defines proof terms, theorem
  checking, or proof/evidence identity.

## Repository evidence

The repository has no proof IR crate, certificate/query decoder, proof kernel,
assumption registry, source-to-proof mapper, or proof fixtures. The internal
`ling-types` Trait solver resolves accepted Seed type obligations, and
`ling-bytecode` verifies executable bytecode invariants; neither consumes or
produces Contract proofs. `crates/ling-types/src/checked_core.rs` contains an
internal Trait dictionary witness boundary that is deliberately not attached
to the public TypedProgram, Semantic Graph, interpreter, VM, CLI, LSP, or
bytecode protocol.

## Required authority before implementation

An accepted RFC-K505 replacement coordinated with RFC-K503, RFC-K504, RFC-K506,
RFC-K507, and the Critical profile must define at least:

1. A versioned proof/verification IR grammar, sorts, terms, hypotheses,
   theorem and proof-step rules, well-formedness, normalization, canonical
   bytes, unknown fields, migration, and stable semantic/proof IDs with
   original UTF-8 source spans.
2. The exact checked Contract/Typed-Core-to-Proof translation, including
   arithmetic, memory/alias, ownership, Effects/capabilities, bounds,
   recursion, timing, Node/Task/Actor, FFI, ABI, and external assumptions;
   distinguish proof from runtime checks, tests, bounded model checks, and
   unverified claims.
3. A small trusted kernel/checker boundary and soundness statement, proof
   certificate/query linkage, TCB membership, resource/depth limits,
   deterministic behavior, fail-closed malformed/corrupt/unknown handling,
   and independent verification requirements.
4. Evidence/provenance linkage, invalidation and dependency identity,
   redaction, checksums/signatures, replay/counterexample rules, bilingual
   `L-<DOMAIN>-<NUMBER>` diagnostics, and rules preventing proof metadata from
   changing program semantics or Ling identity.
5. Offline positive/negative, malformed, adversarial, arithmetic/alias/effect,
   boundedness, proof-rejection, Unicode 17.0.0, CRLF/BOM/source-span,
   migration, deterministic, checker differential, and corruption fixtures
   before a proof IR is exposed or a support claim is made.

## Compatibility and deferred work

This audit changes no parser, resolver, AST/HIR, Checked Typed Core,
evaluator, bytecode, VM, Trait solver, diagnostics, schema, CLI, LSP,
dependency, Semantic ID, or public protocol. It preserves checked-only
execution, accepted Seed semantics, original UTF-8 byte spans, Unicode
17.0.0, deterministic ordering, and exclusion of host paths, timing,
addresses, and debug output from Ling identity. It makes no proof, soundness,
certification, or Critical support claim.

Implementation remains deferred until RFC-K505 or an accepted replacement and
the coordinated Contract, boundedness, model-checking, Critical profile, and
evidence authorities provide executable fixtures. Do not add a proof IR,
certificate/query format, proof kernel, assumption registry, parser,
diagnostic allocation, CLI/LSP route, public protocol, support claim, or
placeholder API while those authorities remain unresolved.
