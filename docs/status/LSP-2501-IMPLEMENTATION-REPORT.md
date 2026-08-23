# LSP-2501 implementation report

## Result

LSP-2501 is complete as the composed immutable request-analysis boundary for
the current LSP Preview surfaces. The implementation reuses the accepted
`RequestSnapshot` model and request-specific RFC contracts; it does not add a
public snapshot method, serialized internal revision, or placeholder
`CompilerHost` API. Integration-evidence commit:
`e5434f632963d622834d90168980c9524414d12b`.

## Normative clauses covered

- DEC-0002 and DEC-0019: original-byte/span truth, immutable compiler inputs,
  deterministic revisions, invalidation, and path/private-identity exclusion.
- RFC-0023/RFC-0029 and DEC-0030: exact URI/document selection, monotonic client
  versions, owned complete visible captures, deterministic order, and
  client-version/VFS-revision separation.
- RFC-0030: canonical immutable manifest/lock/config/profile/target inputs and
  atomic source/input publication.
- RFC-0026, RFC-0032, RFC-0033, RFC-0036–RFC-0045, and RFC-0048: exact snapshot
  consumption, document-version association, freshness, stale rejection,
  cancellation where applicable, and atomic response/publication behavior for
  every current compiler-backed request family.

## Implementation composition

- `RequestSnapshot` owns lifecycle state, position encoding, complete visible
  documents, project inputs, and internal revisions without retaining a host
  write borrow.
- Request documents retain exact UTF-8 bytes and distinct optional client
  versions. Workspace inputs are captured in canonical declared order.
- Diagnostic analysis uses an owned ticket/result pair. Other compiler-backed
  request handlers capture the complete snapshot and recapture before success
  where required by their Accepted RFC.
- RFC-0026 formatting consumes the exact immutable document snapshot in the
  single-threaded dispatcher, which cannot accept a racing change before the
  response is written.
- Lifecycle, overlay, and workspace-reload calls negotiate or publish state;
  they are not compiler analysis requests and do not fabricate analysis
  snapshots.

## Verification evidence

The request-snapshot, workspace-reload, diagnostics, formatting, symbol,
navigation, rename, completion, code-action, workspace-symbol, and
semantic-token suites collectively exercise the boundary. Repository-wide
locked offline tests, strict Clippy, CI, governance, LSP, support, status, RC0,
v0.0.1 traceability, formatting, diff, and execution-plan checksum gates are
required for the final status binding.

## Specification gaps or conflicts

The former audit blockers were closed by later Accepted RFCs. The execution
plan's `AnalysisSnapshot` and `CompilerHost` names are illustrative, not public
API requirements. General Semantic Transactions, asynchronous scheduling,
wire cancellation, deadlines, public snapshot tokens, and Stable lifecycle
remain future work without weakening current request freshness.

## Compatibility, determinism, and Unicode impact

- No public LSP method, field, capability, protocol version, schema, or error is
  added; current request-specific Preview markers remain authoritative.
- Canonical URI and workspace-input order plus exact owned bytes make capture
  independent of map insertion order. Internal revisions, allocations, paths,
  lock state, timing, and thread scheduling remain unobservable.
- No diagnostic allocation, Semantic ID/canonical-byte change, language or
  runtime behavior, dependency, filesystem/network behavior, or Unicode
  17.0.0 table change occurs.

## Intentionally deferred

Public/cross-session snapshot identities, asynchronous host scheduling,
deadlines, persistent result capabilities, general Semantic Transactions, and
Stable editor lifecycle are outside LSP-2501.
