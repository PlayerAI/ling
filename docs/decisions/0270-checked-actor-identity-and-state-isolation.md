# DEC-0270: Checked Actor identity and state isolation / 已检查 Actor 身份与状态隔离

> 状态：Accepted<br>
> 提出日期：2026-08-30<br>
> 决定日期：2026-08-30<br>
> Owner role：actor-semantics<br>
> 相关 RFC/缺口：DEC-0090 | DEC-0095 | GAP-ACTOR-AWAIT-REENTRY-001 | GAP-ACTOR-MAILBOX-SUPERVISOR-001 | GAP-ACTOR-REMOTE-DELIVERY-001 | ACT-2301<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision defines only the checked, non-executable ACT-2301 Actor slice:
source declarations, semantic type identity, a typed local-reference descriptor,
and an isolated pure state transition. It does not authorize Actor construction,
send, mailbox, scheduling, suspension, reentry, supervision, remote delivery,
serialization, interpreter execution, bytecode, VM execution, or native ABI.

本决定仅定义 ACT-2301 的已检查、不可执行 Actor 切片：源码声明、语义类型身份、
类型化本地引用描述，以及隔离的纯状态转移。它不授权 Actor 构造、send、mailbox、
调度、挂起、重入、监督、远程交付、序列化、解释器执行、bytecode、VM 执行或
native ABI。

## Question

What is the smallest complete Actor frontend and Checked Core that establishes
Actor type identity and state isolation without pre-deciding message ownership,
mailbox/backpressure, await reentry, runtime scheduling, or remote delivery?

## Decision

1. **Checked-only boundary.** ACT-2301 accepts Actor source only through the
   normal CST → AST → unresolved HIR → resolver → type/effect checker pipeline.
   Successful checking publishes immutable Checked Actor Core. AST, unchecked
   HIR, and malformed Core are never executable. Every Actor declaration remains
   rejected by `run`, interpreter, bytecode, VM, and native entry until ACT-2305
   gains separate Accepted authority.

2. **Contextual source form.** `actor`, `state`, and `receive` are contextual
   identifiers, not reserved lexer keywords. The first profile has exactly one
   message type, one state type and initializer, and one transition body:

   ```ling
   actor Counter : Message =
       state Int = 0
       receive state message =
           state + 1
   ```

   The grammar is:

   ```text
   ActorDeclaration := "actor" Name ":" Type "=" NEWLINE INDENT
                       StateClause ReceiveClause DEDENT
   StateClause       := "state" Type "=" Expression NEWLINE
   ReceiveClause     := "receive" Pattern Pattern "=" NEWLINE INDENT
                       Expression DEDENT
   ```

   The two receive patterns bind current state then message. Duplicate clauses,
   extra clauses, generic Actor parameters, multiple message variants, mailbox
   declarations, methods, guards, lifecycle clauses, and alternate bodies are
   outside this profile and fail before Checked Actor Core publication.

3. **Actor type identity.** Each resolved declaration receives an
   `ActorTypeId` derived from the same package/module/definition semantic
   identity inputs as other nominal declarations. Source names, host paths,
   traversal order, allocation, hash-map order, and Rust type names do not enter
   that identity. Renaming or moving the public declaration changes its identity;
   changing only trivia or source-file identity does not.

4. **Runtime instance identity contract.** `ActorId` denotes one runtime
   incarnation and is opaque, run-scoped, nonzero, and unique among live and
   retired incarnations in that runtime. It is not a Semantic ID, source-derived
   value, address, scheduler index, stable cross-run identifier, or reusable
   identity after stop/restart. ACT-2301 records this contract in Checked Actor
   Core and typed model evidence but does not allocate an `ActorId`; allocation
   and lifecycle belong to ACT-2305.

5. **Typed local reference descriptor.** Each Checked Actor declaration exposes
   an internal `ActorRef<Message>` descriptor pairing its `ActorTypeId` with the
   fully checked message type. It is local, opaque, invariant in `Message`, and
   does not reveal `ActorId` or state. ACT-2301 provides no source constructor,
   value, equality operator, send operation, capability conversion, or runtime
   handle, so the descriptor cannot imply executable Actor support.

6. **Local/remote separation.** `ActorRef<Message>` has no source, canonical,
   wire, or host serialization. It cannot be converted to or represented as
   `RemoteRef<Message>`, an endpoint, network address, or byte sequence. Remote
   identity and serialization remain exclusively under the remote-Actor gap and
   REM tasks. The Local/Remote observation label accepted by DEC-0095 is not a
   conversion or protocol.

7. **State transition typing.** The state initializer must check as the declared
   state type. The receive body is checked as a total transition from the bound
   state and message values to exactly the declared state type. The transition
   has a closed empty residual Effect row in this profile. `await`, `spawn`,
   `send`, handlers with residual Effects, Actor construction, state projection
   from a reference, and runtime operations are unavailable inside it.

8. **Isolation by construction.** Actor state has no external name, getter,
   setter, reference, projection, or borrow operation. Only the first receive
   pattern denotes current state, only within that transition body, and the sole
   accepted result is the next state value. A transition cannot return state as
   another type, store it in an external binding, capture it in a returned
   closure, place it in a message, or expose `&mut state`. Current Ling has no
   public borrow syntax; future borrow/Resource rules cannot weaken these facts
   without a new Accepted decision.

9. **Turn boundary.** One successful transition is the checked representation of
   one indivisible, non-suspending turn. This is not runtime scheduling authority
   and does not decide the future await/reentry policy: there is no suspension
   point in the ACT-2301 profile. ACT-2304 must resolve reentry before adding any
   suspending or Effectful turn.

10. **Message and state bounds.** ACT-2301 accepts only closed, monomorphic types
    already classified as ordinary Value types by the current checker. No
    Resource, Managed graph, borrow, Capability, Task handle, Actor reference,
    function, or open type variable may occur transitively in message or state.
    This conservative boundary is not the ACT-2302 Sendable rule; ACT-2302 must
    define message ownership, schema identity, and any broader admitted class.

11. **Resolution and namespaces.** Actor declarations occupy the nominal type
    namespace and reject duplicate or confusable names under existing Unicode
    normalization/security rules. State/message pattern bindings occupy only the
    transition lexical scope. Actor types are not callable values, and ordinary
    expressions cannot resolve an Actor declaration as a constructor.

12. **Checked Actor Core.** The immutable projection contains at least the
    declaration definition identity, `ActorTypeId`, checked message/state types,
    local reference descriptor, initializer and transition expression keys,
    bound-pattern identities, closed Effect row, and original declaration,
    keyword, type, initializer, pattern, and body spans. Construction validates
    cross-reference ownership and rejects duplicate identities or inconsistent
    types before publication.

13. **Deterministic projection.** Internal canonical bytes use a new explicit
    version tag and deterministic source/definition order. They omit source-file
    names/IDs, host paths, runtime `ActorId`, allocation addresses, Rust debug
    text, hash-map order, and scheduler facts. This internal projection is test
    evidence, not a public protocol or serialization promise.

14. **Diagnostics.** Invalid Actor syntax uses the existing syntax family.
    Duplicate names, unresolved names, type mismatches, unsupported type forms,
    and residual Effects reuse their existing registered bilingual diagnostics
    when the category is exact. Actor-specific isolation failures require a new
    registered `L-ACTOR-*` code before implementation; internal invariant
    failures remain typed Rust errors and are not exposed as Ling diagnostics.

15. **Compatibility and Unicode.** The accepted source form would change valid
    `actor ...` declarations from syntax rejection to checked-only acceptance,
    while execution still fails explicitly before host Effects. All public
    diagnostics retain original UTF-8 byte spans and Chinese/English messages.
    Unicode XID, normalization, security, and generated tables remain 17.0.0.
    No public schema, Semantic Graph version, CLI exit contract, bytecode, VM ABI,
    network protocol, or Stable feature claim is added.

16. **Completion boundary.** ACT-2301 is complete only when clauses 1 through 15
    have CST/AST/HIR/resolution/type/effect/Core implementation and positive,
    negative, deterministic, Unicode/BOM/CRLF, reconstruction, and no-execution
    evidence; targeted tests and applicable repository gates pass; status and
    traceability are current; and all deferred Actor work remains explicit.

## Conformance plan

- Parse and lower the exact source form, preserving every original UTF-8 span;
  cover Unicode identifiers, BOM, CRLF, comments, malformed layout, duplicate
  clauses, and contextual-keyword use outside Actor declarations.
- Check identity stability, namespace conflicts, initializer/result types,
  lexical state visibility, forbidden escape shapes, closed Effect rows, and the
  conservative closed-Value boundary.
- Construct and validate Checked Actor Core, compare reconstructed canonical
  projections under different source evidence, and reject malformed internal
  cross-references.
- Assert that Actor-bearing checked programs cannot enter interpreter, bytecode,
  VM, native, scheduler, mailbox, send, serialization, or remote paths.

## Compatibility impact

- Adds one Experimental checked-only Actor declaration form and internal Checked
  Actor Core; the previous blanket Actor syntax-rejection fixture is narrowed to
  malformed/out-of-profile forms.
- Defines `ActorTypeId`, the runtime-scoped `ActorId` contract, and an internal
  local `ActorRef<Message>` descriptor without constructing runtime values.
- Adds no Actor execution, public serialization/schema, Semantic Graph version,
  CLI command, bytecode/VM ABI, scheduler order, or Unicode 17.0.0 change.

## Unresolved alternatives

- Sendable/message schema rules, mailbox capacity/backpressure/order, Actor
  construction/send/stop, Effectful turns, await/reentry, supervision, runtime
  scheduling, replay, RemoteRef/transport/delivery, Resource/Managed ownership,
  public Actor protocols, and interpreter/VM/native execution require later
  Accepted authority.
- Multiple handlers, richer state declarations, typed handler parameters,
  guards, lifecycle clauses, and public reference types may be proposed later;
  ACT-2301 does not reserve their syntax or semantics.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
