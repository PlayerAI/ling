# DEC-0015：Seed Audit Source 格式与 Round-trip

> 状态：Accepted
> 提出日期：2026-08-18
> 接受日期：2026-08-19
> 关闭缺口：G-12

## 背景

Audit Source 是 Semantic Graph 的确定性、可读文本投影。现有规范要求 round-trip，但没有冻结语法、显示元数据边界或版本迁移规则，因而 `ling audit` 仍必须显式拒绝执行。

## 建议决议

Seed Audit Source 使用版本化、显式块结构：

```text
audit ling.audit/0.1 {
  language = "0.0.1-dev"
  semantic = "ling.semantic/0.1"
  unicode = "17.0.0"
  program = "experimental:blake3:..."
  entry = "Main"

  module "Main" {
    explicit = true
    capabilities = ["Console.Write"]
    definition "experimental:blake3:..." {
      body = "experimental:blake3:..."
      name = "main"
      kind = "value"
      origin = "user"
      type = "Unit -> Unit"
      effects = ["Console.Write"]
      capabilities = ["Console.Write"]
      unicode_source = "main"
      unicode_nfc = "main"
      unicode_skeleton = "rnain"
      unicode_scripts = ["Latn"]
      unicode_suspicious_mixed_script = false
      implementation = "implemented"
    }
    node "experimental:blake3:..." {
      kind = "expression"
      name = "application"
      owner = "experimental:blake3:..."
      type = "Unit"
      ordinal = 0
      effects = []
      capabilities = []
      unicode_scripts = []
      unicode_suspicious_mixed_script = false
      implementation = "implemented"
    }
    reference expression 0 {
      source = "experimental:blake3:..."
      target_kind = "definition"
      target = "experimental:blake3:..."
    }
  }
}
```

具体规则：

- 编码固定为无 BOM UTF-8，换行固定为 LF，缩进固定为两个 ASCII space，文件末尾恰有一个 LF；
- keyword 与标点为 ASCII；所有用户文本使用 JSON string escaping，解析后必须是合法 Unicode scalar sequence；
- module、definition、node、reference、Capability 和 Effect 使用各自规定的 canonical 顺序；`node` 投影 RFC-0001 §6.11 的 Field、Variant、Binding、Function、Parameter、Pattern、Expression、Effect 与 Capability 类别，Module 与 Type 分别由 module/definition 结构承载；
- parser 接受字段顺序变化和未知 `x-*` 扩展字段，renderer 始终输出唯一顺序；未知非扩展字段是版本错误；
- `AuditModel` 只包含当前 Seed 已实现且可验证的语义。Borrow、Contract、Profile 等未来字段不得以伪造默认值出现；
- Source Span、原始路径和注释属于 display metadata，不参与 round-trip 等价；本版本 renderer 默认不输出它们；
- round-trip 等价定义为 `parse(render(model)) == model`；输入文本自身无需字节级保持，重新 render 后必须规范化；
- Audit parser 产出隔离的 `AuditModel`，不得转换为 `CheckedProgram` 或直接交给 evaluator；
- 不兼容 grammar/model 变化升级 `ling.audit/*`；Semantic Schema 变化不自动改变 Audit 版本，但必须验证组合兼容性。

## 诊断与 CLI

- `ling audit <file>` 对 Author Source 执行正常 check，再向 stdout 写 Audit Source；
- 编译诊断沿用根因 code；Audit parser 使用 `L-AUDIT-*`；
- stdout 写失败按宿主 Runtime Fault（`L-RUNTIME-0001`，exit `4`）处理；Semantic reader round-trip mismatch 独立使用 `L-SNAPSHOT-0001` 与 exit `6`；
- `--format` 只控制诊断格式，不改变成功的 Audit Source bytes。

## 验收证据

- `parse_audit(render_audit(model)) = model`；
- 字段输入顺序与 `x-*` 扩展不改变重新 render 的 canonical bytes；
- bad header、重复字段、非法 escape、非法 ID、错误 node kind、悬空或循环 owner、悬空 source/target reference 和未知核心字段被拒；
- 两个独立进程对同一输入输出逐字节相同 Audit Source；
- Audit model 无法进入 evaluator。
