# LSP-2502 Authority Audit: Request Cancellation

## Outcome

`LSP-2502` is authorized by Accepted RFC-0049 and may advance from
`BlockedSpec` to implementation. RFC-0049 closes the bounded Preview surface
that the earlier audit correctly found missing: standard wire
`$/cancelRequest`, exact live request-ID association, cooperative compiler
propagation, RequestCancelled `-32800`, atomic publication, cleanup, privacy,
and migration rules.

The Accepted contract composes rather than broadens the existing method
authorities. It does not define general Semantic Transactions, parallel request
execution, deadlines, quotas, priorities, progress, VM cancellation, or Stable
editor compatibility.

## Normative traceability

- RFC-0049 §1 fixes `ling.lsp.request-cancellation/0.1` discovery.
- RFC-0049 §2–§4 fixes exact string/number ID lifetime, duplicate-live
  rejection, valid/unknown/duplicate/late behavior, and the single mutable
  executor with an independently progressing bounded reader.
- RFC-0049 §5 fixes transport-to-handler propagation, typed
  `QueryError::Cancelled`, bounded compiler/type/Trait-solver checkpoints, and
  fail-before-cache behavior.
- RFC-0049 §6 fixes cancellation precedence and atomic response, Workspace
  Edit, completion-resolve, workspace-index, semantic-token-history, and
  diagnostic publication.
- RFC-0049 §7 fixes session cleanup, privacy, and the separation from
  RFC-0020 VM/runtime cancellation.
- DEC-0019 remains the query/cache authority; DEC-0030 owns immutable request
  snapshots; DEC-0031 owns the monotonic token primitive; RFC-0041, RFC-0043,
  RFC-0045, and RFC-0048 retain their method-specific publication contracts.

## Resolved specification questions

1. **ID domain and lifetime:** exact JSON string and number IDs are associated
   from accepted frame routing until response selection; null remains accepted
   by RFC-0004 but is not cancellable.
2. **Unknown, duplicate, and late cancellation:** each is an idempotent no-op;
   duplicate live request IDs receive `-32600` and cannot replace the first
   token; late cancellation cannot cross ID reuse.
3. **Wire form:** only notification-form `$/cancelRequest` has an effect;
   malformed notifications remain response-free and request form receives
   `-32600` without an effect.
4. **Precedence:** cancellation wins when observed before final publication;
   a response already selected and cleaned up is complete, so later
   cancellation is a no-op.
5. **Compiler/cache behavior:** cancellation is typed and checked before
   expensive stages, between bounded index/type/Trait-solver stages, and before
   cache insertion. Completed immutable dependencies may remain cached, but no
   partial checked result may be inserted.

## Remaining higher-level gaps

`GAP-LSP-TRANSACTION-PROTOCOL-001` stays Open because RFC-0049 closes only
request cancellation. General scheduling/fairness, resource limits, arbitrary
Workspace Edit/Semantic Transaction publication, and Stable compatibility
remain assigned to LSP-2503, LSP-2504, IDE-2309, or later Accepted authority.
Those broader gaps do not block the exact LSP-2502 contract.

## Compatibility and determinism

The change adds one Preview LSP protocol marker, one standard notification,
one standard JSON-RPC error code, and typed in-process compiler cancellation.
It allocates no Ling diagnostic or standalone schema and changes no Ling
syntax, semantics, Typed Core evaluation, Semantic ID, Definition ID, source
span, runtime, bytecode, VM, ABI, package, filesystem/network, or Unicode
17.0.0 behavior. Request scheduling, timing, thread identity, allocation,
paths, and compiler identities are absent from wire output.
