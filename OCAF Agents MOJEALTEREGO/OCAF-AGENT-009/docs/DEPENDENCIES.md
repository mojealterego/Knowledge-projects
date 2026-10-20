# Zależności / provenance — CLOUD FISCALIS

**Nazwa własna:** CLOUD FISCALIS · **Numer ewidencyjny:** 009 · **Serial:** `AM-CLD-FIS-2026-000009` · **Agent ID:** `AM-OCAF-AI-0009` · **Katalog:** `AM-OCAF-CLD-009` · **Certyfikat:** `CERT-AM-2026-0009` — NIEWYDANY.

Rust 2021, `serde`, `serde_json`, `toml`, opcjonalny `async-trait` i `ocaf_runtime` pin `52d4f597e0b481678f90cf2073b721ad2e99e253`; `tokio` używany w testach. `rust_decimal` implementuje budżet bez błędów arytmetyki zmiennoprzecinkowej.

Bez podpisanego SBOM, binarnego digestu, realnego issuer, reprodukowalnego builda, audytu supply chain. Git dependency może nie przejść docelowego zakazu takich zależności w OCAF; niniejsza implementacja nie rości sobie zgodności całego systemu. Oryginały Python pozostają w `tools/`, nie usunięto plików.
