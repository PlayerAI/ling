"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const packageRoot = path.resolve(__dirname, "..");
const repositoryRoot = path.resolve(packageRoot, "..", "..");
const corpusPath = path.join(__dirname, "fixtures", "unicode-identifiers.tsv");
const derivedCorePropertiesPath = path.join(
  repositoryRoot,
  "tools",
  "unicode-gen",
  "data",
  "17.0.0",
  "ucd",
  "DerivedCoreProperties.txt",
);
const generated = require(path.join(
  packageRoot,
  "src",
  "unicode-identifiers.generated.js",
));
const identifiers = require(path.join(packageRoot, "src", "unicode-identifiers.js"));

function parsePropertyRanges(input, property) {
  return input
    .split(/\r?\n/u)
    .map((line) => line.split("#", 1)[0].trim())
    .filter(Boolean)
    .map((line) => line.split(";").map((field) => field.trim()))
    .filter((fields) => fields[1] === property)
    .map(([range]) => {
      const [start, end = start] = range.split("..");
      return [Number.parseInt(start, 16), Number.parseInt(end, 16)];
    });
}

function decodeCodepoints(input) {
  return String.fromCodePoint(
    ...input.split("+").map((value) => Number.parseInt(value, 16)),
  );
}

function parseCases(input) {
  return input
    .split(/\r?\n/u)
    .filter((line) => line.trim() && !line.startsWith("#"))
    .map((line) => {
      const fields = line.split("\t");
      assert.equal(fields.length, 6, `invalid differential row: ${line}`);
      return {
        id: fields[0],
        spelling: decodeCodepoints(fields[1]),
        treeSitter: fields[2],
      };
    });
}

function contains(ranges, codepoint) {
  let low = 0;
  let high = ranges.length;
  while (low < high) {
    const middle = Math.floor((low + high) / 2);
    const [start, end] = ranges[middle];
    if (codepoint < start) {
      high = middle;
    } else if (codepoint > end) {
      low = middle + 1;
    } else {
      return true;
    }
  }
  return false;
}

function hasXidShape(spelling) {
  const codepoints = Array.from(spelling, (character) => character.codePointAt(0));
  return (
    codepoints.length > 0 &&
    (codepoints[0] === 0x5f || contains(generated.XID_START_RANGES, codepoints[0])) &&
    codepoints.slice(1).every((codepoint) =>
      contains(generated.XID_CONTINUE_RANGES, codepoint),
    )
  );
}

function parserExecutable() {
  const executable = process.platform === "win32" ? "tree-sitter.exe" : "tree-sitter";
  return path.join(packageRoot, "node_modules", "tree-sitter-cli", executable);
}

function assertParserResults(cases) {
  const temporaryRoot = fs.mkdtempSync(path.join(os.tmpdir(), "ling-unicode-"));
  try {
    const caseByFile = new Map();
    const paths = cases.map((testCase, index) => {
      const fileName = `${String(index).padStart(3, "0")}-${testCase.id}.ling`;
      const sourcePath = path.join(temporaryRoot, fileName);
      fs.writeFileSync(sourcePath, `let ${testCase.spelling} = 0\n`, "utf8");
      caseByFile.set(fileName, testCase);
      return sourcePath;
    });
    const pathsFile = path.join(temporaryRoot, "paths.txt");
    fs.writeFileSync(pathsFile, `${paths.join("\n")}\n`, "utf8");

    const result = spawnSync(
      parserExecutable(),
      ["parse", "--quiet", "--json-summary", "--paths", pathsFile],
      { cwd: packageRoot, encoding: "utf8", timeout: 30_000 },
    );
    assert.notEqual(result.error?.code, "ETIMEDOUT", "Tree-sitter differential timed out");
    assert.ok(result.stdout, `Tree-sitter emitted no JSON summary: ${result.stderr}`);
    const normalizedOutput = result.stdout.replace(/\r\n/gu, "\n");
    const summaryStart = normalizedOutput.lastIndexOf('{\n  "parse_summaries"');
    assert.notEqual(
      summaryStart,
      -1,
      `Tree-sitter summary was not machine-readable: ${result.stdout}`,
    );
    const summary = JSON.parse(normalizedOutput.slice(summaryStart));
    assert.equal(summary.parse_summaries.length, cases.length);

    for (const parsed of summary.parse_summaries) {
      const testCase = caseByFile.get(path.basename(parsed.file));
      assert.ok(testCase, `unexpected parser result for ${parsed.file}`);
      assert.equal(
        parsed.successful,
        testCase.treeSitter === "identifier",
        `${testCase.id} parsed unexpectedly`,
      );
    }
  } finally {
    fs.rmSync(temporaryRoot, { recursive: true, force: true });
  }
}

const ucd = fs.readFileSync(derivedCorePropertiesPath, "utf8");
assert.equal(generated.UNICODE_VERSION, "17.0.0");
assert.equal(identifiers.UNICODE_VERSION, generated.UNICODE_VERSION);
assert.deepEqual(
  generated.XID_START_RANGES,
  parsePropertyRanges(ucd, "XID_Start"),
  "generated XID_Start ranges differ from the pinned UCD",
);
assert.deepEqual(
  generated.XID_CONTINUE_RANGES,
  parsePropertyRanges(ucd, "XID_Continue"),
  "generated XID_Continue ranges differ from the pinned UCD",
);
assert.ok(!identifiers.IDENTIFIER_PATTERN.includes("\\p{"));

const cases = parseCases(fs.readFileSync(corpusPath, "utf8"));
assert.ok(cases.length >= 18, "the differential corpus unexpectedly shrank");
for (const testCase of cases) {
  const xidShaped = hasXidShape(testCase.spelling);
  assert.equal(
    xidShaped,
    testCase.treeSitter === "identifier" || testCase.id === "and-keyword",
    `${testCase.id} has an inconsistent pinned-XID expectation`,
  );
}
assertParserResults(cases);

console.log(
  `Unicode identifier differential passed (${cases.length} shared cases, Unicode ${generated.UNICODE_VERSION}).`,
);
