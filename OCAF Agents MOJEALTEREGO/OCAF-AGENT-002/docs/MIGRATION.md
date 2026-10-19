# VIGILIS SRE — OCAF-AGENT-002

**Nazwa własna:** VIGILIS SRE · **Numer ewidencyjny:** 002 · **Numer seryjny:** `AM-SRE-VIG-2026-000002` · **Agent ID:** `AM-OCAF-AI-0002` · **Nr katalogowy:** `AM-OCAF-SRE-002` · **Certyfikat:** `CERT-AM-2026-0002` — REZERWACJA, NIEWYDANY.

**Stan:** `INCOMPLETE_DEVELOPMENT`. Lokalny kod Rust do celu: diagnostyka incydentów infrastruktury. Nie jest to kompletna migracja projektu źródłowego ani aktywny produkt OMAs/OMAt/OSTc.

## Pochodzenie i decyzja o wydzieleniu

Źródło: [projekty/121-infrasentinel-agentic-autonomous-sre-infrastructure/README.md](../../../projekty/121-infrasentinel-agentic-autonomous-sre-infrastructure/README.md). W repozytorium weryfikowalnie znajduje się przede wszystkim **dokumentacja architektury i planu**, a nie samodzielny runtime tej specjalizacji. Dlatego nie dokonano pozornego `git mv` źródeł, lecz utworzono nową implementację Rust dokładnie oznaczoną jako **pochodna projektu**. Plik projektu pozostał na swoim miejscu; jego pozostałe funkcje nie zostały przyjęte jako ukończone.

Granica nowego agenta: `incident.triage`, bez sieci i skutków ubocznych. Żadne zewnętrzne rzeczywiste narzędzia nie zostały wywołane. Docelowa integracja z OCAF/OMAs/OMAt przez Zenoh i dostęp OSTc są wymaganiami dalszej pracy, nie uzyskanymi zdolnościami.
