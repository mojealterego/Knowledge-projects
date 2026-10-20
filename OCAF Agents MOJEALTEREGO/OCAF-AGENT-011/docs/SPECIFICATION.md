# Charakterystyka techniczna — LIMES GIT

**Nazwa:** LIMES GIT · **Ewidencja:** 011 · **Serial:** `AM-GIT-LIM-2026-000011` · **Agent ID:** `AM-OCAF-AI-0011` · **Katalog:** `AM-OCAF-GIT-011` · **Certyfikat:** `CERT-AM-2026-0011` — NIEWYDANY.

**Cel:** Lokalne badanie operacji GitHub przed dopuszczeniem, z rozdzieleniem operacji odczytu od zapisu i rygorystycznym ograniczeniem zapisów do Knowledge-projects.

**Narzędzie:** `github.scope_review`. Wyniki przeznaczone do oceny przez operatora, nie do wywołania bezpośrednich skutków ubocznych.

Wejście `GitOperation`: `repository_full_name`, `operation`, `target_branch`, `pull_base`, `pull_head`. Odczyt z repo zewnętrznego dopuszczony dla rozpoznanych operacji; zapis wyłącznie w Knowledge-projects na roboczą gałąź, PR head na roboczej gałęzi i base=main. Wynik `ScopeVerdict` zawiera `permitted`, `reasons[]`, `execution_authorized=false`, `api_called=false`. Niczego nie aktualizuje.

**Zachowanie:** Odczyt innych repozytoriów jest dopuszczony; zapis wyłącznie do mojealterego/Knowledge-projects przez roboczą gałąź knowledge/, policy/, feature/ lub fix/. PR i merge wymagają pull_base=main i poprawnego pull_head. Bezpośredni zapis do main jest blokowany.

**Granice:** Wynik permitted=true oznacza jedynie lokalny preflight. Agent nie wykonuje połączeń GitHub, nie ma tokenu, nie podpisuje merge, nie autoryzuje wywołania; execution_authorized=false i api_called=false.

Niezależny audyt, podpisy, OSTc, rzeczywiste sieciowe OMAs/OMAt/Zenoh, multi-host i operator acceptance pozostają niewykonane. Cena i wycena brak.
