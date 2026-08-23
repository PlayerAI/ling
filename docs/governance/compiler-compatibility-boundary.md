# Current Compiler Compatibility Boundary

This generated matrix describes the development compiler's verified input boundary. It is not a Ling 1.0 compatibility promise.

- Authority: `DEC-0231`
- Compiler: `0.0.1-dev` (`Development`)
- Unicode: `17.0.0`
- Seed corpus SHA-256: `caafb863d530bddd7b22a2de4aa0a8db374bd46f407e602fa78276a126442cd8`
- Verified general N-1 edges: `0`

| Release | Outcome | Authority | Reason | Evidence |
| --- | --- | --- | --- | --- |
| `v0.0.1` | `AcceptUnchanged` | `CONFORMANCE` | The active Seed authority and frozen corpus provide executable unchanged-input evidence. | `tests/conformance`<br>`docs/governance/seed-corpus-freeze.toml`<br>`docs/SEED-TRACEABILITY.md` |
| `v0.1` | `NoReleasedVersion` | `DEC-0231` | No Accepted v0.1 language release specification or historical corpus exists. | `docs/status/COMPAT-6502-AUTHORITY-AUDIT.md`<br>`docs/governance/seed-corpus-freeze.toml` |
| `v0.2` | `NoReleasedVersion` | `DEC-0231` | No Accepted v0.2 language release specification or historical corpus exists. | `docs/status/COMPAT-6502-AUTHORITY-AUDIT.md`<br>`docs/governance/seed-corpus-freeze.toml` |
| `v0.3` | `NoReleasedVersion` | `DEC-0231` | No Accepted v0.3 language release specification or historical corpus exists. | `docs/status/COMPAT-6502-AUTHORITY-AUDIT.md`<br>`docs/governance/seed-corpus-freeze.toml` |
| `v0.4` | `NoReleasedVersion` | `DEC-0231` | No Accepted v0.4 language release specification or historical corpus exists. | `docs/status/COMPAT-6502-AUTHORITY-AUDIT.md`<br>`docs/governance/seed-corpus-freeze.toml` |
| `v0.5` | `NoReleasedVersion` | `DEC-0231` | No Accepted v0.5 language release specification or historical corpus exists. | `docs/status/COMPAT-6502-AUTHORITY-AUDIT.md`<br>`docs/governance/seed-corpus-freeze.toml` |

`NoReleasedVersion` is not rejection, warning, or migration. Those outcomes require an actual historical input and separate Accepted authority.
