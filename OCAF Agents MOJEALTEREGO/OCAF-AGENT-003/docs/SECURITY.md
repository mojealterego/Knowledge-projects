# EVIDENTIA RESEARCH — OCAF-AGENT-003

**Nazwa własna:** EVIDENTIA RESEARCH · **Numer ewidencyjny:** 003 · **Numer seryjny:** `AM-RES-EVI-2026-000003` · **Agent ID:** `AM-OCAF-AI-0003` · **Nr katalogowy:** `AM-OCAF-RES-003` · **Certyfikat:** `CERT-AM-2026-0003` — REZERWACJA, NIEWYDANY.

**Stan:** `INCOMPLETE_DEVELOPMENT`. Lokalny kod Rust do celu: lokalna kontrola jakości przesłanek badawczych. Nie jest to kompletna migracja projektu źródłowego ani aktywny produkt OMAs/OMAt/OSTc.

## Model zagrożeń

| Zagrożenie | Zabezpieczenie lokalne | Dowód wymagany przed produkcją |
|---|---|---|
| Payload z nieznanymi polami lub nadmierny JSON | `serde(deny_unknown_fields)`, limit adaptera 8192 bajtów, walidacja długości | fuzzing, limit na sieci, niezależny audit |
| Skłanianie modelu do nieuprawnionych działań | Brak wykonania zewnętrznych poleceń, output read-only/unverified | OMAt capability scoped auth + real sandbox |
| Fałszywy numer seryjny lub tenant | identyfikatory w TOML, walidacja tras | podpis OMAs związany z tożsamością RPC |
| Dostęp do źródeł bez OSTc | `source_access.enabled=false`, `source_access() => DENY` | pełny OSTc model, vectors i audyt |
| Podszycie się pod agentów w Zenoh | brak aktywnego transportu | PKI/TLS/mTLS, izolacja tenantów |
| Fałszywy wynik z danych operatora | `verified=false` niezależnie od treści | rzeczywiste źródła, readback i niezależna weryfikacja |
| Sfałszowane potwierdzenie gotowości | `Active` zawsze DENY | obiektywny protokół certyfikacji i audyt |
| Supply chain Git dependency | stały `rev`, `cargo test` | SBOM, pełne proweniencje, przegląd polityki zakazu git deps |

## Inwarianty

Opis źródła nie staje się faktograficznie prawdziwy poprzez przekazanie go modelowi ani lokalną analizę; nawet sprzeczność wymaga niezależnej adjudykacji.

**Niedozwolone:** automatyczna publikacja jako produkcyjny agent, reklama usług OMAs/OMAt lub OSTc jako działających, dostarczenie fikcyjnych certyfikatów lub podpisów. OCAF Runtime jest zewnętrznym referencyjnym repozytorium tylko do odczytu w tym zadaniu.
