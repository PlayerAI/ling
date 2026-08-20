# Ling repository instructions

## Authority

For language behavior, use this order and stop when higher-priority documents conflict:

1. Accepted RFCs under `D:/Coding/Ling/docs/`
2. `D:/Coding/Ling/docs/SEMANTICS.md`
3. `D:/Coding/Ling/docs/LANGUAGE.md`
4. Conformance tests under `D:/Coding/Ling/tests/conformance/`
5. Rust implementation under `D:/Coding/Ling/crates/`
6. Code comments

`docs/IMPLEMENTATION.md` defines engineering order but does not create language semantics.

## Implementation boundaries

- Implement only the v0.0.1 Seed subset unless an accepted RFC expands it.
- Do not resolve specification conflicts through code or snapshots.
- Do not interpret unresolved AST nodes; evaluation must consume checked Typed Core.
- Do not expose Rust ownership, allocation, hash-map order, paths, or debug output as Ling semantics.
- Preserve original UTF-8 byte spans throughout the compiler pipeline.
- Keep public diagnostics bilingual and use registered stable error codes.
- Keep Unicode XID, normalization, security, and generated tables on Unicode 17.0.0.
- Keep normal builds and tests offline after dependencies are locked.
- Do not add placeholder public APIs that imply an unimplemented language feature works.

## Pull-request evidence

Each change must state:

- normative clauses covered;
- specification gaps or conflicts encountered;
- tests added or updated;
- diagnostic, schema, or Semantic ID compatibility impact;
- determinism and Unicode-version impact;
- intentionally deferred work.

## Execution-plan governance

- The post-Seed engineering package currently lives under `docs/ling_execution_plan/`. It is non-normative and ranks below accepted RFCs, `docs/SEMANTICS.md`, `docs/LANGUAGE.md`, conformance tests, and `docs/ROADMAP-1.0.md`.
- Files under `docs/ling_execution_plan/baseline/` are historical planning inputs only. Do not copy them over current specifications or treat them as another authority.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` fix the public CLI as `ling` and the source extension as `.ling`; RFC-0001 records the same names but remains Draft. References to `zero`, `.zero`, `zero.*`, or `zero-*` in the execution package are stale placeholders and must not enter implementation, fixtures, schemas, or editor integration.
- Keep the accepted diagnostic format `L-<DOMAIN>-<NUMBER>` and the single registry in `docs/ERROR-CODES.md`; do not apply alternate ranges proposed by a lower-authority plan.
- Treat planned paths, crates, commands, manifests, schemas, backends, and editor repositories as proposals until the current repository and the required accepted RFCs confirm them. Do not create empty placeholder crates or public APIs.
- Track execution tasks in `docs/status/implementation-status.toml`. A task may be implemented only when its dependencies are satisfied and any semantic or public-protocol decisions are Accepted.
- Create RFCs and decisions from `docs/governance/templates/`, record every lifecycle transition in `docs/governance/lifecycle.toml`, and keep the generated lifecycle report current.
- Language-semantic pull requests must cite the Accepted specification IDs and normative clauses that authorize the change. Draft/Proposed documents, gaps, roadmaps, snapshots, and implementation behavior are not authorization.
- Experimental implementation must name its governing Draft RFC or registered specification gap; remove or update that marker when the experiment graduates.
- Register every implemented or planned public protocol in `docs/governance/protocol-inventory.toml`; do not claim `Stable` before the ROADMAP-1.0 gates, an Accepted authority, and executable fixtures are present.

## Task workflow

1. Read the task, higher-authority specifications, accepted decisions, current implementation, and tests before editing.
2. Record plan/repository drift and unresolved semantic questions; stop semantic work when an Accepted decision is missing.
3. Add or update acceptance evidence before implementing behavior, then deliver the smallest complete vertical slice.
4. Run the task-specific checks and relevant repository gates; distinguish commands actually executed from documentary or historical evidence.
5. Update task status and traceability without claiming a commit, CI result, platform, performance property, or feature that was not verified.
