# ACT-2302-MESSAGE-SCHEMA-MODEL Authority Audit

## Outcome

The bounded child `ACT-2302-MESSAGE-SCHEMA-MODEL` is authorized by Accepted
`DEC-0096`. It records immutable, publish-disabled message-schema and field
identities for future evidence. Public `ACT-2302` remains `BlockedSpec`: this
child does not define Sendable, ownership, Capability, payload, serialization,
mailbox, or runtime behavior.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor message ownership,
  mailbox, serialization, and differential contracts before message execution.
- `docs/SEMANTICS.md` keeps Actor messages and Ownership outside the v0.0.1
  Seed subset.
- `DEC-0008`, `DEC-0009`, and `DEC-0010` authorize Seed value, mutation, State,
  and Capability behavior only.
- `DEC-0095` provides the preceding opaque Actor identity boundary.
- `DEC-0096` authorizes only structural schema/field identity validation.

## Current implementation boundary

`MessageSchemaIdentityModel` validates nonzero schema, optional owner, and field
identities, duplicate-free schemas, and repeated-field rejection. Fields carry
no type, payload, ownership, effect, Capability, or encoding meaning. Source
spans are retained as evidence and omitted from path-free canonical bytes.

No Sendable or ownership checker, borrow/move/Resource/Managed rule, Capability
filter, Semantic Graph projection, serializer, mailbox, runtime, diagnostic,
Semantic ID, CLI/LSP command, public protocol, or migration behavior was added.

## Evidence and deferred work

Focused tests cover validation, deterministic schema/field ordering,
canonical-byte independence from source evidence and insertion order, and
invalid/duplicate/repeated identity rejection. The parent remains blocked until
an Accepted Actor authority defines message typing/ownership, profiles,
Capability transfer, schema versioning, local/remote wire rules, runtime ABI,
and interpreter/VM differential and migration evidence.
