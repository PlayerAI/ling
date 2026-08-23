# Deprecation Policy Readiness

This generated report records the current bounded guards and blockers. It is not a public deprecation policy or compatibility promise.

- Authority: `DEC-0233`
- Released major versions: `0`
- Public deprecation policy: `Absent`

| Required policy area | State | Boundary or blocker | Evidence |
| --- | --- | --- | --- |
| `one-x-compatibility-promise` | `Unavailable` | Ling has no released major version and no Accepted 1.x compatibility contract. | `docs/status/COMPAT-6504-AUTHORITY-AUDIT.md`<br>`docs/governance/compiler-compatibility-boundary.toml` |
| `minimum-deprecation-period` | `Unavailable` | No Accepted version cadence, eligible subject inventory, or minimum notice interval exists. | `docs/status/COMPAT-6504-AUTHORITY-AUDIT.md` |
| `diagnostic-lifecycle` | `GuardedSubset` | DEC-0001 guards code non-reuse and retired-code exclusion only; warning, notice, replacement, and removal semantics remain undefined. | `docs/decisions/0001-error-code-policy.md`<br>`docs/ERROR-CODES.md`<br>`docs/governance/error-code-lock.toml` |
| `schema-n-minus-one-policy` | `Unavailable` | The schema lifecycle authority remains Draft; schema-specific evidence does not establish a general N-1 promise. | `docs/status/COMPAT-6504-AUTHORITY-AUDIT.md`<br>`docs/governance/authority.toml`<br>`schemas/registry.toml` |
| `target-profile-support-lifecycle` | `Unavailable` | The support matrix remains Draft and defines no accepted deprecation or removal transitions. | `docs/status/COMPAT-6504-AUTHORITY-AUDIT.md`<br>`docs/governance/authority.toml`<br>`docs/governance/support-matrix.toml` |
| `security-exception` | `Unavailable` | No Accepted authority defines who may shorten a lifecycle, required evidence, notification, or replacement obligations. | `docs/status/COMPAT-6504-AUTHORITY-AUDIT.md`<br>`docs/testing/SECURITY-AUDIT.md` |
| `migration-tooling-commitment` | `Unavailable` | Migration tooling has no accepted version pair, command, transformation contract, or report protocol. | `docs/decisions/0232-migration-tool-deferred-until-version-pair.md`<br>`docs/governance/migration-readiness.toml` |

`GuardedSubset` applies only to diagnostic-code non-reuse and retired-code exclusion. It is not a general deprecation lifecycle. All other rows remain `Unavailable`.
