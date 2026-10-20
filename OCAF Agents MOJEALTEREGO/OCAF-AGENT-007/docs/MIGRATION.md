# Migracja źródła — VIGIA LOOP

**VIGIA LOOP** · nr ewidencyjny **007** · nr seryjny `AM-OPS-VIG-2026-000007` · Agent ID `AM-OCAF-AI-0007` · katalog `AM-OCAF-OPS-007` · certyfikat `CERT-AM-2026-0007` — **NIEWYDANY**.

Dokonano **portu logiki z istniejącego, czytelnego modułu Python**, a nie pozornej migracji nieistniejącej aplikacji: `tools/agent_tool_loop_guard.py` i jego testów `tools/test_agent_tool_loop_guard.py`. Oryginały pozostają w katalogu `tools/` i nie zostały usunięte. Nowy plik `src/agent.rs` implementuje odpowiedni mechanizm w Rust.

**Pochodzenie reguł:** Port Python AgentToolLoopGuard: limit łącznej liczby wywołań, limit zużycia tokenów, maksymalna sekwencja identycznych wywołań bez postępu, deterministyczna odmowa i trwały stan halted, reset powtórzeń tylko po host-owned readback.

**Zakres pominięty lub wymagający środowiska:** wielohostowe współdzielone przechowywanie stanu, niezależne wydawanie readback receipts, śledzenie rzeczywistej liczby tokenów, atomowe mechanizmy HA. OMAs/OMAt/Zenoh i OSTc nie są instalowane ani aktywowane. Nie udawać wykonania testów produkcyjnych i nie wydać certyfikatu.
