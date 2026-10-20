# DGM SENTINEL — OCAF-AGENT-008

**Nazwa własna:** DGM SENTINEL · **Numer ewidencyjny:** 008 · **Serial:** `AM-DGM-SNT-2026-000008` · **Agent ID:** `AM-OCAF-AI-0008` · **Katalog:** `AM-OCAF-DGM-008` · **Certyfikat:** `CERT-AM-2026-0008` — NIEWYDANY.

**Stan: `INCOMPLETE_DEVELOPMENT` · certyfikat niewydany · brak zgody produkcyjnej.**

## Cel i faktyczny zakres

kontrola dowodów dla czterofazowej propozycji promocji DGM na izolowanej gałęzi. Port rzeczywistego modułu [`tools/dgm_cycle_gate.py`](../../../tools/dgm_cycle_gate.py) i testów [`tools/test_dgm_cycle_gate.py`](../../../tools/test_dgm_cycle_gate.py). Strukturalna kontrola 40-znakowych commit SHA, izolowanej nazwy gałęzi integration/, 4 faz ModelScout, TechRecon, Strategist, DGM_Core, listy required_checks, UTC expiry oraz zakazu skutków ubocznych.

**Granica zaufania:** Zatwierdzone kandydatury, potwierdzone CI, receipt faz, rollback i source provenance muszą być wstrzyknięte przez zaufanego hosta jako PromotionTrust. Samo rollback_ref_verified=true lub source_provenance_verified=true w prośbie modelu nie ma autorytatywnej mocy.

Lokalny pakiet Rust `dgm_sentinel_agent` nie wykonuje kodu innych agentów, nie promuje gałęzi, nie wywołuje GCP, nie pobiera opłat, nie uruchamia mikro-VM. Status `admitted/allowed` oznacza wynik przeglądu offline, nie dopuszczenie runtime przez OMAs/OMAt.

## Testy

```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-008/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-008/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-008/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-008/Cargo.toml" -- demo
```

CI uruchamia też źródłowy Python unittest. Dokumentacja: [ARCHITECTURE](ARCHITECTURE.md), [SPECIFICATION](SPECIFICATION.md), [USER_MANUAL](USER_MANUAL.md), [SECURITY](SECURITY.md), [SERVICE](SERVICE.md), [IDENTITY_AND_SERIAL](IDENTITY_AND_SERIAL.md), [PYTHON_PARITY](PYTHON_PARITY.md), [DEPENDENCIES](DEPENDENCIES.md), [MIGRATION](MIGRATION.md), [RELEASE_CHECKLIST](RELEASE_CHECKLIST.md), [TRACEABILITY_MATRIX](TRACEABILITY_MATRIX.md), [TEST_EVIDENCE](TEST_EVIDENCE.md), [CHANGELOG](CHANGELOG.md), [CERTIFICATE_RECORD](../certification/CERTIFICATE_RECORD.md), [Umowa sprzedaży](commercial/SALE-DRAFT.md), [Dzierżawy](commercial/LEASE-DRAFT.md), [Najmu](commercial/RENT-DRAFT.md).

Rzeczywiste serwery OMAs i OMAt, działający transport Zenoh TLS oraz kryptografia OSTc **NIE SĄ wdrożone**. OSTc jest zablokowany do otrzymania dokumentacji i niezależnego audytu.
