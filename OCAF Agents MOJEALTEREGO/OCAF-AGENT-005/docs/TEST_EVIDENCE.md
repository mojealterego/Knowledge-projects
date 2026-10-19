# Dowody techniczne — ARGUS MOBILE

**ARGUS MOBILE** / ewidencja **005** / serial `AM-MOB-ARG-2026-000005` / ID `AM-OCAF-AI-0005` / katalog `AM-OCAF-MOB-005` / certyfikat `CERT-AM-2026-0005` **NIEWYDANY**.

## Rejestr bramek T01–T15

| Test | Cel | Stan |
|---|---|---|
| T01 | zgodność manifest/nazwa/serial/globalny rejestr | PENDING CI |
| T02 | cargo fmt, test, clippy, CLI | PENDING CI |
| T03 | kryptograficznie związany podpis OMAs i tożsamość RPC | NOT RUN |
| T04 | izolacja tenantów na realnych hostach | NOT RUN |
| T05 | autoryzowane jobs | NOT RUN |
| T06 | fenced effects | NOT RUN |
| T07 | crash/restart | NOT RUN |
| T08 | multi-host | NOT RUN |
| T09 | runtime sandbox | local read-only, E2E NOT RUN |
| T10 | sekrety/providerzy | E2E NOT RUN |
| T11 | OSTc test vectors | BLOCKED |
| T12 | SBOM/provenance/signatures | NOT RUN |
| T13 | logi/observability | NOT RUN |
| T14 | revoke/restore | NOT RUN |
| T15 | operator acceptance | NOT RUN |

Ostateczne **run ID i commit** należy wpisać po dowodzie, nie na podstawie zapewnienia modelu. Certyfikat `CERT-AM-2026-0005` zarezerwowany, **niewydany**; `production_approved=false`.

## Potwierdzone GitHub Actions — 20.10.2026

- **[Run #39340442615 — SUCCESS](https://github.com/mojealterego/Knowledge-projects/actions/runs/39340442615)**, commit `b1eae89e9f634a743adbccf291bae7782a08901a`, job `118360346217`, `ubuntu-latest`.
- **13/13 testów Rust PASS, 0 failed**: 2 unit, 3 identity, 2 domain integration, 2 local OCAF adapter, 4 security; Cargo fmt PASS, strict Clippy PASS, CLI smoke PASS.
- T01: spójność administracyjnej tożsamości, serialu i globalnego rejestru PASS; brak realnego podpisanego artifact digest. T02 PASS dla podanego SHA.
- Test zdalnego peer bez podpisu odrzucany przez lokalne API OCAF; to nie jest produkcyjna komunikacja OMAs/OMAt.
- Pozostałe T03–T15 **niezamknięte**, w tym OSTc T11 BLOCKED. Formalny certyfikat nadal niewydany; status `INCOMPLETE_DEVELOPMENT`.
