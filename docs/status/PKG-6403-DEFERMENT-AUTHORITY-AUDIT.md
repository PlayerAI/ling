# PKG-6403-DEFERMENT Authority Audit

- Task: `PKG-6403-DEFERMENT` — Registry deferment strategy evidence
- Parent: `PKG-6403` — Registry Minimum Implementation or Deferment Strategy
- Decision: Accepted `DEC-0228`
- Release: G6
- Status: authorized product decision and bounded evidence

## Authority conclusion

Accepted `DEC-0228` selects registry deferment through Ling 1.0. Publication,
installation, and registry distribution remain Unsupported, while RFC-0002's
local manifest, content identity, vendored dependency, and lock protocols keep
their current Experimental states. This is a product support decision, not a
registry protocol or implementation.

Parent `PKG-6403` remains blocked because its declared predecessor `PKG-6402`
is blocked and the registry alternative is intentionally unimplemented.

## Authorized implementation

1. Update `UNSUP-PACKAGES` to record the exact registry-deferred 1.0 policy and
   cite `DEC-0228`.
2. Keep the protocol inventory free of a package-registry record and preserve
   exact local package protocol states.
3. Add repository governance assertions and a sixty-category deterministic
   test-local inventory.
4. Register decision, lifecycle, implementation report, backlog, and task
   traceability.

## Explicit exclusions

No registry protocol, schema, service, publisher coordinate, source kind,
archive, artifact, signature/provenance, install/update/yank operation,
mirror/cache, CLI, diagnostic, dependency, public API, or Stable claim is
added. Local package protocols are not promoted by this decision.
