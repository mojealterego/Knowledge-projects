# FIDES GRANTIS — OCAF-AGENT-012

**FIDES GRANTIS** · ewidencja **012** · serial `AM-GRT-FID-2026-000012` · Agent ID `AM-OCAF-AI-0012` · katalog `AM-OCAF-GRT-012` · certyfikat `CERT-AM-2026-0012` **NIEWYDANY**.

**STATUS: INCOMPLETE_DEVELOPMENT | `certificate_issued=false` | `production_approved=false`.**

## Cel i granica funkcji

kontrola deklaracji o programach finansowania i zakończonych wydarzeniach, bez wysyłania wniosków. Sprawdza wyłącznie repo Knowledge-projects, działania research/draft, brak credentiali, zakres deklarowanych kredytów i tokenów programu Alibaba AI Catalyst, historyczny status General Learning Hacks i wymaganie zweryfikowanej kwalifikowalności przy tworzeniu szkicu.

Kod oparto na rzeczywistym [`tools/program_claim_gate.py`](../../../tools/program_claim_gate.py) i jego [testach Python](../../../tools/test_program_claim_gate.py). Wynik permitted=true jest lokalną decyzją walidatora; nie potwierdza otrzymania pieniędzy ani grantów, nie składa wniosku, nie uruchamia obliczeń i nie sprawdza na żywo stanu programów. Czas UTC pochodzi od hosta, nie z modelowego JSON.

## Kontrole lokalne

```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-012/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-012/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-012/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-012/Cargo.toml" -- demo
```

Działające funkcje lokalne Rust: `src/agent.rs` oraz lokalny OCAF ToolExecutor w `src/zenoh.rs`. OMAs jest oddzielnym docelowym **serwerem agentów**, OMAt **serwerem narzędzi**, natomiast Zenoh pozostaje wyłączonym transportem sieciowym (tylko klucze tras). OSTc nie ma jeszcze zweryfikowanej specyfikacji; dostęp do źródeł jest zabroniony.

Dokumenty: [ARCHITECTURE](ARCHITECTURE.md), [SPECIFICATION](SPECIFICATION.md), [USER_MANUAL](USER_MANUAL.md), [SECURITY](SECURITY.md), [SERVICE](SERVICE.md), [IDENTITY_AND_SERIAL](IDENTITY_AND_SERIAL.md), [PYTHON_PARITY](PYTHON_PARITY.md), [DEPENDENCIES](DEPENDENCIES.md), [MIGRATION](MIGRATION.md), [RELEASE_CHECKLIST](RELEASE_CHECKLIST.md), [TRACEABILITY_MATRIX](TRACEABILITY_MATRIX.md), [TEST_EVIDENCE](TEST_EVIDENCE.md), [CHANGELOG](CHANGELOG.md), [CERTIFICATE_RECORD](../certification/CERTIFICATE_RECORD.md), [SALE-DRAFT](commercial/SALE-DRAFT.md), [LEASE-DRAFT](commercial/LEASE-DRAFT.md), [RENT-DRAFT](commercial/RENT-DRAFT.md).
