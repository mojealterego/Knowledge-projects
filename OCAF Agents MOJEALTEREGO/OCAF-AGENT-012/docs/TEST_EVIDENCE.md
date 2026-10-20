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

## Renumeracja po współbieżnym scaleniu

Przeniesiono lokalnie testowany kod pod nowy wolny numer, ale historyczny run na innym kodzie/ID nie jest dowodem dla tej wersji. Czekają osobne GitHub Actions dla nowych agentów na nowym SHA.
