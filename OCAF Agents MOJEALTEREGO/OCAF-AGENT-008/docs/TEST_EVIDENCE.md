# Dowody CI — DGM SENTINEL

**Nazwa własna:** DGM SENTINEL · **Numer ewidencyjny:** 008 · **Serial:** `AM-DGM-SNT-2026-000008` · **Agent ID:** `AM-OCAF-AI-0008` · **Katalog:** `AM-OCAF-DGM-008` · **Certyfikat:** `CERT-AM-2026-0008` — NIEWYDANY.

## Zestaw walidacji

Workflow `.github/workflows/ocaf-agents-008-009.yml` uruchamia `cargo fmt --check`, `cargo test --all-features`, strict Clippy, CLI demo oraz referencyjny test Python `tools/test_dgm_cycle_gate.py`.

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

Nie twierdzić, że demonstracyjne `admitted=true` lub `allowed=true` jest produkcyjnym grantem. Certyfikat CERT-AM-2026-0008 jest nadal niewydany.


## ZWERYFIKOWANY PRE-MERGE BUILD — 2026-10-20

- **[GitHub Actions run 39419346496 — SUCCESS](https://github.com/mojealterego/Knowledge-projects/actions/runs/39419346496)**; SHA `e6c7c1257da64bf12aebfaa094e2b9512f611815`; job `118616109453`.
- **16/16 Rust PASS, 0 failures** (domena, unikatowość ID, security, lokalny OCAF); formatowanie PASS, strict Clippy PASS, CLI demo PASS.
- **13/13 oryginalnych Python unittest PASS**; oryginał bez zmian. To potwierdza prawidłową regresję źródła, lecz nie formalną równoważność wszystkich wejść Python/Rust.
- Regresja [002–003](https://github.com/mojealterego/Knowledge-projects/actions/runs/39419346529), [004–005](https://github.com/mojealterego/Knowledge-projects/actions/runs/39419346492) i [006–007](https://github.com/mojealterego/Knowledge-projects/actions/runs/39419347331) na tym samym SHA również SUCCESS.
- T01/T02 są lokalnie potwierdzone. T03–T15 nieukończone, brak OMAs/OMAt/Zenoh network i OSTc. **Brak produkcyjnej certyfikacji**.
