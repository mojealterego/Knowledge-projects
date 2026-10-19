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
