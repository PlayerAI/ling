# TIM-5701 Authority Audit

- Task: `TIM-5701` — Timing IR and Path
- Plan: `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:477-487`
- Release: G5
- Status: `BlockedSpec`

## Decision

TIM-5701 is `BlockedSpec`. The execution plan lists the information that a
future timing analysis might record—target instructions or blocks, control-flow
paths, loop bounds, cache and memory assumptions, interrupt and scheduler
models, device/FFI call bounds, and source maps—but it does not define a
language semantic, an intermediate representation, a target contract, or an
evidence protocol.

No accepted specification authorizes a Timing IR or WCET path implementation.
The repository has no accepted Critical Profile, Node timing, target/ABI,
boundedness, scheduler, device/FFI, or evidence authority that could make the
listed values meaningful and reproducible. Adding an IR, path solver, schema,
diagnostic, or public command would therefore invent semantics beyond the Seed
subset.

## Normative traceability

- `09-G5-V0.5-CRITICAL.md:477-487` is a non-normative planning checklist. It
  specifies neither canonical field types nor how paths, bounds, assumptions,
  targets, or source locations are identified and compared.
- `docs/SEMANTICS.md:1385-1425` sketches `node`, explicit clocks, WCET and
  memory bounds, and deadline evidence, but `SEMANTICS` is Draft in
  `docs/governance/authority.toml` and does not provide an accepted Timing IR
  or target model. `docs/LANGUAGE.md:858-865` is likewise a Draft surface
  illustration, not authorization for `node`, `every`, or `deadline` syntax.
- `docs/ROADMAP-1.0.md:439-466` calls for Critical Profile and Node timing
  gates, virtual-clock tests, and worst-case-path fixtures. The roadmap is
  Planning authority and explicitly depends on the Draft language documents;
  it cannot establish a public representation or WCET claim.
- `GAP-CRITICAL-PROFILE-001` remains Open. It names Node timing/Fault
  semantics, boundedness, and evidence schema as unaccepted, proposes RFC-0012,
  and requires explicit non-claims before any Critical API or evidence format.
  No RFC-0012, timing RFC, or accepted replacement is present.
- Accepted RFC-0014 defines portable bytecode and deterministic VM step,
  frame, and host heap limits. It deliberately leaves target execution,
  cache behavior, common logical heap accounting, native/FFI ABI, and WCET
  outside its authority. Accepted RFC-0018, RFC-0019, and RFC-0020 define
  capability/failure normalization, interpreter–VM differential evidence, and
  host cancellation/resource evidence; none defines timing analysis.
- Accepted DEC-0019's deterministic scheduler is an internal incremental-query
  boundary. It does not specify a Ling execution scheduler, interrupt model,
  deadline interference, target instruction costs, or a persistent timing
  artifact.
- There is no Timing/WCET protocol in `docs/governance/protocol-inventory.toml`.
  The existing bytecode and VM-control entries are unrelated and do not expose
  target timing data as Ling semantics.

## Evidence in this repository

No Timing IR, path representation, WCET analyzer, target-cost model, deadline
checker, timing schema, reader/writer, or timing fixture exists under `crates/`
or `tests/`. Existing CFG/source-map and VM resource-limit code is scoped to
accepted Seed compiler/VM behavior; it cannot be treated as a target timing
model. No `ling` CLI, LSP request, diagnostic, or public protocol claims
TIM-5701 support.

## Required authority before implementation

An accepted RFC or replacement decision must define, at minimum:

1. A versioned canonical Timing IR with target/profile/build identity,
   instruction/block and control-flow records, path/source-map linkage, stable
   Semantic IDs, and explicit treatment of calls, recursion, loops, branches,
   suspension, and dynamic topology.
2. The loop/recursion and resource-bound language, proof or assumption status,
   unknown/infeasible path behavior, path-composition rules, and how timing
   facts interact with the accepted Checked Typed Core, Node, Task, Actor,
   Contract, and boundedness boundaries.
3. A target cost and interference model covering processor and instruction
   identity, cache/memory/bus assumptions, interrupts, scheduler interference,
   device and FFI calls, I/O, and measurement versus static analysis. It must
   state which conclusions are WCET evidence and which are estimates or
   assumptions.
4. Profile and build binding rules for deadline conclusions, including target
   package, compiler/toolchain, scheduler, clock, margin, and TCB identity.
   Host paths, addresses, wall-clock observations, allocator layout, and debug
   text must not become Ling identity.
5. Versioned result/evidence and failure behavior for unsupported targets,
   missing assumptions, unknown or unbounded paths, inconsistent source maps,
   malformed data, and schema migration, with registered bilingual
   `L-<DOMAIN>-<NUMBER>` diagnostics and fail-closed process outcomes.
6. Offline positive, negative, boundary, migration, Unicode 17.0.0,
   BOM/CRLF, source-span, target/profile variation, and repeated-run
   determinism fixtures. Evidence must disclose limitations and must not turn
   measurements or bounded estimates into a WCET proof.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, or
support claim. It preserves the accepted `ling` CLI and `.ling` source
extension, original UTF-8 spans, Unicode 17.0.0, deterministic identity rules,
and the checked Typed Core boundary. It deliberately adds no Timing IR, path
solver, target-cost table, timing reader/writer, deadline hook, or placeholder
API, and it introduces no stale `zero` names.

TIM-5701 remains deferred until Critical Profile, Node, boundedness,
target/ABI, scheduler, device/FFI, Contract/Proof, and evidence authorities are
Accepted and their executable fixtures are available. TIM-5702 and TIM-5703
must not be implemented as if this missing representation or evidence boundary
already existed.
