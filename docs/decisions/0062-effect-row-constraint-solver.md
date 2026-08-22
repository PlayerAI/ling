# DEC-0062: Deterministic effect-row constraint solver / 确定性 Effect Row 约束求解器

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: effect-system-design  
> 相关 RFC/缺口：`RFC-0006`, `GAP-EFFECT-HANDLER-001`, `GAP-EFFECT-STATE-MASKING-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision closes the EFF-2102 solver boundary left open by RFC-0006. It
authorizes a deterministic, in-process solver for the accepted experimental
v0.2 Effect row model. It does not add source syntax, change the Seed checker,
or define a public protocol.

本决定补充 RFC-0006 留给 EFF-2102 的求解器边界，授权针对已接受的实验性
v0.2 Effect Row 模型实现确定性的进程内求解器。它不增加源代码语法、不改变
Seed 检查器，也不定义公共协议。

## Question

RFC-0006 defines canonical rows, row variables, handler residuals, and the
Capability separation, but explicitly leaves row constraint grammar,
unification, occurs checking, generalization, conflict selection, and
diagnostic allocation to EFF-2102. What bounded solver contract lets those
stages be implemented without making host ordering or an unresolved runtime
policy part of Ling semantics?

RFC-0006 已规定 canonical row、row variable、handler residual 及 Capability
分离，但明确将约束语法、统一、occurs check、generalization、冲突选择和诊断
分配留给 EFF-2102。本决定规定一个有限的求解器契约，使实现不把宿主顺序或
尚未决定的运行时策略带入 Ling 语义。

## Decision

1. **Scope and input boundary.** The solver consumes only validated
   `EffectRowModel` values, `EffectLabel` values, row variables, and checked
   provenance supplied by a later checked pipeline. It is an in-process
   `ling-effects` component for Experimental v0.2. It does not parse source,
   interpret unresolved AST nodes, change Typed-Core schemas, execute handlers,
   grant capabilities, or alter v0.0.1 Seed behavior.

2. **Constraint grammar.** A constraint is one of:
   - `Equal(left, right, origin)`, requiring two row terms to denote the same
     row; or
   - `Requires(row, label, origin)`, requiring a label to be present in a row.

   A row term is a sorted, duplicate-free finite label set plus either a closed
   tail or one `RowVariableId`, exactly as defined by RFC-0006. `origin` is a
   checked, non-negative ordinal with an optional original UTF-8 byte span.
   Origins are evidence only; they never enter row identity or canonical row
   bytes. Every diagnostic retains the original span when one is supplied.

3. **Canonical work order.** Before solving, constraints are stably sorted by
   `(origin ordinal, start byte, end byte, kind, left canonical bytes, right
   canonical bytes, required-label canonical bytes)`. Equal constraints are
   deduplicated after sorting. All label sets, substitutions, quantified
   variables, residual rows, conflict facts, and diagnostic facts are emitted
   in their defined ordinal or canonical-byte order. No hash-map iteration,
   thread scheduling, pointer, allocator identity, path, or debug formatting
   may affect a result.

4. **Substitution and unification.** The solver maintains a substitution from
   row variables to normalized row terms. Applying a substitution reaches a
   fixed point and rejects recursive bindings. A variable-to-variable binding
   always binds the numerically larger `RowVariableId` to the smaller one. A
   variable-to-row binding is legal only after the occurs check. Closed rows
   unify only when their label sets are equal. For open rows, missing labels are
   placed in deterministic residual tails; distinct tails are related by a
   fresh binder-local variable allocated from the sorted constraint stream,
   beginning at one greater than the largest input variable. Reordering the
   same constraints therefore produces the same substitution and solution.

5. **Occurs check and conflicts.** A variable occurs in a term when it is the
   term's tail or is reachable through the current substitution. Binding a
   variable to a term containing that variable is rejected as an infinite row.
   A closed-row mismatch, an unsatisfied `Requires` constraint, an invalid
   recursive binding, and an incompatible handler residual are solver
   conflicts. The solver returns a minimal deterministic conflict set: the
   smallest sorted set of origin ordinals that is sufficient to reproduce the
   failure. Conflict facts contain canonical row/label spellings and preserved
   UTF-8 spans, never Rust debug output.

6. **Generalization and instantiation.** Generalization is explicit and is
   allowed only when the checked caller marks the binding as a value and the
   value-restriction boundary permits it. It quantifies exactly the sorted row
   variables free in the solved row and absent from the supplied environment.
   A non-value or effectful binding remains monomorphic. Instantiation takes a
   quantified scheme plus a caller-provided deterministic fresh-variable
   sequence; it never uses process-global allocation or pointer identity.
   Generalization does not authorize ownership, memory, Task, Actor, Replay,
   Remote, Native, GPU, or FFI behavior.

7. **Handlers and State.** Handler subtraction removes only labels whose
   operation is explicitly present in the accepted `HandlerContract` and
   preserves the input tail, including an open tail. Nested subtraction is
   applied from the inner residual to the outer handler. `State<T>` is never
   implicitly masked or removed; only an exact, explicitly declared operation
   can remove a matching label, and handling it still grants no Capability.

8. **Capability separation.** Solving Effect presence never creates,
   deletes, or infers a host Capability. Existing Seed Capability closure and
   the RFC-0006 separation remain authoritative. Missing Capability is reported
   by the Capability checker, not converted into a row-unification result.

9. **Diagnostics.** EFF-2102 diagnostics use the registered `L-EFFECT-*`
   domain and bilingual message/fact fields. This implementation allocates
   `L-EFFECT-0001` for an unsatisfied/incompatible row constraint and
   `L-EFFECT-0002` for an occurs-check (infinite-row) conflict. Diagnostic
   ordering follows the conflict ordering above; codes are stable strings and
   are shared by human and JSON output. No new code is allocated outside
   `docs/ERROR-CODES.md` and its generated lock evidence.

10. **Compatibility and experiment marker.** The solver is an Experimental
    v0.2 in-process API. Existing Seed source acceptance, diagnostics, Semantic
    IDs, schemas, bytecode, VM, CLI, LSP, protocols, and Unicode 17.0.0 data
    remain unchanged. Public source syntax and Checked-Core lowering require
    the separate EFF-2103 authority; runtime and transport behavior require
    their own accepted RFCs.

## Conformance plan

- Add positive fixtures for pure, closed, open, parameterized, reordered, and
  duplicate rows; equality and required-label constraints; polymorphic rows;
  deterministic generalization/instantiation; and nested handler subtraction.
- Add negative fixtures for closed-row conflicts, missing required labels,
  distinct-tail conflicts, occurs cycles, incompatible residuals, and attempts
  to mask `State<T>` implicitly.
- Assert that randomized insertion order, Unicode NFC spelling, CRLF/BOM
  source bytes, and preserved UTF-8 spans produce identical solutions and
  diagnostics.
- Assert that conflict facts are minimal, sorted, bilingual, and stable in
  both human and JSON forms, including `L-EFFECT-0001` and `L-EFFECT-0002`.
- Compare clean and incremental checked inputs and a reference implementation
  on the same canonical constraint corpus before handing results to EFF-2103.
- Keep source syntax, Typed-Core lowering, runtime, Task/Actor, Replay, Remote,
  Native, GPU, FFI, and Stable 1.0 compatibility fixtures deferred to their
  own authorities.

## Compatibility impact

- Source language and Seed checker: None; no v0.0.1 syntax or acceptance rule
  changes.
- Diagnostics: Adds the two registered Experimental v0.2 `L-EFFECT-*` codes
  only when the new in-process solver is explicitly invoked; existing codes
  and bilingual formatting remain unchanged.
- Semantic IDs, schemas, CLI, LSP, bytecode, VM, protocols, package metadata,
  and ABI: None; no public field or wire format is changed.
- Unicode and spans: Canonical identities remain Unicode 17.0.0 and path-free;
  source spans, when present, retain original UTF-8 byte offsets for evidence.
- Runtime and Capability: None; solving does not execute, mask, or authorize
  an Effect.

## Unresolved alternatives

- Source-level row syntax and Checked-Core handler nodes remain EFF-2103.
- Runtime handler execution, continuation storage, Task/Actor lifecycle,
  supervision, Replay, Remote, Native, GPU, and FFI remain separate accepted
  authorities.
- A future RFC may replace the numeric binder-local fresh-variable policy only
  with migration evidence and byte-for-byte differential fixtures.
- A future diagnostic policy may add specialized subcodes only through the
  single error registry and a superseding accepted decision.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
