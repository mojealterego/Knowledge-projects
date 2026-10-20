# Charakterystyka i specyfikacja — VIGIA LOOP

**VIGIA LOOP** · nr ewidencyjny **007** · nr seryjny `AM-OPS-VIG-2026-000007` · Agent ID `AM-OCAF-AI-0007` · katalog `AM-OCAF-OPS-007` · certyfikat `CERT-AM-2026-0007` — **NIEWYDANY**.

## Funkcja

stanowa kontrola zapętleń i budżetów wywołań narzędzi agenta. Port Python AgentToolLoopGuard: limit łącznej liczby wywołań, limit zużycia tokenów, maksymalna sekwencja identycznych wywołań bez postępu, deterministyczna odmowa i trwały stan halted, reset powtórzeń tylko po host-owned readback.

## Wejście i wyjście API

Nazwa narzędzia: `tool.loop_gate`. Wejście modelu/klienta (schemat demonstracyjny):

```json
{"tool":"read_file","args_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","estimated_tokens":1}
```

Wyjście domenowe: `LoopDecision { allowed, reason, total_calls, total_tokens, identical_no_progress, execution_authorized=false }`.

**Kontrakt zaufania:** Trusted progress nie może pochodzić z wiadomości agenta lub pola tool JSON; host musi świadomie wywołać advance_verified_progress po niezależnym readback. Węzeł OCAF musi trwale zachować instancję Guard w obrębie sesji.

## Ograniczenia techniczne

Digest: dokładnie 64 małe znaki ASCII hex, tool niepusty bez control chars, limit tool 128 bajtów; licznik u64 overflow fail closed. Domyślne limity są definiowane przez hosta, nie przez model.

Zadeklarowane `cost_sats=0` dotyczy wyłącznie niepłatnej lokalnej operacji Rust; nie jest ceną usługi. Sieć, zapis repozytoriów i uruchamianie narzędzi niedostępne. Realne źródła/poświadczenia OSTc nie są czytane.

## Status produktów i certyfikacji

Własny pakiet Rust i testy: IMPLEMENTED (aktualny CI według `TEST_EVIDENCE.md`). Prawdziwe OMAs, OMAt, Zenoh mTLS, OSTc i produkcyjne wykonanie: NOT IMPLEMENTED. Pełna usługa biznesowa, niezależny audyt, podpisany release i operator acceptance: NOT COMPLETED. Cena i wartość rynkowa: NIEUSTALONE.
