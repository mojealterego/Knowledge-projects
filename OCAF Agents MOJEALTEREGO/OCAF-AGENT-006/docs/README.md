# CUSTOS ADMISSION — OCAF-AGENT-006

**CUSTOS ADMISSION** · nr ewidencyjny **006** · nr seryjny `AM-SEC-CUS-2026-000006` · Agent ID `AM-OCAF-AI-0006` · katalog `AM-OCAF-SEC-006` · certyfikat `CERT-AM-2026-0006` — **NIEWYDANY**.

**Wydanie `0.1.0-dev` — `INCOMPLETE_DEVELOPMENT`; `production_approved=false`, `certificate_issued=false`.**

## Cel

lokalna kontrola dopuszczenia artefaktu narzędzia, BEZ jego wykonania. Jest to Rust port rzeczywistego pliku [`tools/agent_tool_admission_gate.py`](../../../tools/agent_tool_admission_gate.py) wraz z odniesieniem do oryginalnych [testów Pythona](../../../tools/test_agent_tool_admission_gate.py). Port funkcji Python admit_tool: SHA-256 bajtów, jawna allowlista czterech capability, niezależne zgody oparte o digest, nazwa odizolowanego profilu, timeout 1–60 sekund, ważna data wygaśnięcia oraz zakaz skutków ubocznych.

**Granica:** Hash artefaktu nie stanowi uwierzytelnienia osoby przeglądającej. Mapę approvals i czas 'now' musi dostarczyć oddzielnie zaufany host. Model nie może ich dołączyć w manifest/tool JSON.

## Sposób uruchomienia

```bash
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-006/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-006/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-006/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-006/Cargo.toml" -- demo
```

Ostatni argument `demo` jest nieprodukcyjnym smoke testem, nie uprawnia do narzędzi systemowych i nie przesyła danych do sieci.

## Architektura

Agent Rust `custos_admission_agent` z `#![forbid(unsafe_code)]`, domeną `src/agent.rs`, walidacją `src/security.rs`, lifecycle `src/lifecycle.rs` i adapterem `tool.admit` w `src/zenoh.rs`. OMAs = **serwer agentów**, OMAt = **serwer narzędzi**, Zenoh = docelowy transport; działają tu tylko nazwy tras i lokalny OCAF ToolExecutor. Odczyt zaszyfrowanych źródeł OSTc nadal **DENIED**.

## Dokumenty

[Architektura](ARCHITECTURE.md) · [Specyfikacja](SPECIFICATION.md) · [Instrukcja](USER_MANUAL.md) · [Model bezpieczeństwa](SECURITY.md) · [Serwis](SERVICE.md) · [Tożsamość i serial](IDENTITY_AND_SERIAL.md) · [Zgodność z Python](PYTHON_PARITY.md) · [Zależności](DEPENDENCIES.md) · [Macierz weryfikacji](TRACEABILITY_MATRIX.md) · [Testy](TEST_EVIDENCE.md) · [Kryteria wydania](RELEASE_CHECKLIST.md) · [Migracja](MIGRATION.md) · [Zmiany](CHANGELOG.md) · [Rekord certyfikacyjny](../certification/CERTIFICATE_RECORD.md) · [Sprzedaż](commercial/SALE-DRAFT.md) · [Dzierżawa](commercial/LEASE-DRAFT.md) · [Najem](commercial/RENT-DRAFT.md).

**Nie jest to aktywny serwer, narzędzie wykonujące arbitrary code, działający OSTc ani certyfikowany agent.**
