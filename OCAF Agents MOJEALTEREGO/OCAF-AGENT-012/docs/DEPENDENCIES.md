# Łańcuch zależności — ZIP WARDEN

**ZIP WARDEN** · ewidencja **012** · serial `AM-ARC-ZWD-2026-000012` · Agent ID `AM-OCAF-AI-0012` · katalog `AM-OCAF-ARC-012` · certyfikat `CERT-AM-2026-000012` (**rezerwacja; niewydany**).

Rust stable 2021, `serde`, `serde_json`, `toml`, opcjonalne `ocaf_runtime` na przypiętej rewizji `52d4f597e0b481678f90cf2073b721ad2e99e253`, `async-trait`, `tokio` testowe. `zip 0.6` (obsługa ZIP + deflate), `sha2 0.10` i `hex 0.4` dla rzeczywistego SHA-256 i wymiany binarnych payloadów.

Zależność git ma przypięty `rev`, ale nie istnieje niezależne SBOM, signed binary lub proof reproducibility. W docelowej polityce OCAF Runtime może być konieczna zamiana zależności git na zweryfikowaną lokalną dostawę kodu. Nie określono licencji komercyjnej i audytu third-party.
