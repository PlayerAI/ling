# EVD-5802-OBSERVATION Authority Audit — Independent Verifier Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0210` permits only test-local independent-verifier vocabulary. It
does not authorize a bundle input/result, parser, certificate/signature check,
trust root, verifier TCB, CLI command, diagnostic, protocol, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:543-555` is a
  non-normative verifier checklist.
- `docs/status/EVD-5802-AUTHORITY-AUDIT.md` records missing bundle, check,
  identity, trust, isolation, result, exit, failure, and migration authority.
- RFC-K507 is absent, `PROTO-EVIDENCE` is Future, and
  `GAP-CRITICAL-PROFILE-001` remains open.
- Accepted `DEC-0209` provides only test-local bundle vocabulary, not bytes or
  claims that could be verified.

## Current implementation evidence

The observation adds one isolated test with sixty explicit input/check,
identity, trust/TCB, isolation, result, failure, diagnostic, and fixture
boundaries. It sorts by explicit local rank, rejects duplicates, compares
canonical opaque bytes for forward/reverse input order, and keeps offline,
network denial, and no-code/command/FFI-execution categories distinct. No
verifier, parser, certificate rule, trust root, diagnostic, protocol,
dependency, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define bundle/verifier bytes; identities and recomputed
links; evidence polarity; certificate/signature/trust/TCB rules; deterministic
offline resource-limited verification that never executes bundle code;
fail-closed results, diagnostics, and exits; stable Semantic IDs and UTF-8
spans; and offline tamper/no-code/determinism fixtures. Seed behavior, Semantic
IDs, dependencies, and Unicode 17.0.0 remain unchanged.

## Deferred work

EVD-5802 implementation, bundle verifier, certificate/signature parsing,
trust/TCB and result/exit semantics, diagnostics, protocols, and public support
remain deferred until Accepted authority and executable offline evidence exist.
No placeholder Evidence Bundle verifier API is created.
