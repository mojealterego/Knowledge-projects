# Specyfikacja funkcjonalna — ARGUS MOBILE

**Nazwa własna:** ARGUS MOBILE · **Ewidencja:** 005 · **Numer seryjny:** `AM-MOB-ARG-2026-000005` · **Agent ID:** `AM-OCAF-AI-0005` · **Katalog:** `AM-OCAF-MOB-005` · **Nr certyfikatu:** `CERT-AM-2026-0005` (rezerwacja, niewydany).

## Cele, granice i API

**Cel:** kontroler dopuszczenia propozycji akcji interfejsu Android. **Implementacja:** Ocena deklarowanej zgody i typu akcji bez dostępu do telefonu; stała blokada DELETE, SEND i GRANT_PERMISSION; pozostałe propozycje wymagają weryfikacji przez rzeczywisty Android UI runtime.

**Nie obejmuje:** AccessibilityService, ADB, sterowanie ekranem, przechwytywanie powiadomień, połączenia z LLM i Google Play oraz wykonanie rzeczywistych działań na telefonie.

**Nazwa narzędzia:** `ui.proposal_assess`. Wymiana strukturalna JSON:

```json
{
  "session_id": "s1",
  "snapshot_id": "snap1",
  "node_id": "node1",
  "action": "CLICK",
  "operator_grant": true,
  "expected_postcondition": "new view"
}
```

Wynik: `ActionAssessment: status=blocked|requires_grant|operator_review, execution_authorized=false, readback_required=true, verified=false, reasons[]`. OCAF ToolManifest wymaga znanych pól i limitów, a Rust `serde(deny_unknown_fields)` dodatkowo odrzuca nieznane pola; adapter ogranicza rozmiar serializowanego żądania do 8192 B.

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
| AccessibilityService, ADB, sterowanie ekranem, przechwytywanie powiadomień, połączenia z LLM i Google Play oraz wykonanie rzeczywistych działań na telefonie | NOT IMPLEMENTED |
| Certyfikat i formalny odbiór | NOT ISSUED |

## Wycena

Brak zatwierdzonej ceny netto/brutto, wartości licencji, podpisanego SLA, deklarowanej dostępności i ubezpieczenia. Projekt umów znajduje się w `docs/commercial/`; nie jest ofertą gotowego produktu.
