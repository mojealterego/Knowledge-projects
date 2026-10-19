# EVIDENTIA RESEARCH — OCAF-AGENT-003

**Nazwa własna:** EVIDENTIA RESEARCH · **Numer ewidencyjny:** 003 · **Numer seryjny:** `AM-RES-EVI-2026-000003` · **Agent ID:** `AM-OCAF-AI-0003` · **Nr katalogowy:** `AM-OCAF-RES-003` · **Certyfikat:** `CERT-AM-2026-0003` — REZERWACJA, NIEWYDANY.

**Stan:** `INCOMPLETE_DEVELOPMENT`. Lokalny kod Rust do celu: lokalna kontrola jakości przesłanek badawczych. Nie jest to kompletna migracja projektu źródłowego ani aktywny produkt OMAs/OMAt/OSTc.

## Charakterystyka

Agent realizuje lokalna kontrola jakości przesłanek badawczych. Jest to nowa implementacja części zadań inspirowana specyfikacją [Universal Research Orchestrator / P15](../../../projekty/15-universal-research-orchestrator.md); nie jest przejęciem nieistniejących w repo funkcji binarnych.

## Funkcja i interfejs

Narzędzie: `research.assess`. Struktura: `ResearchInput { claim, evidence[2..8] { source_id, stance, excerpt } }`. Wyjście: `Assessment { agent_id, status=disputed|unverified, source_count, has_contradiction, verified=false, conclusions[] }`. Dostęp do danych tylko ze struktury przekazanej operatorowi lokalnemu. `cost_sats=0` odnosi się do deklaracji narzędzia lokalnego; nie stanowi cennika.

Przykładowy payload:

```json
{
  "claim": "Przykładowa teza",
  "evidence": [
    {
      "source_id": "A",
      "stance": "supports",
      "excerpt": "Rzekomy argument"
    },
    {
      "source_id": "B",
      "stance": "contradicts",
      "excerpt": "Rzekomy kontrargument"
    }
  ]
}
```

**NIE JEST** to prawdziwe źródło dowodowe ani autentykacja danych. Każda obserwacja wymaga niezależnego zatwierdzenia. Agent nie ma autoryzacji do modyfikacji zewnętrznych zasobów.

## Zakresy

| Moduł | Stan |
|---|---|
| Samodzielny pakiet Rust i testy | IMPLEMENTED; realny status CI w TEST_EVIDENCE |
| Ewidencja unikalnej nazwy/serialu | REGISTERED DEVELOPMENT |
| Lokalny ToolExecutor OCAF | IMPLEMENTED, test wymaga PASS |
| OMAs server / OMAt server | NOT DEPLOYED |
| Zenoh network + mTLS | NOT ENABLED |
| OSTc szyfrowany dostęp do źródeł | BLOCKED do dostarczenia dokumentacji |
| wyszukiwanie w sieci, analiza dokumentów, sprawdzanie cytowań, eksperci wieloagentowi, falsyfikacja, niezależna weryfikacja | NOT IMPLEMENTED |
| Niezależny audit i operator acceptance | NOT RUN |

## Modele handlowe

Sprzedaż, dzierżawa (minimum 3 miesiące), najem (minimum 7 dni) opisane jedynie w oddzielnych szkicach. Ceny, VAT, ubezpieczenia i SLA do ustalenia i zatwierdzenia prawnego. Nie ma wydanego certyfikatu, podpisanej binarki ani usługi produkcyjnej.
