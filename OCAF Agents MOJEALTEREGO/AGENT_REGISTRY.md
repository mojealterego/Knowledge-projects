# Rejestr agentów — kandydaci do walidacji

To rejestr kandydatów i odrębnie zarejestrowanych egzemplarzy developerskich, **nie** lista wdrożonych ani certyfikowanych agentów. Przydzielenie **wewnętrznego** Agent ID i numeru seryjnego wymaga wpisu w rejestrze i testów ich spójności; wydanie **autentycznego certyfikatu** wymaga osobno zweryfikowanego artefaktu, podpisu, odbioru operatora i dowodów bezpieczeństwa.

| Kandydat | Gdzie zaobserwowano specyfikację | Klasa wstępna | Status |
|---|---|---|---|
| WDA AURELION (rodzina WDA Photo Agent) | [Project 116](../projekty/116-wda-photo-agent-max/README.md) | Agent Rust z historyczną aplikacją WDA | REGISTERED DEVELOPMENT / INCOMPLETE |
| Omni MAS Android (projekt, ARGUS MOBILE 005 jako moduł) | [Project 119](../projekty/119-omni-mas-local-android-agent-runtime-max/README.md) | System agentowy (INFERRED) | UNVERIFIED |
| InfraSentinel (system; moduł VIGILIS SRE 002) | [Project 121](../projekty/121-infrasentinel-agentic-autonomous-sre-infrastructure/README.md) | System operacyjny/SRE agentowy (INFERRED) | UNVERIFIED |
| Universal Research Orchestrator (moduł EVIDENTIA RESEARCH 003) | [Project 15](../projekty/15-universal-research-orchestrator.md) | Agent badawczy read-only (PARTIAL) | IMPLEMENTED DEVELOPMENT CHILD 003 |
| Omni Autonomous Engineering (projekt, AXIOM FORGE 004 jako moduł) | [Project 115](../projekty/115-omniautonomous-engineering-intelligence-orchestrator-max.md) | Orchestrator (INFERRED) | UNVERIFIED |
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


## Nowi agenci — 20.10.2026

| Numer ewid. | Unikatowa nazwa | Agent ID | Numer seryjny | Numer katalogowy | Źródło | Status |
|---|---|---|---|---|---|---|
| **002** | [**VIGILIS SRE**](OCAF-AGENT-002/docs/README.md) | `AM-OCAF-AI-0002` | `AM-SRE-VIG-2026-000002` | `AM-OCAF-SRE-002` | P121 | INCOMPLETE_DEVELOPMENT; certyfikat `CERT-AM-2026-0002` NIEWYDANY |
| **003** | [**EVIDENTIA RESEARCH**](OCAF-AGENT-003/docs/README.md) | `AM-OCAF-AI-0003` | `AM-RES-EVI-2026-000003` | `AM-OCAF-RES-003` | P15 | INCOMPLETE_DEVELOPMENT; certyfikat `CERT-AM-2026-0003` NIEWYDANY |

**Uwaga o źródłach:** P121/P15 są specyfikacjami większych systemów, nie zaginionymi binariami. Zaimplementowane nowe podagenty tylko w zakresie read-only, dlatego nie przenosimy ani nie kasujemy plików źródłowych. Rejestr numerów i kolizji: [AGENT_IDENTITY_REGISTRY.json](AGENT_IDENTITY_REGISTRY.json). Wymagane dalsze testy, E2E OMAs/OMAt/OSTc, podpisy i odbiór operatora.

## Wpisy ewidencyjne 004–005 — 20.10.2026

| Ewid. | Nazwa własna | Agent ID | Numer seryjny | Katalog | Projekt | Stan |
|---|---|---|---|---|---|---|
| **004** | [**AXIOM FORGE**](OCAF-AGENT-004/docs/README.md) | `AM-OCAF-AI-0004` | `AM-ENG-AXF-2026-000004` | `AM-OCAF-ENG-004` | P115 | INCOMPLETE_DEVELOPMENT; certyfikat `CERT-AM-2026-0004` NIEWYDANY |
| **005** | [**ARGUS MOBILE**](OCAF-AGENT-005/docs/README.md) | `AM-OCAF-AI-0005` | `AM-MOB-ARG-2026-000005` | `AM-OCAF-MOB-005` | P119 | INCOMPLETE_DEVELOPMENT; certyfikat `CERT-AM-2026-0005` NIEWYDANY |

Oryginalne P115/P119 są większymi systemami/projektami; nowe niezależne agenty realizują tylko **offline moduły kontrolne**, bez autonomicznego wykonywania zmian i bez rzeczywistej obsługi Android. OMAs/OMAt nie są wdrożone, Zenoh nie otwiera sesji, OSTc pozostaje zablokowany do otrzymania dokumentacji. Wszystkie ewidencje `001–005` mają odrębne nazwy i seriale w [kanonicznym rejestrze](AGENT_IDENTITY_REGISTRY.json).

## Zarejestrowane porty rzeczywistych narzędzi Python — 20.10.2026

| Nr ewid. | Nazwa własna | Agent ID | Serial | Katalog | Kod źródłowy | Status |
|---|---|---|---|---|---|---|
| **006** | [CUSTOS ADMISSION](OCAF-AGENT-006/docs/README.md) | `AM-OCAF-AI-0006` | `AM-SEC-CUS-2026-000006` | `AM-OCAF-SEC-006` | [agent_tool_admission_gate.py](../tools/agent_tool_admission_gate.py) | INCOMPLETE_DEVELOPMENT; certyfikat `CERT-AM-2026-0006` NIEWYDANY |
| **007** | [VIGIA LOOP](OCAF-AGENT-007/docs/README.md) | `AM-OCAF-AI-0007` | `AM-OPS-VIG-2026-000007` | `AM-OCAF-OPS-007` | [agent_tool_loop_guard.py](../tools/agent_tool_loop_guard.py) | INCOMPLETE_DEVELOPMENT; certyfikat `CERT-AM-2026-0007` NIEWYDANY |

**Klasyfikacja:** to rzeczywiście istniejące nieuruchamiające polityki bezpieczeństwa Python z testami; nowe agenty implementują odpowiednie zabezpieczenia lokalnie w Rust. Nie są uruchomionymi serwerami OMAs/OMAt, nie uruchamiają Zenoh, nie implementują OSTc ani sandboxa, nie mają certyfikatów. `AGENT_IDENTITY_REGISTRY.json` jest kanonicznym źródłem numerów 001–007.

## Nowi agenci 008–009 — 20.10.2026

| Nr | Unikatowa nazwa i dokumentacja | Agent ID | Numer seryjny | Numer katalogowy | Źródło | Status |
|---|---|---|---|---|---|---|
| **008** | [**DGM SENTINEL**](OCAF-AGENT-008/docs/README.md) | `AM-OCAF-AI-0008` | `AM-DGM-SNT-2026-000008` | `AM-OCAF-DGM-008` | [DGM gate Python](../tools/dgm_cycle_gate.py) | INCOMPLETE_DEVELOPMENT; `CERT-AM-2026-0008` NIEWYDANY |
| **009** | [**CLOUD FISCALIS**](OCAF-AGENT-009/docs/README.md) | `AM-OCAF-AI-0009` | `AM-CLD-FIS-2026-000009` | `AM-OCAF-CLD-009` | [Cloud budget gate Python](../tools/cloud_operation_budget_gate.py) | INCOMPLETE_DEVELOPMENT; `CERT-AM-2026-0009` NIEWYDANY |

To porty rzeczywistych źródeł Python z niezmienionymi oryginałami i testami. Testy nie stanowią podpisu poświadczenia. **Żaden z agentów 001–009 nie ma wydanego certyfikatu ani produkcyjnej akceptacji**. Kanoniczne numery w `AGENT_IDENTITY_REGISTRY.json`.

## Nowe porty źródłowe — ewidencja 010–011 (20.10.2026)

| Numer | Unikatowa nazwa | Agent ID | Numer seryjny | Katalog | Źródło | Certyfikat |
|---|---|---|---|---|---|---|
| **010** | [VERITAS TRACE](OCAF-AGENT-010/docs/README.md) | `AM-OCAF-AI-0010` | `AM-OSN-VRT-2026-000010` | `AM-OCAF-OSN-010` | [OSINT provenance Python](../tools/osint_provenance_gate.py) | `CERT-AM-2026-0010` — NIEWYDANY |
| **011** | [LIMES GIT](OCAF-AGENT-011/docs/README.md) | `AM-OCAF-AI-0011` | `AM-GIT-LIM-2026-000011` | `AM-OCAF-GIT-011` | [Repo scope Python](../tools/single_repository_scope_gate.py) | `CERT-AM-2026-0011` — NIEWYDANY |

Dwa nowe pakiety Rust realizują tylko lokalną walidację metryk pochodzenia i zakresu operacji. **Nie** pobierają danych OSINT ani nie wykonują żadnych zmian GitHub. Numer seryjny to ewidencja administracyjna, nie podpis kryptograficzny; `production_approved=false`. Dotychczasowa ewidencja obejmuje 001–011.

## Ewidencja 012–013 — 20.10.2026

| Ewidencja | Nazwa własna | Agent ID | Numer seryjny | Numer katalogowy | Źródło | Certyfikat |
|---|---|---|---|---|---|---|
| **012** | [FIDES GRANTIS](OCAF-AGENT-012/docs/README.md) | `AM-OCAF-AI-0012` | `AM-GRT-FID-2026-000012` | `AM-OCAF-GRT-012` | [program_claim_gate.py](../tools/program_claim_gate.py) | `CERT-AM-2026-0012`, NIEWYDANY |
| **013** | [NEXUS CONSENT](OCAF-AGENT-013/docs/README.md) | `AM-OCAF-AI-0013` | `AM-COM-NEX-2026-000013` | `AM-OCAF-COM-013` | [communication_link_policy.py](../tools/communication_link_policy.py) | `CERT-AM-2026-0013`, NIEWYDANY |

Rejestr kanoniczny `AGENT_IDENTITY_REGISTRY.json` obejmuje teraz 001–013. Wszystkie certyfikaty pozostają niewydane, żaden agent nie jest produkcyjnie zatwierdzony. Lokalne moduły Rust nie są autonomicznymi systemami operującymi w serwisach zewnętrznych.
