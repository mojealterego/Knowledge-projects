# Macierz wymagań — ARGUS MOBILE

**ARGUS MOBILE** / ewidencja **005** / serial `AM-MOB-ARG-2026-000005` / ID `AM-OCAF-AI-0005` / katalog `AM-OCAF-MOB-005` / certyfikat `CERT-AM-2026-0005` **NIEWYDANY**.

| ID | Wymóg | Implementacja | Test lub dowód | Stan |
|---|---|---|---|---|
| ID-01 | Unikatowa nazwa/serial i rozdzielone numery | `agent.rs`, `agent.toml`, JSON manifest i globalny registry | `tests/identity.rs` | TESTED LOCAL: run 39340442615 |
| ID-02 | Niezezwolenie na podmianę identyfikatorów | `security.rs` | `tests/security.rs` | TESTED LOCAL: run 39340442615 |
| FD-01 | Ograniczona funkcja `ui.proposal_assess` | `src/agent.rs` | `tests/integration.rs`, unit | TESTED LOCAL: run 39340442615 |
| OR-01 | Lokalna integracja OCAF | `src/zenoh.rs` | `tests/ocaf_contract.rs` | TESTED LOCAL: run 39340442615 |
| OR-02 | Logiczne rozdzielenie OMAs/OMAt | `src/zenoh.rs` | `tests/security.rs` | ROUTE ONLY |
| SF-01 | Brak działania przy fałszywej deklaracji uprawnień | `src/agent.rs` i `src/lifecycle.rs` | unit/integration/security | TESTED LOCAL: run 39340442615 |
| CR-01 | OSTc dla dostępu do źródeł | `security.rs` deny only | negatywny test | BLOCKED |
| OP-01 | Produkcyjne E2E i formalny operator acceptance | `RELEASE_CHECKLIST.md` | real host logs | NOT RUN |
| CE-01 | Certyfikacja i podpis binarki | `CERTIFICATE_RECORD.md` | signed artifact + acceptance | NOT ISSUED |

To matryca **implementacji bieżącego fragmentu**, nie ocena ukończenia całego projektu P119.
