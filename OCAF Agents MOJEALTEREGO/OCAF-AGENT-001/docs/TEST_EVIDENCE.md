# Macierz bramek T01–T15 — Agent 001

2026-10-19. Aktualne dowody są w sekcji końcowej; poniższa tabela historyczna zachowuje stan początkowy przed uruchomieniem CI.

| Test | Obszar | Stan |
|---|---|---|
| T01 | Manifest, identyfikatory, SHA i schematy | PARTIAL |
| T02 | Rust build, fmt, clippy, unit tests | NOT RUN |
| T03 | OMAs podpis ↔ RPC | NOT RUN |
| T04 | Cross-tenant isolation | NOT RUN |
| T05 | Jobs authorization | NOT RUN |
| T06 | Fenced effects | NOT RUN |
| T07 | Crash SIGKILL/restart | NOT RUN |
| T08 | Multi-host | NOT RUN |
| T09 | Tool sandbox/timeouts | NOT RUN |
| T10 | Secrets leakage | NOT RUN |
| T11 | OSTc test vectors | BLOCKED |
| T12 | Supply chain/SBOM/provenance | NOT RUN |
| T13 | Observability | NOT RUN |
| T14 | Lifecycle/revoke/restore | NOT RUN |
| T15 | Operator acceptance | NOT RUN |

**Wymagany status:** production_approved=false. Dowody, które mogą pojawić się w CI, potwierdzają tylko ich rzeczywisty zakres.


## Zweryfikowane uruchomienie CI — 19.10.2026

- **PASS:** https://github.com/mojealterego/Knowledge-projects/actions/runs/39198615847
- commit testowany: `d8a03875b4ac7b3c7c3e4b3dd3361306541a440a`; job: `117886684565`; środowisko GitHub-hosted ubuntu-latest.
- JS syntax: PASS; Rust fmt: PASS; **3 unit + 3 OCAF integration = 6/6 PASS**; clippy all targets/features, warnings denied: PASS.
- Poprzednia próba `39198511267`: FAIL na `cargo fmt`, skorygowano i powtórzono; nie jest dowodem weryfikacji funkcjonalnej.
- Zakres PASS ogranicza się do modułu `photo.plan`, OCAF local executor i negatywnej kontroli unsigned peer, **nie** pełnego OMAs RPC, multi-host, providerów Adobe/OpenAI czy OSTc.

**T02 = PASS na wyżej wymienionym SHA.** T01 manifest i T04 fragment negatywnych testów = PARTIAL; pozostałe bramki jak powyżej. Numer nie jest certyfikatem wydania.


## AKTUALIZACJA CI — nowa struktura OCAF-AGENT-001 (19.10.2026)

- Zweryfikowany **SUCCESS**: [GitHub Actions run 39203233792](https://github.com/mojealterego/Knowledge-projects/actions/runs/39203233792), SHA źródła `9498d7173c378be8d3b4ddf7d4cbd37d35cf5b56`, job `117900460178`.
- Testy: **12/12 PASS** (2 jednostkowe, 3 integracyjne OCAF, 7 bezpieczeństwa), 0 failed. Rust fmt PASS; strict clippy PASS; historyczny JS syntax PASS; lokalny CLI smoke PASS.
- OMAs i OMAt: zweryfikowano **jedynie odrębne przestrzenie nazw i lokalną rejestrację ToolExecutor**, nie sieć ani serwery w produkcji.
- OSTc: negative tests odmowy źródeł i wymuszenia dezaktywacji sieci PASS; **brak algorytmu i testów kryptograficznych**. Test all-true `ReleaseEvidence` → ACTIVE DENIED PASS.
- T02: **PASS dla tego SHA**; T01 i T04 fragmentaryczne; T03, T05–T08, T10–T15 nadal **NIEPOTWIERDZONE**; T09 częściowo potwierdzony statycznym API. T11 OSTc **BLOCKED do dostarczenia specyfikacji**.
- `certificate_issued=false`, `production_approved=false`.


## Korekta identyfikacyjna 0.2.1-dev (PR #28)

- Dodano `tests/identity.rs` obejmujący rozdzielenie nazwy `WDA AURELION`, numeru ewidencyjnego `001`, technicznego ID, numeru seryjnego i katalogowego.
- Dodano fail-closed walidację w `src/security.rs`: zmieniony serial lub nazwa powoduje błąd konfiguracji, a aktywacja i dostęp OSTc pozostają zablokowane.
- Centralny rejestr JSON musi odrzucać duplikaty ewidencji, nazw i seriali (weryfikacja case-insensitive w testach).
- **Wyniki testów dla 0.2.1-dev:** odczytaj udokumentowany później rzeczywisty run i SHA. Powyższe wyniki 0.2.0 dotyczą poprzednich wersji, nie można ich automatycznie przepisywać na nowy build.

## Bieżący zweryfikowany build identyfikacyjny (przed ostatnią poprawką testów)

- [GitHub Actions run 39324415084 — SUCCESS](https://github.com/mojealterego/Knowledge-projects/actions/runs/39324415084), commit `941a7269907951ce1223cf42df3220489583a6de`, job `118306422437`.
- **15/15 testów PASS:** 2 domenowe, 3 identyfikacyjne, 3 integracyjne OCAF i 7 bezpieczeństwa. `cargo fmt` PASS, strict `clippy` PASS, `node --check` PASS, CLI smoke PASS.
- Numer seryjny i nazwa są spójne w TOML/manifest/centralnym rejestrze, ale to **nie jest autentykacja kryptograficzna**.
- Rzeczywiste serwery OMAs/OMAt, OSTc, podpisy i produkcyjna obsługa obrazów nadal BRAK.
- Po końcowych poprawkach testów i dokumentów wymagane będzie nowe CI oraz readback `main`.

## Aktualna weryfikacja: nazwa, serial i odmowa podmiany (2026-10-19)

- **[GitHub Actions run 39324839309 — SUCCESS](https://github.com/mojealterego/Knowledge-projects/actions/runs/39324839309)**, commit `3fb99da89cc1d7b7ca50277440b8d6ab29888b5d`, job `118307850394`, środowisko `ubuntu-latest`.
- **16/16 Rust testów PASS**: 2 domenowe, **4 identyfikacyjne**, 3 integracyjne OCAF, 7 bezpieczeństwa; 0 failed. `cargo fmt --check` PASS, `clippy -D warnings` PASS, JS syntax PASS, `cargo run` CLI smoke PASS.
- Test `replacing_registered_name_or_serial_is_rejected`: PASS — nazwa lub numer seryjny różne od wersji zarejestrowanej skutkują odmową uruchomienia.
- Test `serial_name_and_registry_number_are_unique_in_registry`: PASS — obecny rejestr nie zawiera duplikatów agent ID, nazwy, serialu, numeru ewidencyjnego ani katalogowego (case-insensitive).
- T01: **częściowo** potwierdzono tożsamość, nie artefakt digest. T02: PASS dla tego konkretnego SHA. T03–T15 nadal niezamknięte; OSTc BLOCKED BY SPECIFICATION.
- **CI nie jest certyfikatem**. Certyfikat `CERT-AM-2026-0001` niewydany, `production_approved=false`, `source_access.enabled=false`.
