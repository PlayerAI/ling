# DEC-0210: Internal Independent Evidence Verifier boundary evidence / 独立 Evidence Verifier 边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: critical-quality
> 相关规范/缺口：`DEC-0209` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`EVD-5802-OBSERVATION`. It records provisional schema/hash/link, certificate,
identity, trust/TCB, isolation, failure, and fixture vocabulary while RFC-K507,
the bundle schema, and verifier semantics remain unresolved.

本决定只授权 `EVD-5802-OBSERVATION` 使用 test-local 的 schema/hash/link、certificate、
identity、trust/TCB、isolation、failure 与 fixture 边界清单；在 RFC-K507、bundle schema 和
verifier semantics 尚未解决时，只记录临时词汇，不实现独立验证器。

## Question

EVD-5802 proposes independently checking bundle schema/version, canonical
hashes, artifact links, proof certificates, test identities/signatures,
lock/toolchain inputs, missing/unknown fields, offline behavior, and no code
execution. Which vocabulary can be retained as bounded evidence without
choosing verifier inputs, trust roots, result semantics, or a public command?

## Decision

1. `crates/ling-types/tests/independent_evidence_verifier_evidence.rs` keeps a
   test-local inventory of sixty provisional input/check, identity, trust/TCB,
   isolation, result, failure, diagnostic, and fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.independent-evidence-verifier-observation/0`.
   These bytes are observation evidence only; they are not a verifier input,
   result, certificate parser, trust store, diagnostic, protocol, or support
   claim.
3. `OfflineMode`, `NetworkDenied`, `NoCodeExecution`, `CommandDenied`, and
   `FfiDenied` remain distinct local categories. Their presence grants no
   execution or verification authority and cannot be used to claim sandboxing.
4. No bundle verifier, parser, certificate/signature dependency, trust root,
   CLI/LSP route, diagnostic allocation, support claim, or placeholder API is
   added. Public `EVD-5802` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:543-555` is a
  non-normative checklist. It defines no verifier input/result, canonical/hash
  domains, trust roots, certificates, process exits, or migration.
- `docs/status/EVD-5802-AUTHORITY-AUDIT.md` records the absent RFC-K507,
  bundle schema, verification rules, trust/TCB, command contract, diagnostics,
  and executable fixtures.
- `PROTO-EVIDENCE` is Planned public/Future and `GAP-CRITICAL-PROFILE-001`
  remains open; neither is implementation authority.
- Accepted bytecode and Audit Source verifiers have distinct inputs and trust
  boundaries. They are not Evidence Bundle verifiers and are not reused here.
- `DEC-0209` authorizes only test-local bundle vocabulary; it does not define
  bytes or claims that a verifier could accept.

## Conformance plan

- Assert all sixty independent-verifier categories and local order; compare
  forward/reverse opaque bytes; reject duplicates; retain offline/network and
  no-code/command/FFI-execution boundaries together.
- Defer verifier implementation, trust/certificate/result/exit semantics,
  diagnostics, protocols, and public support until Accepted authority and
  offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing bytecode/Audit verifiers, governance checks, and internal
evidence are not reinterpreted as an Evidence Bundle verifier; only test-local
evidence is added.

## Unresolved alternatives

Versioned bundle/verifier input and canonical bytes; required fields, size
limits, hash domains, unknown/missing fields and migration; artifact/source/
Semantic/build/target/Profile/toolchain/dependency/test/proof/model/replay/
timing/memory/FFI/TCB/assumption/provenance identities and recomputed links;
evidence polarity and non-claims; certificate/signature formats, trust roots,
keys, revocation, verifier identity/version and TCB; resource/network/offline
isolation and prohibition of bundle code/plugins/commands/FFI/hooks; valid/
invalid/unknown/unsupported, hash/link mismatch, invalid certificate,
unavailable/stale/trust/resource/malformed/corrupt/unsupported-version and
fail-closed behavior; diagnostics/exits, positive/negative/tampered-link/
invalid-certificate/no-code-execution/Unicode/determinism fixtures, protocol
inventory, and public support remain open under EVD-5802, EVD-5801, EVD-5803,
EVD-5804, RFC-K507, GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE, and missing
verifier authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
