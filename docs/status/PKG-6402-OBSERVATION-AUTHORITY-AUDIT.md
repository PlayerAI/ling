# PKG-6402-OBSERVATION Authority Audit

- Task: `PKG-6402-OBSERVATION` — Hermetic-build exclusion boundary evidence
- Parent: `PKG-6402` — Hermetic Build
- Decision: Accepted `DEC-0227`
- Release: G6
- Status: authorized bounded evidence

## Authority conclusion

Accepted `RFC-0002` defines deterministic local/offline project inputs and
explicitly excludes build scripts, plugins, generated source, target/profile
defaults, binary packages, and artifact metadata. Accepted `DEC-0019` and
`DEC-0022` provide internal query/cache slices only. Accepted `DEC-0227`
therefore authorizes negative regression evidence for those exclusions, not a
hermetic build implementation.

No Accepted authority defines typed build nodes, Capabilities, a sandbox,
executor, resource policy, artifact identity, build cache/replay, CLI,
diagnostics, or migration. The parent remains blocked until those contracts
and executable security/reproducibility evidence are Accepted.

## Authorized implementation

1. Reject representative build-system fields at manifest, package, and local
   dependency boundaries.
2. Assert absence of build executor, script, shell, subprocess, and network
   routes from `ling-project`.
3. Add a sixty-category test-local inventory with deterministic ordering,
   duplicate rejection, and opaque bytes outside public semantics.
4. Register decision, lifecycle, implementation report, backlog, and task
   traceability.

## Explicit exclusions

No build graph, node, executor, build script, plugin, generator, sandbox,
Capability, manifest field, artifact, cache/replay, CLI, diagnostic, profile,
target, dependency, public API, or support claim changes. Parent `PKG-6402`
remains `BlockedSpec`.
