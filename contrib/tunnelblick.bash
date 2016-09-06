_tunnelblickctl() {
  local cur prev

  cur="${COMP_WORDS[COMP_CWORD]}"
  prev="${COMP_WORDS[COMP_CWORD - 1]}"

  case $prev in
    $1)
      if [[ $cur == -* ]]; then
        COMPREPLY=($(compgen -W '--help' -- "${cur}"))
        return
      fi
      local commands
      commands="$("${1}" help | awk 'f;/SUBCOMMANDS/{f=1}' | awk '{ print $1 }')"
      commands="${commands} ls"
      COMPREPLY=($(compgen -W  "${commands}" -- "${cur}"))
      return
      ;;

    *connect)
      local tunnels
      local ifs="${IFS}"
      IFS=$'\n'
      tunnels="$("${1}" list)"
      # Escape spaces in configuration names.
      tunnels="${tunnels// /\\\\ }"
      COMPREPLY=($(compgen -W "${tunnels}" -- "${cur}"))
      IFS=$ifs
      return
      ;;
  esac

} &&
complete -F _tunnelblickctl tunnelblickctl
