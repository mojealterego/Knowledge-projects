# Zależności i ryzyka — FIDES GRANTIS

**FIDES GRANTIS** · ewidencja **012** · serial `AM-GRT-FID-2026-000012` · Agent ID `AM-OCAF-AI-0012` · katalog `AM-OCAF-GRT-012` · certyfikat `CERT-AM-2026-0012` **NIEWYDANY**.

Rust edition 2021; `serde`, `serde_json`, `toml`, opcjonalny `ocaf_runtime` przypięty do `git rev=52d4f597e0b481678f90cf2073b721ad2e99e253`, `async-trait` i `tokio` do testów. `chrono` dla wejścia DateTime UTC host-owned.

Brak formalnego SBOM, podpisu artefaktu, digestu binarki, audytu łańcucha dostaw, prawdziwych serwerów OMAs/OMAt/OSTc oraz zewnętrznych testów. Zależność Git mimo SHA może naruszać przyszłą politykę OCAF Runtime. Oryginalny Python jest zachowany, nie nadpisany.
