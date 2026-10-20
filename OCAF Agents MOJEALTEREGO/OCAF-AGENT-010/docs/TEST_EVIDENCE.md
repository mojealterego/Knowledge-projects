# Rejestr testów — VERITAS TRACE

**Nazwa:** VERITAS TRACE · **Ewidencja:** 010 · **Serial:** `AM-OSN-VRT-2026-000010` · **Agent ID:** `AM-OCAF-AI-0010` · **Katalog:** `AM-OCAF-OSN-010` · **Certyfikat:** `CERT-AM-2026-0010` — NIEWYDANY.

Workflow: `.github/workflows/ocaf-agents-010-011.yml`, Rust fmt, `cargo test --all-features`, `cargo clippy -D warnings`, Python `tools/test_osint_provenance_gate.py` oraz CLI demo.

| Grupa | Status przed końcowym run |
|---|---|
| T01 — ewidencja/serial/globalny rejestr | CI PENDING |
| T02 — unit, Rust integration, Clippy, format, demo | CI PENDING |
| Źródłowe Python unittest | CI PENDING |
| T03 — OMAs signed RPC | NOT IMPLEMENTED |
| T04 — tenant multihost | NOT RUN |
| T05/T06 — jobs/fenced effects | NOT RUN |
| T07/T08 — recovery/failure, host isolation | NOT RUN |
| T09 — prawdziwy sandbox narzędzi | NOT RUN |
| T10 — secret isolation i provider access | NOT RUN |
| T11 — OSTc test vectors | BLOCKED |
| T12 — SBOM/podpisany artefakt | NOT RUN |
| T13–T15 — observability/revoke/operator acceptance | NOT RUN |

**Dopisać wyłącznie rzeczywiste run ID, job ID i commit** po zakończeniu GitHub Actions. Brak certyfikatu; `production_approved=false`.


## Rzeczywisty wynik kodu — 20.10.2026

- [GitHub Actions #39422100249 — SUCCESS](https://github.com/mojealterego/Knowledge-projects/actions/runs/39422100249), SHA `5d36fb120f0250a957cc7279e40b4ec1e3c65127`, job `118625348857`.
- **16/16 testów Rust PASS, 0 failed**: domena, tożsamość, testowy OCAF, security.
- **8/8 pierwotnych testów Python PASS**, źródło bez modyfikacji.
- Format PASS, strict Clippy PASS, CLI smoke PASS. Wynik nie potwierdza pełnej równoważności z Python ani produkcyjnego OMAs/OMAt, TLS, OSTc, multi-host, operator acceptance czy certyfikacji.
- Dodatkowo wymagana weryfikacja na SHA ostatniej rewizji PR (dokumentacja/registries), oraz po merge na `main`.
