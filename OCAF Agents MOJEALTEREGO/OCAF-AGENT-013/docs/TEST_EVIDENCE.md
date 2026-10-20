# Dowody testów — NEXUS CONSENT

**NEXUS CONSENT** · ewidencja **013** · serial `AM-COM-NEX-2026-000013` · Agent ID `AM-OCAF-AI-0013` · katalog `AM-OCAF-COM-013` · certyfikat `CERT-AM-2026-0013` **NIEWYDANY**.

Workflow: `.github/workflows/ocaf-agents-012-013.yml`, Rust fmt, `cargo test --all-features`, `cargo clippy -D warnings`, oryginalny Python unittest i CLI smoke.

| Bramka | Stan |
|---|---|
| T01 identyfikatory bez kolizji | CI PENDING |
| T02 local Rust unit/integration + original Python | CI PENDING |
| T03 OMAs signed RPC i OMAt authorization | NOT IMPLEMENTED |
| T04 tenant isolation i multi-host | NOT RUN |
| T05/T06 jobs, fenced effects | NOT RUN |
| T07/T08 failure/recovery | NOT RUN |
| T09 sandbox | NOT RUN |
| T10 secret/provider validation | NOT RUN |
| T11 OSTc crypto vectors | BLOCKED |
| T12 signed artifacts i SBOM | NOT RUN |
| T13/T14/T15 observability, revocation, operator acceptance | NOT RUN |

Dopisać faktyczny run ID/commit dopiero po zakończeniu i odczycie GitHub Actions. Certyfikat `CERT-AM-2026-0013` niewydany.

## Wykonany test CI na kodzie Rust — 20.10.2026

- [GitHub Actions run **39468676335 — SUCCESS**](https://github.com/mojealterego/Knowledge-projects/actions/runs/39468676335).
- Zweryfikowany SHA kodu: `7dd5bbe3de4c0cff05cbcc50c4e324c166efcbf1`; job `118786679101`.
- Agent 013: **16 testów Rust PASS, 0 failed** i **14 oryginalnych testów Python PASS**.
- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo run -- demo` również **PASS**.
- Jest to dowód lokalnego portu; testy realnego OMAs/OMAt/Zenoh TLS, OSTc i wielohostowej infrastruktury **NOT RUN**.
- `certificate_issued=false`; `production_approved=false`; formalny odbiór nie nastąpił.

