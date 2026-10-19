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


## Pierwszy agent z numerem ewidencyjnym 001

- [**OCAF-AGENT-001 — WDA AURELION**](OCAF-AGENT-001/docs/README.md): kanoniczny pakiet Rust z `src/`, `tests/`, `config/`, `docs/`, `certification/`, archiwum kodu i osobnymi umowami.
- **OMAs** to serwer agentów, **OMAt** to serwer narzędzi. **Zenoh** pozostaje magistralą transmisyjną. Docelowy **OSTc** zabezpieczy dostęp do źródeł, ale implementacja jest zablokowana do otrzymania dokumentacji i audytu.
- [Potwierdzone testy struktury i lokalnego OCAF](https://github.com/mojealterego/Knowledge-projects/actions/runs/39203233792): 12 PASS; brak odbioru produkcyjnego, certyfikat niewydany.


## Tożsamość egzemplarza 001

**WDA AURELION** jest nazwą własną; **001** wyłącznie numerem ewidencyjnym, `AM-OCAF-AI-0001` technicznym Agent ID, a `AM-WDA-AUR-2026-000001-DE338AC20B42` przydzielonym numerem seryjnym. Rejestr kanoniczny: [AGENT_IDENTITY_REGISTRY.json](AGENT_IDENTITY_REGISTRY.json). Dokumenty: [indeks agenta](OCAF-AGENT-001/docs/README.md). **Certyfikat nadal NIEWYDANY**. Pozostałe agentowe identyfikatory rezerwować dopiero po kontroli kolizji.
