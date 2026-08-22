# EFF-2101 Authority Audit: Effect Core Model

## Outcome

`RFC-0006` is an Accepted repository authority for the experimental v0.2
Effect core model, and `EFF-2101` is complete. The v0.0.1 Seed `EffectRow`
remains unchanged; EFF-2102 and later tasks own inference, lowering, runtime,
and public protocol integration.

The completed implementation provides a separate `ling-effects::v2` model for
canonical `EffectId`, parameterized `EffectLabel`, closed/open rows, row
variables, operation signatures, resume cardinality, lexical handler clauses,
residual rows, and a versioned in-process graph projection with deterministic
canonical bytes.

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
  Capability closure. The v2 model does not change Seed checking or grant a
  Capability when an Effect is handled.
- RFC-0006 resolves `GAP-EFFECT-STATE-MASKING-001` by keeping `State<T>`
  visible and resolves `GAP-EFFECT-HANDLER-001` with a bounded first-order
  handler model. Solver diagnostics and source syntax are EFF-2102/EFF-2103
  responsibilities.

## Implementation and conformance evidence

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
- `EffectGraphProjection` defines the in-process `ling.effect/0.1` model shape,
  canonical ordering, and length-delimited graph-input bytes without creating
  a public wire field or changing Seed JSON.
- Tests cover pure rows, Clock/Random/Console.Write/State/Task/ActorSend,
  polymorphic caller-row preservation, parameter identity, duplicate/order
  independence, NFC/path rejection, resume modes, nested residual handlers,
  graph projection, and canonical bytes. Existing Seed tests cover missing
  Capability diagnostics and checked call-graph propagation.

## Compatibility and determinism

- Seed source syntax, diagnostics, Semantic IDs, schemas, bytecode, VM,
  interpreter behavior, CLI/LSP protocols, and Unicode 17 data are unchanged.
- No diagnostic code, public wire field, or Semantic Graph schema version is
  allocated by this milestone. The projection is an in-process model boundary
  for a future adapter whose authority must be accepted separately.
- Canonical bytes contain no paths, allocator identity, hash-map order, timing,
  debug formatting, or host state. Source spans are not invented or rewritten.

## Handoff

EFF-2102 now owns row constraints, unification/occurs-check, generalization,
instantiation, conflict explanation, and bilingual solver diagnostics. EFF-2103
owns source handler syntax and Checked-Core lowering. EFF-2104/2105 own runtime
execution and interpreter/VM differential testing. Task, Actor, Replay, Remote,
Native, GPU, and FFI behavior still require their own Accepted authorities.
