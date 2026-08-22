# TASK-2201-CORE-MODEL Authority Audit

## Outcome

The bounded child `TASK-2201-CORE-MODEL` is authorized by Accepted
`DEC-0091`. It implements only a publish-disabled, checked-data identity graph
for future Structured Task lowering. The public `TASK-2201` target remains
`BlockedSpec` because Task grammar, Typed-Core integration, lifecycle,
cancellation, cleanup, suspension semantics, detach authority, and runtime
contracts are still open.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires explicit Task lifecycle, cancellation,
  cleanup, suspension, and deterministic evidence before support is promoted.
- `docs/SEMANTICS.md` §18 and `docs/LANGUAGE.md` §19 keep Task outside the
  v0.0.1 Seed subset.
- `DEC-0002` fixes original UTF-8 byte spans as the source-position authority.
- `DEC-0089` protects the negative Seed boundary and does not authorize Task
  syntax or runtime behavior.
- `DEC-0091` authorizes only the internal identity graph and its validation and
  canonical-byte invariants.

## Current implementation boundary

`crates/ling-concurrency` is `publish = false` and exposes immutable
`TaskCore`, `TaskNode`, `SuspensionPoint`, and identity wrappers only for
checked-data tests and future compiler integration. Construction rejects zero
identities, an absent or parented root, duplicate tasks or suspension points,
unknown parents, parent cycles, and incomplete detach evidence. Source spans
are retained as evidence and omitted from canonical bytes.

No parser, lexer keyword, AST/HIR/typed-program node, Task type, cancellation or
cleanup execution, scheduler, Fault aggregation, detach authority, bytecode,
VM/native ABI, CLI/LSP command, diagnostic, schema, Semantic ID, public
protocol, or migration behavior was added.

## Evidence and deferred work

The focused crate tests cover nested parent/child validation, deterministic
ordering, insertion/source-span independence of canonical bytes, invalid and
duplicate suspension identities, unknown parents, and cycles. The parent
remains blocked until an Accepted RFC-C202/RFC-0008 (or replacement) defines
the source/Core and lifecycle contract and supplies executable positive,
negative, cancellation, cleanup, scheduler, differential, and migration
evidence.
