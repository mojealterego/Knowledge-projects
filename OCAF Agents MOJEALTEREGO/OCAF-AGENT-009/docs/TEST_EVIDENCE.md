# Dowody CI — CLOUD FISCALIS

**Nazwa własna:** CLOUD FISCALIS · **Numer ewidencyjny:** 009 · **Serial:** `AM-CLD-FIS-2026-000009` · **Agent ID:** `AM-OCAF-AI-0009` · **Katalog:** `AM-OCAF-CLD-009` · **Certyfikat:** `CERT-AM-2026-0009` — NIEWYDANY.

## Zestaw walidacji

Workflow `.github/workflows/ocaf-agents-008-009.yml` uruchamia `cargo fmt --check`, `cargo test --all-features`, strict Clippy, CLI demo oraz referencyjny test Python `tools/test_cloud_operation_budget_gate.py`.

| Bramka | Stan na etapie implementacji |
|---|---|
| T01 — spójność ewidencji | PR CI required |
| T02 — Rust unit + integration + security | PR CI required |
| T03 — OMAs RPC certificate and signature | NOT IMPLEMENTED |
| T04 — tenant isolation | NOT RUN |
| T05/T06 — jobs i fenced effects | NOT RUN |
| T07/T08 — failure/recovery niezależnych hostów | NOT RUN |
| T09 — sandbox actual execution | NOT RUN |
| T10 — secret isolation | NOT RUN |
| T11 — OSTc crypto vectors | BLOCKED |
| T12 — SBOM / artifacts / provenance | NOT DONE |
| T13–T15 — audit, revocation, operator acceptance | NOT DONE |

Nie twierdzić, że demonstracyjne `admitted=true` lub `allowed=true` jest produkcyjnym grantem. Certyfikat CERT-AM-2026-0009 jest nadal niewydany.
