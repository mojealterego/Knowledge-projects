# EVIDENTIA RESEARCH — OCAF-AGENT-003

**Nazwa własna:** EVIDENTIA RESEARCH · **Numer ewidencyjny:** 003 · **Numer seryjny:** `AM-RES-EVI-2026-000003` · **Agent ID:** `AM-OCAF-AI-0003` · **Nr katalogowy:** `AM-OCAF-RES-003` · **Certyfikat:** `CERT-AM-2026-0003` — REZERWACJA, NIEWYDANY.

**Stan:** `INCOMPLETE_DEVELOPMENT`. Lokalny kod Rust do celu: lokalna kontrola jakości przesłanek badawczych. Nie jest to kompletna migracja projektu źródłowego ani aktywny produkt OMAs/OMAt/OSTc.

## Serwis i cykl życia

**Identyfikacja serwisowa:** EVIDENTIA RESEARCH, 003, `AM-RES-EVI-2026-000003`. Stan produkcyjny: NIEZATWIERDZONY.

1. Porównaj manifest, `agent.toml`, rejestr `AGENT_IDENTITY_REGISTRY.json` i kod stałych.
2. Uruchom `cargo test --all-features`, `cargo fmt --check` oraz `clippy -D warnings`.
3. Podczas incydentu loguj numer zadania i zanonimizowaną diagnozę; wejście z danymi osobowymi nie jest przechowywane przez ten lokalny Rust.
4. Nie ponawiaj działań mutujących: agent nie obsługuje ich w obecnej wersji. Nie używaj faktycznych kluczy produkcyjnych.
5. Błąd/wymuszenie `Active` powinno zakończyć się odmową; nie ma rzeczywistego protokołu revoke OMAs.
6. Cofnięcie zmian: wskazać konkretny commit i ponownie uruchomić testy; rollback zewnętrznej infrastruktury/źródeł nie dotyczy aktualnej implementacji.

**Docelowe SLA, RTO/RPO, 24/7, ubezpieczenie:** NIEUZGODNIONE. Serwis certyfikowany dopiero po podpisanym odbiorze. Wewnętrzny rejestr seriali nie zastępuje podpisanego release artefaktu.
