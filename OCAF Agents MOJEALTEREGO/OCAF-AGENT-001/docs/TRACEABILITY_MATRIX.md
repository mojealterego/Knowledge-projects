# Matryca wymagań, artefaktów i weryfikacji — WDA AURELION

| ID | Wymóg | Implementacja / dokument | Test / dowód | Wynik |
|---|---|---|---|---|
| ID-01 | Unikatowa nazwa niezależna od numeru 001 | `agent.rs`, `agent.toml`, `AGENT_IDENTITY_REGISTRY.json` | `tests/identity.rs` | TESTED LOCAL (PR #28, run 39324415084) |
| ID-02 | Numer seryjny inny niż numer ewidencyjny | `agent.rs`, manifest, JSON registry | `tests/identity.rs` | TESTED LOCAL (PR #28, run 39324415084) |
| ID-03 | Ochrona spójności nazwy/serialu i unikatowości | `security.rs`, JSON registry | `tests/identity.rs` | TESTED LOCAL (PR #28, run 39324415084) |
| ID-04 | Pełny zestaw technicznych dokumentów agenta | `docs/`, `certification/` | tree inventory | IMPLEMENTED DOCS |
| PL-01 | Plan fotograficzny bez efektów ubocznych | `src/agent.rs` | `tests/integration.rs` | PREVIOUS PASS |
| SE-01 | Limit 1024 bajtów i identity lock | `src/security.rs`, `src/agent.rs` | `tests/security.rs`, unit | PREVIOUS PASS |
| SE-02 | Nieautoryzowana aktywacja zabroniona | `src/lifecycle.rs` | `tests/security.rs` | PREVIOUS PASS |
| SV-01 | OMAs = serwer agentów | `docs/ARCHITECTURE.md`, `src/zenoh.rs` | tylko route test | PARTIAL |
| SV-02 | OMAt = serwer narzędzi | `docs/ARCHITECTURE.md`, `src/zenoh.rs` | tylko route test | PARTIAL |
| SV-03 | Zenoh jako transport | `src/zenoh.rs` | brak network E2E | NOT IMPLEMENTED |
| CR-01 | Dostęp źródeł szyfrowany OSTc | `security.rs` (DENY ONLY) | negatywne testy | BLOCKED BY SPEC |
| MG-01 | Real image engine w Rust + image quality gate | P116 + `legacy-pwa` | nie testowano | NOT IMPLEMENTED |
| MG-02 | Bezpieczny backend providerów | `legacy-pwa` w archiwum | JS syntax tylko | NOT IMPLEMENTED |
| RL-01 | Certyfikat / podpisana proweniencja | `certification/CERTIFICATE_RECORD.md` | odbiór nieprzeprowadzony | NOT ISSUED |
| RL-02 | Specyfikacja handlowa i umowy | `docs/commercial/` | przegląd prawny brak | DRAFT |

**Zasada aktualizacji:** przy każdym PR wykonawczym zmień status w tej tabeli wyłącznie wtedy, gdy dowód i SHA jednoznacznie go potwierdzają. Wiersze z wynikiem `TESTED LOCAL` oznaczają weryfikację lokalnych kontraktów; formalny odbiór na `main` musi nadal nastąpić.
