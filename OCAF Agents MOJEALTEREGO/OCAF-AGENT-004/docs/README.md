# AXIOM FORGE — OCAF-AGENT-004

**Nazwa własna:** AXIOM FORGE · **Ewidencja:** 004 · **Numer seryjny:** `AM-ENG-AXF-2026-000004` · **Agent ID:** `AM-OCAF-AI-0004` · **Katalog:** `AM-OCAF-ENG-004` · **Nr certyfikatu:** `CERT-AM-2026-0004` (rezerwacja, niewydany).

**Status: INCOMPLETE_DEVELOPMENT / CERTIFICATE NOT ISSUED / PRODUCTION NOT APPROVED.**

## Zakres nowego agenta

analizator zależności grafu prac inżynieryjnych. Wykrywanie duplikatów identyfikatorów, brakujących zależności, samoodwołań i cykli; deterministyczne uporządkowanie topologiczne; zero komend, mutacji i połączeń z GitHub.

Opracowano na podstawie istniejącego [projektu P115](../../../projekty/115-omniautonomous-engineering-intelligence-orchestrator-max.md), który jest **szerszą specyfikacją architektoniczną**. Utworzenie tej podfunkcji Rust nie jest portem całego projektu P115. Nie przeniesiono ani nie usunięto jego źródeł.

## Istniejące możliwości

- Pakiet Rust `axiom_forge_agent` z `#![forbid(unsafe_code)]`, walidacją danych i jednoznaczną metryką.
- Offline narzędzie OCAF `graph.review`, lokalny `ToolExecutor`/manifest i testy odmowy dla niezaufanych żądań.
- Oddzielne logiczne prefiksy OMAs (serwer agentów) i OMAt (serwer narzędzi), Zenoh jako docelowa transmisja **nieuruchomiona**.
- Blokada źródeł: `source_access.enabled=false`. Brak rozwiązania OSTc do czasu pełnej specyfikacji i audytu.
- Niemożność przejścia lifecycle do `Active`.

## Test lokalny

```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-004/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-004/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-004/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-004/Cargo.toml" -- demo
```

## Dokumentacja

[ARCHITECTURE](ARCHITECTURE.md) · [SPECIFICATION](SPECIFICATION.md) · [USER_MANUAL](USER_MANUAL.md) · [SECURITY](SECURITY.md) · [SERVICE](SERVICE.md) · [IDENTITY_AND_SERIAL](IDENTITY_AND_SERIAL.md) · [DEPENDENCIES](DEPENDENCIES.md) · [TRACEABILITY_MATRIX](TRACEABILITY_MATRIX.md) · [RELEASE_CHECKLIST](RELEASE_CHECKLIST.md) · [TEST_EVIDENCE](TEST_EVIDENCE.md) · [MIGRATION](MIGRATION.md) · [CHANGELOG](CHANGELOG.md) · [CERTIFICATE_RECORD](../certification/CERTIFICATE_RECORD.md) · [Sprzedaż](commercial/SALE-DRAFT.md) · [Dzierżawa](commercial/LEASE-DRAFT.md) · [Najem](commercial/RENT-DRAFT.md).

**Nie przedstawiać tego modułu jako pełnego autonomicznego agenta, aktywnego klienta OMAs/OMAt, zaimplementowanego OSTc ani certyfikowanego produktu.**
