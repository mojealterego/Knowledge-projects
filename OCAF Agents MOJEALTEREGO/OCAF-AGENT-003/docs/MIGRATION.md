# EVIDENTIA RESEARCH — OCAF-AGENT-003

**Nazwa własna:** EVIDENTIA RESEARCH · **Numer ewidencyjny:** 003 · **Numer seryjny:** `AM-RES-EVI-2026-000003` · **Agent ID:** `AM-OCAF-AI-0003` · **Nr katalogowy:** `AM-OCAF-RES-003` · **Certyfikat:** `CERT-AM-2026-0003` — REZERWACJA, NIEWYDANY.

**Stan:** `INCOMPLETE_DEVELOPMENT`. Lokalny kod Rust do celu: lokalna kontrola jakości przesłanek badawczych. Nie jest to kompletna migracja projektu źródłowego ani aktywny produkt OMAs/OMAt/OSTc.

## Pochodzenie i decyzja o wydzieleniu

Źródło: [projekty/15-universal-research-orchestrator.md](../../../projekty/15-universal-research-orchestrator.md). W repozytorium weryfikowalnie znajduje się przede wszystkim **dokumentacja architektury i planu**, a nie samodzielny runtime tej specjalizacji. Dlatego nie dokonano pozornego `git mv` źródeł, lecz utworzono nową implementację Rust dokładnie oznaczoną jako **pochodna projektu**. Plik projektu pozostał na swoim miejscu; jego pozostałe funkcje nie zostały przyjęte jako ukończone.

Granica nowego agenta: `research.assess`, bez sieci i skutków ubocznych. Żadne zewnętrzne rzeczywiste narzędzia nie zostały wywołane. Docelowa integracja z OCAF/OMAs/OMAt przez Zenoh i dostęp OSTc są wymaganiami dalszej pracy, nie uzyskanymi zdolnościami.
