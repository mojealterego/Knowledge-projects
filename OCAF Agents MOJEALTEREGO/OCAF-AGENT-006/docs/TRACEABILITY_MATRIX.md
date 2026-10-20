# Macierz wymaganie → kod → test — CUSTOS ADMISSION

**CUSTOS ADMISSION** · nr ewidencyjny **006** · nr seryjny `AM-SEC-CUS-2026-000006` · Agent ID `AM-OCAF-AI-0006` · katalog `AM-OCAF-SEC-006` · certyfikat `CERT-AM-2026-0006` — **NIEWYDANY**.

| Req | Istota | Kod | Dowód |
|---|---|---|---|
| ID-01 | Nazwa i numer seryjny niezależne od ewidencji | `agent.rs`, `security.rs`, TOML, JSON | `tests/identity.rs` |
| SEC-01 | Nazwa/serial spoofing DENY, źródła OSTc DENY | `security.rs` | `tests/security.rs` |
| CORE-01 | Port rzeczywistych reguł źródłowego Python | `agent.rs` | `tests/domain.rs`, unit |
| OCI-01 | OCAF ToolExecutor lokalny | `zenoh.rs` | `tests/ocaf_contract.rs` |
| HOST-01 | Hash artefaktu nie stanowi uwierzytelnienia osoby przeglądającej. Mapę approvals i czas 'now' musi dostarczyć oddzielnie zaufany host. Model nie może ich dołączyć w manifest/tool JSON. | `agent.rs` i `zenoh.rs` | tests unit/OCAF; operational NOT VERIFIED |
| POLICY-01 | OMAs i OMAt oddzielne | `zenoh.rs` | route tests |
| RL-01 | brak autonomicznej aktywacji i certyfikacji | `lifecycle.rs`, manifest | negative tests |
| E2E-01 | autentykacja OMAs/OMAt, TLS, OSTc, multihost | NIEZAIMPLEMENTOWANO | NOT RUN |

To jest analiza lokalnej części systemu, a nie dowód produkcyjnego wdrożenia.
