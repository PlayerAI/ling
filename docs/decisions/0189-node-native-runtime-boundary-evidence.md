# DEC-0189: Internal Node Native-runtime boundary evidence / Node Native 运行时边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0188` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-OWNERSHIP-MODEL-001` | `GAP-KERNEL-DEVICE-001` | `GAP-CRITICAL-PROFILE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`NODE-5305-OBSERVATION`. It records provisional Native Node vocabulary for
checked inputs, Native IR and ABI/layout, ownership and static memory,
timer/watchdog lifecycle, target evidence, diagnostics, and fixtures while
RFC-K502, RFC-0011, and the dependent Critical/Kernel/Device authorities
remain unresolved.

本决定只授权 `NODE-5305-OBSERVATION` 使用 test-local 的 Native Node 边界清单；在 RFC-K502、RFC-0011
以及 Critical/Kernel/Device 等依赖权威尚未解决时，只记录临时的 checked input、Native IR/ABI/layout、
ownership/static memory、timer/watchdog lifecycle、target evidence、diagnostic 与 fixture 词汇。

## Question

NODE-5305 proposes a Native Node runtime with static memory, no general
allocation, a fixed schedule, target timer and watchdog primitives,
safe-state transitions, startup/shutdown behavior, and telemetry outside the
hard-real-time path. Which vocabulary can be retained as bounded evidence
without choosing an ABI, object layout, ownership model, target contract,
timer units, watchdog recovery, or Native support claim?

## Decision

1. `crates/ling-types/tests/node_native_runtime_evidence.rs` keeps a
   test-local inventory of sixty provisional Native Node runtime categories,
   ABI/layout, ownership/static-memory, timing/lifecycle, target/evidence,
   diagnostic, and fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.node-native-runtime-observation/0`. These bytes
   are evidence only; they are not a Native backend, ABI, target manifest,
   allocator, timer, watchdog, lifecycle state machine, diagnostic, protocol,
   or support claim.
3. No Native IR/backend, target dependency, runtime, ABI schema, diagnostic
   allocation, CLI/LSP route, protocol, support claim, or placeholder API is
   added. Public `NODE-5305` remains `BlockedSpec`, and `BACKEND-NATIVE`
   remains unsupported.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:266-275` is
  non-normative; it names static memory, fixed schedule, timer/watchdog,
  safe-state, startup/shutdown, and telemetry but defines no ABI, layout,
  ownership, target, timing, lifecycle, or evidence contract.
- `docs/SEMANTICS.md:1560-1568` describes Native conceptually, while
  `:1914-1931` reserves Native and Critical enforcement; it does not
  authorize a Native backend. `docs/LANGUAGE.md:1348-1358` keeps Native,
  ownership, and Node systems features outside the first-version goal.
- `docs/ROADMAP-1.0.md:116-118` and `:352-386` require resource/ownership,
  Native IR/ABI, Fault/FFI, and reproducible target evidence before Native.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`,
  `GAP-KERNEL-DEVICE-001`, and `GAP-CRITICAL-PROFILE-001` leave the required
  contracts open. The support matrix keeps `BACKEND-NATIVE` Unsupported.
- Accepted Seed RFCs, including RFC-0017–RFC-0020, authorize mutable-place,
  effect, differential, and VM host-control slices only; none authorizes a
  Native backend or Native Node runtime.

## Conformance plan

- Assert all sixty Native Node runtime boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer ABI/layout, ownership/drop, static-memory, target, timer/watchdog,
  startup/shutdown, safe-state, telemetry, diagnostics, CLI/LSP, and runtime
  protocol behavior until accepted authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. VM limits, host cancellation, and internal scheduling are not
reinterpreted as Native ownership, static-memory, timer, watchdog, or
hard-real-time guarantees; only test-local evidence is added.

## Unresolved alternatives

Native IR validity; ABI and calling convention; data layout, endianness,
alignment and FFI; target primitive/toolchain identity; ownership, region,
resource, drop, aliasing and cleanup; stack/arena/buffer accounting and no
general allocation; state-cell and safe-state atomicity; schedule/clock/tick/
deadline/jitter/overrun; timer units and drift; watchdog arming/expiry and
recovery; interrupt/preemption; startup/shutdown/Fault/fallback; telemetry
boundary; Critical Profile, Kernel/Device and placement; unsupported-target
behavior; bilingual diagnostics and facts; ABI/layout, ownership, memory,
timer/watchdog, lifecycle, safe-state, target, differential, migration,
Unicode fixtures; protocol inventory and public status remain open under
NODE-5305, NODE-5304, RFC-K502, RFC-0011, the listed gaps, and missing Native
authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
