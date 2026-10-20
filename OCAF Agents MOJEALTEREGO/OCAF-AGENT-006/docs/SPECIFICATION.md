# Charakterystyka i specyfikacja — CUSTOS ADMISSION

**CUSTOS ADMISSION** · nr ewidencyjny **006** · nr seryjny `AM-SEC-CUS-2026-000006` · Agent ID `AM-OCAF-AI-0006` · katalog `AM-OCAF-SEC-006` · certyfikat `CERT-AM-2026-0006` — **NIEWYDANY**.

## Funkcja

lokalna kontrola dopuszczenia artefaktu narzędzia, BEZ jego wykonania. Port funkcji Python admit_tool: SHA-256 bajtów, jawna allowlista czterech capability, niezależne zgody oparte o digest, nazwa odizolowanego profilu, timeout 1–60 sekund, ważna data wygaśnięcia oraz zakaz skutków ubocznych.

## Wejście i wyjście API

Nazwa narzędzia: `tool.admit`. Wejście modelu/klienta (schemat demonstracyjny):

```json
{"manifest":{"artifact_sha256":"<64 hex chars>","capabilities":["parse_document"],"sandbox_profile":"isolated_no_network_read_only","timeout_seconds":30,"expires_at":"2026-10-22T00:00:00Z","external_side_effects":false},"artifact_hex":"<hex encoded bytes>"}
```

Wyjście domenowe: `AdmissionDecision { accepted, status, reasons[], artifact_sha256, execution_authorized=false, sandbox_started=false }`.

**Kontrakt zaufania:** Hash artefaktu nie stanowi uwierzytelnienia osoby przeglądającej. Mapę approvals i czas 'now' musi dostarczyć oddzielnie zaufany host. Model nie może ich dołączyć w manifest/tool JSON.

## Ograniczenia techniczne

Maks. 1 MiB artefaktu, maks. 2,1 MB payloadu w adapterze OCAF, wyłącznie hex; narzędzie nie uruchamia sandboxu, nie instaluje ani nie wykonuje kodu.

Zadeklarowane `cost_sats=0` dotyczy wyłącznie niepłatnej lokalnej operacji Rust; nie jest ceną usługi. Sieć, zapis repozytoriów i uruchamianie narzędzi niedostępne. Realne źródła/poświadczenia OSTc nie są czytane.

## Status produktów i certyfikacji

Własny pakiet Rust i testy: IMPLEMENTED (aktualny CI według `TEST_EVIDENCE.md`). Prawdziwe OMAs, OMAt, Zenoh mTLS, OSTc i produkcyjne wykonanie: NOT IMPLEMENTED. Pełna usługa biznesowa, niezależny audyt, podpisany release i operator acceptance: NOT COMPLETED. Cena i wartość rynkowa: NIEUSTALONE.
