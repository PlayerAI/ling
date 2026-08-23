# PKG-6401-OBSERVATION Authority Audit

- Task: `PKG-6401-OBSERVATION` — Local package and publication-exclusion boundary evidence
- Parent: `PKG-6401` — Package Publication Protocol
- Decision: Accepted `DEC-0226`
- Release: G6
- Status: authorized bounded evidence

## Authority conclusion

Accepted `RFC-0002` defines a deterministic local/offline manifest, package
identity, dependency graph, and lock protocol while explicitly excluding
publication, registry, publisher ownership, installation, network/Git
dependencies, artifacts, signatures, mirrors, and transparency. Accepted
`DEC-0226` therefore authorizes regression evidence for that exact positive
and negative boundary, not a publication implementation.

No Accepted authority defines authenticated publisher coordinates, archives,
artifacts, trust roots, signing/provenance, registry operations, installation,
yanking, mirrors, version selection, CLI behavior, or migration. The parent
remains blocked until those contracts and executable evidence are Accepted.

## Authorized implementation

1. Assert exact local manifest/lock markers and representative Chinese display
   metadata under RFC-0002.
2. Reject representative publication fields and external dependency locators;
   assert absence of registry/network/process/signing implementation routes.
3. Add a sixty-category test-local inventory with deterministic ordering,
   duplicate rejection, and opaque bytes outside public semantics.
4. Register decision, lifecycle, implementation report, backlog, and task
   traceability.

## Explicit exclusions

No manifest, lock, identity, dependency, diagnostic, CLI, registry, publisher,
archive, artifact, checksum/signature, provenance, SBOM, mirror/cache, yanking,
installation, migration, dependency, public API, or support claim changes.
Parent `PKG-6401` remains `BlockedSpec`.
