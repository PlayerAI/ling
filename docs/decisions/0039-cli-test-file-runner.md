# DEC-0039: Explicit `ling test` file runner / 显式 `ling test` 文件运行器

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: cli-design  
> Related authority/gap: `RFC-0002`, `DEC-0003`, `DEC-0013`, `GAP-PROJECT-CLI-INTERFACE-001`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `CLI-1704-FILE` child. It does not
define source-level test declarations, project manifests, package dependency
selection, workspace discovery, filtering, assertions, snapshots, property
tests, parallel scheduling, cancellation, or a general project test API. The
parent `CLI-1704` remains `BlockedSpec` for those surfaces.

## Question

The execution plan names a `test` command, but the accepted language
specifications do not define a test declaration or a project test target. A
small executable slice is still useful for deterministic standalone Ling
programs, provided it does not silently establish a future source-level test
convention.

## Decision

1. The bounded command is `ling test [--format human|json] <file-or-directory>`.
   Exactly one existing operand is required. `-`, missing paths, non-UTF-8
   logical names, and non-`.ling` file operands are rejected as command usage.
   The command is Preview and is not a project-manifest or workspace command.
2. A file operand runs that one `.ling` program. A directory operand is walked
   recursively without following symbolic links; regular files whose exact
   extension is `.ling` are selected. Selection is sorted by slash-normalized,
   UTF-8 relative path bytes. An empty selection is a test failure. Every
   selected file is an independent `Main` entry program compiled through the
   existing checked compiler pipeline; imports are resolved relative to that
   file's directory. No manifest, lockfile, registry, cache, or network is
   consulted or written.
3. Selected programs run sequentially in the canonical order. Their Console
   output is captured in memory and is not written directly to process stdout.
   Each program must satisfy the existing `Main`/`main ()` entry contract.
   Compilation diagnostics and runtime Fault diagnostics are retained in input
   order and emitted on stderr using the existing bilingual Diagnostic
   renderer. A failure in one file does not prevent later files from running.
4. Human success or failure output is one summary line containing the operand
   and total/passed/failed counts. JSON output is exactly one
   `ling.test/0.1` report with `schema`, `status`, `root`, a deterministic
   `tests` array (`name`, `status`, `stdout`), and `counts`. Test names are
   logical relative names, never host absolute paths. The report is emitted
   even when one or more selected programs fail; discovery failures emit only
   a Diagnostic.
5. Exit classes reuse DEC-0013: `0` when all programs pass, `1` when any
   program has a compile/entry failure or the selection is empty, `4` when any
   program has a runtime Fault or test-input filesystem failure, `5` for an
   internal compiler incident, and `6` for a semantic snapshot mismatch.
   `L-IO-0004` describes test-input discovery failures and `L-TEST-0001`
   describes an empty selection. Human versus JSON output never changes these
   classes.

## Conformance plan

- Run one standalone passing file and a directory containing passing files;
  verify deterministic order, captured output, counts, and exact
  `ling.test/0.1` JSON.
- Run a directory containing compile-invalid and runtime-fault files; verify
  all files are attempted, diagnostics remain bilingual and ordered, the
  report is still emitted, and exit precedence is stable.
- Reject missing/non-`.ling` operands, empty directories, duplicate or
  unknown options, non-UTF-8 names, and symbolic-link traversal without
  partial output.
- Repeat the same input in independent processes and with CRLF source bytes;
  verify report bytes, ordering, and captured stdout are deterministic and the
  command remains offline.
- Validate the report schema and register its Preview protocol, diagnostics,
  fixtures, and traceability evidence.

## Compatibility impact

- Adds the Preview `ling test` file-runner command and `ling.test/0.1` JSON
  report. Existing language syntax, project manifest/lock semantics, runtime,
  bytecode, Semantic IDs, and Unicode 17.0.0 behavior are unchanged.
- Adds `L-IO-0004` and `L-TEST-0001`; existing diagnostic meanings and exit
  classes remain unchanged.
- The command deliberately does not claim a project test convention. A future
  source-level or project test surface requires a new Accepted decision and a
  new protocol version or explicit migration.

## Unresolved alternatives

Source annotations, a dedicated test declaration grammar, manifest test
targets, package/workspace selection, filtering, assertions, snapshots,
property tests, parallelism, cancellation, test-specific capabilities, and
artifact/report persistence remain outside this decision.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
