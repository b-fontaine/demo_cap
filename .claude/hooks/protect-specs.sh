#!/usr/bin/env bash
# Hook PreToolUse (P0) : refuse toute ecriture sous
#   crates/boutique/features/, mandates/, ledger/
# Lit le JSON de l'evenement sur stdin. Sortie : JSON hookSpecificOutput
# avec permissionDecision "deny" (et code 0), ou rien (autorise).
# Confort poste developpeur, pas une autorite : l'autorite est la CI et CODEOWNERS.
set -u

input=$(cat)
tool=$(jq -r '.tool_name // empty' <<<"$input" 2>/dev/null) || tool=""
cwd=$(jq -r '.cwd // empty' <<<"$input" 2>/dev/null)
root="${CLAUDE_PROJECT_DIR:-${cwd:-$PWD}}"
root=$(realpath -m -- "$root")

deny() {
  jq -n --arg reason "$1" '{
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "deny",
      permissionDecisionReason: $reason
    }
  }'
  exit 0
}

MSG="Ecriture refusee sous crates/boutique/features/, mandates/ ou ledger/ : ces fichiers appartiennent au sponsor. Propose le diff de scenario dans ta reponse finale au lieu de le modifier."

# Chemin (fichier) protege ? Normalise (.., ./) avant comparaison.
is_protected_path() {
  local p="$1" abs rel
  [ -z "$p" ] && return 1
  case "$p" in
    /*) abs=$(realpath -m -- "$p") ;;
    *)  abs=$(realpath -m -- "${cwd:-$root}/$p") ;;
  esac
  rel="${abs#"$root"/}"
  case "$rel" in
    crates/boutique/features|crates/boutique/features/*|mandates|mandates/*|ledger|ledger/*) return 0 ;;
  esac
  return 1
}

case "$tool" in
  Edit|Write|MultiEdit|NotebookEdit)
    fp=$(jq -r '.tool_input.file_path // .tool_input.notebook_path // empty' <<<"$input")
    if is_protected_path "$fp"; then deny "$MSG"; fi
    ;;
  Bash)
    cmd=$(jq -r '.tool_input.command // empty' <<<"$input")
    # Chemin protege (relatif, absolu, ./), sans prefixe ni suffixe de nom.
    prot='(\./)?(crates/boutique/features|mandates|ledger)'
    after='(/|[[:space:]"'"'"';|&)*]|$)'
    path_re="(^|[^A-Za-z0-9_.-])${prot}${after}"
    # Redirection dont la CIBLE est protegee (lire un fichier protege reste permis).
    redir_re="(^|[^<0-9&>])>>?[[:space:]]*[\"']?(.*/)?${prot}${after}"
    # Commandes d'ecriture/suppression qui citent un chemin protege.
    verb_re='(^|[^A-Za-z0-9_-])(sed[[:space:]]+(-[A-Za-z]*i[^[:space:]]*|--in-place[^[:space:]]*)|perl[[:space:]]+-[A-Za-z]*i[^[:space:]]*|tee|cp|mv|rm|rmdir|dd|install|truncate|touch|ln|rsync|chmod|chown|patch|git[[:space:]]+(checkout|restore|apply|rm|mv))([[:space:]]|$)'
    # Examine chaque segment (; && || | retour ligne) separement.
    while IFS= read -r seg; do
      if [[ "$seg" =~ $redir_re ]] || { [[ "$seg" =~ $path_re ]] && [[ "$seg" =~ $verb_re ]]; }; then
        deny "$MSG (commande shell detectee)"
      fi
    done < <(sed -E 's/(\|\||&&|[;|])/\n/g' <<<"$cmd")
    ;;
esac

exit 0
