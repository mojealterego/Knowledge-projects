# Wydzielenie narzędzia Python — CLOUD FISCALIS

**Nazwa własna:** CLOUD FISCALIS · **Numer ewidencyjny:** 009 · **Serial:** `AM-CLD-FIS-2026-000009` · **Agent ID:** `AM-OCAF-AI-0009` · **Katalog:** `AM-OCAF-CLD-009` · **Certyfikat:** `CERT-AM-2026-0009` — NIEWYDANY.

Zbadano rzeczywisty kod [`tools/cloud_operation_budget_gate.py`](../../../tools/cloud_operation_budget_gate.py) oraz jego testy [`tools/test_cloud_operation_budget_gate.py`](../../../tools/test_cloud_operation_budget_gate.py), a następnie napisano odrębną implementację Rust w `src/agent.rs`. Nie jest to migracja pełnego serwera OMAs, OMAt, Zenoh ani OSTc. Oryginały pozostały bez zmian.

Rust 009 zachowuje reguły plan-only ze źródłowego Python, ale dodatkowo odrzuca secrets_in_prompt nawet w ścieżce public_docs, w której Python kończył pracę przed tą kontrolą. JSON cost musi być stringiem — schema odrzuca liczby i booleans przed wywołaniem domeny. Zakres maksymalnego kosztu to precyzyjna arytmetyka decimal.

Wynik portu to lokalny analizator bez skutków ubocznych i bez wywołań zewnętrznych. Moduł nie zakłada posiadania rzeczywistej zgody właściciela, dostępu do cloud, instancji FIrecracker czy wydanego certyfikatu.
