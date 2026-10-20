# Model zagrożeń — VIGIA LOOP

**VIGIA LOOP** · nr ewidencyjny **007** · nr seryjny `AM-OPS-VIG-2026-000007` · Agent ID `AM-OCAF-AI-0007` · katalog `AM-OCAF-OPS-007` · certyfikat `CERT-AM-2026-0007` — **NIEWYDANY**.

## Inwarianty

Trusted progress nie może pochodzić z wiadomości agenta lub pola tool JSON; host musi świadomie wywołać advance_verified_progress po niezależnym readback. Węzeł OCAF musi trwale zachować instancję Guard w obrębie sesji.

| Ryzyko | Kontrola obecna | Co musi zostać potwierdzone |
|---|---|---|
| Model deklaruje własną zgodę, progress lub certyfikat | **host-only** stan; modelowy JSON ograniczony | issuer identity i niezależny readback |
| Oszustwo przez identyfikator seryjny | TOML + manifest + globalny rejestr, testy fail-closed | podpis wiążący serial z OMAs/RPC |
| Nieznane narzędzia i skutki uboczne | brak side effects/wykonania; walidacja wejścia | per-tool signed OMAt capabilities |
| Złośliwy payload | rozmiar/schemat i obsługa błędów | fuzzing, koszt/DoS i realny sandbox |
| OSTc niekompletne | `source_access.enabled=false`, odczyt DENY | projekt i audyt OSTc, test vectors |
| Zenoh bez tożsamości | sieć wyłączona, tylko klucze trasy | TLS/mTLS, multihost, tenant isolation |
| Atak na stan błędu/guard | odmowy i blokady `Active` | niezależny operator approval, persistence |
| Fałszywa obietnica wydania | `certificate_issued=false`, `production_approved=false` | SBOM, podpisany artefakt, audyt i odbiór |

## Szczególna uwaga

Stan `halted` trzyma jedna instancja guard. Restart procesu lub stworzenie nowego AgentTool zeruje liczniki, więc wymagana jest trwała host-owned sesja lub przyszły podpisany store HA. Deklarowane estimated_tokens nie jest faktycznym rozliczeniem bez provider telemetry.

**Zakaz oznaczania jako produkcyjnego** do wdrożenia serwerów i testów poza obecnym zakresem.
