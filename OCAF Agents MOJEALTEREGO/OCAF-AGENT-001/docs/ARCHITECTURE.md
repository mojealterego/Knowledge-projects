# Architektura techniczna WDA AURELION

## Identyfikacja instancji
Agent `WDA AURELION` (ewidencja 001) ma numer seryjny `AM-WDA-AUR-2026-000001-DE338AC20B42`, techniczne ID `AM-OCAF-AI-0001` i pochodzi z projektu P116. Nazwa nie jest numerem folderu.

## Architektura logiczna (docelowo, nie wdrożono całości)
```text
Operator
   |
   v
OCAF Runtime — policy / executor / release gates
   |                  |
   |                  +--> OMAs = SERVER AGENTÓW
   |                            identity / registry / lifecycle (external, not deployed)
   |
   +--> OMAt = SERVER NARZĘDZI (external, not deployed)
   |          |
   |          +--> capability: photo.plan (ToolManifest + ToolExecutor)
   |                       |
   |                       +--> Rust Agent::plan() -> plan_only
   |
   +--> Zenoh = transport (route contract only; no network session)
   |
   +--> OSTc SOURCE-ACCESS GATE [SPEC PENDING; DENY]
              |
              +--> encrypted agent source [NOT IMPLEMENTED]
```

**W obecnym kodzie OCAF runtime jest lokalnym adapterem ToolExecutor**, nie produkcyjnym serwerem agentowym. `src/zenoh.rs` definiuje `omas/v1/{tenant}/agents/{agent_name}` oraz `omat/v1/{tenant}/tools/{tool_name}`; waliduje segmenty i nie otwiera połączeń, nie dowodzi tożsamości. OMAs i OMAt to odrębne **role serwerowe**, a nie dwie nazwy jednej instancji. Zenoh zapewnia wyłącznie transport.

### Ścieżka wywołania lokalnego
1. `src/main.rs` czyta wbudowane `config/agent.toml` i sprawdza stałą tożsamość: nazwę, numer seryjny, ID, katalog i numer ewidencyjny.
2. `security::validate_goal` odrzuca pusty tekst, znaki sterujące i >1024 bajtów UTF-8.
3. `agent::plan(goal, true)` generuje deterministyczny, odwracalny plan bez I/O.
4. Adapter `zenoh::manifest` udostępnia OCAF lokalnie `photo.plan`, walidując schemat.
5. `lifecycle::transition(Active)` zawsze odrzuca aktywację do wdrożenia rzeczywistych bramek autoryzacyjnych.

### Wymagania niewdrożone
Uwierzytelnienie certyfikatów operatorów OMAs/OMAt, podpis powiązany z RPC, anty-replay, oddzielne tenanty, mTLS, testy awarii i wielohostowe, koperty narzędzi, spójne logi oraz niezależny protokół szyfrowania OSTc. Brak uprawnień do otwierania/odszyfrowywania źródeł przez agenta.

### Struktura Rust
`src/agent.rs` domena i tożsamość; `src/security.rs` walidacja i blokady; `src/zenoh.rs` kontrakty tras oraz adapter OCAF; `src/lifecycle.rs` stany; `src/main.rs` lokalny CLI; `src/lib.rs` API biblioteczne. Schematy `schemas/` zachowują bieżący kontrakt `photo.plan`.

### Zależności i port
`legacy-pwa/` zachowuje nieutwardzony klient i serwer Node. Nie należy uruchamiać publicznie ani przekazywać sekretów. Pełny port do Rust i integracja Firefly/OpenAI/real image processing wymagają osobnej implementacji i weryfikacji jakości.
