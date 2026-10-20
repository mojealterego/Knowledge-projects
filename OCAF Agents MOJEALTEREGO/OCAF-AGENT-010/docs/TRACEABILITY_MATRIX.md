# Macierz wymagań — VERITAS TRACE

**Nazwa:** VERITAS TRACE · **Ewidencja:** 010 · **Serial:** `AM-OSN-VRT-2026-000010` · **Agent ID:** `AM-OCAF-AI-0010` · **Katalog:** `AM-OCAF-OSN-010` · **Certyfikat:** `CERT-AM-2026-0010` — NIEWYDANY.

| ID | Wymaganie | Kod | Test |
|---|---|---|---|
| ID-01 | unikatowa nazwa, serial, katalog, rejestr | `agent.rs`, `agent.toml`, manifest, global registry | `tests/identity.rs` |
| SC-01 | brak sieci/OSTc, brak Active, bezpieczne trasy | `security.rs`, `lifecycle.rs`, `zenoh.rs` | `tests/security.rs` |
| DO-01 | port istniejącej polityki źródłowej | `src/agent.rs` | `tests/domain.rs`, unit, original Python |
| OC-01 | host local ToolExecutor, ograniczony JSON | `src/zenoh.rs` | `tests/ocaf_contract.rs` |
| OC-02 | unsigned peer denied | testowa instancja OCAF | `tests/ocaf_contract.rs` |
| OP-01 | autentyczny OMAs/OMAt/Zenoh i OSTc | NOT IMPLEMENTED | NOT RUN |
| CE-01 | certyfikat i podpisana binarka | rejestr draft | NOT ISSUED |

Lokalny adapter nie jest dowodem wykonania side effects ani połączenia sieciowego.
