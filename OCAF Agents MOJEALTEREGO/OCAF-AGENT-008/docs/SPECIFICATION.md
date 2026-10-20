# Specyfikacja techniczna — DGM SENTINEL

**Nazwa własna:** DGM SENTINEL · **Numer ewidencyjny:** 008 · **Serial:** `AM-DGM-SNT-2026-000008` · **Agent ID:** `AM-OCAF-AI-0008` · **Katalog:** `AM-OCAF-DGM-008` · **Certyfikat:** `CERT-AM-2026-0008` — NIEWYDANY.

## Założenia

kontrola dowodów dla czterofazowej propozycji promocji DGM na izolowanej gałęzi. Strukturalna kontrola 40-znakowych commit SHA, izolowanej nazwy gałęzi integration/, 4 faz ModelScout, TechRecon, Strategist, DGM_Core, listy required_checks, UTC expiry oraz zakazu skutków ubocznych.

## Interfejs wywołania

Nazwa OMAt/OCAF tool (lokalnie): `dgm.assess_promotion`.

Wejście domenowe `PromotionProposal`: `candidate_sha` i `base_sha` — różne SHA-1 identyfikatory Git, `branch` izolowana `integration/*`, `phase_receipts` (dokładnie ModelScout, TechRecon, Strategist, DGM_Core), `required_checks`, `approval_expires_at` UTC-Z, `external_side_effects=false`, `rollback_ref_verified`, `source_provenance_verified`. Poza tym **host-only** `PromotionTrust`: potwierdzone CI, kandydat, fazy, rollback i proweniencja; parametry nie są dostępne w modelowym JSON.

Wynik: `PromotionDecision { admitted, reasons[], execution_authorized=false, branch_mutated=false }`; oba znaczniki wykonania są zawsze **false**. `cost_sats=0` oznacza lokalną operację bez kosztu narzędzia, nie zatwierdzony cennik rynkowy.

## Dokumentacja pochodzenia i ograniczeń

Źródło [Python](../../../tools/dgm_cycle_gate.py) pozostaje w repo i nie jest usunięte. Rust 008 dodaje wymóg hostowych zatwierdzeń receiptów czterech faz, źródła i rollbacku ponad pierwotne pola proposal w Python. Zaostrza format integration branch i ogranicza rozmiar wejścia. Pozostała lista kontroli pochodzi z oryginalnego Python. Nie oznacza to ścisłej bitowej równoważności.

**Poza zakresem:** podpisane prawdziwe CI attestation, weryfikacja git refs z GitHub, operator approvals na serwerze, rzeczywistego uruchamiania etapów DGM i rollback. Brak podpisanego certyfikatu, portretu/logo i uruchomionego środowiska produkcyjnego.
