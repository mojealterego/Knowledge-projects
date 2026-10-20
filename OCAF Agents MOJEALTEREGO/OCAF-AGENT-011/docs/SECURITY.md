# Model zagrożeń — LIMES GIT

**Nazwa:** LIMES GIT · **Ewidencja:** 011 · **Serial:** `AM-GIT-LIM-2026-000011` · **Agent ID:** `AM-OCAF-AI-0011` · **Katalog:** `AM-OCAF-GIT-011` · **Certyfikat:** `CERT-AM-2026-0011` — NIEWYDANY.

**Najważniejsze:** Wynik permitted=true oznacza jedynie lokalny preflight. Agent nie wykonuje połączeń GitHub, nie ma tokenu, nie podpisuje merge, nie autoryzuje wywołania; execution_authorized=false i api_called=false.

| Ryzyko | Kontrola obecna | Bramka docelowa |
|---|---|---|
| Przedstawianie treści modelu jako dowodu lub zgody | wynik lokalny nie oznacza niezależnej weryfikacji/wykonania | podpisany issuer i readback |
| Błędny numer, nazwa lub tenant | Rust constants i TOML fail closed; test rejestru | kryptograficzna tożsamość RPC |
| JSON injection / unknown fields | serde deny_unknown_fields, rozmiar ograniczony | niezależny fuzzing i authz |
| Obejście granicy narzędzia | brak uprawnień do modyfikacji i sieci | OMAt capability scoped grants |
| Sieciowy podszywający się agent | wyłączony transport, lokalny test AuthRequired | prawdziwy Zenoh/TLS/mTLS |
| Odczyt źródeł bez OSTc | trwała odmowa, żadnych fikcyjnych kluczy | implementacja i audyt OSTc |
| Fałszywy certyfikat | `certificate_issued=false`, `Active` DENIED | signed release i operator acceptance |
| Nieukończony produkt opisany jako pełny | jawny zakres lokalny i niewydany certyfikat | formalny audyt wdrożenia |

## Scenariusze specyficzne

Odczyt innych repozytoriów jest dopuszczony; zapis wyłącznie do mojealterego/Knowledge-projects przez roboczą gałąź knowledge/, policy/, feature/ lub fix/. PR i merge wymagają pull_base=main i poprawnego pull_head. Bezpośredni zapis do main jest blokowany. Rzeczywiste przechwytywanie wszystkich zapisów GitHub, integracja z uprawnieniami i tokenem o ograniczonym zakresie, weryfikacja stanu PR/branch z API, polityka podpisów i operator acceptance.

Wersja developerska nie ma poświadczeń i nie może być wdrażana jako kontrola egzekwująca operacje zewnętrzne.
