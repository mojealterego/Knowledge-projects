# CLOUD FISCALIS — OCAF-AGENT-009

**Nazwa własna:** CLOUD FISCALIS · **Numer ewidencyjny:** 009 · **Serial:** `AM-CLD-FIS-2026-000009` · **Agent ID:** `AM-OCAF-AI-0009` · **Katalog:** `AM-OCAF-CLD-009` · **Certyfikat:** `CERT-AM-2026-0009` — NIEWYDANY.

**Stan: `INCOMPLETE_DEVELOPMENT` · certyfikat niewydany · brak zgody produkcyjnej.**

## Cel i faktyczny zakres

ocena planu operacji Google Cloud pod względem produktu, zakresu projektu, kosztu i niezależnej zgody. Port rzeczywistego modułu [`tools/cloud_operation_budget_gate.py`](../../../tools/cloud_operation_budget_gate.py) i testów [`tools/test_cloud_operation_budget_gate.py`](../../../tools/test_cloud_operation_budget_gate.py). Walidacja ośmiu kategorii produktów i akcji, strict decimal-string USD przez rust_decimal, request_id, project_scope, allowlist grantów, budżetu i zakazu sekretów w promptach. Dokumentacja publiczna wyłącznie przy zerowym koszcie i publicznej klasyfikacji.

**Granica zaufania:** CloudApproval powstaje w niezależnym systemie polityki właściciela/hosta i jest przekazywane do AgentTool; pole modelu explicit_external_side_effect_approval nie stanowi IAM, rozliczenia ani zgody na rzeczywiste wykonanie.

Lokalny pakiet Rust `cloud_fiscalis_agent` nie wykonuje kodu innych agentów, nie promuje gałęzi, nie wywołuje GCP, nie pobiera opłat, nie uruchamia mikro-VM. Status `admitted/allowed` oznacza wynik przeglądu offline, nie dopuszczenie runtime przez OMAs/OMAt.

## Testy

```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-009/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-009/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-009/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-009/Cargo.toml" -- demo
```

CI uruchamia też źródłowy Python unittest. Dokumentacja: [ARCHITECTURE](ARCHITECTURE.md), [SPECIFICATION](SPECIFICATION.md), [USER_MANUAL](USER_MANUAL.md), [SECURITY](SECURITY.md), [SERVICE](SERVICE.md), [IDENTITY_AND_SERIAL](IDENTITY_AND_SERIAL.md), [PYTHON_PARITY](PYTHON_PARITY.md), [DEPENDENCIES](DEPENDENCIES.md), [MIGRATION](MIGRATION.md), [RELEASE_CHECKLIST](RELEASE_CHECKLIST.md), [TRACEABILITY_MATRIX](TRACEABILITY_MATRIX.md), [TEST_EVIDENCE](TEST_EVIDENCE.md), [CHANGELOG](CHANGELOG.md), [CERTIFICATE_RECORD](../certification/CERTIFICATE_RECORD.md), [Umowa sprzedaży](commercial/SALE-DRAFT.md), [Dzierżawy](commercial/LEASE-DRAFT.md), [Najmu](commercial/RENT-DRAFT.md).

Rzeczywiste serwery OMAs i OMAt, działający transport Zenoh TLS oraz kryptografia OSTc **NIE SĄ wdrożone**. OSTc jest zablokowany do otrzymania dokumentacji i niezależnego audytu.
