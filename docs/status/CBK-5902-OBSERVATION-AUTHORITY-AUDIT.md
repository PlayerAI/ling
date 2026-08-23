# CBK-5902-OBSERVATION Authority Audit — Lowering Validator Evidence

Status: BlockedSpec
Date: 2026-08-23

## Outcome

Accepted `DEC-0214` permits only test-local lowering-validator vocabulary. It
does not authorize a Native IR, validator input/output, equivalence theorem,
Contract/memory proof, source/binary correspondence, target checker,
diagnostic, protocol, or Native/Critical support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:599-609` is a
  non-normative validation checklist dependent on absent RFC-K508.
- `docs/status/CBK-5902-AUTHORITY-AUDIT.md` records missing route,
  representation, semantic, equivalence, proof/TCB, identity, failure, and
  fixture authority.
- `docs/IMPLEMENTATION.md` excludes Native/proof work from Seed; dependent
  gaps remain Open and `PROTO-ABI`/`PROTO-EVIDENCE` are Future.
- Accepted bytecode lowering/verifier and Interpreter–VM differential evidence
  have distinct portable scopes; `DEC-0213` selects no compiler route.

## Current implementation evidence

The observation adds one isolated test with sixty explicit validation-
boundary, representation, Core/target, semantic-check, identity/
correspondence, equivalence/proof/trust, failure, diagnostic, and fixture
boundaries. It sorts by explicit local rank, rejects duplicates, compares
opaque bytes for forward/reverse input order, and keeps every checklist item
distinct. No validator, IR, lowering, proof, target, diagnostic, protocol,
dependency, or support claim is introduced.

## Required authority and compatibility

Accepted authority must select and define the validated route and
representations; specify type/layout/control/value/Contract/memory/alias/
ownership/ABI behavior; establish source/binary identity, equivalence,
soundness, checker, trust, TCB, and fail-closed rules; define bilingual
diagnostics; preserve checked Typed Core, Semantic IDs, and original UTF-8
spans; and provide offline cross-target fixtures. Seed behavior, dependencies,
support state, and Unicode 17.0.0 remain unchanged.

## Deferred work

CBK-5902 validator implementation, Native/Critical representations,
correspondence and proof checking, target integration, diagnostics, protocols,
and support remain deferred until Accepted authority and executable evidence
exist. No placeholder lowering-validator API is created.
