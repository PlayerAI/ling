# EFF-2101 Authority Audit: Effect Core Model

## Outcome

`RFC-0006` is now an Accepted repository authority for the experimental v0.2
Effect core model. `EFF-2101` is therefore no longer specification-blocked;
it is `In Progress` while the checked implementation and conformance evidence
are completed. The existing v0.0.1 Seed `EffectRow` remains unchanged.

The implementation adds a separate `ling-effects::v2` model for canonical
`EffectId`, parameterized `EffectLabel`, closed/open rows, row variables,
operation signatures, resume cardinality, lexical handler clauses, residual
rows, and deterministic canonical bytes. It does not reinterpret Seed AST or
publish a runtime, protocol, or bytecode contract.

## Normative traceability

- RFC-0006 §§1–4 authorize NFC Effect identities, ordered Typed-Core
  parameters, duplicate-free rows, closed/open tails, deterministic union,
  explicit operation signatures, lexical first-order handlers, residual-row
  preservation, and the separation of Effect presence from Capability
  authorization.
- RFC-0006 §8 reserves `Clock`, `Random`, `Console.Write`, `State<T>`,
  `Task`, and `ActorSend<T>` labels. Task/Actor lifecycle and transport
  semantics remain deferred to their own Accepted authorities.
- Accepted DEC-0010 continues to govern Seed `State<T>` visibility and host
  Capability closure. The new model does not change Seed checking or grant a
  Capability when an Effect is handled.
- RFC-0006 resolves `GAP-EFFECT-STATE-MASKING-001` by keeping `State<T>`
  visible and resolves `GAP-EFFECT-HANDLER-001` with a bounded first-order
  handler model. Solver diagnostics and source syntax remain EFF-2102/EFF-2103
  responsibilities.

## Current implementation evidence

- `EffectId::new` validates Unicode 17 XID segments, normalizes NFC, rejects
  empty/path-like identities, and stores only canonical segments.
- `EffectTypeRef` rejects whitespace, controls, and source path separators;
  labels compare ordered canonical parameters, so `State<Int>` and
  `State<Text>` remain distinct.
- `EffectRowModel` sorts and deduplicates labels, represents `Closed` or a
  binder-local `RowVariableId`, preserves tails through removal, and rejects a
  union of distinct open tails instead of choosing a host-dependent result.
- `EffectOperation` validates operation names and retains ordered input types,
  one output type, and `Never`/`Once`/`Many` resume mode.
- `HandlerContract` canonicalizes clauses, rejects duplicate labels and owner
  mismatches, removes only declared labels, and verifies the declared residual
  row while preserving an open tail for nested handlers.
- Focused tests cover NFC/path rejection, pure/closed/open rows, duplicate and
  presentation-order independence, parameter distinction, union constraints,
  resume modes, nested residual handlers, and canonical bytes.

## Compatibility and determinism

- Seed source syntax, diagnostics, Semantic IDs, schemas, bytecode, VM,
  interpreter behavior, CLI/LSP protocols, and Unicode 17 data are unchanged.
- No diagnostic code, wire field, public protocol, or Semantic Graph schema
  version is allocated by this slice. `canonical_bytes()` is an in-process,
  length-delimited projection for future graph integration and contains no
  paths, allocator identity, hash-map order, timing, or debug formatting.
- Original source spans are not introduced or rewritten; later syntax/Core
  work must attach the existing UTF-8 spans when it creates source nodes.

## Remaining EFF-2101 work

1. Integrate the model into a versioned, optional Semantic Graph/Audit
   projection only after that protocol authority is accepted.
2. Add the full positive/negative fixture matrix for unhandled profile effects,
   polymorphic calls, Capability failures, and the reserved-label examples.
3. Keep row constraint solving, generalization/instantiation, diagnostics,
   handler syntax/Typed Core lowering, and runtime execution in EFF-2102,
   EFF-2103, and EFF-2104/2105.
4. Obtain separate Accepted authorities before implementing Task, Actor,
   Replay, Remote, Native, GPU, or FFI behavior.

Until these items are complete, `EFF-2101` must not be marked `Done` and the
v0.2 model must remain explicitly Experimental.
