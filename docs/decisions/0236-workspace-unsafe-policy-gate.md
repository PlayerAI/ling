# DEC-0236: Workspace unsafe-policy gate / 工作区 unsafe 政策门禁

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: security engineering
> 相关规范/缺口：`DEC-0043` | `REL-6603`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes repository-aware enforcement evidence for the existing
Rust `unsafe_code = "deny"` workspace policy. It prevents a newly declared
workspace member from silently omitting the policy; it is not a dependency,
generated-code, cross-target, or vulnerability audit.

本决定授权为现有 Rust `unsafe_code = "deny"` 工作区政策增加仓库感知的执行证据，防止新声明的
工作区成员静默遗漏该政策；它不构成依赖、生成代码、跨目标或漏洞审计。

## Question

How should the Covered-for-current-crates `Rust unsafe` row remain true as the
workspace membership changes?

## Decision

1. `cargo xtask security verify` parses the root `Cargo.toml` and requires
   `workspace.lints.rust.unsafe_code = "deny"`.
2. Every `workspace.members` entry must be a unique, explicit, non-empty,
   relative path containing only normal path components. Globs, absolute paths,
   parent traversal, and malformed entries fail closed.
3. Every declared member manifest must exist, parse as TOML, and set
   `lints.workspace = true`. Adding or moving a member therefore requires an
   explicit review of its lint inheritance.
4. Rust compilation remains the enforcement oracle for compiled source. The
   manifest verifier is a drift gate and does not replace compiler, target,
   macro, generated-source, dependency, advisory, or license evidence.
5. The security matrix retains nine surfaces and the existing state counts.
   Only the evidence behind the already Covered current-workspace row becomes
   stronger.
6. The verifier exposes only internal `GOV-SECURITY-*` failures. It adds no
   Ling diagnostic, syntax, CLI, schema, protocol, runtime behavior, or public
   security API.
7. Parent `REL-6603` remains `BlockedSpec` for the complete threat model,
   future-system authorities, third-party review, release reports, and incident
   response process.

## Normative basis

- The workspace already declares `unsafe_code = "deny"`, and each current
  member inherits workspace lints.
- Accepted DEC-0043 authorizes the internal security-matrix drift gate and
  explicit bounded evidence without a G6 security sign-off.
- `AGENTS.md` requires deterministic offline gates and forbids unsupported
  public surfaces; this verifier reads only checked-in manifests.

## Conformance plan

- Verify the root unsafe lint is exactly `deny`.
- Verify all current workspace members have safe unique paths, readable TOML
  manifests, and `lints.workspace = true`.
- Reject representative empty, absolute, parent-traversing, wildcard, and
  normalized-parent paths.
- Run the security verifier, focused xtask tests, workspace tests, Clippy,
  governance, status, formatting, deterministic, and offline gates.

## Compatibility impact

This is an internal repository-policy check. It changes no Ling source or
runtime semantics, diagnostics, schemas, Semantic IDs, package/lock behavior,
dependency versions, CLI, editor integration, Unicode 17.0.0 behavior, or
compiled artifact format.

## Unresolved alternatives

Transitive dependency and license review; proc-macro and generated-code audit;
all-target compilation; native/FFI TCB policy; advisory/SBOM/provenance output;
third-party penetration testing; threat modeling; and disclosure/response
policy remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
