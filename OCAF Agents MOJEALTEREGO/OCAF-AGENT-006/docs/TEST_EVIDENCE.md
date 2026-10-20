# Dowody testowania — CUSTOS ADMISSION

**CUSTOS ADMISSION** · nr ewidencyjny **006** · nr seryjny `AM-SEC-CUS-2026-000006` · Agent ID `AM-OCAF-AI-0006` · katalog `AM-OCAF-SEC-006` · certyfikat `CERT-AM-2026-0006` — **NIEWYDANY**.

**Zakres automatyzacji:** `.github/workflows/ocaf-agents-006-007.yml`. Komendy: Rust fmt, `cargo test --all-features`, clippy strict, CLI smoke.

| Bramka | Oczekiwany dowód | Stan przed finalnym CI |
|---|---|---|
| T01 | tożsamość, serial i globalna ewidencja bez duplikatów | PENDING |
| T02 | Rust tests fmt clippy CLI z commit SHA | PENDING |
| T03 | real OMAs signed RPC | NOT RUN |
| T04 | tenant isolation host-to-host | NOT RUN |
| T05/T06 | jobs i fenced effects | NOT RUN |
| T07/T08 | crash/recovery i multi-host | NOT RUN |
| T09 | rzeczywista izolacja procesowa narzędzi | NOT RUN |
| T10 | secret leakage i uprawnienia providerów | NOT RUN |
| T11 | OSTc test vectors | BLOCKED |
| T12 | SBOM/SLSA/provenance/signature | NOT RUN |
| T13–T15 | obserwowalność, revocation, operator acceptance | NOT RUN |

**Potwierdzenie końcowe**: zapisać rzeczywisty run ID i job z SUCCESS, commit, łączną liczbę testów, nie przypisywać starych dowodów do nowego SHA. `production_approved=false`.

## Potwierdzony CI — 2026-10-20

- [GitHub Actions run 39370400084 — SUCCESS](https://github.com/mojealterego/Knowledge-projects/actions/runs/39370400084), commit `942f518a4f59ab70bedeabf2c9e4990067582e4a`, job `118456739252`.
- **Rust 16/16 PASS, 0 failed**; fmt PASS; `clippy -D warnings` PASS; CLI smoke PASS.
- **Python source unittest 14/14 PASS** w oddzielnym kroku. Oryginalny Python nie został zmieniony i nie jest to formalny różnicowy test równoważności programu.
- [Regresje OCAF Agents 002–003](https://github.com/mojealterego/Knowledge-projects/actions/runs/39370400019) i [004–005](https://github.com/mojealterego/Knowledge-projects/actions/runs/39370400026) na tym samym SHA także SUCCESS.
- Spójność nazwy/serialu i globalna kontrola duplikatów są testowane lokalnie. OMAs/OMAt/Zenoh TLS, OSTc, real multi-host, independent signed approval **NOT IMPLEMENTED / NOT VERIFIED**.
- Certyfikat `CERT-AM-2026-0006` **NIEWYDANY**, `production_approved=false`. Nie wolno promować `Active` na podstawie samego CI.
