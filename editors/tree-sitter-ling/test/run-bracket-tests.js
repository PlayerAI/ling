"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const packageRoot = path.resolve(__dirname, "..");
const queryPath = path.join(packageRoot, "queries", "brackets.scm");
const fixtureRoot = path.join(__dirname, "fixtures", "brackets");
const expectedFixtureNames = ["basics.ling", "comments.ling", "recovery.ling"];
const expectedCaptures = ["close", "open"];
const expectedPairs = [
  ["\"", "\""],
  ["(", ")"],
  ["[", "]"],
  ["{", "}"],
];
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

function queryCaptures(query) {
  return Array.from(
    query.matchAll(/@([a-z][a-z0-9.]*)/gu),
    (match) => match[1],
  )
    .filter((capture, index, captures) => captures.indexOf(capture) === index)
    .sort();
}

function fixtureSelectorCounts(source) {
  const counts = new Map();
  for (const line of source.split(/\r?\n/u)) {
    const match = line.match(/\/\/\s*(?:<-|\^+)\s+(!?)(open|close)\s*$/u);
    if (match) {
      const selector = `${match[1]}${match[2]}`;
      counts.set(selector, (counts.get(selector) ?? 0) + 1);
    }
  }
  return Object.fromEntries([...counts].sort(([left], [right]) => left.localeCompare(right)));
}

function queryPairs(query) {
  const pairs = [];
  const pairPattern = /"((?:\\.|[^"\\])*)"\s+@open\s+"((?:\\.|[^"\\])*)"\s+@close/gu;
  for (const match of query.matchAll(pairPattern)) {
    pairs.push([JSON.parse(`"${match[1]}"`), JSON.parse(`"${match[2]}"`)]);
  }
  return pairs.sort(([left], [right]) => left.localeCompare(right));
}

function assertParsePolicy(fixtureName) {
  const result = runTreeSitter(
    ["parse", "--cst", "--no-ranges", path.join(fixtureRoot, fixtureName)],
    `parse ${fixtureName}`,
  );
  const hasRecovery = /\b(?:ERROR|MISSING)\b/u.test(result.stdout);
  if (fixtureName === "recovery.ling") {
    assert.ok(hasRecovery, "the emoji-prefix fixture must exercise recovery");
  } else {
    assert.equal(result.status, 0, `${fixtureName}: parse failed\n${result.stderr}`);
    assert.ok(!hasRecovery, `${fixtureName}: expected a clean CST\n${result.stdout}`);
  }
}

assert.ok(fs.existsSync(queryPath), "missing queries/brackets.scm");
const fixtureNames = discoverFixtures();
assert.deepEqual(fixtureNames, expectedFixtureNames, "the bracket fixture set changed");
const basicFixture = fs.readFileSync(path.join(fixtureRoot, "basics.ling"), "utf8");
assert.match(basicFixture, /let 数据 = \[\("\u96f6"/u, "missing nested Chinese pair fixture");
assert.deepEqual(
  fixtureSelectorCounts(basicFixture),
  { "!close": 1, "!open": 1, close: 6, open: 6 },
  "the basic pair and escaped-quote assertions changed",
);
const commentFixture = fs.readFileSync(path.join(fixtureRoot, "comments.ling"), "utf8");
assert.deepEqual(
  fixtureSelectorCounts(commentFixture),
  { "!open": 4 },
  "the block-comment negative assertions changed",
);
assert.equal(
  (commentFixture.match(/\/\*/gu) ?? []).length,
  2,
  "the comment fixture must contain a nested block comment",
);
const recoveryFixture = fs.readFileSync(path.join(fixtureRoot, "recovery.ling"), "utf8");
assert.deepEqual(
  fixtureSelectorCounts(recoveryFixture),
  { close: 1, open: 1 },
  "the recovery canary pair assertions changed",
);
assert.match(recoveryFixture, /let 😀name/u, "missing emoji-prefix recovery input");

const query = fs.readFileSync(queryPath, "utf8");
assert.deepEqual(
  queryCaptures(query),
  expectedCaptures,
  "brackets.scm must expose only Zed's open and close captures",
);
assert.deepEqual(queryPairs(query), expectedPairs, "the reviewed bracket pairs changed");
assert.ok(
  query.includes('"\\\"" @open "\\\"" @close'),
  "string quotes must be an explicit bracket pair",
);
assert.match(
  query,
  /#set!\s+rainbow\.exclude/u,
  "string quotes must opt out of rainbow coloring",
);
assert.doesNotMatch(
  query,
  /block_comment/u,
  "opaque block-comment contents must not participate in bracket matching",
);

for (const fixtureName of fixtureNames) {
  assertParsePolicy(fixtureName);
}

const fixturePaths = fixtureNames.map((fixtureName) => path.join(fixtureRoot, fixtureName));
const first = runTreeSitter(
  ["query", "--test", queryPath, ...fixturePaths],
  "bracket query fixtures",
);
assert.equal(first.status, 0, `bracket fixtures failed\n${first.stdout}\n${first.stderr}`);
const second = runTreeSitter(
  ["query", "--test", queryPath, ...fixturePaths],
  "bracket query fixture determinism",
);
assert.equal(second.status, 0, `second bracket run failed\n${second.stderr}`);
assert.equal(second.stdout, first.stdout, "bracket query output must be deterministic");
assert.equal(second.stderr, first.stderr, "bracket query diagnostics must be deterministic");

const commentQuery = runTreeSitter(
  [
    "query",
    "--captures",
    queryPath,
    path.join(fixtureRoot, "comments.ling"),
  ],
  "nested block-comment exclusion",
);
assert.equal(commentQuery.status, 0, `comment query failed\n${commentQuery.stderr}`);
assert.doesNotMatch(
  commentQuery.stdout,
  /\b(?:open|close)\b/u,
  "bracket-like text inside nested block comments must produce no captures",
);

console.log(`Bracket queries passed (${expectedPairs.length} pairs, ${fixtureNames.length} fixtures).`);
