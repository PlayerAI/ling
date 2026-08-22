# DEC-0177: Internal Critical Profile boundary evidence / 内部 Critical Profile 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0176` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `GAP-KERNEL-DEVICE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`PROF-5101-OBSERVATION`. It records provisional machine-readable Critical
Profile, policy, evidence, lifecycle, privacy, fixture, diagnostic, and
protocol vocabulary while RFC-K501/RFC-0012 and the G2/G3/G4 prerequisites
remain unresolved.

本决定只授权 `PROF-5101-OBSERVATION` 使用 test-local 的 Critical Profile 边界清单；
在 RFC-K501/RFC-0012 与 G2/G3/G4 前置权威尚未解决时，只记录临时 machine-readable profile、
policy、evidence、lifecycle、privacy、fixture、diagnostic 与 protocol 词汇。

## Question

PROF-5101 sketches a machine-readable Critical Profile with language and
specification versions, compiler ranges, standard-library set, target,
scheduler, allowed effects/capabilities, memory/numeric/concurrency policy,
FFI packages, and verification requirements. Which vocabulary can be retained
as bounded evidence without creating a profile format or proof permission?

## Decision

1. `crates/ling-types/tests/critical_profile_evidence.rs` keeps a test-local
   inventory of sixty provisional Profile fields, policy/capability/effect,
   proof/evidence state, schema/lifecycle/composition, privacy, fixture,
   diagnostic, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.critical-profile-observation/0`. These bytes are
   evidence only; they are not a profile schema, reader/writer, identity,
   policy, proof, target claim, diagnostic, or support.
3. No profile file format, parser, CLI option, dependency, diagnostic,
   protocol, support claim, or placeholder API is added. Public `PROF-5101`
   remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:77-106` is non-normative;
  its illustrative TOML does not define field types, defaults, identity,
  compatibility, inheritance, composition, or proof claims.
- `docs/ROADMAP-1.0.md:118` makes G5 depend on G2 replay, G3 resources, and G4
  restricted lowering; it does not authorize a profile protocol.
- `docs/status/PROF-5101-AUTHORITY-AUDIT.md` records open
  `GAP-CRITICAL-PROFILE-001` and missing RFC-K501/RFC-0012 authority;
  `DEC-0176` remains prerequisite cache evidence only.

## Conformance plan

- Assert all sixty Critical Profile boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer profile schema/reader/writer, selection/composition, checker/proof
  semantics, diagnostics, CLI/editor integration, and support behavior until
  accepted authority exists.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Only test-local evidence is added; no Critical Profile or proof
claim is registered.

## Unresolved alternatives

Schema/version/lifecycle, language/spec/compiler/stdlib/target/scheduler
identity; effect/capability/memory/numeric/concurrency/FFI policies;
verification requirements and forbidden capabilities; Assumed/Unknown/Proved
state; canonical bytes, required/optional/default/unknown fields; migration,
composition/override/conflict and project/CLI precedence; Semantic ID,
reproducibility and evidence binding; static/runtime/proof obligations and
non-claims; privacy/unstable-host exclusions; positive/negative/migration/
composition/conflict/target/effect/memory/numeric/concurrency/independent-
checker/Unicode/determinism fixtures; diagnostics, protocol inventory, and
public Critical status remain open under PROF-5101, PROF-5102, GAP-CRITICAL-
PROFILE-001, GAP-KERNEL-DEVICE-001, and missing RFC-K501/RFC-0012 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
