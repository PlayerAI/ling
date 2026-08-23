# LSP-2404 Authority Audit: Semantic-Token Fixtures

## Outcome

`LSP-2404` is authorized for implementation. Its direct dependencies
`LSP-2401`, `LSP-2402`, and `LSP-2403` are complete under Accepted RFC-0046,
RFC-0047, and RFC-0048 respectively. Together those RFCs now define every
semantic-token value frozen by the fixture corpus: canonical taxonomy and
modifiers, checked versus whole-source fallback generation, original UTF-8
span truth, negotiated position projection, snapshot binding, exact full and
delta transport, deterministic result IDs, bounds, cancellation, freshness,
privacy, and compatibility behavior.

The corpus format marker `ling.test.lsp-semantic-tokens/1` is test-only. It is
an executable representation of already Accepted public behavior, not a new
public protocol, standalone schema, language feature, or compatibility promise.

## Normative traceability

- RFC-0046 §1–3 fixes the canonical token-type/modifier order and source-role
  mapping, including scoped identities, mutable fields, variant constructors,
  and modifier exclusivity.
- RFC-0046 §4–7 fixes evidence precedence, conservative recovery, legend
  projection, UTF-8 span truth, negotiated position units, ordering, privacy,
  and the exclusion of Effect/Capability facts as source-token categories.
- RFC-0047 §1–4 fixes snapshot-bound checked generation, exact identity and
  structure evidence, field/constructor/reference mapping, shadowing, writes,
  and modifier propagation.
- RFC-0047 §5–7 fixes whole-source lexical fallback, multiline splitting,
  non-overlap, atomic failure, and the prohibition on unchecked-AST or private
  compiler data in output.
- RFC-0048 §1–4 fixes the Preview protocol marker, selected legend, tracked
  document/version input, UTF-8/16/32 projection, and standard relative groups.
- RFC-0048 §5–8 fixes deterministic result IDs, bounded history, canonical
  one-edit deltas, full/delta equivalence, cancellation, freshness, limits,
  atomic failure, privacy, and migration boundaries.
- RFC-0048's conformance plan expressly requires exact protocol fixtures for
  Unicode projection, deterministic bytes and IDs, full/delta equivalence,
  compiler failures, exclusions, limits, and privacy.

The lower-authority execution plan selects the requested examples but creates
no additional semantics. General LSP-2501/LSP-2502 work remains future scope;
RFC-0048 already supplies the bounded snapshot and cooperative-cancellation
rules required by this implemented Preview surface.

## Authorized fixture boundary

The fixture corpus may contain only client-visible inputs and results already
defined by the Accepted RFCs:

- a test-only format marker and the three Accepted taxonomy, generation, and
  transport version markers;
- the canonical client legend, negotiated position encoding, exact URI,
  document version, and UTF-8 source text used as test input;
- exact JSON-RPC full/delta results containing standard integer data, canonical
  edits, and opaque `st1-` result IDs; and
- deterministic test metadata such as a unique case name and changed document
  input.

The corpus must not publish compiler identities, Semantic IDs, Definition IDs,
VFS revisions, types, Effect rows, Capability sets, host paths, diagnostics,
debug output, or a new token category. The reader must independently execute
the provider, compare complete values, verify ordering/non-overlap, and reapply
delta edits to the exact base.

## Required cases

The smallest complete corpus covers:

1. UTF-16 positions with a BOM, CRLF, an emoji prefix, combining text, and
   Chinese identifier columns;
2. same spelling at different checked scopes, mutable-field roles and writes,
   variant constructors/patterns, and exclusion of names in a
   `requires Console.Write` clause;
3. whole-source error recovery that emits only unmodified lexer-proven token
   families and never fabricates identifier roles; and
4. exact deterministic full output, one canonical delta edit, the corresponding
   current full output, equal result identity, and executable delta equivalence.

Existing RFC-0048 transport tests remain the evidence for partial legends,
UTF-8/UTF-32, invalid/foreign/expired bases, FIFO retention, temporary/closed
documents, cancellation, malformed inputs, and limits; those cases need not be
duplicated in the exact corpus.

## Specification gaps and compatibility

No unresolved semantic or public-protocol decision blocks LSP-2404. A new RFC
would be required only to change the accepted taxonomy, generation evidence,
position model, transport marker, result identity, delta algorithm, or public
compatibility rules. Updating the test-only fixture format without changing
public behavior does not create a protocol migration.

This task allocates no diagnostic, adds no standalone public schema, changes no
Semantic ID or canonical bytes, and changes no Ling syntax, Typed Core,
interpreter, runtime, bytecode, VM, ABI, package behavior, filesystem/network
behavior, or Unicode 17.0.0 tables.
