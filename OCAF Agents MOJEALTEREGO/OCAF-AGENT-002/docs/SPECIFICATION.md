# VIGILIS SRE — OCAF-AGENT-002

**Nazwa własna:** VIGILIS SRE · **Numer ewidencyjny:** 002 · **Numer seryjny:** `AM-SRE-VIG-2026-000002` · **Agent ID:** `AM-OCAF-AI-0002` · **Nr katalogowy:** `AM-OCAF-SRE-002` · **Certyfikat:** `CERT-AM-2026-0002` — REZERWACJA, NIEWYDANY.

**Stan:** `INCOMPLETE_DEVELOPMENT`. Lokalny kod Rust do celu: diagnostyka incydentów infrastruktury. Nie jest to kompletna migracja projektu źródłowego ani aktywny produkt OMAs/OMAt/OSTc.

## Charakterystyka

Agent realizuje diagnostyka incydentów infrastruktury. Jest to nowa implementacja części zadań inspirowana specyfikacją [InfraSentinel / P121](../../../projekty/121-infrasentinel-agentic-autonomous-sre-infrastructure/README.md); nie jest przejęciem nieistniejących w repo funkcji binarnych.

## Funkcja i interfejs

Narzędzie: `incident.triage`. Struktura: `Incident { incident_id, tenant_id, signal, observations[1..8], risk_tier R0..R4 }`. Wyjście: `Triage { agent_id, status=read_only, signal, risk_tier, evidence_count, allow_mutation=false, verified=false, recommendations[] }`. Dostęp do danych tylko ze struktury przekazanej operatorowi lokalnemu. `cost_sats=0` odnosi się do deklaracji narzędzia lokalnego; nie stanowi cennika.

Przykładowy payload:

```json
{
  "incident_id": "INC-1",
  "tenant_id": "team_a",
  "signal": "CrashLoopBackOff",
  "observations": [
    "operator reports restart loop"
  ],
  "risk_tier": "R3"
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
| Kubernetes API readback, Log/Prometheus, Nebius/Nemotron/Tavily/NeMo, bezpieczny sandbox i faktyczne remediacje | NOT IMPLEMENTED |
| Niezależny audit i operator acceptance | NOT RUN |

## Modele handlowe

Sprzedaż, dzierżawa (minimum 3 miesiące), najem (minimum 7 dni) opisane jedynie w oddzielnych szkicach. Ceny, VAT, ubezpieczenia i SLA do ustalenia i zatwierdzenia prawnego. Nie ma wydanego certyfikatu, podpisanej binarki ani usługi produkcyjnej.
