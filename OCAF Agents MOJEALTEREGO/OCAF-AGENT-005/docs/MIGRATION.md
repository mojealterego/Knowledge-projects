# Pochodzenie i granica wydzielenia — ARGUS MOBILE

**Nazwa własna:** ARGUS MOBILE · **Ewidencja:** 005 · **Numer seryjny:** `AM-MOB-ARG-2026-000005` · **Agent ID:** `AM-OCAF-AI-0005` · **Katalog:** `AM-OCAF-MOB-005` · **Nr certyfikatu:** `CERT-AM-2026-0005` (rezerwacja, niewydany).

Źródło: [projekt P119](../../../projekty/119-omni-mas-local-android-agent-runtime-max/README.md). Zawiera opis większej platformy i szczegółowe wymagania, ale nie potwierdza, że docelowy moduł działał jako samodzielny binarny agent Rust. Dlatego wprowadzono nową ograniczoną implementację, a nie fałszywą migrację binariów i nie zlewano katalogów.

**Wykonany fragment:** Ocena deklarowanej zgody i typu akcji bez dostępu do telefonu; stała blokada DELETE, SEND i GRANT_PERMISSION; pozostałe propozycje wymagają weryfikacji przez rzeczywisty Android UI runtime.

**Poza wydzielonym agentem:** AccessibilityService, ADB, sterowanie ekranem, przechwytywanie powiadomień, połączenia z LLM i Google Play oraz wykonanie rzeczywistych działań na telefonie. Oryginalny projekt pozostaje pod swoją ścieżką i **nie otrzymuje stanu ukończonego**. Dalsza architektura wymaga osobnej integracji i odbioru.
