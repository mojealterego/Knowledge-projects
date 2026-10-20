# Rejestr dowodów — PROVENANT EVIDENCE

**PROVENANT EVIDENCE** · ewidencja **011** · serial `AM-PRV-EVD-2026-000011` · Agent ID `AM-OCAF-AI-0011` · katalog `AM-OCAF-PRV-011` · certyfikat `CERT-AM-2026-000011` (**rezerwacja; niewydany**).

Workflow `.github/workflows/ocaf-agents-010-011.yml` sprawdza Rustfmt, `cargo test --all-features`, Clippy z `-D warnings`, CLI demo i oryginalny Python unittest `tools/test_osint_provenance_gate.py`.

| Bramka | Stan |
|---|---|
| T01 rejestr i niepowtarzalność | WAIT FOR CI |
| T02 Rust unit/integration/security | WAIT FOR CI |
| T03 OMAs signed RPC | NOT RUN |
| T04 multi-tenant real hosts | NOT RUN |
| T05/T06 jobs i fenced effects | NOT RUN |
| T07/T08 failover/HA host recovery | NOT RUN |
| T09 sandbox execution, injection | NO EXECUTION; HOST TESTS NOT RUN |
| T10 provider secrets | E2E NOT RUN |
| T11 OSTc vectors | BLOCKED |
| T12 SBOM/provenance/signatures | NOT RUN |
| T13–T15 logi, revocation, operator acceptance | NOT RUN |

Później wpisać aktualny commit, run ID, liczbę wyników PASS. Certyfikat CERT-AM-2026-000011 niewydany.

## Faktyczne dowody CI — 20.10.2026

- [GitHub Actions 39422580144 — SUCCESS](https://github.com/mojealterego/Knowledge-projects/actions/runs/39422580144), kod commit `24b269a82e208c7eaeccdcdf380f5e61e88dd25b`, job `118626995960`.
- **17/17 Rust PASS, 0 failed**, formatowanie Rustfmt PASS, strict Clippy PASS, CLI demo PASS.
- **8/8 oryginalnych Python unittest PASS**, bez zmiany pierwowzoru. Nie jest to dowód pełnej formalnej równoważności Python↔Rust.
- Regresje na tym samym SHA: [002–003](https://github.com/mojealterego/Knowledge-projects/actions/runs/39422580121), [004–005](https://github.com/mojealterego/Knowledge-projects/actions/runs/39422580167), [006–007](https://github.com/mojealterego/Knowledge-projects/actions/runs/39422579938), [008–009](https://github.com/mojealterego/Knowledge-projects/actions/runs/39422580051) — SUCCESS.
- T01/T02 LOCAL VERIFIED; T03–T15 nadal niezamknięte. Brak certyfikatu, OMAs/OMAt/Zenoh network/OSTc, signed artifact i operator acceptance.
