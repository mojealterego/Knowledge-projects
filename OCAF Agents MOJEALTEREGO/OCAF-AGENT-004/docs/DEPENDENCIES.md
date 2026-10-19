# Zależności i proweniencja — AXIOM FORGE

**Nazwa własna:** AXIOM FORGE · **Ewidencja:** 004 · **Numer seryjny:** `AM-ENG-AXF-2026-000004` · **Agent ID:** `AM-OCAF-AI-0004` · **Katalog:** `AM-OCAF-ENG-004` · **Nr certyfikatu:** `CERT-AM-2026-0004` (rezerwacja, niewydany).

Rust `axiom_forge_agent` (edition 2021); `serde`, `serde_json`, `toml`, opcjonalnie `async-trait`, `ocaf_runtime` i `tokio` w testach. Zależność `ocaf_runtime` przypięta do rewizji `52d4f597e0b481678f90cf2073b721ad2e99e253` za pomocą `Cargo.toml`.

**Ryzyko:** zależność git mimo SHA nie zastępuje SBOM, audytu zależności ani proweniencji podpisanego wydania; może być sprzeczna z polityką docelowego OCAF Runtime. `Cargo.lock`, SBOM, digest binarny i podpis operatora nie są traktowane jako istniejące bez autorytatywnego artefaktu.

Prawdziwe serwery OMAs/OMAt i kanał Zenoh nie zostały uruchomione; OSTc czeka na źródłową specyfikację; żadne dane mobilne/kod projektu źródłowego nie są przetwarzane automatycznie.
