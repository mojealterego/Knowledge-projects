# Porównanie oryginału Python z Rust — CLOUD FISCALIS

**Nazwa własna:** CLOUD FISCALIS · **Numer ewidencyjny:** 009 · **Serial:** `AM-CLD-FIS-2026-000009` · **Agent ID:** `AM-OCAF-AI-0009` · **Katalog:** `AM-OCAF-CLD-009` · **Certyfikat:** `CERT-AM-2026-0009` — NIEWYDANY.

Oryginał: [`tools/cloud_operation_budget_gate.py`](../../../tools/cloud_operation_budget_gate.py). Testy referencyjne: [`tools/test_cloud_operation_budget_gate.py`](../../../tools/test_cloud_operation_budget_gate.py). Nie zmieniono oryginalnych plików Python.

## Zachowane reguły

Walidacja ośmiu kategorii produktów i akcji, strict decimal-string USD przez rust_decimal, request_id, project_scope, allowlist grantów, budżetu i zakazu sekretów w promptach. Dokumentacja publiczna wyłącznie przy zerowym koszcie i publicznej klasyfikacji.

## Celowe odchylenia i wzmocnienia

Rust 009 zachowuje reguły plan-only ze źródłowego Python, ale dodatkowo odrzuca secrets_in_prompt nawet w ścieżce public_docs, w której Python kończył pracę przed tą kontrolą. JSON cost musi być stringiem — schema odrzuca liczby i booleans przed wywołaniem domeny. Zakres maksymalnego kosztu to precyzyjna arytmetyka decimal.

## Walidacja

CI ma oddzielnie uruchamiać `cargo test --all-features`, `cargo clippy -D warnings`, Rustfmt i `python3 -m unittest discover -s tools -p 'test_cloud_operation_budget_gate.py' -v`. Testy oryginału służą potwierdzeniu referencji, ale nie dowodzą pełnej równoważności całej przestrzeni wejścia. Dopiero niezależne golden fixtures i testy różnicowe mogą zwiększyć poziom zaufania.

**Żaden test nie uruchamia sieciowego OMAs/OMAt/OSTc ani nie nadaje uprawnień do wykonywania operacji zewnętrznych.**
