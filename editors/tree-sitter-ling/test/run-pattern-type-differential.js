"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const packageRoot = path.resolve(__dirname, "..");
const corpusPath = path.join(__dirname, "fixtures", "pattern-types.tsv");

function parserExecutable() {
  const executable = process.platform === "win32" ? "tree-sitter.exe" : "tree-sitter";
  return path.join(packageRoot, "node_modules", "tree-sitter-cli", executable);
}

function parseCases(input) {
  return input
    .split(/\r?\n/u)
    .filter((line) => line && !line.startsWith("#"))
    .map((line) => {
      const fields = line.split("\t");
      assert.equal(fields.length, 3, `invalid differential row: ${line}`);
      assert.ok(
        fields[1] === "valid" || fields[1] === "invalid",
        `invalid expectation in row: ${line}`,
      );
      return { id: fields[0], valid: fields[1] === "valid", source: fields[2] };
    });
}

function parseCase(testCase, temporaryRoot) {
  const sourcePath = path.join(temporaryRoot, `${testCase.id}.ling`);
  fs.writeFileSync(sourcePath, `${testCase.source}\n`, "utf8");
  const result = spawnSync(
    parserExecutable(),
    ["parse", "--cst", "--no-ranges", sourcePath],
    { cwd: packageRoot, encoding: "utf8", timeout: 30_000 },
  );
  assert.notEqual(result.error?.code, "ETIMEDOUT", `${testCase.id} timed out`);
  assert.equal(result.error, undefined, `${testCase.id}: ${result.error}`);
  const hasErrorNode = /\b(?:ERROR|MISSING)\b/u.test(result.stdout);
  const accepted = result.status === 0 && !hasErrorNode;
  assert.equal(
    accepted,
    testCase.valid,
    `${testCase.id}: status=${result.status}\n${result.stdout}\n${result.stderr}`,
  );
  assert.ok(result.stdout.length < 100_000, `${testCase.id} produced an unbounded CST`);
}

const cases = parseCases(fs.readFileSync(corpusPath, "utf8"));
assert.equal(cases.length, 41, "the TS-3106 corpus changed unexpectedly");
const temporaryRoot = fs.mkdtempSync(path.join(os.tmpdir(), "ling-pattern-types-"));
try {
  for (const testCase of cases) {
    parseCase(testCase, temporaryRoot);
  }
} finally {
  fs.rmSync(temporaryRoot, { recursive: true, force: true });
}

console.log(`Pattern/type differential passed (${cases.length} shared cases).`);
