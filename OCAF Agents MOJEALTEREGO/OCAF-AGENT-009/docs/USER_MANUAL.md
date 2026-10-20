# Instrukcja obsługi — CLOUD FISCALIS

**Nazwa własna:** CLOUD FISCALIS · **Numer ewidencyjny:** 009 · **Serial:** `AM-CLD-FIS-2026-000009` · **Agent ID:** `AM-OCAF-AI-0009` · **Katalog:** `AM-OCAF-CLD-009` · **Certyfikat:** `CERT-AM-2026-0009` — NIEWYDANY.

## Instalacja i test

W checkout `mojealterego/Knowledge-projects` z Rust stable:

```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-009/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-009/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-009/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-009/Cargo.toml" -- demo
```

`demo` nie łączy się z chmurą ani GitHub, tylko analizuje dane fixture; nie przekazuj sekretów API.

## Lokalny host OCAF

Host rejestruje `zenoh::manifest()` i `AgentTool` pod nazwą `cloud.review_budget`. CloudApproval powstaje w niezależnym systemie polityki właściciela/hosta i jest przekazywane do AgentTool; pole modelu explicit_external_side_effect_approval nie stanowi IAM, rozliczenia ani zgody na rzeczywiste wykonanie.

Zewnętrzny serwer musi sam sprawdzić rzeczywiste identyfikatory, uprawnienia IAM/PR approvals, digest, logi CI, terminy i politykę. Jeśli brakuje danych, wynik odmowy jest prawidłowy i należy zebrać dowody od operatora.

## Niepowodzenia

Niezgodny numer seryjny/Agent ID → `InvalidConfiguration`. Złe dane JSON i nieznane pola → `ToolError::Validation`. OSTc → `SourceEncryptionNotImplemented`. Produkcyjne `Active` → `DeploymentNotAuthorized`. Zabezpieczaj logi przed sekretami, zapisuj commit SHA i run ID.
