# DEC-0058: Locked project snapshot boundary / 锁定项目快照边界

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: project-tooling  
> Related authority/gap: `RFC-0002`, `RFC-0024`, `GAP-PROJECT-CLI-INTERFACE-001`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision authorizes only a read-only library snapshot around the accepted
local package graph and locked-file validation. It does not define project
selection, semantic compilation, execution, testing, building, workspaces,
artifact publication, or a new CLI/protocol surface.

## Question

PRJ-1107 needs a stable input boundary for a future compiler host, while the
accepted project authorities currently define only explicit local manifests,
module discovery, package graphs, and lock policy. What can be implemented
without choosing the still-open project command and artifact behavior?

## Decision

1. `ling-project` exposes `LockedProject` and `load_locked_project`. The value
   owns the validated root `Manifest`, deterministic `PackageGraph`, and the
   canonical `LockFile` projection of that graph.
2. Loading accepts an explicit caller-provided project root and parsed
   `Manifest`, delegates to `resolve_package_graph_with_lock` with
   `LockMode::Locked`, and returns no partial snapshot on failure.
3. Locked loading performs no network request, ambient parent search, lock
   creation or rewrite, source compilation, evaluation, test/build discovery,
   workspace/member selection, artifact generation, path serialization, or
   protocol/CLI dispatch.
4. The snapshot stores no root path or host state. Repeated independent loads
   of unchanged inputs must compare equal and expose the existing path-free
   graph and canonical lock identities.

## Conformance plan

- Load the checked-in single-package project twice and assert snapshot,
  manifest, graph, lock, graph identity, and canonical lock bytes are equal.
- Assert locked loading leaves the existing lock bytes unchanged and returns
  no partial value for a graph/lock failure.
- Verify the API contains no workspace, run/test/build, artifact, network,
  compiler, evaluation, CLI, or protocol state.

## Compatibility impact

- Adds only an in-process `ling-project` Rust value and function layered over
  RFC-0002 APIs. Existing manifest, graph, lock, diagnostic, schema, Semantic
  ID, CLI, runtime, protocol, and Unicode 17.0.0 behavior remain unchanged.
- No path is stored in the snapshot or emitted as a language/protocol value;
  the public project-check Preview command remains unchanged.

## Unresolved alternatives

Compiler-host source loading, workspace/member selection, semantic checking,
incremental revisions, project `run`/`test`/`build`, artifact policy, registry
and network access, package publication, and stable project CLI contracts
remain governed by the blocked PRJ-1107 parent and its registered gap.

## Supersession

- Supersedes: `None`
- Superseded by: `None`

