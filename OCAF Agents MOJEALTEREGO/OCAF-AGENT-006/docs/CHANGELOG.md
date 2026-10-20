# Changelog — CUSTOS ADMISSION

## 0.1.0-dev — 20.10.2026

- Wydzielono nowy autonomicznie adresowalny moduł Rust na podstawie `tools/agent_tool_admission_gate.py`.
- Nadano ewidencję 006, unikatową nazwę `CUSTOS ADMISSION`, serial `AM-SEC-CUS-2026-000006`, osobny agent ID i nr katalogowy.
- Przeniesiono logikę bezpieczeństwa: Port funkcji Python admit_tool: SHA-256 bajtów, jawna allowlista czterech capability, niezależne zgody oparte o digest, nazwa odizolowanego profilu, timeout 1–60 sekund, ważna data wygaśnięcia oraz zakaz skutków ubocznych.
- Związano granice zaufania z host-only API; model nie może sam autoryzować żądań.
- Dodano ograniczony `ToolExecutor` lokalnego OCAF, testy bezpieczeństwa i logi CI.
- Nie uruchomiono sieci Zenoh, produkcyjnych OMAs/OMAt ani OSTc.
- Certyfikat niewydany; wymagana akceptacja operatora i podpisany release.
