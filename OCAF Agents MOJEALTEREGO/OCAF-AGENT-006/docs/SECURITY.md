# Model zagrożeń — CUSTOS ADMISSION

**CUSTOS ADMISSION** · nr ewidencyjny **006** · nr seryjny `AM-SEC-CUS-2026-000006` · Agent ID `AM-OCAF-AI-0006` · katalog `AM-OCAF-SEC-006` · certyfikat `CERT-AM-2026-0006` — **NIEWYDANY**.

## Inwarianty

Hash artefaktu nie stanowi uwierzytelnienia osoby przeglądającej. Mapę approvals i czas 'now' musi dostarczyć oddzielnie zaufany host. Model nie może ich dołączyć w manifest/tool JSON.

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

Napis `isolated_no_network_read_only` jest tylko DEKLARACJĄ manifestu sprawdzaną przez Rust, nie izolacją procesu! `accepted=true` nie jest certyfikacją podpisu ani zgodą na wykonanie kodu.

**Zakaz oznaczania jako produkcyjnego** do wdrożenia serwerów i testów poza obecnym zakresem.
