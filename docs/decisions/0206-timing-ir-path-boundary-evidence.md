# DEC-0206: Internal Timing IR and Path boundary evidence / Timing IR 与路径边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0205` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`TIM-5701-OBSERVATION`. It records provisional Timing IR, target, path, bound,
assumption, identity, source-link, failure, and fixture vocabulary while
Critical Profile, Node timing, target, boundedness, and evidence authorities
remain unresolved.

本决定只授权 `TIM-5701-OBSERVATION` 使用 test-local 的 Timing IR、target、path、bound、assumption、
identity、source-link、failure 与 fixture 边界清单；在 Critical Profile、Node timing、target、
boundedness 与 evidence 权威尚未解决时，只记录临时词汇，不实现时间分析协议。

## Question

TIM-5701 proposes recording target instructions or blocks, control-flow
paths, loop bounds, cache and memory assumptions, interrupt and scheduler
models, device/FFI call bounds, and source maps. Which vocabulary can be
retained as bounded evidence without choosing a Timing IR, target cost model,
path-composition rule, WCET meaning, or public evidence protocol?

## Decision

1. `crates/ling-types/tests/timing_ir_path_evidence.rs` keeps a test-local
   inventory of sixty provisional representation, target, control-flow,
   bound, assumption, identity, source-link, failure, diagnostic, and fixture
   boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.timing-ir-path-observation/0`. These bytes are
   observation evidence only; they are not a Timing IR, cost model, WCET
   result, analyzer input/output schema, diagnostic, protocol, or support
   claim.
3. No Timing IR, path solver, target-cost table, deadline hook, reader/writer,
   dependency, diagnostic allocation, CLI/LSP route, support claim, or
   placeholder API is added. Public `TIM-5701` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:477-487` is a
  non-normative checklist. It defines no field types, canonical identity,
  target model, cost unit, path rule, failure behavior, or file format.
- `docs/status/TIM-5701-AUTHORITY-AUDIT.md` records the absent Timing IR,
  WCET contract, target/ABI and interference models, evidence schema, and
  executable fixtures.
- `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future.
- Draft SEMANTICS/LANGUAGE timing sketches, accepted RFC-0014 VM resource
  limits, RFC-0019 differential evidence, RFC-0020 host cancellation, and
  DEC-0019 incremental scheduling are not target-timing authorities.
- Critical Profile, Node, boundedness, target/ABI, scheduler, device/FFI,
  Contract/Proof, and timing-evidence authorities remain absent or unresolved.

## Conformance plan

- Assert all sixty Timing IR/path categories and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer Timing IR/analyzer implementation, target/path/cost/WCET semantics,
  schemas, diagnostics, protocols, and public support until Accepted authority
  and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing CFG/source-map, bytecode resource, differential,
cancellation, and query-scheduler evidence is not reinterpreted as timing or
WCET evidence; only test-local evidence is added.

## Unresolved alternatives

Versioned canonical Timing IR; target/profile/build and instruction/block/path
identity; calls, returns, branches, loops, recursion and path conditions;
loop/recursion/call/resource bounds and their proof or assumption status;
cost units and instruction/block/path composition; processor, cache, memory,
bus, interrupt and scheduler interference; device/FFI/I/O call bounds;
Checked Core, bytecode, machine-code and source-map linkage; stable Semantic
IDs and original UTF-8 spans; unknown, infeasible, incomplete, malformed,
corrupt, unsupported-version, migration and fail-closed behavior; tool and
evidence identity, provenance/checksum/signature/redaction; measurements,
estimates, static bounds, assumptions and WCET wording; diagnostics,
positive/negative/loop/call/Unicode/differential fixtures, protocol inventory,
and public support remain open under TIM-5701, TIM-5702, TIM-5703,
GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE, and missing timing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
