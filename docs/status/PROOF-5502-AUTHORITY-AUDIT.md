# PROOF-5502 Authority Audit

Task: `PROOF-5502` — Independent Checker
Release: G5
Status: `BlockedSpec`

## Outcome

`PROOF-5502` is not implementable from the current accepted authority. The
execution plan proposes a `zero-proof-check` command or library with no
compiler global state, offline operation, bounded input size/depth,
deterministic behavior, fuzzed decoding, an explicit TCB, and machine-readable
verification results. These are quality constraints, not a checker
algorithm, proof soundness statement, certificate format, result schema, or
public CLI contract. The `zero-proof-check` name is also a stale placeholder;
the accepted public CLI is `ling`.

RFC-K505 is absent, and there is no accepted replacement defining the Proof
IR, certificate/query boundary, trusted kernel, assumptions, timeout/unknown
semantics, or evidence protocol. `PROOF-5501`, the Contract tasks, and the
Critical profile/evidence work remain `BlockedSpec`; `GAP-CRITICAL-PROFILE-001`
is open and `PROTO-EVIDENCE` is Future without a schema or fixtures. No
independent proof checker, dependency, command, result schema, diagnostic,
TCB registry, or placeholder API should be added.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:400-410` is a
  non-normative checklist. It does not define the proof language, checker
  kernel, certificate validation, soundness relation, input envelope, result
  states, or CLI naming.
- The plan assigns proof obligations/checker/trusted assumptions to RFC-K505,
  and evidence/independent verification to RFC-K507. Neither RFC is present as
  an accepted document; RFC-K506 model-check semantics are also absent.
- `docs/governance/gap-register.toml` records open
  `GAP-CRITICAL-PROFILE-001`, which leaves Contract proof/runtime,
  boundedness, model-check claims, and evidence boundaries unaccepted and
  requires independent-checker and reproducible-build evidence.
- `docs/governance/protocol-inventory.toml` records `PROTO-EVIDENCE` as
  Planned public/Future with no version, public schema, canonical form,
  reader/writer policy, migration tool, or fixtures. A checker result cannot
  be machine-readable public output until this protocol is accepted.
- Accepted RFC-0014 through RFC-0020 define Seed bytecode decoding,
  independent executable-bytecode verification, VM execution, host Faults,
  differential runs, and cancellation/resource evidence. The bytecode
  verifier checks executable invariants, not source Contract proofs; the VM
  never consumes proof certificates.
- `docs/decisions/0001-error-code-policy.md` and `DEC-0002` authorize the
  existing bilingual diagnostic/position boundaries only. They do not allocate
  checker result codes or define a proof-result schema.
- The plan's `zero-proof-check` spelling must not enter commands, fixtures,
  schemas, editor integration, or documentation of implemented behavior; the
  public name remains `ling` and source files remain `.ling`.

## Repository evidence

There is no proof-checker crate or binary, no proof/certificate decoder, no
trusted-kernel implementation, no TCB/assumption registry, and no
machine-readable proof-result schema or fixtures. `crates/ling-bytecode`
contains the independent decoder/verifier for `ling.bytecode/*`, while
`crates/ling-vm` accepts only verifier-created bytecode; neither is a source
Contract checker. `crates/ling-types/src/solver.rs` is an internal Trait
solver and is unrelated to proof certificates. The existing diagnostic JSON
writer is a Preview compiler/CLI format, not checker output.

## Required authority before implementation

An accepted RFC-K505/RFC-K507 replacement coordinated with Contract,
boundedness, model-checking, and Critical-profile decisions must define at
least:

1. The versioned Proof IR, certificate/query envelope, canonical bytes,
   source/Semantic-ID mapping, assumptions, proof kernel rules, soundness
   claim, TCB membership, and independent-checker trust boundary.
2. Input limits for bytes, terms, proof depth, recursion, memory, and wall
   resources; deterministic ordering and offline dependency policy; explicit
   malformed, corrupt, unsupported-version, timeout, `unknown`, invalid, and
   counterexample result semantics; and fail-closed behavior.
3. A versioned machine-readable checker-result schema with stable IDs/spans,
   provenance, checksums/signatures, redaction, unknown-field and migration
   rules, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and a fixed public CLI
   name/exit-code contract under `ling` if a command is authorized.
4. Independent validation and TCB disclosure, fuzz/property corpus, proof
   replay and compiler/checker differential evidence, reproducible build
   binding, and rules preventing checker output from changing Ling semantics
   or identity.
5. Offline positive/negative, malformed/deep/oversized, corrupt certificate,
   unsupported-version, unknown/timeout, Unicode 17.0.0, CRLF/BOM/source
   span, deterministic, migration, and cross-checker fixtures before any
   checker support claim.

## Compatibility and deferred work

This audit changes no parser, resolver, AST/HIR, Checked Typed Core,
evaluator, bytecode, VM, Trait solver, diagnostics, schema, CLI, LSP,
dependency, Semantic ID, or public protocol. It preserves checked-only
execution, accepted Seed bytecode/VM semantics, original UTF-8 byte spans,
Unicode 17.0.0, deterministic ordering, and exclusion of host paths, timing,
addresses, and debug output from Ling identity. It makes no proof, checker,
soundness, certification, or Critical support claim.

Implementation remains deferred until RFC-K505/RFC-K507 or accepted
replacements and executable proof/evidence fixtures establish the checker
boundary. Do not add `zero-proof-check`, a `ling` checker command, proof
parser, certificate/query schema, kernel, TCB registry, dependency,
diagnostic allocation, public protocol, support claim, or placeholder API
while those authorities remain unresolved.
