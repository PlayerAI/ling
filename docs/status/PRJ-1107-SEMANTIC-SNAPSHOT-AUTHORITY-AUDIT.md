# PRJ-1107-SEMANTIC-SNAPSHOT Authority Audit

## Outcome

`PRJ-1107-SEMANTIC-SNAPSHOT` is an authorized, bounded child of the blocked
`PRJ-1107` project API task. Accepted DEC-0083 authorizes an internal
`ling-db` query that consumes an already validated `LockedProject` and returns
the existing package-aware `ling.semantic/0.2` snapshot. The child does not
make the parent project API or CLI complete.

## Normative traceability

- RFC-0002 fixes manifest/package identity, deterministic local graph, retained
  source bytes, and package-aware library boundaries.
- DEC-0058 fixes the immutable, read-only `LockedProject` containing the
  manifest, graph, and canonical lock projection.
- DEC-0083 §§Decision 1–4 authorize the query entry point, canonical source
  ordering and names, the existing checked pipeline, graph-identity cache, and
  the negative boundary around host, CLI, execution, artifacts, and protocols.
- DEC-0002 and DEC-0012 continue to govern original UTF-8 spans and semantic
  identity; no new position unit or identity algorithm is introduced.

## Implemented boundary

The query is `CompilerDb::project_semantic_snapshot(&LockedProject)`. It
traverses package/source records in graph order, assigns deterministic source
IDs, names sources as `package:<package-name>/<logical-path>`, and invokes
parse → AST → HIR → `resolve_project` → type check → effect check →
`ling_semantic::build_project`. Results are cached by the immutable graph ID;
errors are retained as typed internal failures and no partial snapshot is
published.

The adapter does not select a manifest or workspace, inspect physical paths,
modify a lock, access the network, create a compiler-host lifecycle, execute or
test programs, produce artifacts, add diagnostics, or serialize a CLI/LSP/DAP
response.

## Specification gap and deferred work

`GAP-PROJECT-CLI-INTERFACE-001` remains open for public semantic project check,
workspace/member selection, command exits and machine output, run/test/build
semantics, artifact policy, and a general compiler-host contract. Those areas
remain in the blocked `PRJ-1107` parent and must not be inferred from this
internal query.

## Evidence and compatibility

- `crates/ling-db/src/project_snapshot.rs` owns the adapter and typed errors.
- `crates/ling-db/src/lib.rs` owns the cached query and the locked offline
  fixture test.
- The test compares repeated pointer reuse, package-graph identity, project
  schema, package count, and absence of fixture/host path text.
- No language semantics, diagnostic allocation, schema, Semantic ID algorithm,
  CLI/LSP/DAP behavior, runtime, bytecode, VM, ABI, dependency version, or
  Unicode 17.0.0 behavior changes.
