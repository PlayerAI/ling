# CLI-1705 Authority Audit: Semantic query and patch commands

## Outcome

`CLI-1705` is correctly recorded as `BlockedSpec`. The execution plan asks the
CLI to expose versioned Semantic Query and Transaction operations, with
read-only queries by default and patches checked against a ProgramSnapshot and
preserve constraints. The repository has no accepted query CLI contract, and
`PROTO-SEMANTIC-TRANSACTION` is explicitly inventoried as Future.

No `query` or `patch` command, mutation API, transaction parser, capability
surface, preserve checker, or placeholder protocol field was added. Existing
file-oriented `ling semantic`/`ling audit` behavior remains unchanged.

## Normative traceability

- Accepted DEC-0012 fixes canonical Semantic IDs and bytes, but does not define
  a query request schema or mutation/transaction protocol.
- `docs/SEMANTICS.md` §25 describes the conceptual Transaction input,
  stale-base rejection, temporary-graph validation, preserve constraints, and
  commit/rollback sequence. It is not a versioned wire/CLI schema and does not
  define command argument, output, or capability negotiation.
- `PROTO-SEMANTIC-GRAPH-JSON` is Experimental and file/project projection
  details remain versioned; `PROTO-SEMANTIC-TRANSACTION` is Planned public and
  Future with no implementation or fixtures.
- `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` and
  `GAP-LSP-TRANSACTION-PROTOCOL-001` leave protocol lifecycle, snapshot/version
  preconditions, Workspace Edits, and Stable versus Experimental fields open.
- `GAP-PROJECT-CLI-INTERFACE-001` leaves project selection, lock/offline
  behavior, command exits, and JSON output open. No accepted CLI can safely
  project a package-aware patch until that contract exists.

## Current interface evidence

The current repository confirms the missing boundary:

- `crates/ling-cli/src/main.rs` implements file-oriented `semantic` and
  `audit` rendering only; it has no query or patch branch.
- `ling-semantic` produces an Experimental canonical snapshot/JSON projection,
  but no public query language, transaction decoder, preserve validator, or
  commit/rollback service is exposed.
- The current project/query database APIs are internal library boundaries;
  they do not establish a CLI protocol or authorization model for
  `Graph.Read`, `Graph.Propose`, or `Graph.Commit`.
- Adding patch operations before the transaction contract would risk applying a
  stale or mispositioned edit and would turn an unversioned internal shape into
  a public compatibility surface.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. versioned query request/response schemas, target identity and selection,
   pagination/limits, deterministic ordering, and file/project scope;
2. Transaction encoding, base graph/program identity, stale-version behavior,
   operation and preserve-constraint schemas, provenance, and capability
   authorization;
3. temporary-graph validation, name/type/effect/capability/ownership/contract
   checks, required-test policy, semantic diff, atomic commit/rollback, and
   filesystem failure behavior;
4. CLI command/option, human/JSON output, localization, exit/error mapping,
   offline/lock selection, and interaction with LSP Workspace Edits; and
5. protocol inventory/lifecycle updates plus positive, negative, stale-base,
   preserve-failure, authorization, deterministic, cross-package, Unicode,
   CRLF, and migration fixtures.

Until those decisions and fixtures are Accepted, implementing `query` or
`patch` would either expose an Experimental graph as a stable API or permit
unsafe mutation without the required stale-snapshot and preserve guarantees.

## Evidence and compatibility

This audit was checked against `docs/decisions/0012-semantic-identity-and-canonical-bytes.md`,
`docs/SEMANTICS.md`, `docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`,
`docs/governance/protocol-inventory.toml`, `docs/governance/gap-register.toml`,
`crates/ling-cli/src/main.rs`, `crates/ling-semantic`, `crates/ling-db`, and
the current status/backlog registries.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

`CLI-1705` can begin after the query and Transaction protocols are Accepted,
their lifecycle/authorization is registered, and executable fixtures exist.
The implementation must default to read-only queries, reject stale bases, and
commit patches atomically without inventing a second semantic authority.
