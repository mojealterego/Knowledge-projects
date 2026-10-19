# Kryteria ukończenia i wydania — WDA AURELION

**Status na 2026-10-19: AGENT NIEUKOŃCZONY / CERTYFIKAT NIEWYDANY.** Numer seryjny jest przydzielony, ale nie potwierdza jakości, funkcjonalności ani bezpieczeństwa.

| Warunek | Stan | Wymagany dowód |
|---|---|---|
| Jedna nazwa własna i serial niezależny od numeru 001 | IMPLEMENTED; CI weryfikuje | manifest + TOML + rejestr JSON + test `identity.rs` |
| Własny folder, kod, testy i dokumentacja techniczna | IMPLEMENTED dla zakresu dev | ścieżki i tree SHA |
| Lokalny planer `photo.plan` | TESTED | GitHub Actions testy Rust |
| Walidacja tożsamości, odrzucenie fałszywych seriali i duplicate | TESTED LOCALLY po CI | test `identity.rs` |
| Pełny port źródła WDA JS → Rust / obsługa fotografii | NOT IMPLEMENTED | build + golden image fixtures + approval |
| Ochrona twarzy/tożsamości i ocena jakości obrazu | NOT RUN | zestaw porównawczy i testy regresji |
| Serwer agentowy OMAs z podpisanym RPC | NOT IMPLEMENTED | auth/mTLS/E2E signed identity tests |
| Serwer narzędzi OMAt i autoryzacja | NOT IMPLEMENTED | real permit/deny i tool isolation |
| Zenoh zabezpieczony i odporny na awarie | NOT IMPLEMENTED | niezależny host, mTLS, reconnect |
| OSTc — szyfrowany dostęp do plików źródłowych | BLOCKED | dostarczenie pełnej specyfikacji, vectors, implementacja, audit |
| Bezpieczny klucz i identyfikacja operatora | NOT DONE | niezależny PKI i rotacja |
| Multi-host, failover i tenant isolation | NOT RUN | logi testu prawdziwych hostów |
| API Adobe/Firefly/OpenAI, budżety i autoryzacja | NOT IMPLEMENTED | readback providerów, zatwierdzenia |
| Supply-chain/SBOM/podpisanego artefaktu | NOT DONE | podpis SHA-256 + SBOM/provenance |
| Formalny niezależny audyt bezpieczeństwa | NOT DONE | raport audytora |
| Certyfikat autent., fotografia/logo/pieczęć/podpis | DRAFT / NOT ISSUED | autentyczne assety, podpis pod release |
| Operator acceptance i protokół odbioru | NOT DONE | osobny podpisany protokół |
| Dokumenty sprzedaży/dzierżawy/najmu, ceny i review prawny | DRAFT | prawnicze zatwierdzenie i ceny |
| Formalna produkcyjna aktywacja | BLOCKED IN CODE | nie można wejść w ACTIVE |

## Status dokumentacji
Dokumenty opisują **kompletnie zakres aktualnie zaimplementowany i jawnie rejestrują braki**. Nie stanowią dowodu ukończenia docelowego agenta fotograficznego. Numer seryjny **zarejestrowany**, certyfikat **niewydany**, produkcja **niezatwierdzona**.

## Protokół wydania — wymagane pola do wypełnienia po wykonaniu
Nazwa unikatowa, serial i rejestr 001; commit; SHA artefaktu; SBOM; wersja binarna; wyniki T01–T15; rzeczywiste podpisy OMAs, OMAt, OSTc; operator; data odbioru; numer wydanego certyfikatu i podpis urzędowego wystawcy; zatwierdzona dokumentacja prawna. Brak któregokolwiek blokującego dowodu oznacza status `INCOMPLETE_DEVELOPMENT`.
