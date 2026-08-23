# STD-6303-OBSERVATION Authority Audit

- Task: `STD-6303-OBSERVATION` — Internal Unicode and Chinese-programming stability boundary evidence
- Parent: `STD-6303` — Unicode and Chinese-Programming Stability
- Decision: Accepted `DEC-0225`
- Release: G6
- Status: authorized bounded evidence

## Authority conclusion

`SEMANTICS.md` and Accepted `DEC-0002`, `DEC-0007`, and `DEC-0012` already fix
the Seed Unicode 17.0.0, XID, NFC, security, source-span, and normalized-name
boundaries. Accepted `DEC-0225` therefore authorizes an exact data-manifest
lock and representative executable observations of those existing rules.

The open alias/localization and LSP transaction gaps do not authorize a
Unicode upgrade, localized syntax/view, profile policy, formatter/editor/CLI
protocol, Windows-path behavior, or Stable 1.0 claim. The parent remains
blocked until those contracts and migration evidence are Accepted.

## Authorized implementation

1. Lock the exact Unicode 17.0.0 version and eleven input path/checksum pairs.
2. Exercise representative Chinese XID, NFC, original spelling, script,
   confusable, mixed-script, all forbidden classes, and byte-offset behavior.
3. Add a sixty-category test-local inventory with deterministic ordering,
   duplicate rejection, and opaque bytes outside public semantics.
4. Register decision, lifecycle, implementation report, backlog, and task
   traceability.

## Explicit exclusions

No Unicode data, generated table, dependency, identifier rule, diagnostic,
alias syntax, localized view, profile, formatter/LSP/Zed/CLI feature,
Windows-path rule, Semantic ID, migration protocol, public API, or support
claim changes. Parent `STD-6303` remains `BlockedSpec`.
