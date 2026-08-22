# LSP-2402 Authority Audit: Typed Token Generation

## Outcome

`LSP-2402` is correctly recorded as `BlockedSpec`. The execution plan requires
semantic tokens to come from Checked Core/Resolved HIR rather than Tree-sitter,
with a clearly marked fallback to already-parsed tokens in syntax-error
regions. The repository has checked compiler and syntax/token data, but no
accepted token taxonomy, source-origin marker, fallback schema, position/version
binding, or semantic-token protocol.

No typed-token generator, syntax-error fallback adapter, source marker,
protocol field, diagnostic allocation, or placeholder LSP surface was added.
Accepted DEC-0085 and the bounded `LSP-2402-CHECKED-IDENTITY` child now add
only an exact join between lexical tokens and existing checked definition
facts; public typed-token generation remains blocked.

## Normative traceability

- The execution package is non-normative; its Checked Core/fallback wording does
  not authorize a public token protocol.
- Repository authority requires evaluation and public projections to consume
  checked Typed Core, never unchecked AST nodes. This permits the proposed
  boundary but does not define token categories or fallback presentation.
- DEC-0002 makes original UTF-8 `SourceId + Span` authoritative and requires an
  explicit SourceMap projection for LSP UTF-16 positions. It does not define
  token origin, version, overlap, or ordering fields.
- DEC-0012 fixes Semantic IDs/canonical bytes. The registered Semantic Graph
  projections are Experimental and do not define typed-token output or source
  provenance labels.
- Accepted DEC-0085 authorizes only
  `CompilerDb::checked_token_source_index`: it joins an existing lexical token
  with a checked definition when source name and original UTF-8 span are exact,
  preserving existing Definition ID/type/effect/capability facts without
  defining presentation categories or fallback states.
- `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` and
  `GAP-LSP-TRANSACTION-PROTOCOL-001` leave token/graph field stability,
  snapshot/version, stale handling, and migration open. LSP-2401's taxonomy
  decision is an unsatisfied prerequisite. RFC-0005/DEC-0027 provide no public
  Trait token projection.

## Current interface evidence

- `ling-types`, `ling-effects`, and `ling-semantic` compute checked types,
  effects, capabilities, definitions, nodes, and references, but expose no
  token-generation or source-origin presentation API.
- `ling-syntax` and Tree-sitter fixtures provide lexical/CST tokens and error
  nodes; they are not a semantic-token authority and no adapter labels their
  output as fallback data.
- `ling-source` preserves original byte spans and scalar columns, but no
  negotiated UTF-16 mapping, same-document-version requirement, non-overlap
  validation, or cancellation behavior exists.
- No fixture covers typed-versus-fallback origin, incomplete/error regions,
  generic/effect/capability categories, Unicode/CRLF/BOM spans, shadowing,
  deterministic order, stale versions, or the prohibition on unchecked AST
  interpretation.
- The checked-identity child does not classify references or non-definition
  tokens and has no fallback, position, version, legend, modifier, or transport
  fields.

## Required authority before implementation

An Accepted decision or RFC must define, at minimum:

1. taxonomy and modifier mapping from Checked Core/Resolved HIR constructs,
   including declarations, references, types, effects, capabilities,
   mutability, generated/dependency/builtin regions, and client fallback;
2. precise source-origin states (`typed`, `parsed-fallback`, and no-token),
   allowed syntax-error boundaries, error-region preservation, and a guarantee
   that unresolved/unchecked AST is never interpreted;
3. source-span truth, UTF-8/UTF-16 conversion, same snapshot/document version,
   non-overlap and deterministic position ordering, duplicate/conflict rules,
   Semantic ID/provenance and redaction;
4. request/response, cancellation/limits, protocol inventory, Stable versus
   Experimental fields, client negotiation, diagnostics interaction, and
   migration; and
5. executable positive/negative fixtures for valid typed programs, each
   semantic category, syntax-error fallback, CRLF/BOM/Unicode/emoji columns,
   nested/shadowed symbols, stale versions, deterministic ordering, and
   unchecked-AST rejection.

Until these decisions are Accepted, a generator could mislabel syntax as
semantically checked, interpret an unresolved node, or emit spans that do not
belong to the requested document version.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0002, DEC-0012, RFC-0005,
DEC-0085,
`docs/decisions/0027-trait-checked-core-dictionary-witness.md`,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the `ling-syntax`, `ling-types`, `ling-effects`, `ling-semantic`, and
`ling-source` crates.

Only the internal checked-token identity observation changed; no compiler
language semantics, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
public source-span projection, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

The bounded checked-identity child is complete under DEC-0085. Public
`LSP-2402` can begin only after LSP-2401 taxonomy, position/version, Semantic
Graph lifecycle, and fallback-source decisions are Accepted. The future
implementation must consume checked Typed Core/Resolved HIR, mark parsed
fallback explicitly, preserve source-span/identity truth, and reject unchecked
AST interpretation.
