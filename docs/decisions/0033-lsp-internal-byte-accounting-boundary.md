# DEC-0033: LSP internal byte-accounting boundary / LSP 内部字节计量边界

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: ide-protocol-design
> Related authority/gap: `DEC-0019`, `GAP-LSP-TRANSACTION-PROTOCOL-001`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision authorizes only deterministic arithmetic for an in-process byte
budget used by future LSP analysis code. It does not define public quotas,
configuration, diagnostics, memory-pressure behavior, or a JSON-RPC response.

## Question

Future LSP work needs a bounded way to account for owned UTF-8 bytes before a
public resource protocol exists. The LSP-2504 plan names open-document bytes,
but no accepted authority defines units, scope, limits, failure precedence, or
client-visible behavior. Existing project, bytecode, and transport limits are
domain-specific and cannot be reused as an LSP quota contract.

## Decision

1. `ling_lsp` may contain an internal `ByteBudget` whose unit is UTF-8 bytes
   and whose limit is supplied by its in-process owner. The budget tracks only
   arithmetic `used` and `limit` values; it does not inspect allocator state,
   process memory, host paths, CPU, or wall-clock time.
2. `try_reserve` succeeds at or below the remaining limit and updates usage
   atomically from the caller's perspective. An over-limit request returns a
   typed internal error without changing usage. No counter wraps.
3. `release` returns a typed internal error when asked to release more than is
   currently used and otherwise decreases usage deterministically. Zero-sized
   reserve/release operations are valid no-ops.
4. The budget is `pub(crate)` and carries no URI, request ID, snapshot/version,
   priority, cancellation, diagnostic, result, Workspace Edit, configuration,
   capability, or serialized state. It is not a public LSP resource response.
5. `LspServer`, the stdio transport, and all public protocol methods remain
   unchanged. Pending requests, completion lists, diagnostics, solver work,
   aggregate workspace limits, host OOM behavior, and publication precedence
   require separate accepted authority under the parent LSP-2504 task.

## Conformance plan

- Verify exact-boundary reserve succeeds and over-limit reserve is rejected
  without mutating usage or remaining capacity.
- Verify release, zero operations, repeated operations, and independent budget
  instances are deterministic and never wrap or underflow.
- Verify the primitive has no timer, allocator probe, thread, JSON encoding,
  transport handler, request association, diagnostic, or partial-result state.

## Compatibility impact

- Adds only `pub(crate)` arithmetic values in `ling-lsp`; language syntax,
  semantics, diagnostics, schemas, Semantic IDs, source spans, CLI, LSP wire
  methods, bytecode, VM, protocol inventory, support matrix, and Unicode
  17.0.0 behavior remain unchanged.
- The UTF-8-byte unit is an internal accounting invariant, not a promise that
  client versions, scalar counts, allocator usage, or host memory are equal.
- No migration, configuration, or stable/experimental protocol field is
  introduced.

## Unresolved alternatives

Public resource units and scopes, defaults and negotiation, hard/soft limits,
pending-request and result accounting, completion/diagnostic/solver quotas,
dependency/generated-file policy, cancellation/stale precedence, retry and
backoff, no-partial-publication guarantees, host-memory failure handling, and
diagnostic allocation require a later Accepted LSP-2504 decision.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
