# OCAF Agents MOJEALTEREGO — rejestr i punkt wejścia

**Stan:** 2026-10-19 · **Etap:** DISCOVER / INVENTORY (częściowy audyt struktury) · **Repozytorium docelowe:** `mojealterego/Knowledge-projects`

## Granica odpowiedzialności

To katalog ewidencyjny dla pojedynczych agentów, systemów agentowych, rojów, serwerów OMAs i narzędzi OMAt. Nie jest sam w sobie działającym runtime ani katalogiem certyfikowanych produktów.

- **Zapis wyłącznie:** [Knowledge-projects](https://github.com/mojealterego/Knowledge-projects), zgodnie z [AGENTS.md](../AGENTS.md). 
- **Odczyt referencyjny:** [OCAF Runtime](https://github.com/mojealterego/ocaf-runtime), bez zmian jego Roadmapy i kodu w ramach tej partii.
- **Zasada:** nie przenosić kodu z `projekty/` na podstawie samej nazwy pliku; dokument specyfikacji projektu to niekoniecznie uruchamialny agent.
- **Statusy:** OBSERVED / INFERRED / UNKNOWN dla ustaleń; PLANNED / PARTIAL / IMPLEMENTED / TESTED / BLOCKED / UNVERIFIED / PRODUCTION APPROVED dla wykonania. TESTED/PRODUCTION APPROVED tylko z logami, SHA i środowiskiem.

## Indeksy tego punktu wejścia

- [REPOSITORY_INVENTORY.md](REPOSITORY_INVENTORY.md) — sprawdzona struktura obu repozytoriów i granice ustaleń.
- [AGENT_REGISTRY.md](AGENT_REGISTRY.md) — kandydaci do odrębnej ewidencji agentów; **nie** certyfikaty.
- [OMAT_TOOL_REGISTRY.md](OMAT_TOOL_REGISTRY.md) — narzędzia Python w repo oraz odrębny przyszły OMAt.
- [SOURCE_REGISTRY.md](SOURCE_REGISTRY.md) — SHA-256 pięciu otrzymanych materiałów, bez publikowania ich treści.
- [PROCESSING_LEDGER.md](PROCESSING_LEDGER.md) — poziomy przetworzenia i rozliczenie plików/stron.
- [AUDIT_REPORT.md](AUDIT_REPORT.md) — wstępne obserwacje, luki, ryzyka i macierz T01–T15.
- [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) — kolejność dalszej pracy i kryteria odbioru.
- [DECISION_LOG.md](DECISION_LOG.md) — uzasadnienie decyzji bez destrukcyjnej reorganizacji.
- [KNOWLEDGE_INDEX.md](KNOWLEDGE_INDEX.md) — wskaźnik istniejącej bazy wiedzy.
- [PROJECT_REGISTRY.md](PROJECT_REGISTRY.md) — wskaźnik istniejącej ewidencji projektów.

**Ważne:** indeksy są referencjami do istniejących kanonicznych plików, nie ich zamiennikami. Nie wydano agenta, licencji, umowy ani certyfikatu.
