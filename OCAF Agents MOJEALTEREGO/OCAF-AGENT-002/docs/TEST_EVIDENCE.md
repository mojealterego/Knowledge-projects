# VIGILIS SRE — OCAF-AGENT-002

**Nazwa własna:** VIGILIS SRE · **Numer ewidencyjny:** 002 · **Numer seryjny:** `AM-SRE-VIG-2026-000002` · **Agent ID:** `AM-OCAF-AI-0002` · **Nr katalogowy:** `AM-OCAF-SRE-002` · **Certyfikat:** `CERT-AM-2026-0002` — REZERWACJA, NIEWYDANY.

**Stan:** `INCOMPLETE_DEVELOPMENT`. Lokalny kod Rust do celu: diagnostyka incydentów infrastruktury. Nie jest to kompletna migracja projektu źródłowego ani aktywny produkt OMAs/OMAt/OSTc.

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
