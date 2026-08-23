# Language Migration Readiness

This generated report records why no public migration tool exists. It is not a migration protocol or implementation.

- Authority: `DEC-0232`
- Released source versions: `1`
- Accepted version pair: `false`
- Public command: `Absent`

| Required capability | State | Blocker | Evidence |
| --- | --- | --- | --- |
| `parser-semantic-transaction` | `Unavailable` | No Accepted source-version pair or semantic transformation contract exists. | `docs/status/COMPAT-6503-AUTHORITY-AUDIT.md`<br>`docs/governance/compiler-compatibility-boundary.toml` |
| `dry-run` | `Unavailable` | No migration operation or observable edit plan is authorized. | `docs/status/COMPAT-6503-AUTHORITY-AUDIT.md` |
| `semantic-diff` | `Unavailable` | No cross-version Semantic ID or equivalence contract exists. | `docs/status/COMPAT-6503-AUTHORITY-AUDIT.md`<br>`docs/decisions/0012-semantic-identity-and-canonical-bytes.md` |
| `stale-edit-check` | `Unavailable` | No migration transaction or source revision protocol exists. | `docs/status/COMPAT-6503-AUTHORITY-AUDIT.md` |
| `backup-transaction` | `Unavailable` | No write, rollback, or failure-atomic migration policy exists. | `docs/status/COMPAT-6503-AUTHORITY-AUDIT.md` |
| `formatter` | `Unavailable` | Author Source formatting is accepted, but no post-migration formatting contract exists. | `docs/status/COMPAT-6503-AUTHORITY-AUDIT.md`<br>`docs/decisions/0023-author-source-formatter-preservation.md` |
| `post-check-test` | `Unavailable` | No target-version compiler outcome or migration success oracle exists. | `docs/status/COMPAT-6503-AUTHORITY-AUDIT.md`<br>`docs/governance/compiler-compatibility-boundary.toml` |
| `machine-readable-report` | `Unavailable` | No public migration report schema or CLI contract exists. | `docs/status/COMPAT-6503-AUTHORITY-AUDIT.md`<br>`docs/governance/protocol-inventory.toml` |
| `human-choice-stop` | `Unavailable` | No ambiguity taxonomy, prompt protocol, or resumable transaction exists. | `docs/status/COMPAT-6503-AUTHORITY-AUDIT.md` |

All rows remain `Unavailable` until an Accepted source-version pair and transformation contract exist. No command is reserved.
