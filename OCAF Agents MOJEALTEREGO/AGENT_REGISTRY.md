# Rejestr agentów — kandydaci do walidacji

To rejestr hipotez klasyfikacyjnych, **nie** lista wdrożonych agentów. Przydzielenie autentycznego `agent_id`, numeru seryjnego lub certyfikatu wymaga manifestu, artefaktu, testów i wpisu rejestru.

| Kandydat | Gdzie zaobserwowano specyfikację | Klasa wstępna | Status |
|---|---|---|---|
| WDA AURELION (rodzina WDA Photo Agent) | [Project 116](../projekty/116-wda-photo-agent-max/README.md) | Agent / aplikacja mobilna (INFERRED) | UNVERIFIED |
| Omni MAS Android | [Project 119](../projekty/119-omni-mas-local-android-agent-runtime-max/README.md) | System agentowy (INFERRED) | UNVERIFIED |
| InfraSentinel | [Project 121](../projekty/121-infrasentinel-agentic-autonomous-sre-infrastructure/README.md) | System operacyjny/SRE agentowy (INFERRED) | UNVERIFIED |
| Omni Autonomous Engineering | [Project 115](../projekty/115-omniautonomous-engineering-intelligence-orchestrator-max.md) | Orchestrator (INFERRED) | UNVERIFIED |
| OMAS-OMEGA ENGINEER | Źródło SRC-002/SRC-003, model dokumentacyjny | Przykład handlowo-techniczny | PLANNED / NOT REGISTERED |

**Walidacja kandydata:** `source_path → właściciel zakresu → binary/entrypoint → manifest → capabilities → test evidence → status`. Nie tworzyć fikcyjnych egzemplarzy ani numerów z briefu. Docelowe pola rejestru: `agent_id, artifact_digest, build_commit, tenant_scope, owner, lifecycle_state, release_version, test_evidence` oraz numery ewidencyjne tylko po formalnym nadaniu.

**Reorganizacja:** agent 001 został wydzielony z P116, pozostałe kandydatury wymagają audytu; nie uznawaj całej reorganizacji za zakończoną. Powód: brak zakończonej analizy zależności; przenoszenie źródeł aplikacji mogłoby zerwać importy i testy. Po audycie samodzielne byty przenosić z zachowaniem historii; integralne moduły rejestrować po ścieżce.


## Ewidencja pierwszego agenta — 2026-10-19

| Numer ewid. | Nazwa własna | Agent ID | Numer seryjny | Numer katalogowy | Status | Certyfikat |
|---|---|---|---|---|---|---|
| **001** | **[WDA AURELION](OCAF-AGENT-001/docs/README.md)** | **AM-OCAF-AI-0001** | **AM-WDA-AUR-2026-000001-DE338AC20B42** | **AM-OCAF-WDA-001** | INCOMPLETE_DEVELOPMENT / NOT FOR PRODUCTION | [CERT-AM-2026-0001 — niewydany](OCAF-AGENT-001/certification/CERTIFICATE_RECORD.md) |

Numer **001 jest pierwszym przydzielonym wpisem ewidencyjnym**; projekt certyfikatu nie jest poświadczeniem testów, numer seryjny i katalogowy zostały nadane w rejestrze deweloperskim. Brak innych zarejestrowanych egzemplarzy w tym rejestrze.


## Standard folderu agentowego — 2026-10-19

Dla agentów w ewidencji obowiązuje `OCAF-AGENT-NNN/` z głównym `Cargo.toml`, rozdzielonymi `src/`, `tests/`, `config/`, `docs/` i `certification/`. OMAs = serwer agentów, OMAt = serwer narzędzi, Zenoh = transport; OSTc dla zaszyfrowanego dostępu do plików źródłowych po późniejszym dostarczeniu specyfikacji. Agent 001 korzysta z nowego folderu. Numery ewidencyjne nie są numerami projektów.


## Reguła bezwzględna — unikatowa nazwa i numer seryjny

Każdy agent ma **własną nazwę, własny numer seryjny oraz oddzielny numer ewidencyjny**. `001` jest numerem porządkowym, nie nazwą ani numerem seryjnym. Reguły unikatowości i obecne wpisy znajdują się w [AGENT_IDENTITY_REGISTRY.json](AGENT_IDENTITY_REGISTRY.json), a waliduje je Rust `tests/identity.rs`. Numer seryjny jest nadany ewidencyjnie i nie zależy od ukończenia certyfikacji. Certyfikat wydajemy dopiero po podpisanym protokole i komplecie bramek T01–T15.
