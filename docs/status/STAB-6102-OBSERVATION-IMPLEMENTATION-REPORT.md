# STAB-6102-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0217` as test-only boundary evidence and a black-box
CLI rejection regression. The local inventory records sixty public-surface,
placeholder, support-state, classification-action, authority, compatibility,
and evidence categories; it has explicit ordering, duplicate rejection, and
order-independent opaque bytes.

The CLI help test now covers all plan-only root commands named by the current
G6 plan and governance metadata: `build`, `query`, `patch`, `replay`,
`explain`, `evidence`, `version`, `support`, and `features`. Each remains absent
from help and exits with code 2, empty stdout, and usage on stderr.

The parent audit was corrected to distinguish production `todo!()`/
`unimplemented!()` absence from intentional post-dispatch `unreachable!()`
invariants.

## Verification

- `cargo test -p ling-types --test false_entry_point_audit_evidence --locked --offline`
- `cargo test -p ling-cli --test help --locked --offline`
- `cargo clippy -p ling-types -p ling-cli --all-targets --locked --offline -- -D warnings`
- `cargo xtask support verify`

## Compatibility and deferral

No command, API, grammar/completion item, profile/backend, default, diagnostic,
protocol, support state, dependency, Semantic ID rule, source-span rule, or
Unicode behavior changed. Public `STAB-6102` remains `BlockedSpec` pending an
Accepted complete cleanup inventory and compatibility policy.
