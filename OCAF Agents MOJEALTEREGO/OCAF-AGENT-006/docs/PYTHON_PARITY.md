# Porównanie z oryginalnym Python — CUSTOS ADMISSION

**CUSTOS ADMISSION** · nr ewidencyjny **006** · nr seryjny `AM-SEC-CUS-2026-000006` · Agent ID `AM-OCAF-AI-0006` · katalog `AM-OCAF-SEC-006` · certyfikat `CERT-AM-2026-0006` — **NIEWYDANY**.

**Źródło oryginału:** [`tools/agent_tool_admission_gate.py`](../../../tools/agent_tool_admission_gate.py) i [`tools/test_agent_tool_admission_gate.py`](../../../tools/test_agent_tool_admission_gate.py). Oryginalne narzędzie i testy Pythona pozostają niezmienione i są materiałem do porównania.

## Zrealizowane mechanizmy

Port funkcji Python admit_tool: SHA-256 bajtów, jawna allowlista czterech capability, niezależne zgody oparte o digest, nazwa odizolowanego profilu, timeout 1–60 sekund, ważna data wygaśnięcia oraz zakaz skutków ubocznych.

## Zmiany w porcie i jawne różnice

| Zachowanie | Python | Rust 006 |
|---|---|---|
| Hash | `hashlib.sha256(bytes)` | `sha2::Sha256` |
| Approval | `trusted_approvals` dostarczone poza manifestem | `BTreeMap<String,BTreeSet<String>>` w host-owned AgentTool |
| Capabilities | cztery stałe dozwolone | identyczna allowlista |
| Expiry | aware `datetime`, RFC3339-Z | `DateTime<Utc>`, `parse_from_rfc3339` z końcowym Z |
| Timeout bool | Python odrzuca `bool` jako `int` | `serde` nie deserializuje boolean do `u32` |
| Nieznane pola | Python je ignoruje | Rust `deny_unknown_fields` odrzuca |
| Artifact | Python `bytes`, brak limitu | lokalne `&[u8]` do 1 MiB, OCAF `artifact_hex` |
| Execution | nigdy nie wykonuje | nigdy nie wykonuje; `execution_authorized=false` |

## Status weryfikacji

CI Rust obejmuje testy odpowiadające scenariuszom oryginalnym i nowe negatywne testy OCAF. Porównanie nie jest pełnym testem formalnej równoważności programów; upstream oryginalne testy można dodatkowo uruchamiać jako osobną macierz, bez twierdzenia że Rust i Python są tożsame dla wszystkich wejść. W szczególności niezależny podpis approval, działający izolator, walidacja sygnatur i proweniencji, storage i komunikacja OMAt/OMAs.
