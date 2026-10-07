# Démos rejouables à froid : chaque cible se place sur son tag git.
# `make demo-N` ne lance jamais l'agent tout seul, sauf demo-1 (déterministe).
MANDAT = cargo run -q -p mandat-cli --

.PHONY: ci demo-0 demo-1 demo-2 demo-3 demo-4

ci:
	cargo test --workspace

define goto
	git switch -q --detach $(1)
	git restore -q --source=$(1) --worktree -- ledger
endef

demo-0:
	$(call goto,demo-0-base)
	@echo "Base verte. Lance : make ci"

demo-1:
	$(call goto,demo-1-mandat)
	-$(MANDAT) run FEAT-043 --prompt prompts/0-sans-mandat.md
	$(MANDAT) check FEAT-042

demo-2:
	$(call goto,demo-2-derive)
	@echo "Chemin 1 : claude -p \"$$(cat prompts/1-code-seul.md)\" puis make ci (rouge attendu)"
	@echo "Chemin 2 : claude -p \"$$(cat prompts/2-faire-passer.md)\" (écriture dans features/ refusée par le hook)"

demo-3:
	$(call goto,demo-3-spec-approuvee)
	@echo "Lance : $(MANDAT) run FEAT-042 --prompt prompts/3-implementer.md puis $(MANDAT) report"

demo-4:
	$(call goto,demo-4-budget)
	$(MANDAT) report
	@echo "Lance : $(MANDAT) run FEAT-041 --prompt prompts/3-implementer.md (refus code 3 attendu)"
