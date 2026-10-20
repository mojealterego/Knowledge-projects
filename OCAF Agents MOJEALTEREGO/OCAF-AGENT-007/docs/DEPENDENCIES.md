# Zależności i supply chain — VIGIA LOOP

**VIGIA LOOP** · nr ewidencyjny **007** · nr seryjny `AM-OPS-VIG-2026-000007` · Agent ID `AM-OCAF-AI-0007` · katalog `AM-OCAF-OPS-007` · certyfikat `CERT-AM-2026-0007` — **NIEWYDANY**.

Rust 2021, `serde`, `serde_json`, `toml`, opcjonalny `ocaf_runtime` na stałym `git rev=52d4f597e0b481678f90cf2073b721ad2e99e253`, `async-trait`; `tokio` testy. Przechowywanie stanu wyłącznie przez `std::sync::Mutex`, bez zewnętrznego storage.

Oryginalne pliki Pythona nie są kasowane ani przepisywane. Nie ma rzeczywistego signed release, SBOM, audit, independent artifact digest, publicznego URL artefaktu ani pomiarów RTO/RPO. Git dependency może naruszać docelową politykę OCAF Runtime — wymaga przeglądu i ewentualnej zamiany na zweryfikowany vendor/crate. **Nie wyciągać wniosków o produkcyjnej zgodności z samego CI.**
