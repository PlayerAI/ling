"use strict";

const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const packageRoot = path.resolve(__dirname, "..");
const repositoryRoot = path.resolve(packageRoot, "..", "..");
const conformanceRoot = path.join(repositoryRoot, "tests", "conformance");
const manifestPath = path.join(__dirname, "fixtures", "conformance-syntax.tsv");
const snapshotsPath = path.join(
  __dirname,
  "fixtures",
  "conformance-cst-sha256.txt",
);
const nodeTypesPath = path.join(packageRoot, "src", "node-types.json");
const parserTimeoutMicros = "500000";
const processTimeoutMillis = 10_000;
const mutationProcessTimeoutMillis = 20_000;
const maximumOutputBytes = 500_000;
const expectedCaseCount = 42;
const expectedValidCount = 34;
const expectedMutationCount = expectedCaseCount * 2;

function parserExecutable() {
  const executable = process.platform === "win32" ? "tree-sitter.exe" : "tree-sitter";
  return path.join(packageRoot, "node_modules", "tree-sitter-cli", executable);
}

function assertSafeRelativePath(relativePath, lineNumber) {
  assert.equal(
    relativePath,
    relativePath.replaceAll("\\", "/"),
    `manifest row ${lineNumber} must use forward slashes`,
  );
  assert.equal(
    path.posix.normalize(relativePath),
    relativePath,
    `manifest row ${lineNumber} has a non-normal path`,
  );
  assert.ok(
    !path.posix.isAbsolute(relativePath) &&
      !relativePath.split("/").includes("..") &&
      relativePath.endsWith("/case.ling"),
    `manifest row ${lineNumber} has an unsafe path: ${relativePath}`,
  );
}

function readManifest() {
  const entries = [];
  let previousPath;
  for (const [index, line] of fs
    .readFileSync(manifestPath, "utf8")
    .split(/\r?\n/u)
    .entries()) {
    if (!line || line.startsWith("#")) {
      continue;
    }
    const fields = line.split("\t");
    assert.equal(fields.length, 3, `invalid manifest row ${index + 1}: ${line}`);
    const [relativePath, compilerSyntax, treeSitterPolicy] = fields;
    assertSafeRelativePath(relativePath, index + 1);
    if (previousPath !== undefined) {
      assert.ok(
        previousPath < relativePath,
        `manifest paths must be unique and sorted: ${previousPath}, ${relativePath}`,
      );
    }
    previousPath = relativePath;
    assert.ok(
      compilerSyntax === "valid" || compilerSyntax === "invalid",
      `unknown compiler syntax in row ${index + 1}: ${compilerSyntax}`,
    );
    assert.ok(
      ["clean", "error", "tolerated"].includes(treeSitterPolicy),
      `unknown Tree-sitter policy in row ${index + 1}: ${treeSitterPolicy}`,
    );
    assert.equal(
      compilerSyntax === "valid",
      treeSitterPolicy === "clean",
      `${relativePath}: valid input must be clean; invalid input must error or be tolerated`,
    );
    entries.push({ relativePath, compilerSyntax, treeSitterPolicy });
  }

  assert.equal(entries.length, expectedCaseCount, "the TS-3108 manifest changed");
  assert.equal(
    entries.filter(({ compilerSyntax }) => compilerSyntax === "valid").length,
    expectedValidCount,
    "the compiler syntax classification changed",
  );
  assert.equal(
    entries.filter(({ treeSitterPolicy }) => treeSitterPolicy === "tolerated").length,
    1,
    "each Tree-sitter tolerance must remain explicit and reviewed",
  );
  return entries;
}

function discoverConformanceSources() {
  return fs
    .readdirSync(conformanceRoot, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => `${entry.name}/case.ling`)
    .filter((relativePath) =>
      fs.statSync(path.join(conformanceRoot, ...relativePath.split("/"))).isFile(),
    )
    .sort();
}

function assertFiniteProcess(result, id) {
  assert.notEqual(result.error?.code, "ETIMEDOUT", `${id}: parser process timed out`);
  assert.equal(result.error, undefined, `${id}: ${result.error}`);
  assert.notEqual(result.status, null, `${id}: parser process did not terminate`);
  assert.ok(result.stdout.length < maximumOutputBytes, `${id}: stdout is unbounded`);
  assert.ok(result.stderr.length < maximumOutputBytes, `${id}: stderr is unbounded`);
  assert.ok(
    !/panicked at|fatal runtime error|stack overflow/iu.test(
      `${result.stdout}\n${result.stderr}`,
    ),
    `${id}: parser crashed`,
  );
}

function parseSource(entry) {
  const sourcePath = path.join(
    conformanceRoot,
    ...entry.relativePath.split("/"),
  );
  const result = spawnSync(
    parserExecutable(),
    ["parse", "--cst", "--no-ranges", "--timeout", parserTimeoutMicros, sourcePath],
    {
      cwd: packageRoot,
      encoding: "utf8",
      timeout: processTimeoutMillis,
      maxBuffer: maximumOutputBytes,
    },
  );
  assertFiniteProcess(result, entry.relativePath);

  const hasRecoveryNode = /\b(?:ERROR|MISSING)\b/u.test(result.stdout);
  const accepted = result.status === 0 && !hasRecoveryNode;
  switch (entry.treeSitterPolicy) {
    case "clean":
    case "tolerated":
      assert.ok(
        accepted,
        `${entry.relativePath}: expected a clean finite CST\n${result.stdout}\n${result.stderr}`,
      );
      break;
    case "error":
      assert.ok(
        !accepted && hasRecoveryNode,
        `${entry.relativePath}: expected a finite recovery CST\n${result.stdout}\n${result.stderr}`,
      );
      break;
    default:
      assert.fail(`unreachable Tree-sitter policy: ${entry.treeSitterPolicy}`);
  }
  return result.stdout;
}

function normalizeCst(cst) {
  return `${cst
    .replaceAll("\r\n", "\n")
    .replace(/\u001b\[[0-9;]*m/gu, "")
    .split("\n")
    .filter((line) => !line.includes("\tParse:"))
    .join("\n")
    .trimEnd()}\n`;
}

function canonicalizeJson(value) {
  if (Array.isArray(value)) {
    return value.map(canonicalizeJson);
  }
  if (value !== null && typeof value === "object") {
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .map((key) => [key, canonicalizeJson(value[key])]),
    );
  }
  return value;
}

function sha256(text) {
  return crypto.createHash("sha256").update(text, "utf8").digest("hex");
}

function currentSnapshots(entries, csts) {
  const snapshots = new Map();
  const nodeTypes = JSON.parse(fs.readFileSync(nodeTypesPath, "utf8"));
  snapshots.set(
    "node-types.json",
    sha256(`${JSON.stringify(canonicalizeJson(nodeTypes))}\n`),
  );
  for (const entry of entries) {
    snapshots.set(entry.relativePath, sha256(normalizeCst(csts.get(entry.relativePath))));
  }
  return snapshots;
}

function formatSnapshots(snapshots) {
  return [
    "# TS-3108 stable Tree-sitter node mapping and whole-program CST hashes.",
    "# Format: source identifier, tab, normalized SHA-256.",
    ...Array.from(snapshots, ([id, hash]) => `${id}\t${hash}`),
    "",
  ].join("\n");
}

function readSnapshots() {
  const snapshots = new Map();
  let previousPath;
  for (const [index, line] of fs
    .readFileSync(snapshotsPath, "utf8")
    .split(/\r?\n/u)
    .entries()) {
    if (!line || line.startsWith("#")) {
      continue;
    }
    const fields = line.split("\t");
    assert.equal(fields.length, 2, `invalid CST snapshot row ${index + 1}: ${line}`);
    const [id, hash] = fields;
    assert.match(hash, /^[0-9a-f]{64}$/u, `invalid SHA-256 in row ${index + 1}`);
    if (previousPath !== undefined) {
      assert.ok(
        previousPath === "node-types.json" || previousPath < id,
        `CST snapshot identifiers must be unique and sorted: ${previousPath}, ${id}`,
      );
    }
    assert.ok(!snapshots.has(id), `duplicate CST snapshot identifier: ${id}`);
    snapshots.set(id, hash);
    previousPath = id;
  }
  return snapshots;
}

function nextRandom(state) {
  let value = state.value;
  value ^= value << 13;
  value ^= value >>> 17;
  value ^= value << 5;
  state.value = value >>> 0;
  return state.value;
}

function mutationPair(source, state) {
  const codepoints = Array.from(source);
  assert.ok(codepoints.length > 0, "conformance source must not be empty");
  const deletionStart = nextRandom(state) % codepoints.length;
  const maximumDeletion = Math.min(4, codepoints.length - deletionStart);
  const deletionLength = 1 + (nextRandom(state) % maximumDeletion);
  const deletion = codepoints.toSpliced(deletionStart, deletionLength).join("");

  const insertions = ['"', "'", "{", "(", "|>", "->", "人", "\n  "];
  const insertionStart = nextRandom(state) % (codepoints.length + 1);
  const inserted = insertions[nextRandom(state) % insertions.length];
  const insertion = codepoints.toSpliced(insertionStart, 0, inserted).join("");
  return [deletion, insertion];
}

function writeMutations(entries, temporaryRoot) {
  const state = { value: 0x3108c0de };
  const paths = [];
  for (const [caseIndex, entry] of entries.entries()) {
    const sourcePath = path.join(
      conformanceRoot,
      ...entry.relativePath.split("/"),
    );
    const source = fs.readFileSync(sourcePath, "utf8");
    for (const [mutationIndex, mutation] of mutationPair(source, state).entries()) {
      const basename = `${String(caseIndex).padStart(2, "0")}-${mutationIndex}.ling`;
      const mutationPath = path.join(temporaryRoot, basename);
      fs.writeFileSync(mutationPath, mutation, "utf8");
      paths.push(mutationPath);
    }
  }
  assert.equal(paths.length, expectedMutationCount);
  const pathsFile = path.join(temporaryRoot, "paths.txt");
  fs.writeFileSync(pathsFile, `${paths.join("\n")}\n`, "utf8");
  return pathsFile;
}

function parseMutationBatch(pathsFile) {
  const result = spawnSync(
    parserExecutable(),
    [
      "parse",
      "--quiet",
      "--json-summary",
      "--timeout",
      parserTimeoutMicros,
      "--paths",
      pathsFile,
    ],
    {
      cwd: packageRoot,
      encoding: "utf8",
      timeout: mutationProcessTimeoutMillis,
      maxBuffer: maximumOutputBytes,
    },
  );
  assertFiniteProcess(result, "whole-corpus deterministic mutations");
  const output = result.stdout.replaceAll("\r\n", "\n");
  const summaryStart = output.lastIndexOf('{\n  "parse_summaries"');
  assert.notEqual(summaryStart, -1, `mutation summary is not JSON\n${result.stdout}`);
  const summary = JSON.parse(output.slice(summaryStart));
  assert.equal(summary.parse_summaries.length, expectedMutationCount);
  assert.equal(summary.source_count, expectedMutationCount);
  return {
    sourceCount: summary.source_count,
    parses: summary.parse_summaries.map((item) => ({
      file: path.basename(item.file),
      successful: item.successful,
      start: item.start,
      end: item.end,
    })),
  };
}

function assertMutationSmoke(entries) {
  const temporaryRoot = fs.mkdtempSync(path.join(os.tmpdir(), "ling-ts3108-"));
  try {
    const pathsFile = writeMutations(entries, temporaryRoot);
    const first = parseMutationBatch(pathsFile);
    const second = parseMutationBatch(pathsFile);
    assert.deepEqual(second, first, "fixed-seed mutation results must be deterministic");
  } finally {
    fs.rmSync(temporaryRoot, { recursive: true, force: true });
  }
}

const entries = readManifest();
assert.deepEqual(
  entries.map(({ relativePath }) => relativePath),
  discoverConformanceSources(),
  "the TS-3108 manifest must cover every compiler conformance source exactly once",
);

const csts = new Map(
  entries.map((entry) => [entry.relativePath, parseSource(entry)]),
);
const snapshots = currentSnapshots(entries, csts);
if (process.argv.includes("--print-snapshots")) {
  process.stdout.write(formatSnapshots(snapshots));
  process.exit(0);
}
assert.deepEqual(
  readSnapshots(),
  snapshots,
  "Tree-sitter node mapping changed; review the CSTs before updating snapshots",
);
assertMutationSmoke(entries);

console.log(
  `Conformance differential passed (${entries.length} programs, ${expectedMutationCount} fixed-seed edits, ${snapshots.size} stable mappings).`,
);
