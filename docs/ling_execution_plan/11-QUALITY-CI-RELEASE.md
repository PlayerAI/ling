# Ling 跨阶段质量、CI、模糊测试与发布工程计划

> 适用范围：G0～G6 全阶段  
> 目标：让规范一致性、Unicode、安全、确定性、性能和编辑器集成成为持续验证系统，而不是发布前临时补课

## 1. 质量工程总原则

1. 测试按语义层组织，不按 crate 私有实现组织；
2. 每个新能力先写正例、反例和错误码，再实现；
3. Interpreter 是参考执行路径，但自身也必须接受差分和性质测试；
4. Parser、Tree-sitter、Formatter、LSP、VM、Native、设备 backend 共用 corpus，但各自职责不同；
5. 所有不可信输入入口都有 fuzz/size/depth/time limit；
6. clean build、incremental build、并行调度的最终规范产物必须一致；
7. CI 默认锁定依赖并尽可能离线；
8. 性能门禁来自稳定测量，不凭空设置宣传数字；
9. 失败必须分类为 Ling Diagnostic/Fault/Tool Error，不允许“未知 panic”成为正常结果；
10. 发布证据从日常 CI 自动积累，而不是最后手工拼接。

# 2. 测试金字塔

```text
Specification examples / counterexamples
              ↓
Unit tests for pure domain models
              ↓
Conformance by language feature
              ↓
Round-trip / canonicality / property tests
              ↓
Cross-engine differential tests
              ↓
Fuzz / malformed input / security tests
              ↓
Workspace / editor / end-to-end tests
              ↓
Performance / soak / failure injection
              ↓
Release reproducibility / independent verification
```

## 2.1 单元测试

仅用于：

- tokenizer primitive；
- row unifier；
- symbol table；
- place overlap；
- schema encoder；
- line/position conversion；
- deterministic sorting；
- protocol framing。

不能用大量 Rust 单元测试替代语言 conformance。

## 2.2 Conformance

目录建议：

```text
tests/conformance/
├── syntax/
├── resolution/
├── types/
├── effects/
├── capabilities/
├── values/
├── mutable/
├── traits/
├── tasks/
├── actors/
├── ownership/
├── native/
├── kernels/
├── critical/
└── diagnostics/
```

每个 case 包含：

```text
source
expected phase
expected result or error code
minimal related spans
profile/target
feature state
spec/RFC reference
```

## 2.3 Differential

统一 harness：

| 阶段 | 比较路径 |
| --- | --- |
| Seed | Interpreter ↔ expected |
| v0.1 | Interpreter ↔ VM |
| v0.2 | deterministic scheduler/replay ↔ VM/runtime |
| v0.3 | Interpreter ↔ VM ↔ Native |
| v0.4 | CPU scalar ↔ SIMD ↔ GPU/accelerator |
| v0.5 | reference Node ↔ Critical Native；proof/model trace ↔ replay |

差异 registry 采用机器可读文件：

```toml
[[allowed_difference]]
id = "FP_NAN_PAYLOAD_TARGET"
profiles = ["Native"]
conditions = ["numeric_mode=relaxed"]
```

禁止在测试代码中散落没有说明的 backend 例外。

# 3. Corpus 管理

## 3.1 单一语法 Corpus

维护 `tests/source-corpus/`，由：

- compiler parser；
- Tree-sitter grammar；
- formatter；
- LSP document model；
- Zed extension tests

共同消费。

Tree-sitter 可以为了 error recovery 接受不完整 CST，但不能把非法源码当成成功的 Ling `check`。

## 3.2 Unicode Corpus

固定覆盖：

- 中文、日文、韩文、阿拉伯文、天城文等 XID；
- NFC/NFD 等价输入；
- emoji 在标识符之前对 LSP UTF-16 position 的影响；
- supplementary plane；
- combining marks；
- confusable Latin/Cyrillic；
- bidi controls；
- zero-width/hidden characters；
- CRLF/LF；
- BOM；
- invalid UTF-8（文件读取层）；
- Windows/Unix path；
- 中文文件、模块、package、field、diagnostic。

固定不变量：compiler 内部 span 始终使用原始 UTF-8 byte offsets，只有 protocol adapter 转换到 LSP UTF-16 line/character。

## 3.3 Historical Corpus

每个 release tag 将代表性 source、diagnostic、schema、bytecode、replay、evidence 拷贝/引用到不可变历史 corpus，供 1.0 compatibility testing。

# 4. Round-trip 与 Canonicality

必须持续验证：

```text
parse(format(parse(source))) semantic-equivalent
fmt(fmt(source)) == fmt(source)
encode(decode(canonical)) == canonical
parse(render(AuditGraph)) == same semantic graph
clean build artifact == incremental artifact
serial build artifact == randomized parallel build artifact
```

Canonical 输出不得包含：

- HashMap 随机顺序；
- host absolute path；
- allocation address；
- thread id；
- wall-clock timestamp（除非非 canonical metadata）；
- Rust `Debug` 文本；
- locale-dependent format。

# 5. Fuzz 计划

## 5.1 Harness 注册表

创建 `fuzz/registry.toml`：

```toml
[[harness]]
id = "parser_utf8"
owner = "compiler-syntax"
input = "bytes"
max_input = 1048576
expected = "no_panic_bounded_time"
```

每个 harness 记录：

- 输入模型；
- corpus/dictionary；
- 最大输入；
- timeout；
- memory limit；
- sanitizer；
- expected invariant；
- triage owner；
- last known coverage。

## 5.2 必须 Fuzz 的入口

### G0/G1

- UTF-8 reader/normalization/security scanner；
- lexer/parser；
- Tree-sitter external scanner（若有）；
- formatter；
- diagnostic JSON；
- Semantic Graph reader；
- bytecode verifier；
- package manifest/lock；
- LSP JSON-RPC/framing/document edits。

### G2

- Effect handler nesting；
- task cancellation schedules；
- actor mailbox/turn；
- replay log decoder；
- remote protocol/message decoder。

### G3

- ownership Core generator；
- Native IR verifier；
- ABI metadata；
- FFI shim；
- GC object graph；
- debug/source map。

### G4

- Kernel Core/Device IR verifier；
- shape/index；
- device metadata/cache；
- vendor diagnostic mapper。

### G5/G6

- profile/evidence/proof decoder；
- proof checker；
- model checker input；
- package archive；
- migration transaction；
- Zed LS binary manifest/download metadata。

## 5.3 Fuzz 退出标准

“运行若干小时未崩”不是语义正确证明，但每个 Stable decoder/verifier 至少应满足：

- 无 host panic/UB；
- 输入限制生效；
- deterministic failure；
- stable error category；
- no path traversal/arbitrary execution；
- regression corpus 保存所有修复 crash。

# 6. Property / Model-based Testing

推荐性质：

- type substitution 保持 well-formedness；
- row unification 对称/幂等（在定义范围）；
- rename 后 symbol graph 等价；
- formatter 幂等；
- Semantic ID 对非语义变化稳定；
- bytecode verifier 接受 compiler 产生的全部合法 bytecode；
- borrow checker 不接受构造的 alias violation；
- Resource 恰好 drop 一次；
- bounded mailbox 永不超容量；
- replay 同输入重现同可观察结果；
- CPU/GPU 结果符合 numeric class；
- Evidence verifier 不执行 bundle 内容。

# 7. CI 拓扑

## 7.1 Pull Request 快速流水线

建议目标：尽量控制反馈时间，但不写死未测量分钟数。

```text
format/lint
→ compile workspace
→ changed-crate unit
→ relevant conformance
→ schema/canonical checks
→ no-op incremental check
→ LSP protocol fixtures
→ tree-sitter corpus/query tests
→ docs/link/task registry validation
```

## 7.2 合并后完整流水线

```text
all conformance
all engines differential
all profiles negative tests
cross-platform Tier 1
fuzz smoke corpus
property tests
Zed extension build wasm32-wasip2
clean/offline/locked build
reproducibility sample
performance smoke
```

## 7.3 Nightly/周期性

- long fuzz；
- sanitizer/Miri（适用部分）；
- stress/soak；
- randomized scheduler；
- failure injection；
- all hardware/device runners；
- large workspace/LSP benchmark；
- dependency/license/SBOM；
- reproducible build full comparison；
- stale RFC/traceability/status audit。

## 7.4 Release Candidate

- clean tag build；
- full support matrix；
- protocol old/new/corruption suite；
- historical corpus；
- security audit；
- independent evidence verify；
- binary install/upgrade/uninstall；
- Zed extension dev and packaged install；
- offline sample project；
- checksums/SBOM/provenance。

# 8. 平台矩阵

在 G0 定义并持续更新：

```text
Host Tier 1/Tier 2
Native Target Tier 1/Tier 2
Zed host platforms
GPU/accelerator combinations
Critical target profiles
```

每个 Tier 声明：

- CI 频率；
- blocker policy；
- artifact availability；
- support period；
- known limitations；
- required toolchain。

未被 CI 覆盖的平台不得只因“理论可编译”标为稳定。

# 9. Determinism 与可重复构建

## 9.1 测试模式

每次至少比较：

- clean × 2；
- incremental；
- randomized query scheduling；
- different absolute checkout path；
- different locale/timezone；
- offline；
- parallelism 1 与 N。

## 9.2 产物层级

分别判断：

```text
Semantic equivalence
Canonical metadata equality
Byte-identical bytecode
Byte-identical object/binary
Evidence identity equality
```

不能把 semantic equivalent 误报成 byte-identical。

## 9.3 非确定输入清单

所有已知非确定输入进入 build manifest：

- compiler/toolchain；
- linker；
- target feature；
- driver/backend；
- environment whitelist；
- source date epoch；
- build path mapping；
- dependency lock；
- random seed；
- profile/config。

# 10. 性能工程

## 10.1 先建立测量系统

目录：

```text
benchmarks/
├── compiler/
├── lsp/
├── runtime/
├── actor/
├── native/
├── kernel/
├── critical/
└── zed/
```

结果记录：

```text
commit/toolchain
hardware/OS
cold/warm
sample size
median/p95/p99
variance
memory/allocations
profile/target
```

## 10.2 编译器指标

- lex/parse/resolve/type/effect；
- Semantic Graph build；
- bytecode/native codegen；
- cold build；
- no-op warm；
- one private body edit；
- one public API edit；
- cache hit/miss/corruption recovery；
- peak RSS。

## 10.3 LSP/Zed 指标

- open workspace；
- initial diagnostics；
- keystroke-to-diagnostic；
- hover/definition/references；
- rename；
- completion first/total；
- semantic tokens full/delta；
- cancellation/stale request；
- 10K/100K/1M line synthetic workspace；
- Tree-sitter parse after local edit。

## 10.4 Runtime 指标

- VM startup；
- function/ADT/match；
- task spawn/join；
- actor send/turn；
- mailbox pressure；
- replay overhead；
- Native compile/runtime；
- GC pause/throughput；
- kernel transfer/launch；
- evidence generation。

回归阈值在有足够历史后再设，并区分 noise 与真实 regression。

# 11. 安全工程

## 11.1 Threat Model

至少覆盖：

- 恶意 Ling 源码；
- 恶意 package/manifest/archive；
- 恶意 bytecode/IR/schema/replay/evidence；
- 恶意 LSP client/workspace；
- 恶意 remote actor peer；
- 恶意/失效 device driver；
- 恶意 FFI library；
- 被篡改的 Zed language server 下载；
- Unicode spoofing；
- resource exhaustion。

## 11.2 Sandboxing

- compiler/checker 不需要网络；
- package build steps 只获得声明 Capability；
- LSP 不执行 workspace 任意脚本；
- formatter/parser 在资源限制下运行；
- proof/evidence verifier 不执行输入；
- Zed extension 下载后先验证再执行；
- test runners 使用隔离目录和 resource limits。

## 11.3 Secrets/Privacy

- replay 默认不记录 secret/PII；
- diagnostic 不泄露文件外内容；
- build manifest 环境变量 whitelist；
- crash report opt-in/redaction；
- AI provenance 不包含不必要私密 prompt；
- remote logs 具有 retention/redaction policy。

# 12. Diagnostic 质量门禁

每个 public error：

- stable code；
- Chinese + English message；
- primary UTF-8 byte span；
- minimal related spans；
- category/phase；
- machine-readable payload；
- actionable suggestion when safe；
- no raw Rust debug/panic；
- deterministic ordering；
- LSP mapping fixture；
- CLI human/golden fixture。

Error code 注册在单一 registry，Codex 不得随意复用或改义。

# 13. Traceability 自动检查

CI 脚本验证：

```text
Task ID exists
RFC/decision exists and status valid
spec section link resolves
implementation path exists
conformance test ids exist
error codes registered
protocol version updated when needed
status/support matrix changed
```

脚本只能验证链接完整性；语义覆盖仍需 reviewer。

# 14. PR 统一质量模板

每个 PR 必答：

- 覆盖的 Task/RFC/spec；
- 是否发现规范缺口；
- 修改了哪些编译管线层；
- 新增正例/反例/property/fuzz/differential；
- Diagnostic/Schema/Semantic ID/CLI/ABI 影响；
- Determinism/Unicode/position 影响；
- 性能数据或为何无需；
- 安全/Capability/TCB 影响；
- 明确非目标；
- 可重复验收命令。

模板见 `templates/PR-DESCRIPTION.md`。

# 15. 发布故障等级

| 等级 | 示例 | 处理 |
| --- | --- | --- |
| P0 | 任意代码执行、数据损坏、错误证明/证据接受 | 立即阻断发布/安全响应 |
| P1 | 错误编译、内存不安全、稳定协议破坏、Critical 错误结论 | 阻断 release |
| P2 | 错误诊断、LSP 重命名破坏源码、显著回归 | 通常阻断 RC，按风险决定 |
| P3 | 文档、次要 UX、非稳定功能问题 | 可排入后续 |

未知 host panic 初始按至少 P1/P2 triage，直到分类。

# 16. 完成门禁

- [ ] 所有 Stable parser/verifier/decoder 有 fuzz 与输入限制；
- [ ] 所有 Stable 语义有正反 conformance；
- [ ] 多执行引擎有统一 differential harness；
- [ ] formatter/schema/audit 有 round-trip/canonical tests；
- [ ] clean/incremental/parallel 构建一致；
- [ ] Unicode/LSP position corpus 完整；
- [ ] CI 包含 PR/merge/nightly/RC 分层；
- [ ] performance baseline 可复现；
- [ ] threat model/sandbox/secret policy 已审查；
- [ ] Zed extension 与 LSP 进入同一发布矩阵；
- [ ] release artifact 可从 tag/lock/toolchain 重建并验证。
