# ACT-2301 Implementation Report: Checked Actor Identity and State Isolation

## Outcome

ACT-2301 implements the checked-only Actor slice authorized by Accepted
DEC-0270. The exact contextual declaration form now passes through CST, AST,
HIR, resolution, type/effect checking, and publishes immutable Checked Actor
Core. Actor-bearing programs remain non-executable: file/project run, test, and
build stop at `L-ACTOR-0002` before interpreter, bytecode, VM, native, mailbox,
scheduler, or host effects.

This completion does not implement Actor instances, send, mailbox, turns,
supervision, remote references, serialization, or runtime scheduling.

## Normative clauses covered

- DEC-0270 clauses 1-2: exact contextual `actor`/`state`/`receive` grammar and
  the normal checked compiler pipeline.
- Clauses 3-6: deterministic `ActorTypeId`, an explicit unallocated
  runtime-scoped `ActorId` contract, and an internal local invariant typed
  reference descriptor with no source value or serialization surface.
- Clauses 7-11: initializer/result typing, empty residual Effect rows,
  binding-only transition scope, conservative transitive closed-Value bounds,
  nominal namespace resolution, and non-first-class Actor declarations.
- Clauses 12-13: validated Checked Actor Core with definition/type/expression/
  binding identities, complete original source spans, an explicit internal
  version, deterministic path-free canonical bytes, and collision rejection.
- Clauses 14-15: registered bilingual `L-ACTOR-0001` and `L-ACTOR-0002`, exact
  UTF-8 spans, Unicode 17.0.0 preservation, and no Semantic Graph, bytecode,
  VM ABI, CLI exit, or public protocol version change.

## Implementation evidence

- `ling-syntax`, `ling-ast`, and `ling-hir` represent and lower the accepted
  declaration without reserving new lexer keywords.
- `ling-resolve` places Actor declarations in the shared nominal namespace and
  limits state/message bindings to the transition body.
- `ling-types` accepts only closed monomorphic ordinary Value message/state
  shapes, checks initializer and transition result types, and rejects Actor as
  a first-class expression.
- `ling-effects::CheckedActorCore` validates pure non-suspending transitions,
  checked cross-references, source spans, identity evidence, and internal
  canonical bytes before publication.
- `ling-cli` permits `check`/semantic construction but rejects executable
  stages with `L-ACTOR-0002`; bytecode lowerers independently reject Actor
  types if reached.
- Semantic Graph v0.1/v0.2 deliberately omits the checked-only Actor extension,
  while editor indexes classify its declaration conservatively as a type.

## Tests added or updated

- Positive checked-core publication and local typed-reference evidence.
- Path/source-name/CRLF-independent Actor identity and canonical bytes.
- BOM/CRLF/Unicode preservation for every Actor Core source component.
- Negative effectful transition, non-Value message type, and first-class Actor
  use.
- Explicit execution boundary and bilingual diagnostic evidence.
- Formatter coverage and AST snapshot exhaustiveness.

## Compatibility impact

- Diagnostic registry: adds Preview codes `L-ACTOR-0001` and `L-ACTOR-0002`;
  the compatibility lock and high-water mark are updated.
- Schema/Semantic IDs: no public schema revision and no Actor node is added to
  the current Semantic Graph. Existing non-Actor identities remain unchanged.
- Determinism: Actor canonical bytes exclude source identity, paths, spans,
  allocation, scheduler state, and Rust debug/layout details.
- Unicode: XID, normalization, security rules, and generated tables remain
  pinned to Unicode 17.0.0.

## Specification gaps encountered

No conflict was resolved through code. DEC-0270 supplies the accepted
checked-only authority. The open await/reentry, mailbox/supervision, and remote
delivery gaps continue to govern all deferred executable behavior.

## Intentionally deferred

- ACT-2302: Sendable/message ownership and schema rules.
- ACT-2303: mailbox capacity, ordering, and backpressure.
- ACT-2304: turn execution, state mutation runtime, and await/reentry.
- ACT-2305: ActorId allocation/lifecycle and interpreter/VM runtime.
- ACT-2306 and later work: determinism properties, supervision, replay, remote
  delivery, serialization, and public protocols.

## Verification

The completion milestone runs the focused Actor and formatter tests, workspace
tests, formatting checks, and aggregate governance/status gates offline. Exact
commands and their successful results are recorded in the completion commit.
