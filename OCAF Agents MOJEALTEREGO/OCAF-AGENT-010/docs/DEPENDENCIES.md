# Zależności i supply chain — VERITAS TRACE

**Nazwa:** VERITAS TRACE · **Ewidencja:** 010 · **Serial:** `AM-OSN-VRT-2026-000010` · **Agent ID:** `AM-OCAF-AI-0010` · **Katalog:** `AM-OCAF-OSN-010` · **Certyfikat:** `CERT-AM-2026-0010` — NIEWYDANY.

Rust 2021, `serde`, `serde_json`, `toml`, opcjonalnie `async-trait`, `ocaf_runtime` na stałym `git rev=52d4f597e0b481678f90cf2073b721ad2e99e253`, `tokio` w testach. `#![forbid(unsafe_code)]` w `src/lib.rs`.

Brak rzeczywistego podpisu zależności, zewnętrznego audytu, SBOM, proweniencji CI dostarczonej operatorowi, reprodukowalnej binarki i runtime źródeł OSTc. Git dependency mimo przypiętego SHA może naruszać docelową politykę produkcyjnego OCAF Runtime; potrzebny przegląd. Oryginał `tools/osint_provenance_gate.py` został zachowany w repo.
