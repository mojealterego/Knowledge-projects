# Zależności i ryzyka — NEXUS CONSENT

**NEXUS CONSENT** · ewidencja **013** · serial `AM-COM-NEX-2026-000013` · Agent ID `AM-OCAF-AI-0013` · katalog `AM-OCAF-COM-013` · certyfikat `CERT-AM-2026-0013` **NIEWYDANY**.

Rust edition 2021; `serde`, `serde_json`, `toml`, opcjonalny `ocaf_runtime` przypięty do `git rev=52d4f597e0b481678f90cf2073b721ad2e99e253`, `async-trait` i `tokio` do testów. Macierz protokołów zapisana w statycznym match Rust — nie instaluje radio SDK.

Brak formalnego SBOM, podpisu artefaktu, digestu binarki, audytu łańcucha dostaw, prawdziwych serwerów OMAs/OMAt/OSTc oraz zewnętrznych testów. Zależność Git mimo SHA może naruszać przyszłą politykę OCAF Runtime. Oryginalny Python jest zachowany, nie nadpisany.
