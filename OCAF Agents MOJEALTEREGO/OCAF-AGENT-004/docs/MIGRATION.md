# Pochodzenie i granica wydzielenia — AXIOM FORGE

**Nazwa własna:** AXIOM FORGE · **Ewidencja:** 004 · **Numer seryjny:** `AM-ENG-AXF-2026-000004` · **Agent ID:** `AM-OCAF-AI-0004` · **Katalog:** `AM-OCAF-ENG-004` · **Nr certyfikatu:** `CERT-AM-2026-0004` (rezerwacja, niewydany).

Źródło: [projekt P115](../../../projekty/115-omniautonomous-engineering-intelligence-orchestrator-max.md). Zawiera opis większej platformy i szczegółowe wymagania, ale nie potwierdza, że docelowy moduł działał jako samodzielny binarny agent Rust. Dlatego wprowadzono nową ograniczoną implementację, a nie fałszywą migrację binariów i nie zlewano katalogów.

**Wykonany fragment:** Wykrywanie duplikatów identyfikatorów, brakujących zależności, samoodwołań i cykli; deterministyczne uporządkowanie topologiczne; zero komend, mutacji i połączeń z GitHub.

**Poza wydzielonym agentem:** orchestracja modeli, generacja lub wdrażanie kodu, realizacja task graph, samoakceptacja builda, niezależne podpisy i testy e2e. Oryginalny projekt pozostaje pod swoją ścieżką i **nie otrzymuje stanu ukończonego**. Dalsza architektura wymaga osobnej integracji i odbioru.
