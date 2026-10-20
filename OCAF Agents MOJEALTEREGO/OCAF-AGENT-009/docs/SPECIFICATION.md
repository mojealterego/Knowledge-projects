# Specyfikacja techniczna — CLOUD FISCALIS

**Nazwa własna:** CLOUD FISCALIS · **Numer ewidencyjny:** 009 · **Serial:** `AM-CLD-FIS-2026-000009` · **Agent ID:** `AM-OCAF-AI-0009` · **Katalog:** `AM-OCAF-CLD-009` · **Certyfikat:** `CERT-AM-2026-0009` — NIEWYDANY.

## Założenia

ocena planu operacji Google Cloud pod względem produktu, zakresu projektu, kosztu i niezależnej zgody. Walidacja ośmiu kategorii produktów i akcji, strict decimal-string USD przez rust_decimal, request_id, project_scope, allowlist grantów, budżetu i zakazu sekretów w promptach. Dokumentacja publiczna wyłącznie przy zerowym koszcie i publicznej klasyfikacji.

## Interfejs wywołania

Nazwa OMAt/OCAF tool (lokalnie): `cloud.review_budget`.

Wejście `CloudPlan`: `action`, `product`, `maximum_cost_usd` jako string decimal, `request_id`, `project_scope`, `data_classification`, `credentials_in_prompt`, `explicit_external_side_effect_approval`. `CloudApproval` jest przekazywane wyłącznie przez host i zawiera scope, allowed_actions, allowed_products, Decimal max_cost_usd i human_reviewed. Żadne opłaty nie są pobierane.

Wynik: `CloudVerdict { allowed, reasons[], execution_authorized=false, charge_incurred=false }`; oba znaczniki wykonania są zawsze **false**. `cost_sats=0` oznacza lokalną operację bez kosztu narzędzia, nie zatwierdzony cennik rynkowy.

## Dokumentacja pochodzenia i ograniczeń

Źródło [Python](../../../tools/cloud_operation_budget_gate.py) pozostaje w repo i nie jest usunięte. Rust 009 zachowuje reguły plan-only ze źródłowego Python, ale dodatkowo odrzuca secrets_in_prompt nawet w ścieżce public_docs, w której Python kończył pracę przed tą kontrolą. JSON cost musi być stringiem — schema odrzuca liczby i booleans przed wywołaniem domeny. Zakres maksymalnego kosztu to precyzyjna arytmetyka decimal.

**Poza zakresem:** Google Cloud IAM auth, billing/quota API, zaszyfrowane sekrety, wykonanie działań, niezależnie podpisane granty i interfejs dla użytkownika. Brak podpisanego certyfikatu, portretu/logo i uruchomionego środowiska produkcyjnego.
