# ACT-2304 Authority Audit: Actor Turn and Reentry Rules

## Outcome

`ACT-2304` is authorized by Accepted `DEC-0273` for one bounded checked-only
profile: exactly one message per turn, no suspension, no reentry, atomic state
publication only on normal return, and mailbox-only routing for any future
self-send. ACT-2301 through ACT-2303 are Done, so ACT-2304 may consume their
immutable Actor identity, message-schema, and bounded-mailbox contracts.

This authority does not permit Actor execution. It permits an immutable checked
turn contract, Actor identity participation, and a data-only
`x-ling-actor/0.3` Semantic Graph projection. ACT-2305 remains responsible for
runtime dequeue, transition invocation, state storage/publication, lifecycle,
and executable failure behavior.

## Normative traceability

- Accepted `DEC-0270` fixes checked Actor identity, invariant local references,
  isolated ordinary-Value state, pure typed initialization/transition, and the
  pre-execution implementation boundary.
- Accepted `DEC-0271` fixes `SendableLocal(Value)` message admission and the
  closed canonical local message schema.
- Accepted `DEC-0272` fixes explicit capacity `1..=65535`, `Reject`, pure
  `Accepted`/`Full` admission classification, and the checked local mailbox
  canonical domain.
- Accepted `DEC-0273` clauses 1–17 fix the first turn profile, its canonical
  identity, `x-ling-actor/0.3`, diagnostics compatibility, and the exact
  completion/deferred-work boundary.
- `docs/SEMANTICS.md`, `docs/LANGUAGE.md`, and `docs/ROADMAP-1.0.md` remain the
  higher-level constraints. The execution package describes engineering order
  but creates no additional semantics.

## Authorized implementation surface

The implementation may add:

1. a checked, immutable, path-free `ling.checked-actor-turn/1` contract covering
   owner/type, one-message dispatch, forbidden suspension/reentry,
   publish-on-normal-return, mailbox-only future self-send, transition and
   binding identities, and receive/body source evidence;
2. checked Actor Core version `/3`, with the turn canonical bytes participating
   in Actor Body/Program identity while source evidence remains excluded;
3. exact Experimental `x-ling-actor/0.3` writer/schema/isolated-reader support;
4. pure normal/unsuccessful completion classification as checked evidence; and
5. positive, corruption, determinism, Unicode/BOM/CRLF, effect rejection, and
   no-execution tests.

Construction must remain after parsing, resolution, typing, and Effect
checking. The isolated reader returns data only and cannot create checked or
executable Core.

## Compatibility and diagnostics

- Actor source syntax does not change.
- `x-ling-actor/0.2` consumers must explicitly migrate to `0.3`; no adapter may
  infer a turn contract absent from `0.2`.
- Non-Actor `ling.semantic/0.1` bytes and IDs, package-aware
  `ling.semantic/0.2`, bytecode/VM formats, and Unicode 17.0.0 remain unchanged.
- Existing syntax diagnostics and bilingual `L-ACTOR-0001` retain authority for
  malformed/effectful transitions. No new public diagnostic code is authorized.
- Actor-bearing execution continues to stop at bilingual `L-ACTOR-0002`.

## Intentionally deferred

No Actor instance, queue, dequeue, state cell, runtime commit, `spawn`, `send`,
`stop`, `await`, continuation, suspending/reentrant profile, state-version
guard, cancellation, timeout, retry, restart, Fault outcome, scheduler,
watchdog clock/event, supervisor, serializer, remote endpoint, Replay event,
interpreter path, bytecode instruction, VM instruction, native ABI, Resource,
Managed, Borrow, or Capability transfer is authorized by ACT-2304.

Any suspending/reentrant profile requires a new Accepted decision and explicit
compatibility version; it cannot reinterpret `ling.checked-actor-turn/1` or
`x-ling-actor/0.3`.
