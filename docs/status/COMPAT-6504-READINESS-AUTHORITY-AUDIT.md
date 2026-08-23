# COMPAT-6504-READINESS Authority Audit

- Task: `COMPAT-6504-READINESS` — Deprecation-policy readiness boundary evidence
- Parent: `COMPAT-6504` — Deprecation Policy
- Decision: Accepted `DEC-0233`
- Release: G6
- Status: authorized bounded evidence

## Authority conclusion

Accepted `DEC-0233` authorizes an internal readiness inventory and drift gate
for the exact seven G6 policy areas. Ling has zero released major versions and
no public deprecation policy. Only DEC-0001's diagnostic-code non-reuse and
retirement rules form a `GuardedSubset`; the other six areas are unavailable.

Parent `COMPAT-6504` remains `BlockedSpec` pending an Accepted public policy
covering lifecycle subjects, timing, transitions, diagnostics, compatibility,
security exceptions, and migration commitments.

## Authorized implementation

1. Record the exact seven requirements and their truthful current states.
2. Verify the retired diagnostic guard, Draft schema/support authorities, and
   absent migration version pair/command against repository sources.
3. Generate a readiness report and enforce it through the CI contract,
   governance lifecycle, backlog, and task status.

## Explicit exclusions

No public policy, compatibility promise, minimum period, warning/rejection,
attribute, suppression, transition, schema reader range, support guarantee,
security exception, migration commitment, diagnostic allocation, protocol,
dependency, or placeholder API is added.
