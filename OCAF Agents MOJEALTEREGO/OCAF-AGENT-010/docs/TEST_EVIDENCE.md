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
