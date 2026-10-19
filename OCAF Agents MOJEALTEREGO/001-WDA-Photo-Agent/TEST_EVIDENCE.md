# Macierz bramek T01–T15 — Agent 001

2026-10-19. Wyniki nowej gałęzi **NOT RUN, dopóki nie ma konkretnych run ID i logów**.

| Test | Obszar | Stan |
|---|---|---|
| T01 | Manifest, identyfikatory, SHA i schematy | PARTIAL |
| T02 | Rust build, fmt, clippy, unit tests | NOT RUN |
| T03 | OMAs podpis ↔ RPC | NOT RUN |
| T04 | Cross-tenant isolation | NOT RUN |
| T05 | Jobs authorization | NOT RUN |
| T06 | Fenced effects | NOT RUN |
| T07 | Crash SIGKILL/restart | NOT RUN |
| T08 | Multi-host | NOT RUN |
| T09 | Tool sandbox/timeouts | NOT RUN |
| T10 | Secrets leakage | NOT RUN |
| T11 | OSTc test vectors | BLOCKED |
| T12 | Supply chain/SBOM/provenance | NOT RUN |
| T13 | Observability | NOT RUN |
| T14 | Lifecycle/revoke/restore | NOT RUN |
| T15 | Operator acceptance | NOT RUN |

**Wymagany status:** production_approved=false. Dowody, które mogą pojawić się w CI, potwierdzają tylko ich rzeczywisty zakres.


## Zweryfikowane uruchomienie CI — 19.10.2026

- **PASS:** https://github.com/mojealterego/Knowledge-projects/actions/runs/39198615847
- commit testowany: `d8a03875b4ac7b3c7c3e4b3dd3361306541a440a`; job: `117886684565`; środowisko GitHub-hosted ubuntu-latest.
- JS syntax: PASS; Rust fmt: PASS; **3 unit + 3 OCAF integration = 6/6 PASS**; clippy all targets/features, warnings denied: PASS.
- Poprzednia próba `39198511267`: FAIL na `cargo fmt`, skorygowano i powtórzono; nie jest dowodem weryfikacji funkcjonalnej.
- Zakres PASS ogranicza się do modułu `photo.plan`, OCAF local executor i negatywnej kontroli unsigned peer, **nie** pełnego OMAs RPC, multi-host, providerów Adobe/OpenAI czy OSTc.

**T02 = PASS na wyżej wymienionym SHA.** T01 manifest i T04 fragment negatywnych testów = PARTIAL; pozostałe bramki jak powyżej. Numer nie jest certyfikatem wydania.
