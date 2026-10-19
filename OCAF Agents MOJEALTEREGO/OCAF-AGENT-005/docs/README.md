# ARGUS MOBILE — OCAF-AGENT-005

**Nazwa własna:** ARGUS MOBILE · **Ewidencja:** 005 · **Numer seryjny:** `AM-MOB-ARG-2026-000005` · **Agent ID:** `AM-OCAF-AI-0005` · **Katalog:** `AM-OCAF-MOB-005` · **Nr certyfikatu:** `CERT-AM-2026-0005` (rezerwacja, niewydany).

**Status: INCOMPLETE_DEVELOPMENT / CERTIFICATE NOT ISSUED / PRODUCTION NOT APPROVED.**

## Zakres nowego agenta

kontroler dopuszczenia propozycji akcji interfejsu Android. Ocena deklarowanej zgody i typu akcji bez dostępu do telefonu; stała blokada DELETE, SEND i GRANT_PERMISSION; pozostałe propozycje wymagają weryfikacji przez rzeczywisty Android UI runtime.

Opracowano na podstawie istniejącego [projektu P119](../../../projekty/119-omni-mas-local-android-agent-runtime-max/README.md), który jest **szerszą specyfikacją architektoniczną**. Utworzenie tej podfunkcji Rust nie jest portem całego projektu P119. Nie przeniesiono ani nie usunięto jego źródeł.

## Istniejące możliwości

- Pakiet Rust `argus_mobile_agent` z `#![forbid(unsafe_code)]`, walidacją danych i jednoznaczną metryką.
- Offline narzędzie OCAF `ui.proposal_assess`, lokalny `ToolExecutor`/manifest i testy odmowy dla niezaufanych żądań.
- Oddzielne logiczne prefiksy OMAs (serwer agentów) i OMAt (serwer narzędzi), Zenoh jako docelowa transmisja **nieuruchomiona**.
- Blokada źródeł: `source_access.enabled=false`. Brak rozwiązania OSTc do czasu pełnej specyfikacji i audytu.
- Niemożność przejścia lifecycle do `Active`.

## Test lokalny

```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-005/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-005/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-005/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-005/Cargo.toml" -- demo
```

## Dokumentacja

[ARCHITECTURE](ARCHITECTURE.md) · [SPECIFICATION](SPECIFICATION.md) · [USER_MANUAL](USER_MANUAL.md) · [SECURITY](SECURITY.md) · [SERVICE](SERVICE.md) · [IDENTITY_AND_SERIAL](IDENTITY_AND_SERIAL.md) · [DEPENDENCIES](DEPENDENCIES.md) · [TRACEABILITY_MATRIX](TRACEABILITY_MATRIX.md) · [RELEASE_CHECKLIST](RELEASE_CHECKLIST.md) · [TEST_EVIDENCE](TEST_EVIDENCE.md) · [MIGRATION](MIGRATION.md) · [CHANGELOG](CHANGELOG.md) · [CERTIFICATE_RECORD](../certification/CERTIFICATE_RECORD.md) · [Sprzedaż](commercial/SALE-DRAFT.md) · [Dzierżawa](commercial/LEASE-DRAFT.md) · [Najem](commercial/RENT-DRAFT.md).

**Nie przedstawiać tego modułu jako pełnego autonomicznego agenta, aktywnego klienta OMAs/OMAt, zaimplementowanego OSTc ani certyfikowanego produktu.**
