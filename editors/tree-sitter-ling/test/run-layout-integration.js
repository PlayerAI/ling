const { mkdirSync, writeFileSync } = require("node:fs");
const { resolve } = require("node:path");
const { spawnSync } = require("node:child_process");

const cacheDirectory = resolve(".cache", "layout-integration");
const treeSitter =
  process.env.TREE_SITTER_CLI ||
  resolve(
    "node_modules",
    "tree-sitter-cli",
    process.platform === "win32" ? "tree-sitter.exe" : "tree-sitter",
  );

mkdirSync(cacheDirectory, { recursive: true });

function parseSource(name, source, edits = []) {
  const sourcePath = resolve(cacheDirectory, `${name}.ling`);
  writeFileSync(sourcePath, source, { encoding: "utf8" });

  const args = ["parse", "--quiet", "--json-summary"];
  if (edits.length > 0) {
    args.push("--edits", ...edits, "--");
  }
  args.push(sourcePath);

  const result = spawnSync(treeSitter, args, {
    cwd: resolve(__dirname, ".."),
    encoding: "utf8",
  });
  if (result.error) {
    throw result.error;
  }

  let summary;
  try {
    const jsonStart = result.stdout.indexOf("{");
    if (jsonStart < 0) {
      throw new Error("missing JSON object");
    }
    summary = JSON.parse(result.stdout.slice(jsonStart));
  } catch (error) {
    throw new Error(
      `${name}: Tree-sitter did not return a JSON summary\n${result.stdout}\n${result.stderr}`,
      { cause: error },
    );
  }

  if (summary.parse_summaries.length !== 1) {
    throw new Error(
      `${name}: expected one parse summary\n${result.stdout}\n${result.stderr}`,
    );
  }
  return { result, summary: summary.parse_summaries[0] };
}

function assertSuccessfulParse(name, source, edits = []) {
  const { result, summary } = parseSource(name, source, edits);
  if (result.status !== 0 || summary.successful !== true) {
    throw new Error(`${name}: parse failed\n${result.stdout}\n${result.stderr}`);
  }
}

function assertRejectedParse(name, source) {
  const { result, summary } = parseSource(name, source);
  if (result.status === 0 || summary.successful !== false) {
    throw new Error(
      `${name}: invalid source parsed successfully\n${result.stdout}\n${result.stderr}`,
    );
  }
}

assertSuccessfulParse(
  "crlf",
  "module Main\r\n" +
    "    requires Console.Write\r\n" +
    "\r\n" +
    "let main () =\r\n" +
    "    Console.write \"ok\"\r\n",
);

assertSuccessfulParse(
  "lone-cr",
  "let main () =\r" +
    "  let value =\r" +
    "    1\r" +
    "  value\r",
);

assertSuccessfulParse(
  "eof-dedents",
  "let main () =\n" +
    "  let value =\n" +
    "    1\n" +
    "  value",
);

assertSuccessfulParse(
  "comment-only-lines",
  "let main () =\n" +
    "  // line comment\n" +
    "  /* outer\n" +
    "     /* nested */\n" +
    "  */\n" +
    "  1\n",
);

assertSuccessfulParse(
  "nested-comment-depth-256",
  `${"/*".repeat(256)}comment${"*/".repeat(256)}\nlet value = 1\n`,
);
assertRejectedParse(
  "nested-comment-depth-257",
  `${"/*".repeat(257)}comment${"*/".repeat(257)}\nlet value = 1\n`,
);
assertRejectedParse("unclosed-block-comment", "/* unclosed\nlet value = 1\n");

assertSuccessfulParse(
  "delimiter-soft-newlines",
  "let update person =\n" +
    "  { person with\n" +
    "      health = max 0 person.health }\n",
);

const incrementalSource =
  "let main () =\n" +
  "  let value =\n" +
  "    1\n" +
  "  value\n";
const insertionPoint = incrementalSource.indexOf("  value");
const insertedDeclaration = "  let extra =\n    2\n";
assertSuccessfulParse("incremental-reparse", incrementalSource, [
  `${insertionPoint} 0 ${insertedDeclaration}`,
]);
