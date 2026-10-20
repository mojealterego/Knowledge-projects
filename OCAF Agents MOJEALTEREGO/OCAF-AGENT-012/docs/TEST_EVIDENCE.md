# Rejestr dowodów — ZIP WARDEN

**ZIP WARDEN** · ewidencja **012** · serial `AM-ARC-ZWD-2026-000012` · Agent ID `AM-OCAF-AI-0012` · katalog `AM-OCAF-ARC-012` · certyfikat `CERT-AM-2026-000012` (**rezerwacja; niewydany**).

Workflow `.github/workflows/ocaf-agents-012-013.yml` sprawdza Rustfmt, `cargo test --all-features`, Clippy z `-D warnings`, CLI demo i oryginalny Python unittest `tools/test_source_bundle_audit.py`.

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

Później wpisać aktualny commit, run ID, liczbę wyników PASS. Certyfikat CERT-AM-2026-000012 niewydany.

## Faktyczne dowody CI — 20.10.2026

- [GitHub Actions 39422580144 — SUCCESS](https://github.com/mojealterego/Knowledge-projects/actions/runs/39422580144), kod commit `24b269a82e208c7eaeccdcdf380f5e61e88dd25b`, job `118626996236`.
- **18/18 Rust PASS, 0 failed**, formatowanie Rustfmt PASS, strict Clippy PASS, CLI demo PASS.
- **11/11 oryginalnych Python unittest PASS**, bez zmiany pierwowzoru. Nie jest to dowód pełnej formalnej równoważności Python↔Rust.
- Regresje na tym samym SHA: [002–003](https://github.com/mojealterego/Knowledge-projects/actions/runs/39422580121), [004–005](https://github.com/mojealterego/Knowledge-projects/actions/runs/39422580167), [006–007](https://github.com/mojealterego/Knowledge-projects/actions/runs/39422579938), [008–009](https://github.com/mojealterego/Knowledge-projects/actions/runs/39422580051) — SUCCESS.
- T01/T02 LOCAL VERIFIED; T03–T15 nadal niezamknięte. Brak certyfikatu, OMAs/OMAt/Zenoh network/OSTc, signed artifact i operator acceptance.
