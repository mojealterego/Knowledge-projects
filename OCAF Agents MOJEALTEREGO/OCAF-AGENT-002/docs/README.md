# VIGILIS SRE — OCAF-AGENT-002

**Nazwa własna:** VIGILIS SRE · **Numer ewidencyjny:** 002 · **Numer seryjny:** `AM-SRE-VIG-2026-000002` · **Agent ID:** `AM-OCAF-AI-0002` · **Nr katalogowy:** `AM-OCAF-SRE-002` · **Certyfikat:** `CERT-AM-2026-0002` — REZERWACJA, NIEWYDANY.

**Stan:** `INCOMPLETE_DEVELOPMENT`. Lokalny kod Rust do celu: diagnostyka incydentów infrastruktury. Nie jest to kompletna migracja projektu źródłowego ani aktywny produkt OMAs/OMAt/OSTc.

## Wykonany fragment

analiza operatorowych sygnałów K8s bez czytania klastra, bez wykonywania shell/kubectl i bez automatycznych napraw. Lokalna funkcja `incident.triage` w Rust działa bez sieci, kluczy dostawców, źródeł OSTc i efektów ubocznych. `Cargo.toml` zawiera przypięty git `ocaf_runtime`, adapter `ToolManifest/ToolExecutor` jest lokalny. Rejestracja i transport OMAs/OMAt przez Zenoh **nie zostały wdrożone sieciowo**.

## Sposób uruchomienia

```sh
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-002/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-002/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-002/Cargo.toml" -- "CrashLoopBackOff"
```

## Spis własnych dokumentów

[ARCHITECTURE](ARCHITECTURE.md) · [SPECIFICATION](SPECIFICATION.md) · [USER_MANUAL](USER_MANUAL.md) · [SECURITY](SECURITY.md) · [SERVICE](SERVICE.md) · [IDENTITY_AND_SERIAL](IDENTITY_AND_SERIAL.md) · [MIGRATION](MIGRATION.md) · [TEST_EVIDENCE](TEST_EVIDENCE.md) · [RELEASE_CHECKLIST](RELEASE_CHECKLIST.md) · [CHANGELOG](CHANGELOG.md) · [CERTIFICATE_RECORD](../certification/CERTIFICATE_RECORD.md) · [Umowa sprzedaży](commercial/SALE-DRAFT.md) · [Umowa dzierżawy](commercial/LEASE-DRAFT.md) · [Umowa najmu](commercial/RENT-DRAFT.md).

**Pochodzenie:** [specyfikacja InfraSentinel / P121](../../../projekty/121-infrasentinel-agentic-autonomous-sre-infrastructure/README.md); z projektu pochodzi koncepcja, nie plik wykonywalny. Projekt pozostaje w `projekty/` jako system macierzysty.
