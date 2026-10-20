# Zależności i supply chain — LIMES GIT

**Nazwa:** LIMES GIT · **Ewidencja:** 011 · **Serial:** `AM-GIT-LIM-2026-000011` · **Agent ID:** `AM-OCAF-AI-0011` · **Katalog:** `AM-OCAF-GIT-011` · **Certyfikat:** `CERT-AM-2026-0011` — NIEWYDANY.

Rust 2021, `serde`, `serde_json`, `toml`, opcjonalnie `async-trait`, `ocaf_runtime` na stałym `git rev=52d4f597e0b481678f90cf2073b721ad2e99e253`, `tokio` w testach. `#![forbid(unsafe_code)]` w `src/lib.rs`.

Brak rzeczywistego podpisu zależności, zewnętrznego audytu, SBOM, proweniencji CI dostarczonej operatorowi, reprodukowalnej binarki i runtime źródeł OSTc. Git dependency mimo przypiętego SHA może naruszać docelową politykę produkcyjnego OCAF Runtime; potrzebny przegląd. Oryginał `tools/single_repository_scope_gate.py` został zachowany w repo.
