# Korekta zakresu repozytoriów — 2026-10-09

**Obowiązująca decyzja użytkownika:** do wszystkich paczek źródeł i linków jedynym docelowym repozytorium zapisu jest **`mojealterego/Knowledge-projects`**.

## Błąd w partii 20

Mimo polecenia rozwijania jednego repozytorium agent niepotrzebnie wykonał zapis także w `mojealterego/ODYN-AI` — [PR #17](https://github.com/mojealterego/ODYN-AI/pull/17), scalony do `codex/termux-five-goals`. Wcześniejszy [PR #18 w Knowledge-projects](https://github.com/mojealterego/Knowledge-projects/pull/18) był zgodny z wyborem docelowego repozytorium; dodatkowy PR w ODYN-AI **nie był zgodny**.

Nie zmieniamy historii ani nie przedstawiamy tego drugiego PR jako autoryzowanego wdrożenia. **Nie wykonujemy kolejnych operacji zapisu w ODYN-AI, także automatycznego cofania zmian**; ewentualny rollback wymaga nowego, jednoznacznego zlecenia użytkownika dotyczącego tego repo.

## Nowa reguła

`ODYN-AI`, repozytoria `NousResearch` i wszystkie inne otrzymywane strony/źródła mogą być analizowane **tylko do odczytu**. Cała synteza, nowe projekty i demonstracyjne implementacje trafiają wyłącznie do `Knowledge-projects`.

W repo dodano [główną instrukcję agentów](../AGENTS.md) i [offline gate zakresu](../tools/single_repository_scope_gate.py) z testami. Jest to kontrola deklaratywna/orchestratora, **nie serwerowa blokada uprawnień GitHub**. Prawdziwa blokada platformowa wymaga osobnych uprawnień i ochrony gałęzi; dopiero one uniemożliwią narzędziu dostęp do innych repozytoriów.

Poprzednia dokumentacja partii 20 pozostaje śladem faktycznego przebiegu. Jej sekcja „Dwa repozytoria docelowe” należy czytać jako historię **błędu**, nie nową politykę lub szablon kolejnych prac.
