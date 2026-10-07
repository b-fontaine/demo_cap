#!/usr/bin/env bash
cd "$(dirname "$0")/../.."
# Test du hook en isolation avec des JSON simules (aucun appel a Claude).
S=.claude/hooks/protect-specs.sh
P=$PWD
run(){ o=$(printf "%s" "$1" | CLAUDE_PROJECT_DIR=$P $S); [ -z "$o" ] && echo allow || jq -r ".hookSpecificOutput.permissionDecision" <<<"$o"; }
fail=0
t(){ r=$(run "$2"); [ "$r" = "$1" ] && s=OK || { s=KO; fail=1; }; echo "$s ($r, attendu $1) : $3"; }
t deny '{"tool_name":"Write","cwd":"'$P'","tool_input":{"file_path":"'$P'/crates/boutique/features/livraison.feature"}}' Write-abs
t deny '{"tool_name":"Edit","cwd":"'$P'","tool_input":{"file_path":"crates/boutique/features/x.feature"}}' Edit-rel
t deny '{"tool_name":"MultiEdit","cwd":"'$P'","tool_input":{"file_path":"mandates/FEAT-042.toml"}}' MultiEdit
t deny '{"tool_name":"Write","cwd":"'$P'","tool_input":{"file_path":"ledger/FEAT-042.jsonl"}}' ledger
t deny '{"tool_name":"Write","cwd":"'$P'/crates","tool_input":{"file_path":"boutique/features/a"}}' cwd-sub
t deny '{"tool_name":"Write","cwd":"'$P'","tool_input":{"file_path":"crates/boutique/src/../features/a"}}' dotdot
t allow '{"tool_name":"Write","cwd":"'$P'","tool_input":{"file_path":"crates/boutique/src/lib.rs"}}' lib
t allow '{"tool_name":"Write","cwd":"'$P'","tool_input":{"file_path":"crates/boutique/tests/bdd.rs"}}' bdd
t allow '{"tool_name":"Write","cwd":"'$P'","tool_input":{"file_path":"crates/mandat-core/src/mandate.rs"}}' mandate.rs
t allow '{"tool_name":"Write","cwd":"'$P'","tool_input":{"file_path":"docs/mandates-notes.md"}}' docs
b(){ j=$(jq -nc --arg c "$2" --arg p "$P" '{tool_name:"Bash",cwd:$p,tool_input:{command:$c}}'); t "$1" "$j" "Bash: $2"; }
b deny 'echo x > crates/boutique/features/livraison.feature'
b deny 'echo x >> mandates/FEAT-042.toml'
b deny 'echo x>ledger/FEAT-042.jsonl'
b deny "sed -i 's/50/40/' crates/boutique/features/livraison.feature"
b deny 'sed -i.bak s/a/b/ mandates/FEAT-042.toml'
b deny 'echo hi | tee ledger/FEAT-042.jsonl'
b deny 'cp /tmp/a crates/boutique/features/livraison.feature'
b deny 'mv x.toml mandates/'
b deny 'rm -rf ./ledger'
b deny 'cd /tmp && rm mandates/FEAT-041.toml'
b deny 'cat foo; rm crates/boutique/features/livraison.feature'
b deny "rm $P/mandates/FEAT-041.toml"
b deny 'git checkout -- mandates/FEAT-041.toml'
b allow 'cat crates/boutique/features/livraison.feature'
b allow 'ls mandates ledger'
b allow 'grep -r LIV crates/boutique/features/ | head'
b allow 'cargo test --workspace 2>&1 | tail -5'
b allow 'cat mandates/FEAT-042.toml > /tmp/copie.toml'
b allow 'echo x > crates/boutique/src/lib.rs'
b allow 'sed -i s/a/b/ crates/mandat-core/src/mandate.rs'
echo '{"tool_name":"Read","tool_input":{"file_path":"mandates/x"}}' | $S; echo "read exit=$?"
echo 'garbage' | $S; echo "garbage exit=$?"
exit $fail
