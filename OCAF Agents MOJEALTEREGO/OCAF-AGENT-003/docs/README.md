# EVIDENTIA RESEARCH — OCAF-AGENT-003

**Nazwa własna:** EVIDENTIA RESEARCH · **Numer ewidencyjny:** 003 · **Numer seryjny:** `AM-RES-EVI-2026-000003` · **Agent ID:** `AM-OCAF-AI-0003` · **Nr katalogowy:** `AM-OCAF-RES-003` · **Certyfikat:** `CERT-AM-2026-0003` — REZERWACJA, NIEWYDANY.

**Stan:** `INCOMPLETE_DEVELOPMENT`. Lokalny kod Rust do celu: lokalna kontrola jakości przesłanek badawczych. Nie jest to kompletna migracja projektu źródłowego ani aktywny produkt OMAs/OMAt/OSTc.

## Wykonany fragment

oznaczanie sprzecznych stanowisk w przekazanych dowodach; brak autonomicznego researchu i brak walidacji oryginalnych źródeł. Lokalna funkcja `research.assess` w Rust działa bez sieci, kluczy dostawców, źródeł OSTc i efektów ubocznych. `Cargo.toml` zawiera przypięty git `ocaf_runtime`, adapter `ToolManifest/ToolExecutor` jest lokalny. Rejestracja i transport OMAs/OMAt przez Zenoh **nie zostały wdrożone sieciowo**.

## Sposób uruchomienia

```sh
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-003/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-003/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-003/Cargo.toml" -- "Przykładowa teza"
```

## Spis własnych dokumentów

[ARCHITECTURE](ARCHITECTURE.md) · [SPECIFICATION](SPECIFICATION.md) · [USER_MANUAL](USER_MANUAL.md) · [SECURITY](SECURITY.md) · [SERVICE](SERVICE.md) · [IDENTITY_AND_SERIAL](IDENTITY_AND_SERIAL.md) · [MIGRATION](MIGRATION.md) · [TEST_EVIDENCE](TEST_EVIDENCE.md) · [RELEASE_CHECKLIST](RELEASE_CHECKLIST.md) · [CHANGELOG](CHANGELOG.md) · [CERTIFICATE_RECORD](../certification/CERTIFICATE_RECORD.md) · [Umowa sprzedaży](commercial/SALE-DRAFT.md) · [Umowa dzierżawy](commercial/LEASE-DRAFT.md) · [Umowa najmu](commercial/RENT-DRAFT.md).

**Pochodzenie:** [specyfikacja Universal Research Orchestrator / P15](../../../projekty/15-universal-research-orchestrator.md); z projektu pochodzi koncepcja, nie plik wykonywalny. Projekt pozostaje w `projekty/` jako system macierzysty.
