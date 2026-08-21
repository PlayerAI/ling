# TRAIT-1309 Authority Audit: Solver performance and termination

## Outcome

`TRAIT-1309` is correctly recorded as `BlockedSpec`. The current internal
solver already implements the accepted first-slice termination evidence, but
the execution plan also asks for a deep-chain/diamond/failure/cross-package
benchmark and an explainable resource limit suitable for an LSP. No accepted
authority defines those performance and cancellation semantics yet.

No benchmark command, public timing schema, wall-clock guarantee, LSP budget,
or placeholder cancellation API was added. The existing Seed and internal
solver behavior remains unchanged.

## Normative traceability

- Accepted RFC-0005 §2.5 requires recursive obligations to terminate on the
  ordered active-obligation key and bounds nesting at 64; it does not define a
  wall-clock, allocation, candidate-count, or editor cancellation budget.
- Accepted RFC-0005 §3.5 and §5.3 require deterministic candidate/error
  behavior and forbid public CLI/LSP/protocol Trait claims without independent
  fixtures. The RFC does not freeze a benchmark output format or a performance
  threshold.
- Accepted DEC-0026 defines the crate-private solver boundary, the active-key
  cycle check, and the 64-level `DepthLimit` evidence. It intentionally leaves
  generic HIR integration, public diagnostics, Semantic Graph projection, and
  CLI/LSP behavior to later decisions.
- `docs/SEMANTICS.md` requires deterministic behavior and bounded structured
  diagnostics, while `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leave request cancellation, document
  versions, and Stable versus Experimental editor behavior open.

## Current implementation evidence

The repository establishes the safe part of the boundary:

- `crates/ling-types/src/solver.rs` uses an ordered `BTreeSet` active key,
  rejects repeated obligations as `Cycle`, and reports `DepthLimit` at the
  accepted 64-level boundary.
- The solver scans the internal coherence index and consumes a crate-private
  `BTreeMap<ImplId, Vec<Obligation>>`; production HIR/Checked Core integration
  and LSP query cancellation are not present.
- Existing solver tests cover a concrete selection, unsatisfied/variable and
  ambiguous candidates, active cycles, invalid arity, and the 64-level limit.
  They are correctness tests, not a versioned performance benchmark corpus.
- The repository has a general opt-in incremental-query timing tool, but it
  does not measure Trait obligations and makes no absolute performance
  promise. Reusing it for Traits would require an accepted fixture and scope.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. the production obligation graph and the work units to count (obligations,
   candidates, recursive edges, allocations, or another deterministic metric);
2. the relationship between the RFC-backed 64-level semantic limit and any
   additional resource budget, including precedence and bilingual diagnostic
   behavior;
3. cancellation, deadline, document-version, and stale-result behavior when a
   solver is invoked from an LSP or incremental query;
4. the internal benchmark corpus and machine-readable evidence scope for deep
   chains, diamonds, failures, cross-package ownership, repeated runs, and
   hash/filesystem/process variation; and
5. acceptance thresholds, variance policy, hardware/environment metadata, and
   the rule that benchmark observations do not become language semantics or a
   public protocol without an explicit versioned decision.

Until those decisions and the production integration exist, changing the
solver limit or publishing a benchmark budget would either alter accepted
termination semantics or imply an unsupported LSP performance contract.

## Evidence and compatibility

This audit was checked against `docs/RFC-0005.md`,
`docs/decisions/0026-trait-solver-v0-boundary.md`, `docs/SEMANTICS.md`,
`docs/ROADMAP-1.0.md`,
`docs/governance/gap-register.toml`,
`docs/ling_execution_plan/03-G1-V0.1-LIVING.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`crates/ling-types/src/solver.rs`, and `tools/xtask/src/performance.rs`.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

`TRAIT-1309` can begin after the solver is attached to the accepted HIR/Typed
Core pipeline and the resource/cancellation and benchmark evidence contracts
are Accepted. The implementation should preserve the RFC-0005 64-level
termination rule, keep measurements opt-in and deterministic, and avoid
turning host timing into Ling semantics.
