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


### Weryfikacja KA-014 po korekcie

Uruchomienie [39203233792](https://github.com/mojealterego/Knowledge-projects/actions/runs/39203233792) dla `9498d7173c378be8d3b4ddf7d4cbd37d35cf5b56`: **SUCCESS**, 12 Rust tests PASS, fmt/clippy PASS, Node syntax i CLI smoke PASS. Zapis nie weryfikuje operacyjnej integracji OMAs/OMAt ani OSTc. KA-014 = `IMPLEMENTED_AND_TESTED_LOCAL` (do readback na main), KA-015 = `PARTIAL`, KA-016 = `BLOCKED_BY_OSTC_DOCUMENTATION`.


### KA-017–KA-020 — tożsamość i kompletność agenta 001

| Zadanie | Wymóg | Status |
|---|---|---|
| KA-017 | Każdy agent otrzymuje unikatową nazwę własną, odrębną od `OCAF-AGENT-NNN`, i bezkolizyjny w rejestrze numer seryjny | IMPLEMENTED FOR 001 / CI TO VERIFY |
| KA-018 | Centralna ewidencja `AGENT_IDENTITY_REGISTRY.json`, walidacja nazw/seriali/ID, synchronizacja manifest/TOML/certyfikat | IMPLEMENTED FOR 001 / CI TO VERIFY |
| KA-019 | Szczegółowe dokumenty architektury, API, serwisu, bezpieczeństwa, zależności, testów, wydania, umów i certyfikacji | DOCUMENTED FOR CURRENT DEV SCOPE |
| KA-020 | Pełny port P116 + serwery OMAs/OMAt, transport Zenoh, OSTc, SBOM, formalny certyfikat, akceptacja operatora | INCOMPLETE / OSTc BLOCKED BY SOURCE MATERIAL |


## Prace wykonawcze 20.10.2026 — agenci 002 i 003

| ID | Zadanie | Stan |
|---|---|---|
| KA-021 | Audyt P121: InfraSentinel jest projektem/systemem, nie gotowym kodem agenta. Wyodrębnij samodzielny read-only SRE triage. | SOURCE VERIFIED; NEW AGENT 002 IN DEVELOPMENT |
| KA-022 | Audyt P15: Universal Research Orchestrator to specyfikacja systemu; wyodrębnij niezależny offline agent kontroli konfliktu źródeł. | SOURCE VERIFIED; NEW AGENT 003 IN DEVELOPMENT |
| KA-023 | Unikatowe nazwy, seriale, katalogi, konfiguracje, podpisane zgodnie z rejestrem (bez fikcyjnych certyfikatów). | DRAFT REGISTRY / CI TO VERIFY |
| KA-024 | Rust `cargo test --all-features`, CLI smoke i lokalny OCAF ToolExecutor dla agentów 002–003 | CI TO VERIFY ON PR #29 |
| KA-025 | Port pełnych funkcji i rzeczywiste OMAs + OMAt + Zenoh + OSTc | NOT IMPLEMENTED / OSTc SOURCE DOCUMENTATION PENDING |
| KA-026 | Kontynuować semantyczną klasyfikację pozostałych projektów i rozdzielenie prawdziwych agentów od projektów/systemów | IN PROGRESS — 100% NIEOSIĄGNIĘTE |

**Nie uznawać KA-021/022 za pełną implementację odpowiednich projektów P121 i P15.** Każdy z nich jest większą platformą; nowa domenowa funkcja to fragment wymagający osobnego testowania.


### Dowód testów 002–003 — 20.10.2026

[GitHub Actions run 39337503859](https://github.com/mojealterego/Knowledge-projects/actions/runs/39337503859) na commit `22133d7ede2125425cb48ceadf2ffbbe1a3bb1d7`: **SUCCESS na obu jobach**; 002 i 003 uzyskały po 12 PASS, fmt PASS, Clippy PASS i CLI smoke PASS. KA-023 (registry consistency) i KA-024 (lokalne testy Rust/OCAF) **TESTED LOCAL**; KA-025 **INCOMPLETE**, KA-026 **IN PROGRESS**. Nie zastępuje testów OMAs/OMAt/OSTc ani audytu całego repo.

## Kontynuacja 20.10.2026 — agenci 004–005

| ID | Zadanie i kryterium | Stan |
|---|---|---|
| KA-027 | Z P115 wydzielić samodzielny walidator grafów: zależności, cykle, brakujące węzły, deterministyczny porządek, brak side effects. | AGENT 004 IMPLEMENTED LOCAL / 13 TESTS PASS |
| KA-028 | Z P119 wydzielić bezpieczny moduł oceny propozycji akcji UI; skutki uboczne zabronione, operator grant nie jest autoryzacją sieciową. | AGENT 005 IMPLEMENTED LOCAL / 13 TESTS PASS |
| KA-029 | Unikatowe nazwy, numery seryjne, katalogowe, manifest, TOML, testy kolizji globalnej. | REGISTERED DEVELOPMENT / CI PASS |
| KA-030 | Testy Rust i lokalnego OCAF `graph.review`, `ui.proposal_assess`, odmowy i read-only invariants. | CI PASS / PR #30 / RUN 39340442615 |
| KA-031 | Dokumentacja specyfikacji, instrukcji, lifecycle, bezpieczeństwa, testów, certyfikatów i 3 odrębnych umów. | DOCUMENTED DEVELOPMENT SCOPE |
| KA-032 | Kompletny workflow P115 oraz rzeczywisty Android P119, integracja OMAs/OMAt+Zenoh i OSTc. | NOT IMPLEMENTED / OSTc BLOCKED |
| KA-033 | Kontynuować audyt semantyczny pozostałych projektów, narzędzi i agentów z źródłowym SHA oraz macierzą zależności. | IN PROGRESS / NOT COMPLETE |

**Zasada postępu:** nie naliczać pełnego agenta produkcyjnego za lokalny wycinek. Zawsze zachować jawnie nieukończone funkcje, brak certyfikacji i brak formalnego operator acceptance. Przydzielanie numeru ewidencyjnego nie kończy audytu całego repo.


### Faktyczny dowód CI 004–005

[GitHub Actions run 39340442615](https://github.com/mojealterego/Knowledge-projects/actions/runs/39340442615), commit `b1eae89e9f634a743adbccf291bae7782a08901a`: **SUCCESS na obu jobach**, 004 i 005 po **13/13 testów PASS** (łącznie 26), fmt, Clippy i CLI smoke PASS. [Regresja rejestru agentów 002–003](https://github.com/mojealterego/Knowledge-projects/actions/runs/39340442555) na tym samym commit również **SUCCESS**. Dowodzi tylko kodu lokalnego, nie produkcyjnego OMAs/OMAt, Android UI, OSTc lub ukończenia P115/P119.

## KA-034–KA-040 — rzeczywisty port Python → Rust (20.10.2026)

| ID | Zadanie | Stan |
|---|---|---|
| KA-034 | Zaewidencjonować pierwowzory `tools/agent_tool_admission_gate.py` i `tools/agent_tool_loop_guard.py` wraz z ich testami i kontraktami | SOURCE READ / REVIEWED |
| KA-035 | Agent 006 CUSTOS ADMISSION: SHA-256, niezależna zgoda spoza modelu, allowlista, expiry, brak side effects | IMPLEMENTED LOCAL / CI VERIFIED 39370400084 |
| KA-036 | Agent 007 VIGIA LOOP: limity budżetu, repeated calls bez postępu, trwały halt, host-owned progress | IMPLEMENTED LOCAL / CI VERIFIED 39370400084 |
| KA-037 | Dwa odrębne foldery Rust, nazwy, seriale, ID, cert records, 3 umowy każdy | DOCUMENTED DEVELOPMENT SCOPE |
| KA-038 | Przetestować Rust 006/007 i pierwotne Python unittest, wersja + SHA + CI | RUST 32/32 PASS + PYTHON 26/26 PASS (run 39370400084) |
| KA-039 | Rzeczywiste serwery OMAs agentowy i OMAt narzędziowy, Zenoh PKI + OSTc, independent audit i operator acceptance | NOT IMPLEMENTED / OSTc SPEC PENDING |
| KA-040 | Kontynuacja audytu pozostałych skryptów tools/, port kolejnych użytecznych polityk do Rust | IN PROGRESS |

**Istotna korekta klasyfikacji:** ani `accepted=true` agenta 006, ani `allowed=true` agenta 007 nie stanowi niezależnej autoryzacji wykonania narzędzia. Wymagają właściwych grantów hosta/OMAt oraz prawdziwego sprawdzania postcondition. Nie przerabiać demonstracji na działający system przez zmianę flagi.


### Weryfikacja wykonania KA-034–040

[GitHub Actions 39370400084](https://github.com/mojealterego/Knowledge-projects/actions/runs/39370400084) na SHA `942f518a4f59ab70bedeabf2c9e4990067582e4a`: **SUCCESS** w obu jobach; 006 (16 Rust + 14 Python PASS), 007 (16 Rust + 12 Python PASS), fmt, strict Clippy i CLI PASS. Równoległe regresje 002–003 [39370400019](https://github.com/mojealterego/Knowledge-projects/actions/runs/39370400019) i 004–005 [39370400026](https://github.com/mojealterego/Knowledge-projects/actions/runs/39370400026) — SUCCESS. **KA-035/036/038 TESTED LOCAL**; KA-039 BLOCKED, KA-040 nadal nieukończone. CI to nie test produkcyjnej infrastruktury OMAs/OMAt/OSTc.

## Etap KA-041–KA-048 — kolejna para rzeczywistych portów Rust (20.10.2026)

| ID | Zadanie | Status |
|---|---|---|
| KA-041 | Audyt i port rzeczywistego `tools/dgm_cycle_gate.py`, test źródłowy 4 faz DGM i warunków promocji | IMPLEMENTED 008 / CI VERIFICATION |
| KA-042 | Agent 008 z niezależnymi, host-only zatwierdzeniami CI, faz, source provenance i rollback; izolowana gałąź i expiry UTC | IMPLEMENTED LOCAL / CI VERIFICATION |
| KA-043 | Audyt i port rzeczywistego `tools/cloud_operation_budget_gate.py`, test kosztu decimal string i IAM/project scope | IMPLEMENTED 009 / CI VERIFICATION |
| KA-044 | Agent 009 host-owned CloudApproval, zakres produktu/akcji i budżetu, brak wykonywania kosztownych side effects | IMPLEMENTED LOCAL / CI VERIFICATION |
| KA-045 | Unikatowe nazwy/seriale 008–009, oddzielne numery agent ID, katalog i certyfikat, bez kolizji w globalnym rejestrze | REGISTRY ENTERED / TESTING |
| KA-046 | Pełne dossier techniczne, instrukcja, bezpieczeństwo, migracja Python, cert record i 3 odrębne projekty umów | DOCUMENTED DEVELOPMENT SCOPE |
| KA-047 | Testy Rust + original Python, CI run IDs, fmt/clippy i regresje wcześniejszych agentów | IN PROGRESS / PR #32 |
| KA-048 | Kompletne integracje serwerów OMAs/OMAt, TLS Zenoh, OSTc, multi-host i formalny odbiór | NOT IMPLEMENTED / OSTc BLOCKED |

Nowe agenty 008–009 są wąskimi modułami offline, nie autonomicznymi administratorami chmury ani wykonawcami DGM. Dopisanie wpisów do ewidencji nie zamyka audytu wszystkich projektów, wszystkich narzędzi ani prac nad produkcyjnymi agentami.
