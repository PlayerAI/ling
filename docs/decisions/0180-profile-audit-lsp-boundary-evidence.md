# DEC-0180: Internal Profile Audit/LSP boundary evidence / 内部 Profile 审计与 LSP 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0179` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `GAP-LSP-TRANSACTION-PROTOCOL-001` | `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`PROF-5104-OBSERVATION`. It records provisional Profile audit, explanation,
CLI, diagnostics, source-span, and LSP lifecycle vocabulary while Profile,
CLI, and Semantic Transaction authorities remain unresolved.

本决定只授权 `PROF-5104-OBSERVATION` 使用 test-local 的 Profile 审计与 LSP 边界清单；
在 Profile、CLI 与 Semantic Transaction 权威尚未解决时，只记录临时的 audit、explanation、CLI、
diagnostic、source-span 与 LSP lifecycle 词汇。

## Question

PROF-5104 proposes Profile checks, audits, explanations, and editor feedback
for Effects, capabilities, and unbounded sources. Which vocabulary can be
retained as bounded evidence without selecting a Profile checker, CLI contract,
diagnostic payload, or LSP protocol?

## Decision

1. `crates/ling-types/tests/profile_audit_lsp_evidence.rs` keeps a test-local
   inventory of sixty provisional audit, explanation, CLI, diagnostic, source,
   LSP, and fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.profile-audit-lsp-observation/0`. These bytes are
   evidence only; they are not a checker, report schema, diagnostic, CLI
   command, LSP method, protocol, or support claim.
3. No Profile audit/checker, explanation report, diagnostic allocation, CLI
   route, LSP method, dependency, protocol, support claim, or placeholder API
   is added. Public `PROF-5104` remains `BlockedSpec`.
4. The plan's stale `zero` command examples remain excluded; accepted public
   names `ling` and `.ling` are unchanged.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:126-136` is
  non-normative; it defines no command lifecycle, profile selection, report
  fields, diagnostic mapping, document identity, position encoding, or LSP
  transaction rules.
- `docs/ROADMAP-1.0.md:142-151` keeps CLI, diagnostics, Semantic Graph, and
  tooling protocols as separate compatibility surfaces and does not authorize
  Profile audit or editor behavior.
- Accepted DEC-0002 and DEC-0012 preserve source-position and Semantic ID
  domains; they do not authorize a Profile audit schema or LSP publication.
- `docs/status/PROF-5104-AUTHORITY-AUDIT.md` records open Profile, LSP, and
  Semantic Transaction gaps and prohibits copying stale `zero` names.

## Conformance plan

- Assert all sixty Profile Audit/LSP boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer Profile checker/report semantics, `ling` command routes, diagnostic
  payloads, LSP lifecycle and publication, quick fixes, migration, and
  protocol behavior until accepted authority exists.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, original UTF-8
spans, CLI/LSP implementation, runtime, bytecode, VM, dependencies, and
Unicode 17.0.0 remain unchanged. No stale `zero` command, profile input,
diagnostic code, LSP method, or public protocol is registered; only test-local
evidence is added.

## Unresolved alternatives

Audit schema/version/lifecycle; checked-fact provenance; Effect/capability/
unbounded finding taxonomy; Profile/target identity; Semantic ID and UTF-8
byte-span relation; related ranges, severity, stable codes, facts,
localization, redaction, unknown fields and migration; `ling` profile
selection, manifest/config precedence, check/audit/explain command behavior,
human/JSON formats, exit statuses and offline rules; LSP initialization,
capability negotiation, URI/FileId, document version, UTF-16/UTF-8 position
encoding, workspace/profile context, cancellation, stale-result rejection,
deterministic publication, limits and error mapping; source mapping and quick
fix safety; Unicode/CRLF, transitive, bounds, revision, cancellation, stale,
JSON migration, privacy and differential fixtures; protocol inventory and
public status remain open under PROF-5104, PROF-5103, GAP-CRITICAL-PROFILE-001,
GAP-LSP-TRANSACTION-PROTOCOL-001, GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001, and
missing Profile/RFC-K501/RFC-0012 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
