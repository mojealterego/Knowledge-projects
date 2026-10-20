# Migracja źródła — CUSTOS ADMISSION

**CUSTOS ADMISSION** · nr ewidencyjny **006** · nr seryjny `AM-SEC-CUS-2026-000006` · Agent ID `AM-OCAF-AI-0006` · katalog `AM-OCAF-SEC-006` · certyfikat `CERT-AM-2026-0006` — **NIEWYDANY**.

Dokonano **portu logiki z istniejącego, czytelnego modułu Python**, a nie pozornej migracji nieistniejącej aplikacji: `tools/agent_tool_admission_gate.py` i jego testów `tools/test_agent_tool_admission_gate.py`. Oryginały pozostają w katalogu `tools/` i nie zostały usunięte. Nowy plik `src/agent.rs` implementuje odpowiedni mechanizm w Rust.

**Pochodzenie reguł:** Port funkcji Python admit_tool: SHA-256 bajtów, jawna allowlista czterech capability, niezależne zgody oparte o digest, nazwa odizolowanego profilu, timeout 1–60 sekund, ważna data wygaśnięcia oraz zakaz skutków ubocznych.

**Zakres pominięty lub wymagający środowiska:** niezależny podpis approval, działający izolator, walidacja sygnatur i proweniencji, storage i komunikacja OMAt/OMAs. OMAs/OMAt/Zenoh i OSTc nie są instalowane ani aktywowane. Nie udawać wykonania testów produkcyjnych i nie wydać certyfikatu.
