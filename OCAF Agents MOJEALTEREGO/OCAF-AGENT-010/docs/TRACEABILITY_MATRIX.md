# Macierz wymagań → kod → dowód — ZIP WARDEN

**ZIP WARDEN** · ewidencja **010** · serial `AM-ARC-ZWD-2026-000010` · Agent ID `AM-OCAF-AI-0010` · katalog `AM-OCAF-ARC-010` · certyfikat `CERT-AM-2026-000010` (**rezerwacja; niewydany**).

| Req | Treść | Lokalna realizacja | Test |
|---|---|---|---|
| ID-01 | unikatowa nazwa/serial i numery | `agent.rs`, JSON registry, TOML | `tests/identity.rs` |
| ID-02 | odmowa manipulacji źródłami OSTc | `src/security.rs` | `tests/security.rs` |
| BND-01 | zawężona funkcja portu Python | `src/agent.rs` | `tests/domain.rs`, unit |
| BND-02 | fail-closed OCAF ToolExecutor | `src/zenoh.rs` | `tests/ocaf_contract.rs` |
| BND-03 | host-only granica zaufania | `src/agent.rs`, adapter | negative integration |
| BND-04 | brak aktywnej sesji Zenoh | `src/security.rs` | security tests |
| BND-05 | brak certyfikacji na podstawie CI | `src/lifecycle.rs` | negative tests |
| PROD-01 | niezależny operator approval + OSTc | NOT IMPLEMENTED | NOT RUN |

Wyniki testów muszą mieć rzeczywiste SHA i run ID, samo istnienie macierzy nie jest dowodem.
