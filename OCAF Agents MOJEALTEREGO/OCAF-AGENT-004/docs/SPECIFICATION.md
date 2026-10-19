# Specyfikacja funkcjonalna — AXIOM FORGE

**Nazwa własna:** AXIOM FORGE · **Ewidencja:** 004 · **Numer seryjny:** `AM-ENG-AXF-2026-000004` · **Agent ID:** `AM-OCAF-AI-0004` · **Katalog:** `AM-OCAF-ENG-004` · **Nr certyfikatu:** `CERT-AM-2026-0004` (rezerwacja, niewydany).

## Cele, granice i API

**Cel:** analizator zależności grafu prac inżynieryjnych. **Implementacja:** Wykrywanie duplikatów identyfikatorów, brakujących zależności, samoodwołań i cykli; deterministyczne uporządkowanie topologiczne; zero komend, mutacji i połączeń z GitHub.

**Nie obejmuje:** orchestracja modeli, generacja lub wdrażanie kodu, realizacja task graph, samoakceptacja builda, niezależne podpisy i testy e2e.

**Nazwa narzędzia:** `graph.review`. Wymiana strukturalna JSON:

```json
{
  "graph_id": "g1",
  "tasks": [
    {
      "task_id": "plan",
      "dependencies": [],
      "stage": "plan"
    },
    {
      "task_id": "build",
      "dependencies": [
        "plan"
      ],
      "stage": "build"
    }
  ]
}
```

Wynik: `GraphReview: status=structurally_valid|blocked, task_count, order[], issues[], execution_authorized=false, verified=false`. OCAF ToolManifest wymaga znanych pól i limitów, a Rust `serde(deny_unknown_fields)` dodatkowo odrzuca nieznane pola; adapter ogranicza rozmiar serializowanego żądania do 8192 B.

## Status elementów

| Zakres | Stan |
|---|---|
| Nazwa, numer ewidencyjny, numer seryjny, techniczny ID | REGISTERED DEVELOPMENT |
| Kod Rust domeny, walidacja danych i testy | IMPLEMENTED; wynik CI w TEST_EVIDENCE |
| Lokalny adapter OCAF ToolExecutor | IMPLEMENTED; CI |
| Rzeczywisty serwer OMAs | NOT IMPLEMENTED |
| Rzeczywisty serwer OMAt | NOT IMPLEMENTED |
| Zenoh network TLS/mTLS i multi-host | NOT IMPLEMENTED |
| OSTc szyfrowany dostęp do źródeł | BLOCKED / SPECIFICATION PENDING |
| orchestracja modeli, generacja lub wdrażanie kodu, realizacja task graph, samoakceptacja builda, niezależne podpisy i testy e2e | NOT IMPLEMENTED |
| Certyfikat i formalny odbiór | NOT ISSUED |

## Wycena

Brak zatwierdzonej ceny netto/brutto, wartości licencji, podpisanego SLA, deklarowanej dostępności i ubezpieczenia. Projekt umów znajduje się w `docs/commercial/`; nie jest ofertą gotowego produktu.
