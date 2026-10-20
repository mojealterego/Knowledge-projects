# Macierz wymaganie → kod → test — VIGIA LOOP

**VIGIA LOOP** · nr ewidencyjny **007** · nr seryjny `AM-OPS-VIG-2026-000007` · Agent ID `AM-OCAF-AI-0007` · katalog `AM-OCAF-OPS-007` · certyfikat `CERT-AM-2026-0007` — **NIEWYDANY**.

| Req | Istota | Kod | Dowód |
|---|---|---|---|
| ID-01 | Nazwa i numer seryjny niezależne od ewidencji | `agent.rs`, `security.rs`, TOML, JSON | `tests/identity.rs` |
| SEC-01 | Nazwa/serial spoofing DENY, źródła OSTc DENY | `security.rs` | `tests/security.rs` |
| CORE-01 | Port rzeczywistych reguł źródłowego Python | `agent.rs` | `tests/domain.rs`, unit |
| OCI-01 | OCAF ToolExecutor lokalny | `zenoh.rs` | `tests/ocaf_contract.rs` |
| HOST-01 | Trusted progress nie może pochodzić z wiadomości agenta lub pola tool JSON; host musi świadomie wywołać advance_verified_progress po niezależnym readback. Węzeł OCAF musi trwale zachować instancję Guard w obrębie sesji. | `agent.rs` i `zenoh.rs` | tests unit/OCAF; operational NOT VERIFIED |
| POLICY-01 | OMAs i OMAt oddzielne | `zenoh.rs` | route tests |
| RL-01 | brak autonomicznej aktywacji i certyfikacji | `lifecycle.rs`, manifest | negative tests |
| E2E-01 | autentykacja OMAs/OMAt, TLS, OSTc, multihost | NIEZAIMPLEMENTOWANO | NOT RUN |

To jest analiza lokalnej części systemu, a nie dowód produkcyjnego wdrożenia.
