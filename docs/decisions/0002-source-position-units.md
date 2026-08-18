# DEC-0002：Source 位置单位

> 状态：Accepted  
> 日期：2026-08-18  
> 关闭缺口：G-13

## 决议

1. `SourceId + Span` 是唯一权威位置；Span 使用**原始 UTF-8 文件的零基、半开 byte 区间** `[start_byte, end_byte)`。
2. 文件头 BOM 保留在原始 Source 中，但不进入词法视图。第一个 token 的原始 byte offset 位于 BOM 之后。
3. LF、CRLF、CR 在词法视图中统一为 LF。`SourceMap` 必须保存词法 byte 边界到原始 byte 边界的映射；CRLF 中间不是可寻址的词法边界。
4. Human diagnostic 的 line 与 column 均为一基。line 基于规范化换行；column 以该行从开头到目标位置的 **Unicode scalar value 数量**计算，不按 UTF-16 code unit、grapheme cluster 或终端显示宽度计算。
5. JSON Diagnostic 以 `start_byte` / `end_byte` 为协议真值。未来 LSP 若需要 UTF-16 位置，必须从 SourceMap 显式派生并标注编码，不能改变 Span 身份。
6. 位于 UTF-8 scalar 内部或规范化序列内部的 offset 对 line/column 查询无效，必须返回错误，禁止静默取整。

## 理由

原始 byte span 可精确引用输入、支持零拷贝切片和稳定机器协议。Unicode scalar column 比 byte column 更适合 human diagnostic，同时不引入 grapheme/终端宽度表的额外版本依赖。LSP 的 UTF-16 需求是独立投影，不应污染编译器内部身份。

