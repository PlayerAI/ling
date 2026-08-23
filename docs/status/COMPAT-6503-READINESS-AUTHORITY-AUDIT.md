# COMPAT-6503-READINESS Authority Audit

- Task: `COMPAT-6503-READINESS` — Migration-tool readiness and absence evidence
- Parent: `COMPAT-6503` — Language Migration Tool
- Decision: Accepted `DEC-0232`
- Release: G6
- Status: authorized bounded evidence

## Authority conclusion

Accepted `DEC-0232` authorizes an internal readiness inventory and executable
absence guard only. Ling has one released source version and no accepted
version pair, so all nine migration capabilities remain `Unavailable` and no
public command is authorized or reserved.

Parent `COMPAT-6503` remains `BlockedSpec` pending a concrete accepted source-
version pair and a complete semantic transformation/transaction contract.

## Authorized implementation

1. Add `migrate` to the CLI catalog's explicit plan-only rejection corpus and
   assert no command variant/parser route exists.
2. Validate exact one-version/no-pair/Absent markers and the canonical nine
   `Unavailable` requirements with blockers and evidence.
3. Generate the readiness report, require its verifier in CI, and register
   governance, lifecycle, backlog, report, and status evidence.

## Explicit exclusions

No migration command, parser transformation, semantic transaction, diff,
write/backup/rollback path, formatter orchestration, report schema, diagnostic,
public API, dependency, protocol, or support claim is added.
