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
