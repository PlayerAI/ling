"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const packageRoot = path.resolve(__dirname, "..");
const queryPath = path.join(packageRoot, "queries", "highlights.scm");
const fixtureRoot = path.join(__dirname, "highlight");
const expectedFixtureNames = ["basics.ling", "recovery.ling", "unicode.ling"];
const expectedCaptures = [
  "boolean",
  "comment",
  "comment.doc",
  "constructor",
  "function",
  "function.definition",
  "keyword",
  "number",
  "operator",
  "property",
  "punctuation.bracket",
  "punctuation.delimiter",
  "string",
  "string.escape",
  "type",
  "type.builtin",
  "variable",
  "variable.parameter",
];
const expectedKeywords = [
  "and",
  "as",
  "else",
  "if",
  "import",
  "let",
  "match",
  "module",
  "mutable",
  "of",
  "rec",
  "requires",
  "then",
  "type",
  "when",
  "with",
];
const expectedBuiltinTypes = ["Bool", "Int", "List", "Text", "Unit", "f64"];
const processTimeoutMillis = 10_000;
const maximumOutputBytes = 200_000;

function parserExecutable() {
  const executable = process.platform === "win32" ? "tree-sitter.exe" : "tree-sitter";
  return path.join(packageRoot, "node_modules", "tree-sitter-cli", executable);
}

function normalizeOutput(output) {
  return output.replaceAll("\r\n", "\n").replace(/\u001b\[[0-9;]*m/gu, "");
}

function runTreeSitter(args, id) {
  const result = spawnSync(parserExecutable(), args, {
    cwd: packageRoot,
    encoding: "utf8",
    timeout: processTimeoutMillis,
    maxBuffer: maximumOutputBytes,
  });
  assert.notEqual(result.error?.code, "ETIMEDOUT", `${id}: process timed out`);
  assert.equal(result.error, undefined, `${id}: ${result.error}`);
  assert.notEqual(result.status, null, `${id}: process did not terminate`);
  assert.ok(result.stdout.length < maximumOutputBytes, `${id}: stdout is unbounded`);
  assert.ok(result.stderr.length < maximumOutputBytes, `${id}: stderr is unbounded`);
  assert.ok(
    !/panicked at|fatal runtime error|stack overflow/iu.test(
      `${result.stdout}\n${result.stderr}`,
    ),
    `${id}: Tree-sitter crashed`,
  );
  return {
    ...result,
    stdout: normalizeOutput(result.stdout),
    stderr: normalizeOutput(result.stderr),
  };
}

function discoverFixtures() {
  return fs
    .readdirSync(fixtureRoot, { withFileTypes: true })
    .filter((entry) => entry.isFile() && entry.name.endsWith(".ling"))
    .map((entry) => entry.name)
    .sort();
}

function fixtureSelectors(fixtureNames) {
  const selectors = new Set();
  for (const fixtureName of fixtureNames) {
    const source = fs.readFileSync(path.join(fixtureRoot, fixtureName), "utf8");
    for (const line of source.split(/\r?\n/u)) {
      const match = line.match(/\/\/\s*(?:<-|\^+)\s+(!?)([a-z][a-z0-9.]*)\s*$/u);
      if (match && match[1] !== "!") {
        selectors.add(match[2]);
      }
    }
  }
  return [...selectors].sort();
}

function selectorCounts(source) {
  const counts = new Map();
  for (const line of source.split(/\r?\n/u)) {
    const match = line.match(/\/\/\s*(?:<-|\^+)\s+(!?)([a-z][a-z0-9.]*)\s*$/u);
    if (match && match[1] !== "!") {
      counts.set(match[2], (counts.get(match[2]) ?? 0) + 1);
    }
  }
  return Object.fromEntries([...counts].sort(([left], [right]) => left.localeCompare(right)));
}

function queryCaptures(query) {
  return Array.from(
    query.matchAll(/@([a-z][a-z0-9.]*)/gu),
    (match) => match[1],
  )
    .filter((capture) => !capture.startsWith("_"))
    .filter((capture, index, captures) => captures.indexOf(capture) === index)
    .sort();
}

function queryKeywords(query) {
  const match = query.match(/\[\s*((?:"[a-z]+"\s*)+)\]\s*@keyword/u);
  assert.ok(match, "highlights.scm must contain one explicit keyword capture list");
  return Array.from(
    match[1].matchAll(/"([a-z]+)"/gu),
    (keyword) => keyword[1],
  ).sort();
}

function queryBuiltinTypes(query) {
  const match = query.match(
    /\(#any-of\?\s+@type\.builtin((?:\s+"[A-Za-z0-9]+")+)\s*\)/u,
  );
  assert.ok(match, "highlights.scm must explicitly classify Seed built-in types");
  return Array.from(
    match[1].matchAll(/"([A-Za-z0-9]+)"/gu),
    (builtin) => builtin[1],
  ).sort();
}

function assertParsePolicy(fixtureName) {
  const fixturePath = path.join(fixtureRoot, fixtureName);
  const result = runTreeSitter(
    ["parse", "--cst", "--no-ranges", fixturePath],
    `parse ${fixtureName}`,
  );
  const hasRecovery = /\b(?:ERROR|MISSING)\b/u.test(result.stdout);
  if (fixtureName === "recovery.ling") {
    assert.ok(hasRecovery, "the emoji-prefix recovery fixture must exercise recovery");
  } else {
    assert.equal(result.status, 0, `${fixtureName}: parse failed\n${result.stderr}`);
    assert.ok(!hasRecovery, `${fixtureName}: expected a clean CST\n${result.stdout}`);
  }
}

assert.ok(fs.existsSync(queryPath), "missing queries/highlights.scm");
const fixtureNames = discoverFixtures();
assert.deepEqual(fixtureNames, expectedFixtureNames, "the highlight fixture set changed");
assert.deepEqual(
  fixtureSelectors(fixtureNames),
  expectedCaptures,
  "every supported ZQ-3201 capture must have a positive fixture assertion",
);

const unicodeFixture = fs.readFileSync(path.join(fixtureRoot, "unicode.ling"), "utf8");
assert.match(unicodeFixture, /let 增加 数值/u, "missing Chinese function fixture");
assert.match(unicodeFixture, /type 人物/u, "missing Chinese type fixture");
assert.match(unicodeFixture, /\{ 姓名:/u, "missing Chinese property fixture");
assert.match(unicodeFixture, /\| 是/u, "missing Chinese constructor fixture");
assert.ok(unicodeFixture.includes("cafe\u0301"), "missing decomposed combining identifier");
assert.notEqual(
  unicodeFixture,
  unicodeFixture.normalize("NFC"),
  "the combining-identifier fixture must remain decomposed",
);
assert.deepEqual(
  selectorCounts(unicodeFixture),
  {
    constructor: 2,
    function: 2,
    "function.definition": 3,
    property: 2,
    type: 2,
    variable: 2,
  },
  "ASCII and Chinese identifiers must retain paired structural assertions",
);

const recoveryFixture = fs.readFileSync(path.join(fixtureRoot, "recovery.ling"), "utf8");
assert.match(recoveryFixture, /let 😀name/u, "missing emoji-prefix recovery fixture");

const query = fs.readFileSync(queryPath, "utf8");
assert.deepEqual(
  queryCaptures(query),
  expectedCaptures,
  "highlights.scm must expose exactly the reviewed ZQ-3201 capture set",
);
assert.deepEqual(
  queryKeywords(query),
  expectedKeywords,
  "highlight only current Ling keywords; do not color future reserved words",
);
assert.deepEqual(
  queryBuiltinTypes(query),
  expectedBuiltinTypes,
  "built-in type highlighting must match the implemented Seed type checker",
);
assert.match(
  query,
  /@function\s+@function\.definition/u,
  "function definitions must prefer @function.definition with @function fallback",
);

for (const fixtureName of fixtureNames) {
  assertParsePolicy(fixtureName);
}

const fixturePaths = fixtureNames.map((fixtureName) => path.join(fixtureRoot, fixtureName));
const first = runTreeSitter(
  ["query", "--test", queryPath, ...fixturePaths],
  "highlight query fixtures",
);
assert.equal(
  first.status,
  0,
  `highlight query fixtures failed\n${first.stdout}\n${first.stderr}`,
);
const second = runTreeSitter(
  ["query", "--test", queryPath, ...fixturePaths],
  "highlight query fixture determinism",
);
assert.equal(second.status, 0, `second highlight run failed\n${second.stderr}`);
assert.equal(second.stdout, first.stdout, "highlight query output must be deterministic");
assert.equal(second.stderr, first.stderr, "highlight query diagnostics must be deterministic");

console.log(
  `Highlight queries passed (${expectedCaptures.length} captures, ${fixtureNames.length} fixtures).`,
);
