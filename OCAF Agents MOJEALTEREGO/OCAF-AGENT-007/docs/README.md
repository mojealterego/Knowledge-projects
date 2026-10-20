# VIGIA LOOP — OCAF-AGENT-007

**VIGIA LOOP** · nr ewidencyjny **007** · nr seryjny `AM-OPS-VIG-2026-000007` · Agent ID `AM-OCAF-AI-0007` · katalog `AM-OCAF-OPS-007` · certyfikat `CERT-AM-2026-0007` — **NIEWYDANY**.

**Wydanie `0.1.0-dev` — `INCOMPLETE_DEVELOPMENT`; `production_approved=false`, `certificate_issued=false`.**

## Cel

stanowa kontrola zapętleń i budżetów wywołań narzędzi agenta. Jest to Rust port rzeczywistego pliku [`tools/agent_tool_loop_guard.py`](../../../tools/agent_tool_loop_guard.py) wraz z odniesieniem do oryginalnych [testów Pythona](../../../tools/test_agent_tool_loop_guard.py). Port Python AgentToolLoopGuard: limit łącznej liczby wywołań, limit zużycia tokenów, maksymalna sekwencja identycznych wywołań bez postępu, deterministyczna odmowa i trwały stan halted, reset powtórzeń tylko po host-owned readback.

**Granica:** Trusted progress nie może pochodzić z wiadomości agenta lub pola tool JSON; host musi świadomie wywołać advance_verified_progress po niezależnym readback. Węzeł OCAF musi trwale zachować instancję Guard w obrębie sesji.

## Sposób uruchomienia

```bash
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-007/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-007/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-007/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-007/Cargo.toml" -- demo
```

Ostatni argument `demo` jest nieprodukcyjnym smoke testem, nie uprawnia do narzędzi systemowych i nie przesyła danych do sieci.

## Architektura

Agent Rust `vigia_loop_agent` z `#![forbid(unsafe_code)]`, domeną `src/agent.rs`, walidacją `src/security.rs`, lifecycle `src/lifecycle.rs` i adapterem `tool.loop_gate` w `src/zenoh.rs`. OMAs = **serwer agentów**, OMAt = **serwer narzędzi**, Zenoh = docelowy transport; działają tu tylko nazwy tras i lokalny OCAF ToolExecutor. Odczyt zaszyfrowanych źródeł OSTc nadal **DENIED**.

## Dokumenty

[Architektura](ARCHITECTURE.md) · [Specyfikacja](SPECIFICATION.md) · [Instrukcja](USER_MANUAL.md) · [Model bezpieczeństwa](SECURITY.md) · [Serwis](SERVICE.md) · [Tożsamość i serial](IDENTITY_AND_SERIAL.md) · [Zgodność z Python](PYTHON_PARITY.md) · [Zależności](DEPENDENCIES.md) · [Macierz weryfikacji](TRACEABILITY_MATRIX.md) · [Testy](TEST_EVIDENCE.md) · [Kryteria wydania](RELEASE_CHECKLIST.md) · [Migracja](MIGRATION.md) · [Zmiany](CHANGELOG.md) · [Rekord certyfikacyjny](../certification/CERTIFICATE_RECORD.md) · [Sprzedaż](commercial/SALE-DRAFT.md) · [Dzierżawa](commercial/LEASE-DRAFT.md) · [Najem](commercial/RENT-DRAFT.md).

**Nie jest to aktywny serwer, narzędzie wykonujące arbitrary code, działający OSTc ani certyfikowany agent.**
