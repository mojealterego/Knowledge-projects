# EVIDENTIA RESEARCH — OCAF-AGENT-003

**Nazwa własna:** EVIDENTIA RESEARCH · **Numer ewidencyjny:** 003 · **Numer seryjny:** `AM-RES-EVI-2026-000003` · **Agent ID:** `AM-OCAF-AI-0003` · **Nr katalogowy:** `AM-OCAF-RES-003` · **Certyfikat:** `CERT-AM-2026-0003` — REZERWACJA, NIEWYDANY.

**Stan:** `INCOMPLETE_DEVELOPMENT`. Lokalny kod Rust do celu: lokalna kontrola jakości przesłanek badawczych. Nie jest to kompletna migracja projektu źródłowego ani aktywny produkt OMAs/OMAt/OSTc.

## Granica odpowiedzialności

```text
Operator --> OCAF Runtime (host policy, ToolExecutor)
               |--- OMAs: SERVER AGENTÓW (zewnętrzny, niewdrożony)
               |--- OMAt: SERVER NARZĘDZI (zewnętrzny, niewdrożony)
               |--- Zenoh: transport routingu (TYLKO klucze, brak sesji)
               |--- research.assess: offline evidentia_research_agent
               +--- OSTc: dostęp do źródeł WYŁĄCZONY do pełnej specyfikacji
```

## Przepływ lokalny

`config/agent.toml` → `security::load_config` → `agent::assess` → wynik disputed/unverified. Opcjonalny adapter w `src/zenoh.rs` rejestruje `research.assess` jako `ToolExecutor` w testowej instancji OCAF. Trasy mają oddzielne prefiksy `omas/v1/` i `omat/v1/`; samo wyliczenie klucza nie jest dowodem autoryzacji.

## Kontrakt wejścia

`ResearchInput { claim, evidence[2..8] { source_id, stance, excerpt } }`. Warianty obsługiwane: stance supports, contradicts, unclear; 2–8 niezależnie identyfikowanych zgłoszeń. Uwierzytelnienie źródła i prawdziwość przekazanych danych **nie są gwarantowane**.

## Kontrakt wyjścia

`Assessment { agent_id, status=disputed|unverified, source_count, has_contradiction, verified=false, conclusions[] }`. Danych nie wolno traktować jako dowodu działania zewnętrznego systemu.

## Architektura kodu

- `src/agent.rs` — logika dziedziny i stałe tożsamości
- `src/zenoh.rs` — kontrakt tras OMAs/OMAt oraz lokalny OCAF adapter, bez otwierania sieci
- `src/security.rs` — walidacja nazwy, numeru seryjnego, tenantów i zakaz źródeł
- `src/lifecycle.rs` — `Draft/Tested/Staged/Retired`, przejście `Active` zawsze DENIED
- `src/main.rs` — lokalne, demonstracyjne CLI, żaden zewnętrzny zasób nie jest odpytywany

**Docelowo:** podpisane żądania OMAs związane z RPC, niezależna autoryzacja OMAt, Zenoh TLS/mTLS, OSTc, multi-host i formalna akceptacja; nie implementowano.
