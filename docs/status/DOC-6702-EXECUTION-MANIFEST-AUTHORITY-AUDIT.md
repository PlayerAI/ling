# DOC-6702-EXECUTION-MANIFEST Authority Audit

- Parent: `DOC-6702` — Two-layer Examples
- Child: `DOC-6702-EXECUTION-MANIFEST` — Seed example execution manifest
- Release: G6
- Decision: `Done` is authorized only for this bounded internal child by
  Accepted `DEC-0239`; the parent remains `BlockedSpec`.

## Authority and gap

Accepted DEC-0046 protects the seven requirement and seven feature rows in the
Seed example inventory. Before this child, the inventory verifier did not
validate example source paths or execution metadata, while the CLI process test
maintained its own hard-coded five-case list and omitted `examples/hello.ling`.

DEC-0239 authorizes a strict internal six-case manifest shared by the inventory
gate and the real CLI process test. Successful execution is evidence for the
current Experimental/Preview Seed boundary only.

## Authorized implementation

- Record the exact six example paths, their minimal/realistic/tutorial roles,
  expected UTF-8 stdout, Semantic witnesses, and identifier classification.
- Strictly validate manifest schema, cardinality, uniqueness, classification,
  safe existing `.ling` paths, stdout shape, and witnesses.
- Drive `ling check`, `ling run`, and `ling semantic` over every manifest case.
- Preserve the separate registered negative conformance corpus and Audit test.

## Explicit exclusions

The manifest is not a public protocol, Stable-support declaration, future
syntax registry, package example format, profile/target matrix, benchmark, or
cross-platform release certification.

No language semantic, diagnostic, schema, Semantic ID, package, dependency,
CLI/editor, runtime, Unicode, protocol, support, or public API changes.
