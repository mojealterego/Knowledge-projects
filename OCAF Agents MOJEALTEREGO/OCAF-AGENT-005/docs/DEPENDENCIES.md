# Zależności i proweniencja — ARGUS MOBILE

**Nazwa własna:** ARGUS MOBILE · **Ewidencja:** 005 · **Numer seryjny:** `AM-MOB-ARG-2026-000005` · **Agent ID:** `AM-OCAF-AI-0005` · **Katalog:** `AM-OCAF-MOB-005` · **Nr certyfikatu:** `CERT-AM-2026-0005` (rezerwacja, niewydany).

Rust `argus_mobile_agent` (edition 2021); `serde`, `serde_json`, `toml`, opcjonalnie `async-trait`, `ocaf_runtime` i `tokio` w testach. Zależność `ocaf_runtime` przypięta do rewizji `52d4f597e0b481678f90cf2073b721ad2e99e253` za pomocą `Cargo.toml`.

**Ryzyko:** zależność git mimo SHA nie zastępuje SBOM, audytu zależności ani proweniencji podpisanego wydania; może być sprzeczna z polityką docelowego OCAF Runtime. `Cargo.lock`, SBOM, digest binarny i podpis operatora nie są traktowane jako istniejące bez autorytatywnego artefaktu.

Prawdziwe serwery OMAs/OMAt i kanał Zenoh nie zostały uruchomione; OSTc czeka na źródłową specyfikację; żadne dane mobilne/kod projektu źródłowego nie są przetwarzane automatycznie.
