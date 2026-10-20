# Łańcuch zależności — PROVENANT EVIDENCE

**PROVENANT EVIDENCE** · ewidencja **013** · serial `AM-PRV-EVD-2026-000013` · Agent ID `AM-OCAF-AI-0013` · katalog `AM-OCAF-PRV-013` · certyfikat `CERT-AM-2026-000013` (**rezerwacja; niewydany**).

Rust stable 2021, `serde`, `serde_json`, `toml`, opcjonalne `ocaf_runtime` na przypiętej rewizji `52d4f597e0b481678f90cf2073b721ad2e99e253`, `async-trait`, `tokio` testowe. Hostowy trust registry oparty na `BTreeSet<String>`; brak zewnętrznej biblioteki sieciowej OSINT.

Zależność git ma przypięty `rev`, ale nie istnieje niezależne SBOM, signed binary lub proof reproducibility. W docelowej polityce OCAF Runtime może być konieczna zamiana zależności git na zweryfikowaną lokalną dostawę kodu. Nie określono licencji komercyjnej i audytu third-party.
