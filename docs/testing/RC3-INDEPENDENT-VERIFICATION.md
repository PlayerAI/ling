# RC-6903 Independent Verification

Status: `BlockedSpec` (2026-08-22). This matrix defines readiness for an
independent release-candidate review; it is not an independent sign-off and
does not create a tag, artifact, reviewer identity, or Go decision.

## Independence boundary

The G6 plan requires a reviewer, agent, or team that did not implement the
candidate to build the tag, verify artifacts, run conformance and protocol
corruption suites, inspect TCB/unsafe/FFI, reproduce representative evidence,
and compare tag/hash/release-manifest identities. The current workspace has
no accepted candidate package, independent reviewer record, evidence-bundle
protocol, or sign-off schema. A run by the implementing agent is repository
validation, not RC3 independence.

## Verification matrix

| RC3 check | Current evidence | State | Required independent evidence |
| --- | --- | --- | --- |
| Clean-tag build | The repository has no v1.0 source tag or candidate manifest; the v0.0.1 Seed release report is historical candidate evidence. | BlockedSpec | Reviewer-controlled clean checkout, locked/offline toolchain, exact tag, reproducible build log, and artifact digest. |
| Verify artifacts | Existing bytecode and schema fixtures are bounded implementation evidence; no public RC artifact bundle or verifier manifest exists. | BlockedSpec | Independent decoding/signature/checksum/provenance verification against a candidate manifest. |
| Run conformance | Seed conformance is executable and is covered by current gates and historical CI evidence; no independent RC1/1.0 conformance scope is accepted. | Partial Seed evidence | Independent runner, pinned environment, complete candidate scope, logs, and signed result. |
| Protocol corruption suite | `xtask` schema/governance tests and bytecode malformed vectors cover current surfaces; protocol inventory has no Stable 1.0 set. | Partial Seed evidence | Candidate-wide corruption corpus, expected rejection taxonomy, deterministic outputs, and independent replay. |
| TCB/unsafe/FFI inspection | Workspace unsafe denial and the Seed security audit are present; no FFI or Native implementation and no independent security review exists. | Partial Seed evidence | Reviewer conflict disclosure, unsafe/TCB/dependency/license/FFI inspection report, and unresolved-risk disposition. |
| Representative evidence reproduction | Examples, project fixtures, tutorials, and Seed release evidence exist; no independent reproduction report is tied to a v1.0 candidate. | Partial Seed evidence | Reproduction manifest, exact commands, normalized outputs, source/toolchain hashes, and reviewer sign-off. |
| Tag/hash/release-manifest comparison | No v1.0 tag, artifact manifest, evidence bundle, or candidate identity is registered. | BlockedSpec | Independent comparison of source tag, artifact digests, schema/protocol versions, evidence bundle, and publication manifest. |

## Required reviewer record

Before RC3 can start, an Accepted release decision must define the candidate
identity, reviewer independence/conflict disclosure, clean-environment policy,
command and toolchain capture, artifact/provenance verification, failure and
rerun rules, evidence retention, and the sign-off format. The reviewer must
not silently widen the support matrix or treat an implementation agent's
self-check as independent evidence.

## Verification boundary

The following commands validate repository consistency only; they do not
constitute RC3 independent verification:

```text
cargo run -p xtask --locked --offline -- status verify
cargo run -p xtask --locked --offline -- governance check-all
cargo run -p xtask --locked --offline -- support verify
cargo run -p xtask --locked --offline -- traceability verify --release v0.0.1
```

No tag was built, no artifact was published, no external reviewer or service
was contacted, and no network or system configuration was changed by this
readiness audit.

## Promotion rules

RC-6903 may leave `BlockedSpec` only after RC0 and RC1 are complete, the
candidate is immutable and versioned, and an independent reviewer reproduces
the complete relevant evidence with a recorded Go/No-Go decision. Any source,
schema, protocol, dependency, or artifact change invalidates the candidate and
requires a new identity and repeat of affected checks.

No placeholder command, tag, artifact, reviewer identity, signature, protocol,
or stale legacy name is added here.
