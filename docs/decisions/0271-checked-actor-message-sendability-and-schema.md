# DEC-0271: Checked Actor message sendability and schema / 已检查 Actor 消息可发送性与 schema

> 状态：Proposed<br>
> 提出日期：2026-08-30<br>
> 决定日期：Pending<br>
> Owner role：actor-semantics<br>
> 相关 RFC/缺口：DEC-0008 | DEC-0009 | DEC-0010 | DEC-0012 | DEC-0096 | DEC-0270 | GAP-OWNERSHIP-MODEL-001 | GAP-ACTOR-REMOTE-DELIVERY-001 | ACT-2302<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This proposal defines the smallest checked-only, local Actor message profile
that can complete ACT-2302 without inventing Resource, Managed, Capability
transfer, mailbox, runtime, or remote-delivery behavior.

本提案定义可完成 ACT-2302 的最小已检查、本地 Actor 消息 profile；它不虚构
Resource、Managed、Capability transfer、mailbox、runtime 或远程交付行为。

## Question

What exact Sendable judgment and Semantic Graph projection can be applied to
the checked Actor declaration accepted by DEC-0270 while every message remains
non-executable and all future ownership and transport categories default to
rejection?

## Decision

1. **Checked-only local boundary.** ACT-2302 consumes only the immutable
   `CheckedActorCore` published under DEC-0270 and the same `TypedProgram` and
   `TypeArena` that produced it. It adds no `send` expression, Actor value,
   mailbox, queue, scheduler, serialization, interpreter path, bytecode, VM
   instruction, native ABI, or remote reference. An Actor-bearing program
   continues to fail every executable entry at `L-ACTOR-0002`.

2. **Judgment.** The checker publishes `SendableLocal(T)` only for a fully
   resolved, closed and monomorphic checked type `T`. The judgment is a
   structural property of the checked type graph, not a user-implementable
   Trait, implicit conversion, host-language marker, or promise that a runtime
   send operation exists.

3. **Admitted type closure.** `Unit`, `Bool`, `Int`, `Float64`, and `Text` are
   `SendableLocal`. Tuples and `List<T>` are admitted exactly when every
   component is admitted. A nominal record or variant is admitted exactly when
   all substituted type arguments are closed and every reachable field or case
   payload is admitted. Recursive nominal graphs use a deterministic
   coinductive visit keyed by resolved definition identity plus substituted
   arguments; traversal order cannot change the result.

4. **Default-deny categories.** Functions, Trait members, open type variables,
   error types, Task signatures, Task handles, Actor declaration types,
   `ActorRef`, and any unknown future type category are not `SendableLocal`.
   A new type category remains rejected until a later Accepted decision gives
   it an explicit rule. The checker must not infer sendability from Rust
   `Send`, `Sync`, `Clone`, ownership, allocation, or representation details.

5. **Borrow and alias boundary.** The admitted set contains no borrow,
   reference, mutable Place, externally shared Cell, or captured environment.
   Therefore no active borrow can cross the checked message boundary in this
   profile. This is rejection by type representation, not a general lifetime
   theorem and not authority for ACT-2304 suspension/reentry. Any future Borrow
   or reference type is rejected until its governing ownership decision is
   Accepted.

6. **Resource, Managed, and Capability boundary.** Resource values are not
   sendable in this profile, so no move, copy, duplication, drop, rollback, or
   failed-send ownership transfer occurs. Managed graphs are rejected in every
   profile; no sharing or collector assumption is selected. Capability is an
   authorization fact rather than an ordinary payload value and cannot be
   forged, captured, reconstructed, or transferred through this message
   profile. Later Accepted ownership/profile decisions may broaden these sets
   but cannot silently reinterpret this profile.

7. **Local message schema.** Every accepted Actor declaration has exactly one
   immutable local message schema derived from its checked message type. The
   schema records the root type and the complete reachable primitive,
   tuple/list, nominal-record, nominal-variant, field and case-payload graph
   after type-argument substitution. It carries no runtime payload bytes,
   offsets, alignment, Rust layout, allocator data, source path, wire tag,
   endpoint, delivery guarantee, or migration promise.

8. **Canonical schema identity.** The schema ID is the lowercase
   `experimental:blake3:` digest of a length-prefixed canonical encoding with
   domain `ling.actor-message-schema-id/v1`, the Ling language version, and the
   normalized closed type graph. Primitive and container tags are explicit;
   nominal nodes use resolved definition identities; fields and cases use
   normalized semantic order; cycles use canonical definition references.
   Source spans, spelling, comments, file IDs, host paths, arena `TypeId`
   numbers, insertion order, allocation addresses, and Rust debug output are
   excluded. Equivalent reconstructions must produce identical bytes and a
   detected digest collision must fail checked publication rather than merge
   schemas.

9. **DEC-0096 relationship.** The publish-disabled
   `MessageSchemaIdentityModel` remains bounded identity evidence. ACT-2302 may
   construct it only from validated checked schemas, using deterministic
   nonzero internal IDs and rejecting collisions. Its `u32` identities are not
   the public schema ID, do not appear in Semantic Graph JSON, and have no wire
   meaning.

10. **Checked Actor message core.** Each `CheckedActorCore` gains an immutable
    message contract containing the `SendableLocal` result, canonical schema
    identity, normalized schema graph, governing actor definition and original
    message-type span. Construction is atomic: a missing type, unsupported
    category, open variable, malformed/cyclic reference, inconsistent owner,
    duplicate identity, schema collision, or non-canonical graph prevents the
    checked Actor snapshot from being published.

11. **Diagnostic behavior.** A source message type outside the admitted set
    reports the existing bilingual `L-ACTOR-0001` at the original message-type
    UTF-8 byte span with a stable machine `reason`. Distinct unsupported
    categories receive distinct registered reason strings; internal schema
    invariant failures remain typed Rust errors unless a source program can
    trigger them. No new diagnostic code is allocated unless implementation
    proves that the existing category is insufficient.

12. **Semantic Graph projection.** A checked Actor program adds the optional
    Experimental `x-ling-actor/0.1` extension to `ling.semantic/0.1`. Actors are
    sorted by definition identity and each entry records the actor definition,
    canonical message type, `SendableLocal` class, schema ID, canonical schema
    nodes/edges, and original message-type byte span. The isolated reader
    validates the exact extension version, identities, ownership, sorting,
    closure, edge targets, schema recomputation, and correspondence with an
    Actor definition. It remains data-only and cannot construct executable
    Core. Graphs without Actors retain byte-identical `ling.semantic/0.1`
    output.

13. **Identity and compatibility.** Actor definitions participate in Actor
    program identity only through versioned checked Actor/message canonical
    bytes. Adding or changing an Actor message schema changes the Actor-bearing
    Program ID; trivia, source path, BOM/LF/CRLF choice, equivalent insertion
    order, and source-file identity do not. Existing non-Actor Definition IDs,
    Body IDs, Program IDs and JSON bytes remain unchanged.

14. **No remote or runtime meaning.** `SendableLocal` and
    `x-ling-actor/0.1` do not assert serializability, wire compatibility,
    delivery, ordering, mailbox admission, backpressure outcome, ownership
    after send, cancellation cleanup, or cross-process security. Remote
    messages remain blocked by `GAP-ACTOR-REMOTE-DELIVERY-001`; runtime send
    behavior remains blocked by ACT-2303 through ACT-2305.

15. **Completion boundary.** ACT-2302 is complete only when clauses 1 through
    14 are implemented through checked Core and Semantic Graph; positive,
    negative, recursive, deterministic, reader-corruption, Unicode/BOM/CRLF,
    schema-collision and no-execution evidence passes; protocol inventory,
    schema fixtures, authority/status traceability and compatibility notes are
    current; and every deferred ownership/runtime/remote category remains
    unavailable rather than represented by a placeholder API.

## Conformance plan

- Accept primitives, nested tuples/lists, generic records/variants after closed
  substitution, and finite values of recursive nominal message graphs; compare
  the checker result with independently reconstructed type graphs.
- Reject functions, Tasks, Task handles, Actors, open variables, malformed
  types, and every future/unknown type category at the exact original message
  type span with deterministic bilingual diagnostics.
- Prove that Borrow/reference/mutable Place/Cell, Resource, Managed, ActorRef,
  and Capability payloads cannot enter the admitted checked representation;
  add explicit negative fixtures when each source type category later exists.
- Freeze canonical schema IDs and `x-ling-actor/0.1` JSON for primitive,
  aggregate, generic, Unicode-named and recursive messages; reconstruct with
  different insertion order, source IDs, paths, BOM and LF/CRLF evidence and
  require identical semantic output except for recorded byte spans.
- Corrupt version, identity, type closure, owner, node order, edge target,
  schema digest and span fields independently and require the isolated reader
  to reject them without producing executable Core.
- Assert that ordinary non-Actor semantic JSON and IDs remain byte-identical
  and that Actor-bearing `run`, `test`, REPL, bytecode, VM, native, mailbox,
  serialization and remote entry points still stop before execution.

## Compatibility impact

- Would replace ACT-2301's private conservative message-value predicate with a
  named checked `SendableLocal` contract for the same currently accepted type
  set; it would not broaden source acceptance to Resource, Managed, Borrow,
  Capability, Task or Actor-reference payloads.
- Would add an Experimental optional `x-ling-actor/0.1` field and Actor-bearing
  program identity inputs. Existing non-Actor `ling.semantic/0.1` bytes and
  identities would remain unchanged.
- Would add no send syntax, runtime value, serialization, mailbox, scheduler,
  bytecode/VM/native ABI, remote protocol, Stable compatibility claim, package
  behavior, or Unicode 17.0.0 change.
- No automatic migration is promised. Consumers that preserve unknown `x-*`
  fields remain forward-compatible; consumers that interpret the Actor
  extension must gate on its exact version.

## Unresolved alternatives

- Resource move-on-success/failure, Managed sharing by profile, transferable
  Capabilities, borrow lifetimes, public `ActorRef`, message unions beyond the
  current Actor declaration, mailbox outcomes, runtime ownership, Audit Source
  projection, serialization, remote schemas, schema evolution and migration
  remain later work under their Accepted decisions and registered gaps.
- A user-implementable `Sendable` Trait, Rust-style auto trait, structural wire
  layout, nominal-only marker, implicit serializer, or remote-capable default
  is rejected for this profile. Evidence from later ownership and transport
  work may justify a new version rather than changing `SendableLocal` silently.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
