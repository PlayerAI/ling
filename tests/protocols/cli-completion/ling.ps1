# ling.cli-completion/0.1
Register-ArgumentCompleter -Native -CommandName ling -ScriptBlock {
  param($wordToComplete, $commandAst, $cursorPosition)
  $elements = @($commandAst.CommandElements | ForEach-Object { $_.Extent.Text })
  $command = if ($elements.Count -gt 1) { $elements[1] } else { '' }
  $previousIndex = $elements.Count - 1
  if ($wordToComplete.Length -gt 0) { $previousIndex-- }
  $previous = if ($previousIndex -ge 0) { $elements[$previousIndex] } else { '' }
  $candidates = switch ($previous) {
    '--format' { @('human', 'json'); break }
    '--language' { @('bilingual', 'zh-CN', 'en'); break }
    '--color' { @('auto', 'always', 'never'); break }
    '--capability' { @('Console.Write'); break }
    '--profile' { @('explore'); break }
    '--target' { @('semantic'); break }
    default {
      if ($elements.Count -le 2) { @('run', 'check', 'repl', 'semantic', 'audit', 'query', 'patch', 'fmt', 'init', 'test', 'build', 'project', 'lsp', 'completion', '--help', '-h', '--version', '-V') }
      elseif ($command -eq 'project' -and $elements.Count -le 3) { @('check') }
      elseif ($command -eq 'completion' -and $elements.Count -le 3) { @('bash', 'zsh', 'fish', 'powershell') }
      else {
        switch ($command) {
          'run' { @('--format', '--language', '--color', '--quiet', '--verbose', '--manifest-path', '--locked', '--offline') }
          'check' { @('--format', '--language', '--color', '--quiet', '--verbose', '--manifest-path', '--locked', '--offline') }
          'repl' { @('--format', '--language', '--color', '--quiet', '--verbose', '--capability') }
          'semantic' { @('--format', '--language', '--color', '--quiet', '--verbose') }
          'audit' { @('--format', '--language', '--color', '--quiet', '--verbose') }
          'query' { @('--format', '--language', '--color', '--quiet', '--verbose', '--symbol') }
          'patch' { @('--format', '--language', '--color', '--quiet', '--verbose') }
          'fmt' { @('--format', '--language', '--color', '--quiet', '--verbose', '--check', '--stdin-name') }
          'init' { @('--format', '--language', '--color', '--quiet', '--verbose', '--name', '--display-name') }
          'test' { @('--format', '--language', '--color', '--quiet', '--verbose', '--manifest-path', '--locked', '--offline') }
          'build' { @('--format', '--language', '--color', '--quiet', '--verbose', '--manifest-path', '--locked', '--offline', '--profile', '--target', '--output') }
          'project' { @('--format', '--language', '--color', '--quiet', '--verbose', '--manifest-path', '--locked') }
          'lsp' { @('--stdio') }
          default { @() }
        }
      }
    }
  }
  $candidates | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
    [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
  }
}
