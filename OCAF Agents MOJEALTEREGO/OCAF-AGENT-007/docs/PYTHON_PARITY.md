# Porównanie z oryginalnym Python — VIGIA LOOP

**VIGIA LOOP** · nr ewidencyjny **007** · nr seryjny `AM-OPS-VIG-2026-000007` · Agent ID `AM-OCAF-AI-0007` · katalog `AM-OCAF-OPS-007` · certyfikat `CERT-AM-2026-0007` — **NIEWYDANY**.

**Źródło oryginału:** [`tools/agent_tool_loop_guard.py`](../../../tools/agent_tool_loop_guard.py) i [`tools/test_agent_tool_loop_guard.py`](../../../tools/test_agent_tool_loop_guard.py). Oryginalne narzędzie i testy Pythona pozostają niezmienione i są materiałem do porównania.

## Zrealizowane mechanizmy

Port Python AgentToolLoopGuard: limit łącznej liczby wywołań, limit zużycia tokenów, maksymalna sekwencja identycznych wywołań bez postępu, deterministyczna odmowa i trwały stan halted, reset powtórzeń tylko po host-owned readback.

## Zmiany w porcie i jawne różnice

| Zachowanie | Python | Rust 007 |
|---|---|---|
| Liczniki | ints, identyczne/budżetowe | `u64`, overflow fail-closed |
| Odcisk argumentów | 64 małe hex | identyczny wzorzec |
| Progress | `trusted_progress_version` argument kontrolowany przez hosta | **separatne `advance_verified_progress`**, modelowy input nie ma tego pola |
| Loop state | mutable object | `Guard`, trwały `Mutex<Guard>` per OCAF adapter instance |
| Przekroczenie budżetu | `halted=True` | sticky halt, odmowa kolejnych prób |
| Execution | brak | brak; `execution_authorized=false` |

## Status weryfikacji

CI Rust obejmuje testy odpowiadające scenariuszom oryginalnym i nowe negatywne testy OCAF. Porównanie nie jest pełnym testem formalnej równoważności programów; upstream oryginalne testy można dodatkowo uruchamiać jako osobną macierz, bez twierdzenia że Rust i Python są tożsame dla wszystkich wejść. W szczególności wielohostowe współdzielone przechowywanie stanu, niezależne wydawanie readback receipts, śledzenie rzeczywistej liczby tokenów, atomowe mechanizmy HA.
