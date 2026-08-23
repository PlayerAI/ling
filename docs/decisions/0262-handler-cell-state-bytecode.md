# DEC-0262: Complete Handler Cell/State bytecode / 完整 Handler Cell/State 字节码

> 状态：Draft
> 提出日期：2026-08-24
> 决定日期：Pending
> Owner role：effect-runtime-design
> 相关 RFC/缺口：RFC-0006 | DEC-0260 | DEC-0261 | GAP-EFFECT-HANDLER-BYTECODE-001
> 生命周期记录：`docs/governance/lifecycle.toml`

This draft preserves DEC-0261's checked interpreter semantics while closing
the implementation gap exposed by its exact bytecode 1.3 record. It is not
implementation authority unless it becomes Accepted.

本草案保留 DEC-0261 的已检查解释器语义，同时关闭其精确 bytecode 1.3 记录在实现中
暴露的缺口；在进入 Accepted 之前，它不是实现权威。

## Question

How must portable bytecode represent shared lexical Cells and unmasked
`State<T>` rows, and which operation-clause parameter patterns are valid, so
that every checked Handler has deterministic interpreter/VM behavior without
changing an already implemented 1.3 wire under the same version number?

## Decision

If accepted, this decision replaces DEC-0261 with the following refinements;
all DEC-0261 interpreter, dispatch, continuation, Fault, Capability,
cancellation, resource, determinism, and deferral rules not changed below are
retained verbatim.

1. **Irrefutable operation inputs.** A Handler operation input parameter must
   be a binding or wildcard pattern. The checker rejects literals, tuples,
   records, constructors, and every other refutable or structurally redundant
   input pattern before publishing `HandlerCore`. The existing
   `L-EFFECT-0005` contract reports reason `refutable_parameter`, canonical
   operation name, and the original pattern span. Resume remains a separate
   checked binding. This removes the invalid state in which a verified
   operation is selected by name but its input dynamically fails to bind.

2. **New minor revision.** Complete Handler lowering selects
   `ling.bytecode/1.4`. Revision 1.3 remains readable and executable only as the
   historical immutable/irrefutable Experimental slice; its bytes and verifier
   rules do not change. The 1.4 reader accepts exact 1.0–1.4 artifacts, while
   every older reader rejects 1.4. No writer silently rewrites a 1.3 artifact.

3. **Internal Cell type.** Version 1.4 adds type tag `0x14` `Cell<T>` containing
   one earlier valid `TypeIndex`. `Cell<T>` is verifier-internal: it may appear
   only in closure capture-prefix parameters and Cell instruction registers.
   It is forbidden in source parameters, function results, tuple/record/variant
   fields, constants, entry signatures, comparison, formatting, Console
   inputs, and public source-type projections. It is not printable,
   comparable, serializable as a runtime value, or a Semantic ID.

4. **Cell instructions.** Version 1.4 adds `0x1d CellNew(destination, initial)`,
   `0x1e CellGet(destination, cell)`, and
   `0x1f CellSet(destination_unit, cell, value)`. The verifier requires exact
   `Cell<T>`/`T` relationships, SSA destinations, source-map coverage, and
   version 1.4. VM Cells are heap-accounted private reference identities.
   `CellSet` commits before its Unit result; Fault/cancellation never rolls it
   back.

5. **Canonical State rows.** Version 1.4 Effect records are version-aware:
   tag `1` remains payload-free `Console.Write`; tag `2` is `State<T>` followed
   by one earlier valid source-value `TypeIndex`. Rows are sorted by
   `(tag, TypeIndex)`, duplicate-free, and bounded by the existing effect-vector
   limit. Function types and function declarations use the same encoding.
   `State<T>` never requires a host Capability and is never masked by Handler.

6. **Lowering and identity.** When a mutable binding crosses a Handler body or
   clause boundary, lowering emits exactly one `CellNew` in the owning lexical
   scope and rewrites every subsequent read/write and every body/clause capture
   to `CellGet`/`CellSet` over that same register identity. Captures retain
   existing register-capture encoding; they carry a typed Cell value rather
   than adding capture tags. Immutable bindings retain ordinary SSA lowering.
   Continuation-frame cloning clones the private Cell reference, never its
   contents, so prior resume mutations remain visible in source order.

7. **Verification and capability closure.** Handler residual rows are body
   rows minus handled labels, union clause rows, with every `State<T>` retained.
   Capability reachability continues to use the unmasked body/clause closure.
   Cell instructions whose State row is missing or mistyped are rejected as
   bytecode Effect/type failures before `VerifiedProgramV1` publication.

8. **Compatibility boundary.** Complete Handler compiler entry points use 1.4;
   the bounded `lower_v1_3` API remains Experimental evidence and continues to
   reject mutable captures. No CLI artifact/default-backend promise exists.
   Bytecode 1.0–1.3 bytes, Semantic Graph 0.1, Audit 0.1/0.2, Program IDs,
   diagnostics other than the registered `L-EFFECT-0005` reason, source
   spellings, and Unicode 17.0.0 remain unchanged.

## Conformance plan

- Reject every refutable Handler input form before checked publication; prove
  binding/wildcard inputs retain original Unicode/BOM/CRLF spans and stable
  bilingual `L-EFFECT-0005` facts.
- Freeze exact 1.4 Cell type, State Effect, and instruction tags; cover
  canonical encoding/re-encoding/disassembly, malformed type indexes,
  forbidden Cell escape, row order/duplicates, missing State rows, old-reader
  rejection, and 1.0–1.3 compatibility.
- Execute mutation before/after zero and one resume, mutation through nested
  function calls, two aliases to one Cell, a second deep operation, Fault after
  commit, and cancellation before restoration under exact heap/frame/handler/
  continuation limits.
- Compare interpreter and VM values, events, resume counts, committed mutation
  observations, Fault projections, spans, Program IDs, and path-independent
  bytes. Repeat deterministic construction and bounded malformed/fuzz cases.
- Run all locked-offline workspace, Clippy, CI, governance, status, RC0,
  traceability, formatting, and diff gates before resolving the gap or marking
  EFF-2104 Done.

## Compatibility impact

- **Source:** narrows Experimental v0.2 Handler input patterns to the only
  operation-binding forms with total semantics; no Seed behavior changes.
- **Bytecode:** adds backward-reading 1.4 and preserves exact 1.0–1.3 bytes.
- **Diagnostics:** allocates no code or schema field; adds one deterministic
  reason value under existing `L-EFFECT-0005`.
- **Runtime:** adds private heap-accounted Cells and exact State rows; exposes no
  address, Rust ownership, host handle, allocation order, or debug text.
- **Semantic data:** no Semantic Graph, Audit, Program ID, package, CLI, LSP,
  editor, Native/Wasm, or Stable contract changes.
- **Determinism/Unicode:** canonical indexes and source order determine bytes;
  original UTF-8 spans and Unicode 17.0.0 remain authoritative.

## Unresolved alternatives

Changing the meaning of already implemented 1.3 bytes, adding new capture
tags, copying Cell contents on continuation restore, dynamically faulting a
valid checked refutable clause, masking State, rollback, public references,
general closure mutable capture, Many-producing source operations, and
Native/Wasm Cell lowering remain rejected or separate work. General mutable
closure capture may reuse the accepted Cell primitives only under its own task
and evidence.

## Supersession

- Intended to supersede on acceptance: `DEC-0261`
- Superseded by: `None`
