# VIGILIS SRE — OCAF-AGENT-002

**Nazwa własna:** VIGILIS SRE · **Numer ewidencyjny:** 002 · **Numer seryjny:** `AM-SRE-VIG-2026-000002` · **Agent ID:** `AM-OCAF-AI-0002` · **Nr katalogowy:** `AM-OCAF-SRE-002` · **Certyfikat:** `CERT-AM-2026-0002` — REZERWACJA, NIEWYDANY.

**Stan:** `INCOMPLETE_DEVELOPMENT`. Lokalny kod Rust do celu: diagnostyka incydentów infrastruktury. Nie jest to kompletna migracja projektu źródłowego ani aktywny produkt OMAs/OMAt/OSTc.

## Granica odpowiedzialności

```text
Operator --> OCAF Runtime (host policy, ToolExecutor)
               |--- OMAs: SERVER AGENTÓW (zewnętrzny, niewdrożony)
               |--- OMAt: SERVER NARZĘDZI (zewnętrzny, niewdrożony)
               |--- Zenoh: transport routingu (TYLKO klucze, brak sesji)
               |--- incident.triage: offline vigilis_sre_agent
               +--- OSTc: dostęp do źródeł WYŁĄCZONY do pełnej specyfikacji
```

## Przepływ lokalny

`config/agent.toml` → `security::load_config` → `agent::triage` → wynik read_only. Opcjonalny adapter w `src/zenoh.rs` rejestruje `incident.triage` jako `ToolExecutor` w testowej instancji OCAF. Trasy mają oddzielne prefiksy `omas/v1/` i `omat/v1/`; samo wyliczenie klucza nie jest dowodem autoryzacji.

## Kontrakt wejścia

`Incident { incident_id, tenant_id, signal, observations[1..8], risk_tier R0..R4 }`. Warianty obsługiwane: CrashLoopBackOff, NodeNotReady, PodPending, ServiceUnavailable. Uwierzytelnienie źródła i prawdziwość przekazanych danych **nie są gwarantowane**.

## Kontrakt wyjścia

`Triage { agent_id, status=read_only, signal, risk_tier, evidence_count, allow_mutation=false, verified=false, recommendations[] }`. Danych nie wolno traktować jako dowodu działania zewnętrznego systemu.

## Architektura kodu

- `src/agent.rs` — logika dziedziny i stałe tożsamości
- `src/zenoh.rs` — kontrakt tras OMAs/OMAt oraz lokalny OCAF adapter, bez otwierania sieci
- `src/security.rs` — walidacja nazwy, numeru seryjnego, tenantów i zakaz źródeł
- `src/lifecycle.rs` — `Draft/Tested/Staged/Retired`, przejście `Active` zawsze DENIED
- `src/main.rs` — lokalne, demonstracyjne CLI, żaden zewnętrzny zasób nie jest odpytywany

**Docelowo:** podpisane żądania OMAs związane z RPC, niezależna autoryzacja OMAt, Zenoh TLS/mTLS, OSTc, multi-host i formalna akceptacja; nie implementowano.
