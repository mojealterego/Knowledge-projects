# Architektura — CUSTOS ADMISSION

**CUSTOS ADMISSION** · nr ewidencyjny **006** · nr seryjny `AM-SEC-CUS-2026-000006` · Agent ID `AM-OCAF-AI-0006` · katalog `AM-OCAF-SEC-006` · certyfikat `CERT-AM-2026-0006` — **NIEWYDANY**.

```text
Operator/Host --> OCAF Runtime (lokalny ToolExecutor)
                        |
                        +--> OMAs SERVER AGENTÓW [EXTERNAL / NOT DEPLOYED]
                        +--> OMAt SERVER NARZĘDZI [EXTERNAL / NOT DEPLOYED]
                        +--> Zenoh routing [KEYSPACE ONLY; NETWORK OFF]
                        +--> tool.admit [RUST custos_admission_agent]
                                    |
                                    +--> external approvals injected into AgentTool by trusted host
                                    +--> OSTc SOURCE ACCESS [DENY]
```

## Komponenty i granica zaufania

`src/agent.rs`: Port funkcji Python admit_tool: SHA-256 bajtów, jawna allowlista czterech capability, niezależne zgody oparte o digest, nazwa odizolowanego profilu, timeout 1–60 sekund, ważna data wygaśnięcia oraz zakaz skutków ubocznych.

`src/zenoh.rs`: implementuje tylko lokalny adapter `ocaf_runtime::executor::ToolExecutor`; osobno rezerwuje przestrzenie `omas/v1/<tenant>/agents/<agent>` i `omat/v1/<tenant>/tools/<tool>`. Routing nie tworzy mTLS, PKI, sesji Zenoh ani potwierdzonych zdarzeń na zdalnym hoście.

`src/security.rs`: weryfikuje tożsamość `AM-OCAF-AI-0006`, serial `AM-SEC-CUS-2026-000006`, `network_enabled=false`, `authenticated_tls_required=true`, serwery odrębne OMAs/OMAt oraz niedostępne źródła OSTc. `src/lifecycle.rs` zawsze odrzuca `Active`.

**Zaufanie do hosta:** Hash artefaktu nie stanowi uwierzytelnienia osoby przeglądającej. Mapę approvals i czas 'now' musi dostarczyć oddzielnie zaufany host. Model nie może ich dołączyć w manifest/tool JSON. Model nie może poświadczać swojej własnej autoryzacji. `ExecutionContext` aktualnego adaptera nie zastępuje zewnętrznego niezależnego issuera.

**Brakujące systemy:** niezależny podpis approval, działający izolator, walidacja sygnatur i proweniencji, storage i komunikacja OMAt/OMAs. Nie ma realnego serwera ani podpisu RPC i niezależnych dowodów wielohostowych.
