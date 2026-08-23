# IDE-2307 Implementation Report: Checked Completion v0

## Outcome

IDE-2307 implements the Accepted RFC-0042 Preview contract as standard
`textDocument/completion` with exact `ling.lsp.completion/0.1` discovery. The
server classifies six compiler-owned contexts, obtains candidates from one
complete checked snapshot, validates every proposed replacement with a fresh
compiler, ranks and deduplicates deterministically, and emits bounded standard
Completion Lists with exact PlainText text edits.

The implementation does not interpret unresolved AST, recover incomplete
source, expose internal identity/type/effect/capability facts, or call an AI or
network ranking path.

## Normative clauses covered

- RFC-0042 §1: unconditional provider/discovery, request context validation,
  notification silence, and exact Preview marker.
- RFC-0042 §2: Ready-state immutable capture, checked baseline, original-byte
  token/Qualified Name replacement, negotiated position projection, and final
  freshness comparison.
- RFC-0042 §3: expression, projection member, type, pattern/variant,
  import/module, and keyword context classification from the valid CST.
- RFC-0042 §4: checked definitions, bindings, aliases, modules, record fields,
  constructors, Prelude/builtin entries, fixed keywords/wildcard, and fresh
  resolution/type/Effect replacement validation.
- RFC-0042 §5: deterministic prefix/scope/import/name/identity ordering,
  duplicate-label collapse, 512 catalog/probe bound, 256 item cap, and explicit
  `isIncomplete` behavior.
- RFC-0042 §6-§7: exact standard item fields and CompletionItemKind values,
  single-range PlainText edits, fixed bilingual failures, response-size bound,
  and absence of internal metadata.
- DEC-0002/DEC-0029: all request and response positions project through the
  negotiated SourceMap encoding while compiler spans remain original UTF-8
  bytes.
- DEC-0019/DEC-0071: analyses use owned request snapshots and fresh compiler
  instances without persistence or mutation promises.
- DEC-0079/DEC-0080: existing source/metadata observations remain unchanged;
  the new catalog is a separate bounded checked fact projection.

## Implementation evidence

- `crates/ling-db/src/checked_completion_catalog.rs` owns the immutable,
  wire-agnostic checked catalog and deterministic compiler-fact ordering.
- `crates/ling-db/src/lib.rs` exposes the catalog only through a complete
  checked-workspace query.
- `crates/ling-lsp/src/completion.rs` owns request validation, context
  classification, candidate filtering, checked replacement probes, ranking,
  exact edits, bounds, freshness, and failures.
- `crates/ling-lsp/src/lib.rs` dispatches the standard method and advertises the
  exact standard and Experimental capabilities.
- `crates/ling-lsp/tests/completion.rs` drives the real JSON-RPC handler across
  all six contexts, module and record members, import aliases, checked types,
  constructors/wildcards, keyword kinds, exact edits, malformed contexts,
  notifications, and incomplete-source failure.
- `tests/fixtures/lsp-diagnostics-v1/` records the capability in exact stdio
  transcript bodies and active protocol metadata.
- `tests/protocols/lsp-completion/README.md` records the fixture boundary and
  no-network/no-AI constraint.

## Checks executed before status binding

- `cargo test -p ling-db --locked --offline checked_completion_catalog`
- `cargo test -p ling-lsp --test completion --locked --offline`
- `cargo test -p ling-lsp --all-targets --locked --offline`

The implementation tree also passed the locked/offline workspace test suite,
strict workspace Clippy, `cargo xtask ci verify`, governance, LSP, support,
status, RC0, v0.0.1 traceability, formatting, and diff checks immediately
before its implementation commit. These are local command results; no hosted
CI result is claimed. The same gate set is repeated after status binding.

## Compatibility impact

- Protocol: adds Public Preview `ling.lsp.completion/0.1` with standard
  `completionProvider`, `textDocument/completion`, and exact `lingCompletion`
  discovery. There is no predecessor or resolve support.
- Compiler API: adds an internal bounded immutable checked catalog query. The
  catalog has no serialization or persistence compatibility promise.
- Diagnostics: no registered diagnostic code or compiler diagnostic changes;
  protocol errors remain fixed JSON-RPC InvalidParams/RequestFailed values.
- Identity/schema: resolver identities are internal ordering facts only; no
  Semantic ID, diagnostic schema, package schema, or cache schema changes.
- Language/runtime: no syntax, language semantics, Typed Core evaluation,
  interpreter, bytecode, VM, ABI, filesystem/network, or package behavior
  changes.
- Unicode: all compiler identifier facts and spans remain on Unicode 17.0.0;
  no generated table or normalization policy changes.

## Determinism and safety

Candidate input uses ordered checked compiler facts. Ranking is a total tuple,
duplicate labels collapse deterministically, JSON arrays preserve that order,
and `sortText` records the final ordinal. Each candidate is checked in an
isolated owned-byte override; failed probes emit nothing and mutate no server
state. Hash-map iteration, locale, wall clock, randomness, host paths, debug
formatting, remote services, and AI do not affect output.

## Specification gaps and conflicts

RFC-0042 closes the IDE-2307 request/context/ranking/insertion/snapshot
authority gap only for its bounded Preview. No higher-authority conflict was
encountered. The historical execution-plan wording was constrained to the
Accepted current CLI/source names and the existing checked compiler pipeline.

## Intentionally deferred

Incomplete-source recovery, zero-width sites, declaration-name completion,
auto-imports, snippets, additional edits, item labels differing from inserted
text, lazy `completionItem/resolve`, documentation/signatures,
Effect/Capability presentation, generated sources, dynamic registration,
cancellation, persistent caches, AI ranking, and Stable lifecycle remain
deferred. IDE-2308 owns the next presentation/resolve decision and may not
infer it from this implementation.
