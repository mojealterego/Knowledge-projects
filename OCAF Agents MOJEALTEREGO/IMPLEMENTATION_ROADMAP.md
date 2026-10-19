# Roadmapa wykonawcza reorganizacji i budowy wiedzy — nie zastępuje OCAF ROADMAP.md

Nie renumerować ani nie modyfikować kanonicznej [Roadmapy OCAF](https://github.com/mojealterego/ocaf-runtime/blob/main/ROADMAP.md) przy prowadzeniu tej odrębnej pracy wiedzoznawczej.

| Zadanie | Wynik i kryterium odbioru | Zależność | Status |
|---|---|---|---|
| KA-001 | Snapshot wszystkich ścieżek obu repozytoriów i SHA | Dostęp GitHub | DONE — drzewa pobrane |
| KA-002 | Audyt zawartości, historii i zależności 215 plików `projekty/` oraz narzędzi | KA-001 | PARTIAL — struktura tylko |
| KA-003 | Macierz: specyfikacja projektu vs pojedynczy agent / system / rój | KA-002 | PLANNED |
| KA-004 | Reorganizacja przez `git mv` tylko dla niezależnych, przetestowanych granic | KA-003 | BLOCKED until dependency proof |
| KA-005 | Aktualizacja importów, odnośników i regresyjne CI po przeniesieniu | KA-004 | NOT RUN |
| KA-006 | Pełny audyt Rust OCAF względem manifestów agentów i T01–T15 | KA-001, dostępne logi i hosty | PLANNED — read-only reference |
| KA-007 | Inwentaryzacja oryginalnego korpusu OMAt, zachowanie numerów i SHA | Dostarczenie źródeł | BLOCKED |
| KA-008 | Bezpieczne porty Python→Rust i kontrakty OMAs/Zenoh wg potrzeb | KA-007, testy gold | PLANNED |
| KA-009 | Dalsze paczki PDF: page ledger, dedup, mapowanie do projektów i testy | Dostarczenie paczek | CONTINUOUS PER BATCH |
| KA-010 | Certyfikacja/specyfikacja handlowa dopiero po dowodach bezpieczeństwa i legal review | KA-006, zależne dowody | NOT APPROVED |

**Definition of Done dla reorganizacji:** 100% zasobów sklasyfikowanych z dowodem, wszystkie migracje śledzalne, brak uszkodzonych importów/linków, PASS istniejącego CI i dodatkowych testów zależności, rejestry z SHA i stanem produkcyjnym uzasadnionym dowodami. Bez tego nie deklarować zamknięcia Etapu 1.


## Stan po CI agenta 001 — 2026-10-19

- Pierwszy egzemplarz 001 wydzielony z P116, historia blobów zachowana (8/8).
- [Rust/OCAF CI PASS — run 39198615847](https://github.com/mojealterego/Knowledge-projects/actions/runs/39198615847), SHA `d8a03875b4ac7b3c7e4b3dd3361306541a440a` — 6 testów PASS + fmt/clippy + Node syntax.
- **T03–T15 niekompletne.** Brak pełnego portu obrazów i API Adobe do Rust, brak formalnego certyfikatu oraz produkcyjnego OMAs/OSTc. Pozostałe 215 źródeł w `projekty/` sklasyfikowane strukturalnie; audyt semantyczny wszystkich jeszcze otwarty.


### Korekta struktury — 19.10.2026

- KA-014: wymagana struktura `OCAF-AGENT-NNN/{Cargo.toml,src,tests,config,docs,certification}` — zapis i CI do weryfikacji.
- KA-015: OMAs jako serwer agentów, OMAt jako serwer narzędzi; moduł Zenoh komunikuje się przez zewnętrzny zaufany gateway, nie zastępuje tych usług. Obecnie kontrakty trasowania i adapter plan-only (lokalne), **brak produkcyjnych sesji OMAs/OMAt**.
- KA-016: każdy agent ma mieć zaszyfrowany dostęp do źródeł przez OSTc; wykonanie odłożone do dostarczenia pełnej dokumentacji i audytu; obecnie `source_access.enabled=false`, testy fail-closed.
