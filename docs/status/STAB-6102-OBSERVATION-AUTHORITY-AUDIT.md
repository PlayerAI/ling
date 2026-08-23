# STAB-6102-OBSERVATION Authority Audit

## Result

Accepted `DEC-0217` authorizes a bounded observation and CLI rejection
regression only. `STAB-6102` remains `BlockedSpec`: neither the execution plan
nor current Accepted specifications identify a complete cleanup set or define
delete, hide, deprecate, migrate, retain, and reject outcomes for every public
surface.

## Current evidence

- Production crate sources contain no `todo!()` or `unimplemented!()` macro.
- CLI `unreachable!()` sites are post-dispatch internal invariants, not public
  success paths.
- The implemented command catalog and help omit plan-only root commands.
- Future CLI commands in the support matrix are explicitly marked future;
  unavailable profiles and unsupported backends are explicit negative claims.
- Tree-sitter recovery and future-keyword corpora do not produce successful
  post-Seed feature nodes or editor completion claims.
- No current audited finding authorizes a deletion.

## Authorized slice

The child task may record deterministic test-local audit vocabulary and extend
black-box CLI tests so nine plan-only root commands remain absent from help and
fail closed. It may correct documentary scan evidence. It may not remove or
change any public surface, support state, diagnostic, protocol, grammar, or
runtime behavior.

## Deferred authority

The complete public-surface inventory, ownership, classification, cleanup
actions, compatibility/migration policy, editor completion contract, supported
profile/backend/default set, stable diagnostics, and release evidence remain
unresolved. No observation tag or passing negative test closes those gaps.
