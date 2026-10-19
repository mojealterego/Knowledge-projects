# EVIDENTIA RESEARCH — OCAF-AGENT-003

**Nazwa własna:** EVIDENTIA RESEARCH · **Numer ewidencyjny:** 003 · **Numer seryjny:** `AM-RES-EVI-2026-000003` · **Agent ID:** `AM-OCAF-AI-0003` · **Nr katalogowy:** `AM-OCAF-RES-003` · **Certyfikat:** `CERT-AM-2026-0003` — REZERWACJA, NIEWYDANY.

**Stan:** `INCOMPLETE_DEVELOPMENT`. Lokalny kod Rust do celu: lokalna kontrola jakości przesłanek badawczych. Nie jest to kompletna migracja projektu źródłowego ani aktywny produkt OMAs/OMAt/OSTc.

## Macierz bieżących dowodów

W chwili utworzenia pakietu nowej implementacji **nie wolno przypisywać wyników CI agenta 001 agentom 002/003**. Dedykowany [workflow agents 002–003](../../../.github/workflows/ocaf-agents-002-003.yml) musi osobno wykazać `fmt`, Rust unit, security, OCAF integration, clippy i demo CLI na rzeczywistym SHA.

| Bramka | Zakres | Stan przed końcowym CI |
|---|---|---|
| T01 | zgodność registry name/serial/manifest/TOML | TO TEST |
| T02 | Rust fmt/test/clippy/unit/integration | TO TEST |
| T03 | podpis OMAs związany z RPC | NOT IMPLEMENTED |
| T04 | tenant isolation na niezależnych hostach | NOT RUN |
| T05–T06 | autoryzowane jobs/fenced effects | N/A w plan-only; E2E NOT RUN |
| T07–T08 | awarie/recovery multi-host | NOT RUN |
| T09 | sandbox narzędzi | lokalne read-only, brak sandbox hosta |
| T10 | wyciek sekretów/providerów | brak integracji, E2E NOT RUN |
| T11 | test vectors OSTc | BLOCKED |
| T12 | SBOM/provenance/signed artifact | NOT RUN |
| T13–T15 | obserwowalność, odwołanie, odbiór | NOT RUN |

**Dowód:** wymagane konkretne run ID, commit SHA, liczba PASS i logi; sam opis nie stanowi dowodu.

## ZWERYFIKOWANY CI — 20.10.2026

- **[GitHub Actions #39337503859 — SUCCESS](https://github.com/mojealterego/Knowledge-projects/actions/runs/39337503859)**, commit `22133d7ede2125425cb48ceadf2ffbbe1a3bb1d7`, job `118350558201`.
- **12/12 testów Rust PASS, 0 failed**: 2 domenowe, 3 tożsamości/rejestru, 1 podstawowy test integracyjny, 2 kontraktów OCAF, 4 testy bezpieczeństwa.
- Rustfmt PASS, strict Clippy PASS, lokalny smoke CLI PASS.
- T01: lokalna spójność nazwy/serialu/manifestu i globalnego rejestru PASS, **nie** podpisany build/digest.
- T02: Rust PASS dla wskazanego SHA. T03–T15 pozostają NIEZAMKNIĘTE, z wyjątkiem lokalnych testów odmowy i trasowania, bez rzeczywistych hostów. OSTc T11 = BLOCKED BY SPEC.
- Certyfikat **nie został wydany**, podpisany artefakt nie istnieje, produkcja niezatwierdzona.
