# PKG-6404-LOCAL Authority Audit

- Task: `PKG-6404-LOCAL` — Local supply-chain attack-boundary evidence
- Parent: `PKG-6404` — Supply-Chain Attack Tests
- Decision: Accepted `DEC-0229`
- Release: G6
- Status: authorized bounded evidence

## Authority conclusion

Accepted `DEC-0229` authorizes executable evidence for the RFC-0002 local
package subset and an exact classification of all ten `PKG-6404` attacks.
Dependency confusion, namespace spoofing, Unicode-confusable packages, and
malicious manifests receive local-subset tests. Archive traversal,
decompression bombs, signature/key mismatch, yanked packages, compromised
package caches, and build-capability escalation remain unavailable because
their prerequisite protocols are absent or explicitly deferred.

Parent `PKG-6404` remains `BlockedSpec`; this child is not a complete
supply-chain security claim.

## Authorized implementation

1. Add representative hostile local manifest/name/locator tests and preserve
   existing collision, visibility, path/symlink, Unicode, lock, deterministic,
   offline, resource, and failure-atomic fixtures.
2. Strengthen DEC-0022 internal cache corruption evidence without describing
   that disposable query cache as a package cache.
3. Add an exact ten-attack test-local assessment with two dispositions:
   `LocalSubset` and `UnavailableProtocol`.
4. Register the decision, lifecycle, implementation report, backlog, and task
   traceability and run all repository gates offline.

## Explicit exclusions

No registry/archive/decompression/signature/yanking/package-cache/build
protocol, schema, service, executor, sandbox, CLI, diagnostic, dependency,
public API, support promotion, or complete security guarantee is added.
