# Architektura — VIGIA LOOP

**VIGIA LOOP** · nr ewidencyjny **007** · nr seryjny `AM-OPS-VIG-2026-000007` · Agent ID `AM-OCAF-AI-0007` · katalog `AM-OCAF-OPS-007` · certyfikat `CERT-AM-2026-0007` — **NIEWYDANY**.

```text
Operator/Host --> OCAF Runtime (lokalny ToolExecutor)
                        |
                        +--> OMAs SERVER AGENTÓW [EXTERNAL / NOT DEPLOYED]
                        +--> OMAt SERVER NARZĘDZI [EXTERNAL / NOT DEPLOYED]
                        +--> Zenoh routing [KEYSPACE ONLY; NETWORK OFF]
                        +--> tool.loop_gate [RUST vigia_loop_agent]
                                    |
                                    +--> one shared Guard in host-owned Mutex; progress injected through host API
                                    +--> OSTc SOURCE ACCESS [DENY]
```

## Komponenty i granica zaufania

`src/agent.rs`: Port Python AgentToolLoopGuard: limit łącznej liczby wywołań, limit zużycia tokenów, maksymalna sekwencja identycznych wywołań bez postępu, deterministyczna odmowa i trwały stan halted, reset powtórzeń tylko po host-owned readback.

`src/zenoh.rs`: implementuje tylko lokalny adapter `ocaf_runtime::executor::ToolExecutor`; osobno rezerwuje przestrzenie `omas/v1/<tenant>/agents/<agent>` i `omat/v1/<tenant>/tools/<tool>`. Routing nie tworzy mTLS, PKI, sesji Zenoh ani potwierdzonych zdarzeń na zdalnym hoście.

`src/security.rs`: weryfikuje tożsamość `AM-OCAF-AI-0007`, serial `AM-OPS-VIG-2026-000007`, `network_enabled=false`, `authenticated_tls_required=true`, serwery odrębne OMAs/OMAt oraz niedostępne źródła OSTc. `src/lifecycle.rs` zawsze odrzuca `Active`.

**Zaufanie do hosta:** Trusted progress nie może pochodzić z wiadomości agenta lub pola tool JSON; host musi świadomie wywołać advance_verified_progress po niezależnym readback. Węzeł OCAF musi trwale zachować instancję Guard w obrębie sesji. Model nie może poświadczać swojej własnej autoryzacji. `ExecutionContext` aktualnego adaptera nie zastępuje zewnętrznego niezależnego issuera.

**Brakujące systemy:** wielohostowe współdzielone przechowywanie stanu, niezależne wydawanie readback receipts, śledzenie rzeczywistej liczby tokenów, atomowe mechanizmy HA. Nie ma realnego serwera ani podpisu RPC i niezależnych dowodów wielohostowych.
