# CLI completion 0.1 fixtures / CLI 补全 0.1 夹具

This directory contains the exact BOM-free UTF-8/LF output of
`ling completion <shell>` for protocol `ling.cli-completion/0.1`:

- `ling.bash` for Bash;
- `_ling` for Zsh;
- `ling.fish` for Fish;
- `ling.ps1` for PowerShell.

本目录保存 `ling completion <shell>` 在 `ling.cli-completion/0.1` 下的精确
BOM-free UTF-8/LF 输出，分别覆盖 Bash、Zsh、Fish 与 PowerShell。

The process fixture regenerates every script, compares its bytes with these
files, verifies repeatability and invalid usage, and asks each shell available
on the test host to parse its corresponding fixture. Absence of an optional
shell does not fail the cross-platform test; release evidence records which
parsers were actually available.

进程测试会重新生成所有脚本并逐字节比较，验证重复生成与非法用法；测试主机上存在
的 shell 还必须成功解析相应夹具。缺少可选 shell 不会使跨平台测试失败，发布证据
应记录实际执行过的解析器。

These scripts complete only the accepted static command, option, subcommand,
and enumerated-value inventory. They do not scan the filesystem, inspect a
project, read environment configuration, contact a network service, or promise
ordinary help-text byte compatibility.

这些脚本仅补全已接受的静态命令、选项、子命令和枚举值；它们不会扫描文件系统、
检查项目、读取环境配置、访问网络，也不承诺普通帮助文本的字节兼容性。
