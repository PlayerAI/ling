# tree-sitter-ling

`tree-sitter-ling` is the editor-oriented concrete-syntax parser for Ling. This directory is a standalone-ready development mirror kept in the compiler repository while the grammar and shared corpus evolve together.

The grammar is not an authority for Ling validity or semantics. Accepted RFCs and decisions, the compiler specifications, conformance tests, and `ling-syntax` remain authoritative in that order. A tolerant Tree-sitter parse never makes invalid source valid Ling.

See [KNOWN-DIFFERENCES.md](KNOWN-DIFFERENCES.md) for the exact compiler/grammar differences that remain assigned to later tasks.

## Development

Requirements:

- Node.js 20 or newer;
- a C compiler available to Tree-sitter;
- the exact `tree-sitter-cli` version locked by `package-lock.json`.

Run:

```sh
npm ci
npm run verify
```

After the first locked installation, grammar generation and tests run without network access:

```sh
npm run generate --offline
npm test --offline
npm run parse:examples --offline
```

Generated parser sources under `src/` are committed. A change to `grammar.js` is complete only when regeneration produces no uncommitted diff and all corpus and example parses pass.

## TS-3102 boundary

This first breadth-first skeleton covers source files, declarations, types, patterns, expressions, identifiers, literals, and comments. The following accuracy work is intentionally assigned to later execution-plan tasks:

- exact offside indentation and nested block-comment scanning (`TS-3103`);
- generated Unicode 17.0.0 identifier parity, compiler-reserved `and` exclusion, and compiler differential tests (`TS-3104`);
- exhaustive expression precedence evidence (`TS-3105`);
- complete pattern/type edge coverage (`TS-3106`);
- edit-state and malformed-input recovery hardening (`TS-3107`);
- shared compiler/Tree-sitter corpus differential testing (`TS-3108`).

Language-specific package bindings and publication metadata remain disabled until an editor consumer requires them; the committed generated C parser is the only integration artifact introduced by TS-3102.

Until the corresponding accepted decision closes `GAP-SEED-BOOLEAN-OPERATORS-001`, `&&` and `||` remain error input.

## 中文说明

`tree-sitter-ling` 是面向编辑器的 Ling 具体语法解析器。本目录暂作为可独立拆分的开发镜像，与编译器共享演进过程。

Tree-sitter 不决定 Ling 源码是否合法，也不定义语言语义。语言行为仍以 Accepted RFC/decision、编译器规范、conformance tests 和 `ling-syntax` 为准。当前骨架只完成 `TS-3102` 的横向覆盖；缩进、Unicode 17.0.0 精确一致性、完整优先级、错误恢复和差分测试由后续任务完成。
