# Zależności / provenance — DGM SENTINEL

**Nazwa własna:** DGM SENTINEL · **Numer ewidencyjny:** 008 · **Serial:** `AM-DGM-SNT-2026-000008` · **Agent ID:** `AM-OCAF-AI-0008` · **Katalog:** `AM-OCAF-DGM-008` · **Certyfikat:** `CERT-AM-2026-0008` — NIEWYDANY.

Rust 2021, `serde`, `serde_json`, `toml`, opcjonalny `async-trait` i `ocaf_runtime` pin `52d4f597e0b481678f90cf2073b721ad2e99e253`; `tokio` używany w testach. `chrono` odpowiada za aware UTC expiration.

Bez podpisanego SBOM, binarnego digestu, realnego issuer, reprodukowalnego builda, audytu supply chain. Git dependency może nie przejść docelowego zakazu takich zależności w OCAF; niniejsza implementacja nie rości sobie zgodności całego systemu. Oryginały Python pozostają w `tools/`, nie usunięto plików.
