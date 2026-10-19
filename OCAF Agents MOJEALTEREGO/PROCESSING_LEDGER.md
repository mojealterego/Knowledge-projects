# Dziennik przetwarzania — partia 2026-10-19

| Jednostka | Pozyskanie | Weryfikacja | Właściciel / porównanie | Stan końcowy tej partii |
|---|---|---|---|---|
| SRC-001 | BYTES_RECEIVED | HASHED + CONTENT_READ | OCAF governance | Ingested |
| SRC-002 / SRC-003 | BYTES_RECEIVED | MD read; PDF 4/4 strony tekstowo wyekstrahowane | Dokumentacja wydania i komercjalizacji | PARTIAL — semantyczna walidacja PDF pending |
| SRC-004 / SRC-005 | BYTES_RECEIVED | MD read; PDF 4/4 strony tekstowo wyekstrahowane | Kontrakty, manifest, T01–T15 | PARTIAL — pełne mapowanie na symbole kodu pending |
| `mojealterego/Knowledge-projects` | TREE_DISCOVERED | 745 wpisów drzewa; SHA commita main | Repo robocze | INVENTORIED_STRUCTURE; FULL AUDIT NOT RUN |
| `mojealterego/ocaf-runtime` | TREE_DISCOVERED | 623 wpisów drzewa; SHA commita main | Referencja OCAF | INVENTORIED_STRUCTURE; RUNTIME TESTS NOT RUN |
| Odzyskana historia OMAt | NOT_RECEIVED | Nie otrzymano korpusu 333+ pozycji | OMAt | BLOCKED_ON_SOURCE |

**Reguła wznowienia:** przetwarzanie dzielić na strony/zasoby i każdemu przypisywać `HASHED → CONTENT_READ → OWNER_MAPPED → VERIFIED → COMMITTED`. Nie podwyższać statusu zbiorczego ponad stan najsłabiej rozliczonego elementu. Każda przyszła paczka ma zachować oryginały i metadane oraz raport pokrycia stron.
