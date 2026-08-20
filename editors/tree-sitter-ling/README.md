# tree-sitter-ling

`tree-sitter-ling` is the editor-oriented concrete-syntax parser for Ling. This directory is a standalone-ready development mirror kept in the compiler repository while the grammar and shared corpus evolve together.

The grammar is not an authority for Ling validity or semantics. Accepted RFCs and decisions, the compiler specifications, conformance tests, and `ling-syntax` remain authoritative in that order. A tolerant Tree-sitter parse never makes invalid source valid Ling.

See [KNOWN-DIFFERENCES.md](KNOWN-DIFFERENCES.md) for the exact compiler/grammar differences that remain assigned to later tasks.
The non-normative scanner decision is recorded in [docs/ADR-0001-layout-scanner.md](docs/ADR-0001-layout-scanner.md).
The generated Unicode-token decision is recorded in [docs/ADR-0002-unicode-identifiers.md](docs/ADR-0002-unicode-identifiers.md).

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

## TS-3104 identifier boundary

The identifier token is generated from the repository's checksummed Unicode 17.0.0 `XID_Start` and `XID_Continue` ranges. It adds `_` as a Ling identifier start and uses Tree-sitter's global reserved-word mechanism for every Seed keyword, including the lexically reserved but syntactically deferred `and`. No host or Tree-sitter Unicode property version is consulted during grammar generation.

The shared differential corpus is consumed by both `ling-syntax` and the Tree-sitter integration runner. It covers ASCII, Chinese, NFC-equivalent spellings, combining continuations, supplementary-plane boundaries, `_`, `and`/`and_then`, emoji, and XID-shaped characters rejected by compiler security rules. Those permissive editor parses remain intentional: the compiler emits `L-LEX-0004` with an original UTF-8 byte span and bilingual messages suitable for the future LSP diagnostic adapter.

The stateful external scanner remains limited to DEC-0006 layout, nested comments, and private root-declaration synchronization. It does not duplicate Unicode tables, NFC normalization, general keyword classification, or identifier-security checks. The remaining accuracy work is assigned to:

- shared compiler/Tree-sitter corpus differential testing (`TS-3108`).

Language-specific package bindings and publication metadata remain disabled until an editor consumer requires them; the generated C parser and its private scanner are the committed integration artifacts.

TS-3105 implements Accepted DEC-0017 end to end. The compiler and Tree-sitter consume the same 29-case expression corpus, covering every neighboring precedence pair, reverse parentheses, associativity, signed application arguments, assignment-chain rejection, and textual non-aliases. Tree-sitter uses private precedence layers while preserving the shallow public `binary_expression` CST node; the compiler preserves distinct `BooleanAnd` and `BooleanOr` operators through checked evaluation.

TS-3106 covers the Seed Pattern and Type surface without inventing post-Seed nodes. The compiler and Tree-sitter consume the same 41-case validity corpus for bindings, wildcard, Unit, grouped and tuple patterns, literal patterns, qualified and nested constructors, nonempty record patterns, guards, generic declarations, qualified/applied/product/tuple/function types, and explicit invalid/future forms. At that milestone the grammar corpus contained 37 cases. Bare identifiers remain syntax-neutral until name resolution, and effect-row or borrow author syntax remains an error.

TS-3107 hardens malformed editing states without changing compiler validity. A private scanner boundary retains surrounding root declarations after unclosed strings/records/tuples, missing `=`, `->`, or `with`, partial Chinese identifiers, incomplete pipelines, inconsistent indentation, and incomplete control flow. The permanent suite covers 41 grammar cases, 10 static recovery cases, 9 incremental edits, and 64 fixed-seed mutations; every malformed recovery parse must terminate, remain bounded, expose built-in `ERROR`/`MISSING`, and retain both canary declarations.

## 中文说明

`tree-sitter-ling` 是面向编辑器的 Ling 具体语法解析器。本目录暂作为可独立拆分的开发镜像，与编译器共享演进过程。

Tree-sitter 不决定 Ling 源码是否合法，也不定义语言语义。语言行为仍以 Accepted RFC/decision、编译器规范、conformance tests 和 `ling-syntax` 为准。`TS-3103` 已实现有状态 offside scanner；`TS-3104` 已从校验过哈希的 Unicode 17.0.0 数据生成精确 XID 范围；`TS-3105` 已依据 Accepted DEC-0017 实现 `&&`、`||` 与完整表达式优先级，并通过 29 个共享 compiler/Tree-sitter cases 验证；`TS-3106` 已通过 41 个共享 cases 覆盖 Seed pattern/type 合法性边界；`TS-3107` 已通过 10 个静态、9 个增量和 64 个定种子 mutation cases 加固错误恢复，grammar corpus 现有 41 个 cases。NFC、禁止字符、混合书写系统、Confusable 诊断、pattern 名称角色以及源码合法性仍由编译器权威判定。
