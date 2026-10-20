# Test traceability — NEXUS CONSENT

**NEXUS CONSENT** · ewidencja **013** · serial `AM-COM-NEX-2026-000013` · Agent ID `AM-OCAF-AI-0013` · katalog `AM-OCAF-COM-013` · certyfikat `CERT-AM-2026-0013` **NIEWYDANY**.

| ID | Kontrakt | Implementacja | Test |
|---|---|---|---|
| ID-01 | unikatowy serial, katalog i Agent ID | agent.rs, config, registry | `tests/identity.rs` |
| SEC-01 | brak network/OSTc, odmowa Active | `security.rs`, `lifecycle.rs` | `tests/security.rs` |
| POLICY-01 | reguły źródła Python | `agent.rs` | unit + `tests/domain.rs` |
| POLICY-02 | model nie nadaje proof/execution | `agent.rs`, serde | unit + OCAF |
| OCI-01 | bezpieczny lokalny `ToolExecutor` | `src/zenoh.rs` | `tests/ocaf_contract.rs` |
| OCI-02 | unsigned remote peer denied | testowy OCAF runtime | OCAF integration |
| OPS-01 | production signed OMAs/OMAt/Zenoh/OSTc | NOT IMPLEMENTED | NOT RUN |
| CERT-01 | certyfikat i podpis binarki | DRAFT ONLY | NOT ISSUED |
