# TRAIT-1307 Implementation Report

## Scope

This report records the current vertical slice authorized by Accepted RFC-0021
for the G1/v0.1 static Trait boundary. It is not a v0.0.1 support-matrix
claim.

## Normative clauses covered

- RFC-0005 §1.5, §2, and §4.1–§4.2: concrete coherence selection, immutable
  witness lowering, and no unresolved obligation in executable Typed Core.
- DEC-0027: witness/member ordering and canonical Checked Core identity.
- RFC-0021 §1–§8: qualified static member calls, deterministic implementation
  identities, checked backend mapping, interpreter dispatch, existing direct
  calls, partial application, and Semantic Program identity.

## Changes

- Resolver indexes Trait and implementation member identities and resolves local
  and imported qualified member references.
- Type checking records a witness-bound `TraitMemberCall` for each concrete
  application and rejects bare/unsatisfied calls before Typed Core publication.
- Effects, Semantic identity, interpreter, bytecode lowering, and the v1.2 VM
  consume the checked implementation identity; no backend re-runs selection.
- Bytecode uses existing `Call` and `CallClosure` instructions; no wire format
  or verifier revision changed.

## Evidence

- `ling-types`: 36 tests pass, including positive dictionary lowering and bare/
  unsatisfied rejection.
- `ling-effects`: 9 tests pass.
- `ling-semantic`: 13 unit tests and 5 project tests pass, including witness
  identity and implementation-body invalidation.
- `ling-vm` execution suite: 22 tests pass, including interpreter/v1.2 VM
  differential execution with a two-argument member and partial application.
- `cargo fmt --all` passes after formatting.

## Compatibility

- Diagnostics continue using the existing `L-<DOMAIN>-<NUMBER>` registry; no
  allocation or message-code change was required.
- Semantic schema markers and bytecode revisions are unchanged.
- Canonical witness bytes include Trait/impl/member identity and omit paths,
  spans, allocation details, and map iteration order.
- Unicode 17.0.0 tables and original UTF-8 byte spans are unchanged.

## Specification gaps and conflicts

- `docs/SEMANTICS.md` still excludes Trait from v0.0.1; RFC-0021 targets the
  v0.1 Living implementation slice and does not override that support claim.
- Generic receiver substitution, blanket implementations, trait objects,
  associated types, default methods, and specialization remain outside the
  accepted contract.

## Deferred work

Add ambiguous/malformed-witness and over-application differential fixtures,
cross-module/package determinism evidence, remaining v1.0/v1.1 aggregate
coverage, and downstream IDE/LSP projections before marking TRAIT-1307 Done.
