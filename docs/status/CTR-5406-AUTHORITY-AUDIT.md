# CTR-5406 Authority Audit

Task: `CTR-5406` — Contract-aware optimizer rules
Release: G5
Status: `BlockedSpec`

## Outcome

`CTR-5406` is not implementable from the current accepted authority. The
execution plan says that only `Proved` Contract facts may drive
semantics-changing optimization and that `RuntimeChecked`, `Assumed`, and
`Unknown` facts must not justify removing safety checks. That is a useful
safety principle, but it does not define the Contract status algebra, proof
soundness boundary, optimizer transformation set, or the evidence needed to
show that a transformation preserves Ling behavior.

All preceding Contract tasks (`CTR-5401` through `CTR-5405`) are
`BlockedSpec`. RFC-K503 and RFC-K505 are absent, `GAP-CRITICAL-PROFILE-001`
is open, and `PROTO-EVIDENCE` is Future and unversioned. The repository has
no Contract-aware optimizer, pass contract, proof/assumption interface, or
optimization protocol. Implementing this item would silently turn Draft
status names and a planning sentence into language semantics and could alter
effects, Fault visibility, evaluation order, resource behavior, or source
identity. No optimizer pass, safety-check elimination, diagnostic,
dependency, public protocol, or placeholder API should be added.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:371-373` is a
  non-normative checklist. It does not specify which transformations are
  semantics-changing, how a fact is authenticated, or what happens when a
  proof is absent, stale, unknown, or invalid.
- `docs/ROADMAP-1.0.md:468-474` repeats the intended G5 Contract direction,
  but the roadmap is planning authority only and does not authorize a pass
  pipeline or a proof-backed optimization claim.
- `docs/SEMANTICS.md:1214-1238` is Draft. Its Contract status examples
  (`Proved`, `RuntimeChecked`, `ModelChecked`, `Tested`, `Assumed`, and
  `Unverified`) and optimization notes cannot establish a public status
  schema or soundness relation.
- `docs/LANGUAGE.md:775-792` is also Draft and uses a different status set
  (`Proved`, `RuntimeChecked`, `Tested`, `Assumed`, and `Unverified`), with no
  accepted ordering or optimizer contract.
- The plan assigns Contract syntax/runtime/VC/solver work to RFC-K503 and
  RFC-K505; neither RFC exists as an accepted document. The open
  `GAP-CRITICAL-PROFILE-001` explicitly includes unresolved Contract proof,
  boundedness, model-checking, and evidence boundaries.
- `PROTO-EVIDENCE` in `docs/governance/protocol-inventory.toml` is Planned
  public/Future, with no version, schema, canonical form, reader/writer
  policy, migration tool, or fixtures. It cannot authenticate an optimization
  fact or support a release claim.
- Accepted RFC-0014 through RFC-0020 define Seed bytecode/VM lowering,
  verification, effects/host Faults, differential execution, and
  cancellation/resource evidence. They do not authorize optimizer passes or
  prove source-level Contract facts. RFC-0015 explicitly leaves tail-call
  optimization out of scope, and RFC-0014 separates bytecode identity from
  semantic identity.
- Accepted DEC-0019's later optimization concerns deterministic internal query
  scheduling only; it does not define a Ling optimizer or change program
  semantics.

## Repository evidence

There is no optimizer/pass/NIR/native implementation under `crates/`, no
Contract proof or assumption reader, no optimization metadata schema, and no
optimized-versus-unoptimized semantic/differential fixture set. The Seed
lowerer and verifier accept checked Typed Core and enforce executable
invariants; they do not establish source Contract proofs or permit removal of
safety checks. Rust compiler optimizations and bytecode lowering choices are
not Ling optimizer semantics.

## Required authority before implementation

Accepted Contract, verification, profile, and evidence decisions must define
at least:

1. A versioned status/evidence model with a precise trust ordering and
   provenance for `Proved`, runtime/model/test evidence, `Assumed`, unknown,
   stale, invalid, and failed results. The model must resolve the current
   Draft/plan vocabulary conflict without treating tests or runtime checks as
   proofs.
2. The proof/checker/assumption boundary, canonical IDs and original UTF-8
   spans, profile admission rules, invalidation on source or dependency
   changes, and fail-closed behavior for missing, unknown, corrupt, or
   unverifiable facts.
3. A transformation catalogue and per-pass preconditions/postconditions for
   constant folding, check elimination, dead-code removal, inlining, effect
   and capability preservation, evaluation order, short-circuiting, Fault and
   cleanup visibility, ownership/resources, Task/Actor/Node behavior, numeric
   overflow, FFI, ABI, stack/debug mappings, and Semantic IDs.
4. Explicit rules for whether and when a proof may change runtime checks or
   observable behavior, with deterministic pass ordering, bounded resource
   use, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and no host timing,
   allocation, path, address, or debug data in Ling identity.
5. Offline positive/negative, proof-rejection, stale/corrupt/unknown,
   effect/Fault/cleanup, Unicode 17.0.0, CRLF/BOM/source-span, migration,
   optimized/unoptimized interpreter/VM differential, and profile-gating
   fixtures before any optimization support claim.

## Compatibility and deferred work

This audit changes no parser, resolver, AST/HIR, Checked Typed Core,
evaluator, bytecode, VM, optimizer, diagnostic registry, schema, CLI, LSP,
dependency, or public protocol. It preserves checked-only execution,
accepted Seed bytecode/VM semantics, Unicode 17.0.0, original UTF-8 byte
spans, deterministic ordering, and exclusion of host details from Ling
identity. It makes no optimization, proof, performance, or safety-check
elimination claim.

Implementation remains deferred until RFC-K503/RFC-K505 or accepted
replacements, Critical profile/evidence decisions, and executable semantic
preservation fixtures define the Contract and optimization boundary. Do not
add an optimizer pass, proof/assumption schema, safety-check elimination,
diagnostic allocation, CLI/LSP route, public protocol, support claim, or
placeholder API while those authorities remain unresolved.
