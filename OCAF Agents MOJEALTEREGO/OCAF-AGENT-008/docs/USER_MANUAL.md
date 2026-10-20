# Instrukcja obsługi — DGM SENTINEL

**Nazwa własna:** DGM SENTINEL · **Numer ewidencyjny:** 008 · **Serial:** `AM-DGM-SNT-2026-000008` · **Agent ID:** `AM-OCAF-AI-0008` · **Katalog:** `AM-OCAF-DGM-008` · **Certyfikat:** `CERT-AM-2026-0008` — NIEWYDANY.

## Instalacja i test

W checkout `mojealterego/Knowledge-projects` z Rust stable:

```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-008/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-008/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-008/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-008/Cargo.toml" -- demo
```

`demo` nie łączy się z chmurą ani GitHub, tylko analizuje dane fixture; nie przekazuj sekretów API.

## Lokalny host OCAF

Host rejestruje `zenoh::manifest()` i `AgentTool` pod nazwą `dgm.assess_promotion`. Zatwierdzone kandydatury, potwierdzone CI, receipt faz, rollback i source provenance muszą być wstrzyknięte przez zaufanego hosta jako PromotionTrust. Samo rollback_ref_verified=true lub source_provenance_verified=true w prośbie modelu nie ma autorytatywnej mocy.

Zewnętrzny serwer musi sam sprawdzić rzeczywiste identyfikatory, uprawnienia IAM/PR approvals, digest, logi CI, terminy i politykę. Jeśli brakuje danych, wynik odmowy jest prawidłowy i należy zebrać dowody od operatora.

## Niepowodzenia

Niezgodny numer seryjny/Agent ID → `InvalidConfiguration`. Złe dane JSON i nieznane pola → `ToolError::Validation`. OSTc → `SourceEncryptionNotImplemented`. Produkcyjne `Active` → `DeploymentNotAuthorized`. Zabezpieczaj logi przed sekretami, zapisuj commit SHA i run ID.
