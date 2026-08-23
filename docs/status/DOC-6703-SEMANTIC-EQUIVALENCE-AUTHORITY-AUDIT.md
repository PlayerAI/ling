# DOC-6703-SEMANTIC-EQUIVALENCE Authority Audit

- Parent: `DOC-6703` — Bilingual Chinese-first Tutorial
- Child: `DOC-6703-SEMANTIC-EQUIVALENCE` — Tutorial Semantic-shape equivalence
- Release: G6
- Decision: `Done` is authorized only for this bounded internal child by
  Accepted `DEC-0240`; the parent remains `BlockedSpec`.

## Authority and gap

Accepted DEC-0047 protects tutorial inventory and source markers, and accepted
DEC-0239 executes both tutorials from one strict manifest. Before this child,
the repository validated each source independently but did not compare their
claim of equivalent checked semantics.

DEC-0240 authorizes a private comparison projection over the actual emitted
Semantic Graphs. The projection preserves checked structural evidence while
excluding exactly the localized and experimental fields that are not an
equivalence or compatibility requirement.

## Authorized implementation

- Collect the Chinese and ASCII tutorial Semantic Graphs from the shared
  six-case process loop.
- Normalize only the one user nominal type spelling per tutorial.
- Compare module requirements, user definitions, all nodes, and reference
  topology by their checked kind/type/effect/capability shapes.
- Retain schema, language version, Unicode version, entry module, exact
  localized stdout, and individual definition witnesses.

## Explicit exclusions

The projection does not define localized keywords, aliases, a public graph
schema, ID compatibility, prose translation, source equivalence, or a Stable
tutorial/support policy.

No language semantic, diagnostic, schema, Semantic ID, package, dependency,
CLI/editor, runtime, Unicode, protocol, support, or public API changes.
