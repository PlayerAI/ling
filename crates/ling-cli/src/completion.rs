use std::fmt::Write as _;

const PROTOCOL: &str = "ling.cli-completion/0.1";
const ROOT_COMMANDS: &[&str] = &[
    "run",
    "check",
    "repl",
    "semantic",
    "audit",
    "query",
    "patch",
    "fmt",
    "init",
    "test",
    "build",
    "project",
    "lsp",
    "completion",
];
const SHELLS: &[&str] = &["bash", "zsh", "fish", "powershell"];
const ROOT_OPTIONS: &[&str] = &["--help", "-h", "--version", "-V"];
const OUTPUT_OPTIONS: &[&str] = &["--format", "--language", "--color", "--quiet", "--verbose"];
const RUN_OPTIONS: &[&str] = &[
    "--format",
    "--language",
    "--color",
    "--quiet",
    "--verbose",
    "--manifest-path",
    "--locked",
    "--offline",
];
const QUERY_OPTIONS: &[&str] = &[
    "--format",
    "--language",
    "--color",
    "--quiet",
    "--verbose",
    "--symbol",
];
const FORMAT_OPTIONS: &[&str] = &[
    "--format",
    "--language",
    "--color",
    "--quiet",
    "--verbose",
    "--check",
    "--stdin-name",
];
const INIT_OPTIONS: &[&str] = &[
    "--format",
    "--language",
    "--color",
    "--quiet",
    "--verbose",
    "--name",
    "--display-name",
];
const BUILD_OPTIONS: &[&str] = &[
    "--format",
    "--language",
    "--color",
    "--quiet",
    "--verbose",
    "--manifest-path",
    "--locked",
    "--offline",
    "--profile",
    "--target",
    "--output",
];
const PROJECT_OPTIONS: &[&str] = &[
    "--format",
    "--language",
    "--color",
    "--quiet",
    "--verbose",
    "--manifest-path",
    "--locked",
];
const REPL_OPTIONS: &[&str] = &[
    "--format",
    "--language",
    "--color",
    "--quiet",
    "--verbose",
    "--capability",
];
const LSP_OPTIONS: &[&str] = &["--stdio"];
const NO_OPTIONS: &[&str] = &[];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Shell {
    Bash,
    Zsh,
    Fish,
    Pwsh,
}

impl Shell {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "bash" => Some(Self::Bash),
            "zsh" => Some(Self::Zsh),
            "fish" => Some(Self::Fish),
            "powershell" => Some(Self::Pwsh),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Bash => "bash",
            Self::Zsh => "zsh",
            Self::Fish => "fish",
            Self::Pwsh => "powershell",
        }
    }
}

pub(crate) fn render(shell: Shell) -> String {
    match shell {
        Shell::Bash => render_bash(),
        Shell::Zsh => render_zsh(),
        Shell::Fish => render_fish(),
        Shell::Pwsh => render_powershell(),
    }
}

fn options_for(command: &str) -> &'static [&'static str] {
    match command {
        "run" | "check" | "test" => RUN_OPTIONS,
        "semantic" | "audit" | "patch" => OUTPUT_OPTIONS,
        "query" => QUERY_OPTIONS,
        "fmt" => FORMAT_OPTIONS,
        "init" => INIT_OPTIONS,
        "build" => BUILD_OPTIONS,
        "project" => PROJECT_OPTIONS,
        "repl" => REPL_OPTIONS,
        "lsp" => LSP_OPTIONS,
        "completion" => NO_OPTIONS,
        _ => NO_OPTIONS,
    }
}

fn render_bash() -> String {
    let mut output = format!(
        "# {PROTOCOL}\n_ling() {{\n  local current previous command\n  COMPREPLY=()\n  current=\"${{COMP_WORDS[COMP_CWORD]}}\"\n  previous=\"${{COMP_WORDS[COMP_CWORD-1]}}\"\n  command=\"${{COMP_WORDS[1]}}\"\n  case \"$previous\" in\n    --format) COMPREPLY=( $(compgen -W 'human json' -- \"$current\") ); return ;;\n    --language) COMPREPLY=( $(compgen -W 'bilingual zh-CN en' -- \"$current\") ); return ;;\n    --color) COMPREPLY=( $(compgen -W 'auto always never' -- \"$current\") ); return ;;\n    --capability) COMPREPLY=( $(compgen -W 'Console.Write' -- \"$current\") ); return ;;\n    --profile) COMPREPLY=( $(compgen -W 'explore' -- \"$current\") ); return ;;\n    --target) COMPREPLY=( $(compgen -W 'semantic' -- \"$current\") ); return ;;\n  esac\n  if (( COMP_CWORD == 1 )); then\n    COMPREPLY=( $(compgen -W '{}' -- \"$current\") )\n    return\n  fi\n  if [[ \"$command\" == project && COMP_CWORD -eq 2 ]]; then\n    COMPREPLY=( $(compgen -W 'check' -- \"$current\") )\n    return\n  fi\n  if [[ \"$command\" == completion && COMP_CWORD -eq 2 ]]; then\n    COMPREPLY=( $(compgen -W '{}' -- \"$current\") )\n    return\n  fi\n  case \"$command\" in\n",
        joined(ROOT_COMMANDS, ROOT_OPTIONS),
        SHELLS.join(" ")
    );
    for command in ROOT_COMMANDS {
        let options = options_for(command);
        if !options.is_empty() {
            writeln!(
                output,
                "    {command}) COMPREPLY=( $(compgen -W '{}' -- \"$current\") ) ;;",
                options.join(" ")
            )
            .expect("writing to String cannot fail");
        }
    }
    output.push_str("  esac\n}\ncomplete -F _ling ling\n");
    output
}

fn render_zsh() -> String {
    let mut output = format!(
        "#compdef ling\n# {PROTOCOL}\n_ling() {{\n  local current previous command\n  current=\"${{words[CURRENT]}}\"\n  previous=\"${{words[CURRENT-1]}}\"\n  command=\"${{words[2]}}\"\n  case \"$previous\" in\n    --format) compadd -- human json; return ;;\n    --language) compadd -- bilingual zh-CN en; return ;;\n    --color) compadd -- auto always never; return ;;\n    --capability) compadd -- Console.Write; return ;;\n    --profile) compadd -- explore; return ;;\n    --target) compadd -- semantic; return ;;\n  esac\n  if (( CURRENT == 2 )); then\n    compadd -- {}\n    return\n  fi\n  if [[ \"$command\" == project && CURRENT -eq 3 ]]; then\n    compadd -- check\n    return\n  fi\n  if [[ \"$command\" == completion && CURRENT -eq 3 ]]; then\n    compadd -- {}\n    return\n  fi\n  case \"$command\" in\n",
        joined(ROOT_COMMANDS, ROOT_OPTIONS),
        SHELLS.join(" ")
    );
    for command in ROOT_COMMANDS {
        let options = options_for(command);
        if !options.is_empty() {
            writeln!(output, "    {command}) compadd -- {} ;;", options.join(" "))
                .expect("writing to String cannot fail");
        }
    }
    output.push_str("  esac\n}\ncompdef _ling ling\n");
    output
}

fn render_fish() -> String {
    let mut output = format!(
        "# {PROTOCOL}\ncomplete -c ling -f\ncomplete -c ling -f -n '__fish_use_subcommand' -a '{}'\ncomplete -c ling -f -n '__fish_use_subcommand' -s h -l help\ncomplete -c ling -f -n '__fish_use_subcommand' -s V -l version\ncomplete -c ling -f -n '__fish_seen_subcommand_from project' -a 'check'\ncomplete -c ling -f -n '__fish_seen_subcommand_from completion' -a '{}'\n",
        ROOT_COMMANDS.join(" "),
        SHELLS.join(" ")
    );
    for command in ROOT_COMMANDS {
        for option in options_for(command) {
            let flag = option.trim_start_matches('-');
            let values = values_for(option);
            write!(
                output,
                "complete -c ling -f -n '__fish_seen_subcommand_from {command}' -l {flag}"
            )
            .expect("writing to String cannot fail");
            if let Some(values) = values {
                write!(output, " -r -a '{}'", values.join(" "))
                    .expect("writing to String cannot fail");
            }
            output.push('\n');
        }
    }
    output
}

fn render_powershell() -> String {
    let mut output = format!(
        "# {PROTOCOL}\nRegister-ArgumentCompleter -Native -CommandName ling -ScriptBlock {{\n  param($wordToComplete, $commandAst, $cursorPosition)\n  $elements = @($commandAst.CommandElements | ForEach-Object {{ $_.Extent.Text }})\n  $command = if ($elements.Count -gt 1) {{ $elements[1] }} else {{ '' }}\n  $previousIndex = $elements.Count - 1\n  if ($wordToComplete.Length -gt 0) {{ $previousIndex-- }}\n  $previous = if ($previousIndex -ge 0) {{ $elements[$previousIndex] }} else {{ '' }}\n  $candidates = switch ($previous) {{\n    '--format' {{ @('human', 'json'); break }}\n    '--language' {{ @('bilingual', 'zh-CN', 'en'); break }}\n    '--color' {{ @('auto', 'always', 'never'); break }}\n    '--capability' {{ @('Console.Write'); break }}\n    '--profile' {{ @('explore'); break }}\n    '--target' {{ @('semantic'); break }}\n    default {{\n      if ($elements.Count -le 2) {{ @({}) }}\n      elseif ($command -eq 'project' -and $elements.Count -le 3) {{ @('check') }}\n      elseif ($command -eq 'completion' -and $elements.Count -le 3) {{ @({}) }}\n      else {{\n        switch ($command) {{\n",
        powershell_array(&joined(ROOT_COMMANDS, ROOT_OPTIONS)),
        powershell_array(&SHELLS.join(" "))
    );
    for command in ROOT_COMMANDS {
        let options = options_for(command);
        if !options.is_empty() {
            writeln!(
                output,
                "          '{command}' {{ @({}) }}",
                powershell_array(&options.join(" "))
            )
            .expect("writing to String cannot fail");
        }
    }
    output.push_str(
        "          default { @() }\n        }\n      }\n    }\n  }\n  $candidates | Where-Object { $_ -like \"$wordToComplete*\" } | ForEach-Object {\n    [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)\n  }\n}\n",
    );
    output
}

fn joined(left: &[&str], right: &[&str]) -> String {
    left.iter()
        .chain(right)
        .copied()
        .collect::<Vec<_>>()
        .join(" ")
}

fn powershell_array(words: &str) -> String {
    words
        .split_ascii_whitespace()
        .map(|word| format!("'{word}'"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn values_for(option: &str) -> Option<&'static [&'static str]> {
    match option {
        "--format" => Some(&["human", "json"]),
        "--language" => Some(&["bilingual", "zh-CN", "en"]),
        "--color" => Some(&["auto", "always", "never"]),
        "--capability" => Some(&["Console.Write"]),
        "--profile" => Some(&["explore"]),
        "--target" => Some(&["semantic"]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_catalog::Command;
    use std::collections::BTreeSet;

    #[test]
    fn completion_roots_match_the_parser_catalog() {
        let parser_roots = Command::all()
            .iter()
            .map(|command| {
                command
                    .name()
                    .split_once(' ')
                    .map_or(command.name(), |pair| pair.0)
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(parser_roots, ROOT_COMMANDS.iter().copied().collect());
    }

    #[test]
    fn every_shell_is_deterministic_and_inventory_complete() {
        for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::Pwsh] {
            let first = render(shell);
            assert_eq!(first, render(shell));
            assert!(first.starts_with(if shell == Shell::Zsh {
                "#compdef ling\n"
            } else {
                "# ling.cli-completion/0.1\n"
            }));
            assert!(first.contains(PROTOCOL));
            assert!(first.ends_with('\n'));
            assert!(!first.contains("zero"));
            for command in ROOT_COMMANDS {
                assert!(first.contains(command), "{} omits {command}", shell.name());
            }
            let root_options = if shell == Shell::Fish {
                &["-s h -l help", "-s V -l version"][..]
            } else {
                ROOT_OPTIONS
            };
            for option in root_options {
                assert!(first.contains(option), "{} omits {option}", shell.name());
            }
            for option in BUILD_OPTIONS {
                let rendered_option = if shell == Shell::Fish {
                    format!("-l {}", option.trim_start_matches('-'))
                } else {
                    (*option).to_owned()
                };
                assert!(
                    first.contains(&rendered_option),
                    "{} omits {option}",
                    shell.name()
                );
            }
        }
    }
}
