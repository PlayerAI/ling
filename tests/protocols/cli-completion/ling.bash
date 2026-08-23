# ling.cli-completion/0.1
_ling() {
  local current previous command
  COMPREPLY=()
  current="${COMP_WORDS[COMP_CWORD]}"
  previous="${COMP_WORDS[COMP_CWORD-1]}"
  command="${COMP_WORDS[1]}"
  case "$previous" in
    --format) COMPREPLY=( $(compgen -W 'human json' -- "$current") ); return ;;
    --language) COMPREPLY=( $(compgen -W 'bilingual zh-CN en' -- "$current") ); return ;;
    --color) COMPREPLY=( $(compgen -W 'auto always never' -- "$current") ); return ;;
    --capability) COMPREPLY=( $(compgen -W 'Console.Write' -- "$current") ); return ;;
    --profile) COMPREPLY=( $(compgen -W 'explore' -- "$current") ); return ;;
    --target) COMPREPLY=( $(compgen -W 'semantic' -- "$current") ); return ;;
  esac
  if (( COMP_CWORD == 1 )); then
    COMPREPLY=( $(compgen -W 'run check repl semantic audit query patch fmt init test build project lsp completion --help -h --version -V' -- "$current") )
    return
  fi
  if [[ "$command" == project && COMP_CWORD -eq 2 ]]; then
    COMPREPLY=( $(compgen -W 'check' -- "$current") )
    return
  fi
  if [[ "$command" == completion && COMP_CWORD -eq 2 ]]; then
    COMPREPLY=( $(compgen -W 'bash zsh fish powershell' -- "$current") )
    return
  fi
  case "$command" in
    run) COMPREPLY=( $(compgen -W '--format --language --color --quiet --verbose --manifest-path --locked --offline' -- "$current") ) ;;
    check) COMPREPLY=( $(compgen -W '--format --language --color --quiet --verbose --manifest-path --locked --offline' -- "$current") ) ;;
    repl) COMPREPLY=( $(compgen -W '--format --language --color --quiet --verbose --capability' -- "$current") ) ;;
    semantic) COMPREPLY=( $(compgen -W '--format --language --color --quiet --verbose' -- "$current") ) ;;
    audit) COMPREPLY=( $(compgen -W '--format --language --color --quiet --verbose' -- "$current") ) ;;
    query) COMPREPLY=( $(compgen -W '--format --language --color --quiet --verbose --symbol' -- "$current") ) ;;
    patch) COMPREPLY=( $(compgen -W '--format --language --color --quiet --verbose' -- "$current") ) ;;
    fmt) COMPREPLY=( $(compgen -W '--format --language --color --quiet --verbose --check --stdin-name' -- "$current") ) ;;
    init) COMPREPLY=( $(compgen -W '--format --language --color --quiet --verbose --name --display-name' -- "$current") ) ;;
    test) COMPREPLY=( $(compgen -W '--format --language --color --quiet --verbose --manifest-path --locked --offline' -- "$current") ) ;;
    build) COMPREPLY=( $(compgen -W '--format --language --color --quiet --verbose --manifest-path --locked --offline --profile --target --output' -- "$current") ) ;;
    project) COMPREPLY=( $(compgen -W '--format --language --color --quiet --verbose --manifest-path --locked' -- "$current") ) ;;
    lsp) COMPREPLY=( $(compgen -W '--stdio' -- "$current") ) ;;
  esac
}
complete -F _ling ling
