# Changelog — VIGIA LOOP

## 0.1.0-dev — 20.10.2026

- Wydzielono nowy autonomicznie adresowalny moduł Rust na podstawie `tools/agent_tool_loop_guard.py`.
- Nadano ewidencję 007, unikatową nazwę `VIGIA LOOP`, serial `AM-OPS-VIG-2026-000007`, osobny agent ID i nr katalogowy.
- Przeniesiono logikę bezpieczeństwa: Port Python AgentToolLoopGuard: limit łącznej liczby wywołań, limit zużycia tokenów, maksymalna sekwencja identycznych wywołań bez postępu, deterministyczna odmowa i trwały stan halted, reset powtórzeń tylko po host-owned readback.
- Związano granice zaufania z host-only API; model nie może sam autoryzować żądań.
- Dodano ograniczony `ToolExecutor` lokalnego OCAF, testy bezpieczeństwa i logi CI.
- Nie uruchomiono sieci Zenoh, produkcyjnych OMAs/OMAt ani OSTc.
- Certyfikat niewydany; wymagana akceptacja operatora i podpisany release.
