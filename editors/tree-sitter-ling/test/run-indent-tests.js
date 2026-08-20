"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const packageRoot = path.resolve(__dirname, "..");
const queryPath = path.join(packageRoot, "queries", "indents.scm");
const fixtureRoot = path.join(__dirname, "fixtures", "indents");
const expectedFixtureNames = [
  "blocks.ling",
  "delimiters.ling",
  "pipeline.ling",
  "recovery.ling",
];
const expectedCaptures = ["end", "indent", "start"];
const expectedIndentNodes = [
  "function_definition",
  "if_expression",
  "let_declaration",
  "list_expression",
  "match_case",
  "module_declaration",
  "pipeline_expression",
  "record_expression",
  "record_pattern",
  "record_type",
  "record_update_expression",
  "tuple_expression",
  "tuple_pattern",
  "tuple_type",
  "type_declaration",
];
const expectedCaptureStarts = {
  "blocks.ling": [
    "end@12:8",
    "end@20:2",
    "indent@0:0",
    "indent@3:0",
    "indent@7:0",
    "indent@9:4",
    "indent@10:8",
    "indent@10:8",
    "indent@14:4",
    "indent@17:0",
    "indent@18:2",
    "indent@18:2",
    "indent@23:0",
    "start@12:8",
    "start@20:2",
  ],
  "delimiters.ling": [
    "end@4:4",
    "end@10:4",
    "end@16:4",
    "end@22:4",
    "end@28:4",
    "end@34:4",
    "end@41:6",
    "end@45:8",
    "end@52:6",
    "indent@0:0",
    "indent@1:4",
    "indent@6:0",
    "indent@7:4",
    "indent@12:0",
    "indent@13:4",
    "indent@18:0",
    "indent@19:4",
    "indent@24:0",
    "indent@25:4",
    "indent@30:0",
    "indent@31:4",
    "indent@36:0",
    "indent@38:4",
    "indent@38:6",
    "indent@42:8",
    "indent@47:0",
    "indent@49:4",
    "indent@49:6",
  ],
  "pipeline.ling": [
    "end@7:8",
    "end@8:7",
    "indent@0:0",
    "indent@2:0",
    "indent@4:0",
    "indent@5:4",
    "indent@5:4",
    "start@6:4",
    "start@8:4",
  ],
  "recovery.ling": [
    "end@6:4",
    "indent@0:0",
    "indent@2:0",
    "indent@3:4",
  ],
};
const processTimeoutMillis = 10_000;
const maximumOutputBytes = 300_000;

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

function indentNodeNames(query) {
  return Array.from(
    query.matchAll(/^\(([a-z][a-z0-9_]*)\b/gmu),
    (match) => match[1],
  )
    .filter((name, index, names) => names.indexOf(name) === index)
    .sort();
}

function captureStarts(output) {
  return Array.from(
    output.matchAll(
      /^\s+pattern:\s+\d+,\s+capture:\s+\d+\s+-\s+(end|indent|start),\s+start:\s+\((\d+),\s+(\d+)\),/gmu,
    ),
    (match) => `${match[1]}@${match[2]}:${match[3]}`,
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
    assert.ok(hasRecovery, "the emoji-prefix fixture must exercise recovery");
  } else {
    assert.equal(result.status, 0, `${fixtureName}: parse failed\n${result.stderr}`);
    assert.ok(!hasRecovery, `${fixtureName}: expected a clean CST\n${result.stdout}`);
  }
}

assert.ok(fs.existsSync(queryPath), "missing queries/indents.scm");
const fixtureNames = discoverFixtures();
assert.deepEqual(fixtureNames, expectedFixtureNames, "the indent fixture set changed");
assert.deepEqual(
  Array.from(
    new Set(
      Object.values(expectedCaptureStarts)
        .flat()
        .map((capture) => capture.slice(0, capture.indexOf("@"))),
    ),
  ).sort(),
  expectedCaptures,
  "every supported ZQ-3203 capture must occur in the exact fixture snapshots",
);

const blocksFixture = fs.readFileSync(path.join(fixtureRoot, "blocks.ling"), "utf8");
assert.match(blocksFixture, /type 状态/u, "missing Chinese type/body fixture");
assert.match(blocksFixture, /let 选择 state/u, "missing Chinese function/body fixture");
assert.ok(blocksFixture.includes("cafe\u0301"), "missing decomposed combining identifier");
assert.notEqual(
  blocksFixture,
  blocksFixture.normalize("NFC"),
  "the combining-identifier fixture must remain decomposed",
);
assert.match(
  blocksFixture,
  /match state with\n    \|/u,
  "match cases must remain aligned with the match expression",
);

const pipelineFixture = fs.readFileSync(path.join(fixtureRoot, "pipeline.ling"), "utf8");
assert.match(
  pipelineFixture,
  /\n    value\n    \|>/u,
  "line-leading pipeline operators must remain aligned with the pipeline start",
);
const recoveryFixture = fs.readFileSync(path.join(fixtureRoot, "recovery.ling"), "utf8");
assert.match(recoveryFixture, /let 😀name/u, "missing emoji-prefix recovery input");

const query = fs.readFileSync(queryPath, "utf8");
assert.deepEqual(
  queryCaptures(query),
  expectedCaptures,
  "indents.scm must expose only Zed's indent-range captures",
);
assert.deepEqual(
  indentNodeNames(query),
  expectedIndentNodes,
  "the reviewed ZQ-3203 indentation node set changed",
);
assert.doesNotMatch(
  query,
  /^\(match_expression\b[^\n]*@indent/gmu,
  "DEC-0006 requires match cases to align with match, not indent beneath it",
);
assert.doesNotMatch(
  query,
  /^\(block\b[^\n]*@indent/gmu,
  "capturing a block at its first body token cannot indent that first token",
);
assert.match(query, /"else"\s+@end/u, "the consequence range must end at else");
assert.match(query, /"else"\s+@start/u, "the alternative range must start at else");
assert.match(
  query,
  /"\|>"\s+@start/u,
  "a continued pipeline operand must start its range at the operator",
);

for (const fixtureName of fixtureNames) {
  assertParsePolicy(fixtureName);
}

const fixturePaths = fixtureNames.map((fixtureName) => path.join(fixtureRoot, fixtureName));
const first = runTreeSitter(
  ["query", "--test", queryPath, ...fixturePaths],
  "indent query fixtures",
);
assert.equal(first.status, 0, `indent fixtures failed\n${first.stdout}\n${first.stderr}`);
const second = runTreeSitter(
  ["query", "--test", queryPath, ...fixturePaths],
  "indent query fixture determinism",
);
assert.equal(second.status, 0, `second indent run failed\n${second.stderr}`);
assert.equal(second.stdout, first.stdout, "indent query output must be deterministic");
assert.equal(second.stderr, first.stderr, "indent query diagnostics must be deterministic");

for (const fixtureName of fixtureNames) {
  const firstCaptures = runTreeSitter(
    ["query", "--captures", queryPath, path.join(fixtureRoot, fixtureName)],
    `indent capture inventory ${fixtureName}`,
  );
  assert.equal(
    firstCaptures.status,
    0,
    `${fixtureName}: capture query failed\n${firstCaptures.stderr}`,
  );
  assert.deepEqual(
    captureStarts(firstCaptures.stdout),
    [...expectedCaptureStarts[fixtureName]].sort(),
    `${fixtureName}: the reviewed indentation ranges changed`,
  );
  const secondCaptures = runTreeSitter(
    ["query", "--captures", queryPath, path.join(fixtureRoot, fixtureName)],
    `indent capture determinism ${fixtureName}`,
  );
  assert.equal(
    secondCaptures.status,
    0,
    `${fixtureName}: second capture query failed\n${secondCaptures.stderr}`,
  );
  assert.equal(
    secondCaptures.stdout,
    firstCaptures.stdout,
    `${fixtureName}: capture output must be deterministic`,
  );
  assert.equal(
    secondCaptures.stderr,
    firstCaptures.stderr,
    `${fixtureName}: capture diagnostics must be deterministic`,
  );
}

console.log(
  `Indent queries passed (${expectedIndentNodes.length} CST nodes, ${fixtureNames.length} fixtures).`,
);
