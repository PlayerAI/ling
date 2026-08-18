# Ling repository instructions

## Authority

For language behavior, use this order and stop when higher-priority documents conflict:

1. Accepted RFCs under `D:/Coding/Ling/docs/`
2. `D:/Coding/Ling/docs/SEMANTICS.md`
3. `D:/Coding/Ling/docs/LANGUAGE.md`
4. Conformance tests under `D:/Coding/Ling/tests/conformance/`
5. Rust implementation under `D:/Coding/Ling/crates/`
6. Code comments

`docs/IMPLEMENTATION.md` defines engineering order but does not create language semantics.

## Implementation boundaries

- Implement only the v0.0.1 Seed subset unless an accepted RFC expands it.
- Do not resolve specification conflicts through code or snapshots.
- Do not interpret unresolved AST nodes; evaluation must consume checked Typed Core.
- Do not expose Rust ownership, allocation, hash-map order, paths, or debug output as Ling semantics.
- Preserve original UTF-8 byte spans throughout the compiler pipeline.
- Keep public diagnostics bilingual and use registered stable error codes.
- Keep Unicode XID, normalization, security, and generated tables on Unicode 17.0.0.
- Keep normal builds and tests offline after dependencies are locked.
- Do not add placeholder public APIs that imply an unimplemented language feature works.

## Pull-request evidence

Each change must state:

- normative clauses covered;
- specification gaps or conflicts encountered;
- tests added or updated;
- diagnostic, schema, or Semantic ID compatibility impact;
- determinism and Unicode-version impact;
- intentionally deferred work.

