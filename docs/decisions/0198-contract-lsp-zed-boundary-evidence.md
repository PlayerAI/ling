# DEC-0198: Internal Contract LSP/Zed boundary evidence / Contract LSP/Zed 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0197` | `ROADMAP-1.0` | `GAP-LSP-TRANSACTION-PROTOCOL-001` | `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`CTR-5407-OBSERVATION`. It records provisional Contract editor vocabulary
for hover/status, diagnostics, proof/evidence links, Audit, rename,
snapshots/transactions, position conversion, stale data, and client fixtures
while Contract/proof/evidence/LSP/Semantic Transaction authorities remain
unresolved.

本决定只授权 `CTR-5407-OBSERVATION` 使用 test-local 的 Contract editor 边界清单；在 Contract、proof、
evidence、LSP、Semantic Transaction 等权威尚未解决时，只记录临时的 hover/status、diagnostic、
proof/evidence link、Audit、rename、snapshot/transaction、position conversion、stale data 与 client fixture 词汇。

## Question

CTR-5407 lists hover, counterexample diagnostics, proof/evidence code lenses,
gutter status, Contract-aware rename, and expanded Audit conditions. Which
vocabulary can be retained as bounded evidence without choosing a versioned
LSP protocol, Contract data source, snapshot/transaction rules, or position
and editor compatibility contract?

## Decision

1. `crates/ling-types/tests/contract_lsp_zed_evidence.rs` keeps a test-local
   inventory of sixty provisional Contract editor, protocol, snapshot/
   transaction, position, data-validity, diagnostic, and fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.contract-lsp-zed-observation/0`. These bytes are
   evidence only; they are not an LSP method, JSON schema, Contract
   projection, rename edit, Zed extension, diagnostic, protocol, or support
   claim.
3. No LSP method, Contract status field, proof/evidence link, counterexample
   schema, rename edit, Zed package, dependency, diagnostic allocation,
   public protocol, support claim, or placeholder API is added. Public
   `CTR-5407` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:375-382` is a
  non-normative editor checklist. It defines no requests, responses,
  capabilities, status payload, evidence links, or rename atomicity.
- `docs/status/CTR-5407-AUTHORITY-AUDIT.md` records absent Contract/proof/
  evidence data and LSP/Semantic Transaction authority.
- `GAP-LSP-TRANSACTION-PROTOCOL-001`, `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`,
  and `GAP-CRITICAL-PROFILE-001` remain open; `PROTO-EVIDENCE` is Future.
- `DEC-0002` fixes original UTF-8 byte spans but does not define an LSP
  transport or Contract range model. `DEC-0019` authorizes only an internal
  query boundary. Preview diagnostic JSON is not an LSP or evidence schema.
- Draft `SEMANTICS.md`/`LANGUAGE.md` Contract sketches and status names do
  not authorize a public editor projection.

## Conformance plan

- Assert all sixty Contract LSP/Zed categories and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer LSP methods, Contract projection, proof/evidence/counterexample
  schemas, rename transactions, position conversion, diagnostics, Zed
  integration, and protocol behavior until accepted authority and offline
  fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing byte-span/query/Tree-sitter observations and Preview
diagnostic JSON are not reinterpreted as Contract editor semantics; only
test-local evidence is added.

## Unresolved alternatives

LSP version/capabilities/requests/responses; hover/status/diagnostic/
counterexample/code-lens/gutter/Audit/implicit-condition/rename semantics;
Contract references and stable IDs/provenance/invalidation; snapshots,
Semantic Transactions, stale edits/conflicts and atomic TextEdits; UTF-8
spans, LSP positions/UTF-16, CRLF/BOM/Unicode 17.0.0; unknown/stale/corrupt/
unverifiable data, fallback, redaction/privacy; workspace/document/
incremental/JSON schema, cancellation/ordering; diagnostics and positive/
negative/malformed/stale-version/Unicode/CRLF/incremental/rename/evidence/
determinism/client-capability fixtures; Zed extension, protocol inventory,
and public status remain open under CTR-5407, CTR-5406, LSP-2205,
GAP-LSP-TRANSACTION-PROTOCOL-001, GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001,
GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE, and missing editor authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
