# REL-6601-SEMANTIC-SCHEMA Authority Audit

- Task: `REL-6601-SEMANTIC-SCHEMA` — Semantic Graph reader fuzz coverage
- Parent: `REL-6601` — Fuzz Coverage Inventory
- Decision: Accepted `DEC-0234`
- Release: G6
- Status: authorized bounded implementation

## Authority conclusion

DEC-0012 and RFC-0002 already authorize isolated, exact-version,
data-only readers for `ling.semantic/0.1` and `ling.semantic/0.2`. Accepted
DEC-0234 authorizes a fuzz-only consumer of both readers, deterministic
success/error comparison, bounded inputs, a reviewed corpus, and CI replay.

Parent `REL-6601` remains `BlockedSpec`: several planned families have no
implemented decoder or accepted protocol, and the G6 release gate is not met.

## Authorized implementation

1. Add one fuzz target and direct fuzz-workspace dependency on `ling-semantic`.
2. Add valid Seed and malformed JSON corpus inputs plus a normal regression
   test proving their intended reader outcomes.
3. Extend the inventory, verifier, README, and pinned Ubuntu smoke job.

## Explicit exclusions

No schema, migration, compatibility edge, executable graph path, parser,
Typed Core, diagnostic, public command, runtime, release claim, or placeholder
harness for an unimplemented protocol is added.
