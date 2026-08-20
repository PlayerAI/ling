"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const packageRoot = path.resolve(__dirname, "..");
const fixturePath = path.join(__dirname, "fixtures", "recovery-cases.json");
const parserTimeoutMicros = "500000";
const processTimeoutMillis = 10_000;
const maximumCstBytes = 200_000;
const mutationCount = 64;

function parserExecutable() {
  const executable = process.platform === "win32" ? "tree-sitter.exe" : "tree-sitter";
  return path.join(packageRoot, "node_modules", "tree-sitter-cli", executable);
}

function parserResult(sourcePath, edits = []) {
  const args = [
    "parse",
    "--no-ranges",
    "--timeout",
    parserTimeoutMicros,
  ];
  if (edits.length > 0) {
    args.push("--edits", ...edits, "--");
  }
  args.push(sourcePath);
  return spawnSync(parserExecutable(), args, {
    cwd: packageRoot,
    encoding: "utf8",
    timeout: processTimeoutMillis,
  });
}

function assertFiniteProcess(result, id) {
  assert.notEqual(result.error?.code, "ETIMEDOUT", `${id}: process timed out`);
  assert.equal(result.error, undefined, `${id}: ${result.error}`);
  assert.ok(result.stdout.length < maximumCstBytes, `${id}: CST output is unbounded`);
  assert.ok(result.stderr.length < maximumCstBytes, `${id}: diagnostics are unbounded`);
  assert.ok(
    !/panicked at|fatal runtime error/iu.test(`${result.stdout}\n${result.stderr}`),
    `${id}: parser crashed`,
  );
}

function nodeCount(cst, nodeKind) {
  return (cst.match(new RegExp(`^\\s*\\(${nodeKind}\\b`, "gmu")) ?? []).length;
}

function assertRecoveringParse(temporaryRoot, testCase) {
  const sourcePath = path.join(temporaryRoot, `${testCase.id}.ling`);
  fs.writeFileSync(sourcePath, testCase.source, "utf8");
  const result = parserResult(sourcePath, testCase.edits);
  assertFiniteProcess(result, testCase.id);
  assert.notEqual(
    result.status,
    0,
    `${testCase.id}: malformed edit parsed successfully\n${result.stdout}`,
  );
  assert.match(result.stdout, /\b(?:ERROR|MISSING)\b/u, `${testCase.id}: no recovery node`);
  assert.equal(
    nodeCount(result.stdout, "type_declaration"),
    2,
    `${testCase.id}: surrounding definitions were not retained\n${result.stdout}`,
  );
}

function byteOffset(source, characterOffset) {
  return Buffer.byteLength(source.slice(0, characterOffset), "utf8");
}

function edit(source, needle, replacement = "", occurrence = 0) {
  let characterOffset = -1;
  let searchFrom = 0;
  for (let index = 0; index <= occurrence; index += 1) {
    characterOffset = source.indexOf(needle, searchFrom);
    assert.notEqual(characterOffset, -1, `edit needle not found: ${needle}`);
    searchFrom = characterOffset + needle.length;
  }
  return `${byteOffset(source, characterOffset)} ${Buffer.byteLength(needle, "utf8")} ${replacement}`;
}

function insertion(source, needle, inserted, occurrence = 0) {
  let characterOffset = -1;
  let searchFrom = 0;
  for (let index = 0; index <= occurrence; index += 1) {
    characterOffset = source.indexOf(needle, searchFrom);
    assert.notEqual(characterOffset, -1, `insertion needle not found: ${needle}`);
    searchFrom = characterOffset + needle.length;
  }
  characterOffset += needle.length;
  return `${byteOffset(source, characterOffset)} 0 ${inserted}`;
}

function incrementalCases() {
  const wrap = (body) => `type Before = Int\n${body}\ntype After = Text\n`;
  const stringSource = wrap('let broken = "ok"');
  const recordSource = wrap("let broken = { value = 1 }");
  const tupleSource = wrap("let broken = (1, 2)");
  const equalsSource = wrap("let broken = 1");
  const arrowSource = wrap("let broken input = match input with | Some item -> item");
  const withSource = wrap("let broken input = match input with | _ -> input");
  const pipelineSource = wrap("let broken = input");
  const chineseSource = wrap("let 人物 = 1");
  const indentationSource = wrap("let broken () =\n    let nested = 1\n    nested");

  return [
    {
      id: "incremental-unclosed-string",
      source: stringSource,
      edits: [edit(stringSource, '"', "", 1)],
    },
    {
      id: "incremental-unclosed-record",
      source: recordSource,
      edits: [edit(recordSource, " }")],
    },
    {
      id: "incremental-unclosed-tuple",
      source: tupleSource,
      edits: [edit(tupleSource, ")")],
    },
    {
      id: "incremental-missing-equals",
      source: equalsSource,
      edits: [edit(equalsSource, " =", "", 1)],
    },
    {
      id: "incremental-missing-arrow",
      source: arrowSource,
      edits: [edit(arrowSource, " ->")],
    },
    {
      id: "incremental-missing-with",
      source: withSource,
      edits: [edit(withSource, " with")],
    },
    {
      id: "incremental-incomplete-pipeline",
      source: pipelineSource,
      edits: [insertion(pipelineSource, "input", " |>")],
    },
    {
      id: "incremental-partial-chinese-identifier",
      source: chineseSource,
      edits: [edit(chineseSource, "物 = 1")],
    },
    {
      id: "incremental-inconsistent-indentation",
      source: indentationSource,
      edits: [edit(indentationSource, "    nested", "  nested")],
    },
  ];
}

function nextRandom(state) {
  let value = state.value;
  value ^= value << 13;
  value ^= value >>> 17;
  value ^= value << 5;
  state.value = value >>> 0;
  return state.value;
}

function mutate(source, index, state) {
  const codepoints = Array.from(source);
  const position = nextRandom(state) % codepoints.length;
  const insertions = ['"', "{", "(", "|>", "->", "人", "\n  ", "'"];
  switch (index % 3) {
    case 0: {
      const count = 1 + (nextRandom(state) % Math.min(5, codepoints.length - position));
      codepoints.splice(position, count);
      break;
    }
    case 1:
      codepoints.splice(position, 0, insertions[nextRandom(state) % insertions.length]);
      break;
    default:
      codepoints.splice(position, 1, insertions[nextRandom(state) % insertions.length]);
      break;
  }
  return codepoints.join("");
}

function assertMutationSmoke(temporaryRoot) {
  const source =
    "type Before = Int\n" +
    "let 处理 input = match input with | Some value -> value | None -> 0\n" +
    "let result = 处理 (Some 1) |> Text.format \"{}\"\n" +
    "type After = Text\n";
  const state = { value: 0x3107c0de };
  const paths = [];
  for (let index = 0; index < mutationCount; index += 1) {
    const sourcePath = path.join(temporaryRoot, `mutation-${String(index).padStart(2, "0")}.ling`);
    fs.writeFileSync(sourcePath, mutate(source, index, state), "utf8");
    paths.push(sourcePath);
  }
  const pathsFile = path.join(temporaryRoot, "mutation-paths.txt");
  fs.writeFileSync(pathsFile, `${paths.join("\n")}\n`, "utf8");
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
    { cwd: packageRoot, encoding: "utf8", timeout: processTimeoutMillis },
  );
  assertFiniteProcess(result, "deterministic-mutation-smoke");
  const normalizedOutput = result.stdout.replace(/\r\n/gu, "\n");
  const summaryStart = normalizedOutput.lastIndexOf('{\n  "parse_summaries"');
  assert.notEqual(summaryStart, -1, `mutation summary is not JSON\n${result.stdout}`);
  const summary = JSON.parse(normalizedOutput.slice(summaryStart));
  assert.equal(summary.parse_summaries.length, mutationCount);
  assert.equal(summary.source_count, mutationCount);
}

const cases = JSON.parse(fs.readFileSync(fixturePath, "utf8"));
const incremental = incrementalCases();
assert.equal(cases.length, 10, "the TS-3107 static recovery corpus changed unexpectedly");
assert.equal(incremental.length, 9, "the TS-3107 incremental corpus changed unexpectedly");
const temporaryRoot = fs.mkdtempSync(path.join(os.tmpdir(), "ling-recovery-"));
try {
  for (const testCase of [...cases, ...incremental]) {
    assertRecoveringParse(temporaryRoot, testCase);
  }
  assertMutationSmoke(temporaryRoot);
} finally {
  fs.rmSync(temporaryRoot, { recursive: true, force: true });
}

console.log(
  `Recovery integration passed (${cases.length} static cases, ${incremental.length} incremental edits, ${mutationCount} deterministic mutations).`,
);
