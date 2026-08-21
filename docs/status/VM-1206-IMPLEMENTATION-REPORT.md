# VM-1206 implementation report

## Scope

VM-1206 implements the Accepted RFC-0016 `ling.bytecode/1.2` aggregate and
checked-match slice. The implementation covers tuples, nominal records,
nominal variants, immutable record update, projections, constructor lowering,
pattern decision trees, guards, scalar conditions, and VM execution through
the existing verifier-gated entry point.

## Normative authority

- RFC-0016 sections 1–6 authorize the 1.2 model, wire revision, aggregate
  instructions, checked match control flow, deterministic tables, and runtime
  allocation boundary.
- RFC-0014 and RFC-0015 remain authoritative for the inherited scalar,
  closure, function-type, source-map, effect, capability, and frame rules.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` remain authoritative for source
  syntax and checked-core meaning. No Draft RFC or execution-plan proposal was
  used to define language semantics.

## Delivered evidence

- `ling-bytecode` model, encoder, bounded decoder, disassembler, verifier, and
  v1.2 lowering are implemented with explicit nominal type operands and
  backward-compatible 1.0/1.1 readers.
- The lowering emits deterministic CFG blocks and source-map entries for
  constructor, tuple, record, literal, binding, wildcard, nested, and guarded
  patterns. Aggregate constructors retain nominal identity and generic
  prelude Option/Result identities use the explicit `Ling.Prelude` module
  record.
- `ling-vm` stores tuples, records, and variants behind the abstract value
  model. Record updates allocate a fresh value; variant tests and payload
  extraction are verifier-checked before execution.
- Added lowering round-trip tests for aggregate values, nested tuple payloads,
  guards/scalar control flow, prelude Option constructors, and exhaustive
  variant matches.
- Added VM/interpreter differential tests for variant matches and record/tuple
  immutable-update execution.

## Compatibility and risk

- Diagnostic framing and registered `L-<DOMAIN>-<NUMBER>` identities are
  unchanged. Existing 1.0/1.1 artifacts retain their prior encoding and
  verification paths; 1.0/1.1 readers reject the 1.2 revision before decoding
  revision-specific records.
- Semantic IDs, canonical semantic bytes, CLI behavior, ABI/FFI layout, and
  interpreter behavior are unchanged. The v1.2 module table adds the explicit
  `Ling.Prelude` identity when aggregate lowering is selected.
- Type-table verification now validates the dependency-first canonical order
  required when a function type references an aggregate type; v1.0/1.1 order
  checks remain unchanged.
- Determinism is preserved by declaration-order fields/cases, canonical
  type/string/module tables, stable block construction, and sorted
  `(function, block, ordinal)` source maps. Original UTF-8 byte spans and
  Unicode 17.0.0 policy are unchanged.

## Deferred work

Mutable places/borrow checking (VM-1207), broader effect/fault coverage
(VM-1208), complete interpreter/VM differential inventory (VM-1209), and fuzz,
cancellation, and resource-model work (VM-1210) remain separately gated. Full
polymorphic function-value lowering and recursive aggregate wire types remain
outside RFC-0016 and are not claimed here.
