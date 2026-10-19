# Zależności, artefakty i łańcuch dostaw — WDA AURELION

## Zakres kompilacji
Pakiet Rust: `wda_photo_agent`, `edition=2021`, `version=0.2.1-dev`, `publish=false`; `Cargo.toml` używa `ocaf_runtime` przez Git z `rev=52d4f597e0b481678f90cf2073b721ad2e99e253` oraz `serde`, `serde_json`, `toml`, `async-trait` i testowy `tokio`. Dokładny graf wersji zależności rozwiązuje Cargo podczas CI — nie jest tu deklarowany zmyślony hash binarium.

## Granice
- `ocaf-runtime` jest źródłem referencyjnym **read-only** w ramach prac repo `Knowledge-projects`.
- Wymagania hardeningu OCAF dotyczą także agenta: Git dependency mimo pin SHA może naruszyć docelową politykę supply-chain; wymaga przeglądu, vendoringu/poświadczonej proweniencji i SCA.
- `legacy-pwa/` ma niezależne źródła JS; `node --check` testuje tylko składnię, nie bezpieczeństwo ani integrację.
- Zależności sieciowe/Cloud API nie zostały uruchomione. Nie istnieje finalny build podpisany kluczem operatora.

## Rejestr dowodów wydania (do uzupełnienia autorytatywnie)
| Pole | Stan |
|---|---|
| Commit źródłowy zatwierdzonego release | BRAK |
| Digest artefaktu binarnego SHA-256 | BRAK |
| Podpis artefaktu i certyfikat publiczny | BRAK |
| SBOM CycloneDX/SPDX | BRAK |
| Ocena RUSTSEC/SCA licencji | BRAK |
| Reproducible build / SLSA provenance | BRAK |
| Audyt zależności OSTc | BLOCKED, brak specyfikacji |
| Stan ewidencyjny `AGENT_IDENTITY_REGISTRY.json` | REGISTERED DEVELOPMENT |

**Nie utożsamiać numeru seryjnego z kryptograficznym digested build SHA.**
