"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const packageRoot = path.resolve(__dirname, "..");
const corpusPath = path.join(
  __dirname,
  "fixtures",
  "expression-precedence.tsv",
);

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
      return { id: fields[0], expression: fields[1], expected: fields[2] };
    });
}

function parseCst(input) {
  const roots = [];
  const stack = [];
  for (const line of input.replace(/\r\n/gu, "\n").split("\n")) {
    if (!line.trim()) {
      continue;
    }
    const indentation = line.match(/^ */u)[0].length;
    assert.equal(indentation % 2, 0, `invalid CST indentation: ${line}`);
    const depth = indentation / 2;
    const match = line
      .trim()
      .match(/^(?:(?<field>[a-z_]+): )?(?<value>.+)$/u);
    assert.ok(match, `invalid CST line: ${line}`);
    const { field, value } = match.groups;
    assert.ok(!value.includes("ERROR") && !value.includes("MISSING"), input);

    let node;
    if (value.startsWith('"')) {
      node = { kind: "token", field, text: JSON.parse(value), children: [] };
    } else {
      const named = value.match(/^(?<kind>[a-z_]+)(?: `(?<text>.*)`)$/u);
      node = named
        ? {
            kind: named.groups.kind,
            field,
            text: named.groups.text,
            children: [],
          }
        : { kind: value, field, children: [] };
    }

    stack.length = depth;
    if (depth === 0) {
      roots.push(node);
    } else {
      assert.ok(stack[depth - 1], `orphan CST node: ${line}`);
      stack[depth - 1].children.push(node);
    }
    stack[depth] = node;
  }
  assert.equal(roots.length, 1, "expected one Tree-sitter CST root");
  return roots[0];
}

function child(node, field) {
  const value = node.children.find((candidate) => candidate.field === field);
  assert.ok(value, `${node.kind} has no ${field} child`);
  return value;
}

function namedChild(node) {
  const value = node.children.find((candidate) => candidate.kind !== "token");
  assert.ok(value, `${node.kind} has no named child`);
  return value;
}

function token(node) {
  const value = node.children.find((candidate) => candidate.kind === "token");
  assert.ok(value, `${node.kind} has no operator token`);
  return value.text;
}

function renderExpression(node) {
  switch (node.kind) {
    case "parenthesized_expression":
      return renderExpression(namedChild(node));
    case "assignment_expression":
      return `assign(${renderExpression(child(node, "left"))},${renderExpression(child(node, "right"))})`;
    case "pipeline_expression":
      return `pipe(${renderExpression(child(node, "left"))},${renderExpression(child(node, "right"))})`;
    case "binary_expression": {
      const names = new Map([
        ["||", "or"],
        ["&&", "and"],
        ["==", "eq"],
        ["!=", "neq"],
        ["<", "lt"],
        ["<=", "lte"],
        [">", "gt"],
        [">=", "gte"],
        ["+", "add"],
        ["-", "sub"],
        ["*", "mul"],
        ["/", "div"],
        ["%", "rem"],
      ]);
      const name = names.get(token(node));
      assert.ok(name, `unknown binary operator in ${JSON.stringify(node)}`);
      return `${name}(${renderExpression(child(node, "left"))},${renderExpression(child(node, "right"))})`;
    }
    case "unary_expression": {
      const name = token(node) === "+" ? "pos" : "neg";
      return `${name}(${renderExpression(child(node, "operand"))})`;
    }
    case "application_expression":
      return `apply(${renderExpression(child(node, "function"))},${renderExpression(child(node, "argument"))})`;
    case "projection_expression":
      return `proj(${renderExpression(child(node, "value"))},${child(node, "field").text})`;
    case "name_expression":
      return `n(${child(node, "name").text})`;
    case "literal_expression":
      return renderExpression(namedChild(node));
    case "integer_literal":
      return `i(${node.text})`;
    case "boolean_literal": {
      const value = node.text ?? token(node);
      return `b(${value})`;
    }
    default:
      throw new Error(`unexpected expression node ${node.kind}`);
  }
}

function assertParserResults(cases) {
  const temporaryRoot = fs.mkdtempSync(path.join(os.tmpdir(), "ling-precedence-"));
  try {
    const sourcePath = path.join(temporaryRoot, "precedence.ling");
    const source = cases
      .map((testCase, index) => `let result${index} = ${testCase.expression}\n`)
      .join("");
    fs.writeFileSync(sourcePath, source, "utf8");
    const result = spawnSync(
      parserExecutable(),
      ["parse", "--cst", "--no-ranges", sourcePath],
      { cwd: packageRoot, encoding: "utf8", timeout: 30_000 },
    );
    assert.notEqual(result.error?.code, "ETIMEDOUT", "Tree-sitter differential timed out");
    assert.equal(
      result.status,
      0,
      `Tree-sitter precedence parse failed:\n${result.stdout}\n${result.stderr}`,
    );
    const root = parseCst(result.stdout);
    const declarations = root.children.filter(
      (node) => node.kind === "let_declaration",
    );
    assert.equal(declarations.length, cases.length);
    declarations.forEach((declaration, index) => {
      const testCase = cases[index];
      assert.equal(
        renderExpression(child(declaration, "value")),
        testCase.expected,
        testCase.id,
      );
    });
  } finally {
    fs.rmSync(temporaryRoot, { recursive: true, force: true });
  }
}

const cases = parseCases(fs.readFileSync(corpusPath, "utf8"));
assert.equal(cases.length, 29, "the precedence differential corpus changed unexpectedly");
assertParserResults(cases);

console.log(`Expression precedence differential passed (${cases.length} shared cases).`);
