# Macierz wymagań — DGM SENTINEL

**Nazwa własna:** DGM SENTINEL · **Numer ewidencyjny:** 008 · **Serial:** `AM-DGM-SNT-2026-000008` · **Agent ID:** `AM-OCAF-AI-0008` · **Katalog:** `AM-OCAF-DGM-008` · **Certyfikat:** `CERT-AM-2026-0008` — NIEWYDANY.

| ID | Wymóg | Kod | Test |
|---|---|---|---|
| ID-01 | unikatowy serial niezależny od ewidencji | `config/agent.toml`, `agent.rs`, JSON registry | `tests/identity.rs` |
| ID-02 | odrzucenie niezgodnej metryki | `security.rs` | `tests/security.rs` |
| PF-01 | przeniesienie domenowej polityki Python | `src/agent.rs` | `tests/domain.rs`, unit, original Python |
| PF-02 | wiążące granice zaufania hosta | `src/zenoh.rs` i `agent.rs` | `tests/ocaf_contract.rs` |
| PF-03 | lokalny tool OCAF i unsigned peer denied | `src/zenoh.rs` | `tests/ocaf_contract.rs` |
| OS-01 | rozdzielne prefiksy OMAs/OMAt, brak sieci | `zenoh.rs`, `agent.toml` | `tests/security.rs` |
| CR-01 | OSTc DENY | `security.rs` | `tests/security.rs` |
| PROD-01 | signed release + multi-host + operator | `RELEASE_CHECKLIST.md` | NOT IMPLEMENTED |

Po rzeczywistym CI należy dopisać SHA, liczbę testów i odnośnik do runu; nie przypisywać sukcesu na podstawie starszych przebiegów.
