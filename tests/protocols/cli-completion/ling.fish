# ling.cli-completion/0.1
complete -c ling -f
complete -c ling -f -n '__fish_use_subcommand' -a 'run check repl semantic audit query patch fmt init test build project lsp completion'
complete -c ling -f -n '__fish_use_subcommand' -s h -l help
complete -c ling -f -n '__fish_use_subcommand' -s V -l version
complete -c ling -f -n '__fish_seen_subcommand_from project' -a 'check'
complete -c ling -f -n '__fish_seen_subcommand_from completion' -a 'bash zsh fish powershell'
complete -c ling -f -n '__fish_seen_subcommand_from run' -l format -r -a 'human json'
complete -c ling -f -n '__fish_seen_subcommand_from run' -l language -r -a 'bilingual zh-CN en'
complete -c ling -f -n '__fish_seen_subcommand_from run' -l color -r -a 'auto always never'
complete -c ling -f -n '__fish_seen_subcommand_from run' -l quiet
complete -c ling -f -n '__fish_seen_subcommand_from run' -l verbose
complete -c ling -f -n '__fish_seen_subcommand_from run' -l manifest-path
complete -c ling -f -n '__fish_seen_subcommand_from run' -l locked
complete -c ling -f -n '__fish_seen_subcommand_from run' -l offline
complete -c ling -f -n '__fish_seen_subcommand_from check' -l format -r -a 'human json'
complete -c ling -f -n '__fish_seen_subcommand_from check' -l language -r -a 'bilingual zh-CN en'
complete -c ling -f -n '__fish_seen_subcommand_from check' -l color -r -a 'auto always never'
complete -c ling -f -n '__fish_seen_subcommand_from check' -l quiet
complete -c ling -f -n '__fish_seen_subcommand_from check' -l verbose
complete -c ling -f -n '__fish_seen_subcommand_from check' -l manifest-path
complete -c ling -f -n '__fish_seen_subcommand_from check' -l locked
complete -c ling -f -n '__fish_seen_subcommand_from check' -l offline
complete -c ling -f -n '__fish_seen_subcommand_from repl' -l format -r -a 'human json'
complete -c ling -f -n '__fish_seen_subcommand_from repl' -l language -r -a 'bilingual zh-CN en'
complete -c ling -f -n '__fish_seen_subcommand_from repl' -l color -r -a 'auto always never'
complete -c ling -f -n '__fish_seen_subcommand_from repl' -l quiet
complete -c ling -f -n '__fish_seen_subcommand_from repl' -l verbose
complete -c ling -f -n '__fish_seen_subcommand_from repl' -l capability -r -a 'Console.Write'
complete -c ling -f -n '__fish_seen_subcommand_from semantic' -l format -r -a 'human json'
complete -c ling -f -n '__fish_seen_subcommand_from semantic' -l language -r -a 'bilingual zh-CN en'
complete -c ling -f -n '__fish_seen_subcommand_from semantic' -l color -r -a 'auto always never'
complete -c ling -f -n '__fish_seen_subcommand_from semantic' -l quiet
complete -c ling -f -n '__fish_seen_subcommand_from semantic' -l verbose
complete -c ling -f -n '__fish_seen_subcommand_from audit' -l format -r -a 'human json'
complete -c ling -f -n '__fish_seen_subcommand_from audit' -l language -r -a 'bilingual zh-CN en'
complete -c ling -f -n '__fish_seen_subcommand_from audit' -l color -r -a 'auto always never'
complete -c ling -f -n '__fish_seen_subcommand_from audit' -l quiet
complete -c ling -f -n '__fish_seen_subcommand_from audit' -l verbose
complete -c ling -f -n '__fish_seen_subcommand_from query' -l format -r -a 'human json'
complete -c ling -f -n '__fish_seen_subcommand_from query' -l language -r -a 'bilingual zh-CN en'
complete -c ling -f -n '__fish_seen_subcommand_from query' -l color -r -a 'auto always never'
complete -c ling -f -n '__fish_seen_subcommand_from query' -l quiet
complete -c ling -f -n '__fish_seen_subcommand_from query' -l verbose
complete -c ling -f -n '__fish_seen_subcommand_from query' -l symbol
complete -c ling -f -n '__fish_seen_subcommand_from patch' -l format -r -a 'human json'
complete -c ling -f -n '__fish_seen_subcommand_from patch' -l language -r -a 'bilingual zh-CN en'
complete -c ling -f -n '__fish_seen_subcommand_from patch' -l color -r -a 'auto always never'
complete -c ling -f -n '__fish_seen_subcommand_from patch' -l quiet
complete -c ling -f -n '__fish_seen_subcommand_from patch' -l verbose
complete -c ling -f -n '__fish_seen_subcommand_from fmt' -l format -r -a 'human json'
complete -c ling -f -n '__fish_seen_subcommand_from fmt' -l language -r -a 'bilingual zh-CN en'
complete -c ling -f -n '__fish_seen_subcommand_from fmt' -l color -r -a 'auto always never'
complete -c ling -f -n '__fish_seen_subcommand_from fmt' -l quiet
complete -c ling -f -n '__fish_seen_subcommand_from fmt' -l verbose
complete -c ling -f -n '__fish_seen_subcommand_from fmt' -l check
complete -c ling -f -n '__fish_seen_subcommand_from fmt' -l stdin-name
complete -c ling -f -n '__fish_seen_subcommand_from init' -l format -r -a 'human json'
complete -c ling -f -n '__fish_seen_subcommand_from init' -l language -r -a 'bilingual zh-CN en'
complete -c ling -f -n '__fish_seen_subcommand_from init' -l color -r -a 'auto always never'
complete -c ling -f -n '__fish_seen_subcommand_from init' -l quiet
complete -c ling -f -n '__fish_seen_subcommand_from init' -l verbose
complete -c ling -f -n '__fish_seen_subcommand_from init' -l name
complete -c ling -f -n '__fish_seen_subcommand_from init' -l display-name
complete -c ling -f -n '__fish_seen_subcommand_from test' -l format -r -a 'human json'
complete -c ling -f -n '__fish_seen_subcommand_from test' -l language -r -a 'bilingual zh-CN en'
complete -c ling -f -n '__fish_seen_subcommand_from test' -l color -r -a 'auto always never'
complete -c ling -f -n '__fish_seen_subcommand_from test' -l quiet
complete -c ling -f -n '__fish_seen_subcommand_from test' -l verbose
complete -c ling -f -n '__fish_seen_subcommand_from test' -l manifest-path
complete -c ling -f -n '__fish_seen_subcommand_from test' -l locked
complete -c ling -f -n '__fish_seen_subcommand_from test' -l offline
complete -c ling -f -n '__fish_seen_subcommand_from build' -l format -r -a 'human json'
complete -c ling -f -n '__fish_seen_subcommand_from build' -l language -r -a 'bilingual zh-CN en'
complete -c ling -f -n '__fish_seen_subcommand_from build' -l color -r -a 'auto always never'
complete -c ling -f -n '__fish_seen_subcommand_from build' -l quiet
complete -c ling -f -n '__fish_seen_subcommand_from build' -l verbose
complete -c ling -f -n '__fish_seen_subcommand_from build' -l manifest-path
complete -c ling -f -n '__fish_seen_subcommand_from build' -l locked
complete -c ling -f -n '__fish_seen_subcommand_from build' -l offline
complete -c ling -f -n '__fish_seen_subcommand_from build' -l profile -r -a 'explore'
complete -c ling -f -n '__fish_seen_subcommand_from build' -l target -r -a 'semantic'
complete -c ling -f -n '__fish_seen_subcommand_from build' -l output
complete -c ling -f -n '__fish_seen_subcommand_from project' -l format -r -a 'human json'
complete -c ling -f -n '__fish_seen_subcommand_from project' -l language -r -a 'bilingual zh-CN en'
complete -c ling -f -n '__fish_seen_subcommand_from project' -l color -r -a 'auto always never'
complete -c ling -f -n '__fish_seen_subcommand_from project' -l quiet
complete -c ling -f -n '__fish_seen_subcommand_from project' -l verbose
complete -c ling -f -n '__fish_seen_subcommand_from project' -l manifest-path
complete -c ling -f -n '__fish_seen_subcommand_from project' -l locked
complete -c ling -f -n '__fish_seen_subcommand_from lsp' -l stdio
