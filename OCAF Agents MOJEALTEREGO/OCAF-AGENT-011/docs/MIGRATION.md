# Pochodzenie i port — LIMES GIT

**Nazwa:** LIMES GIT · **Ewidencja:** 011 · **Serial:** `AM-GIT-LIM-2026-000011` · **Agent ID:** `AM-OCAF-AI-0011` · **Katalog:** `AM-OCAF-GIT-011` · **Certyfikat:** `CERT-AM-2026-0011` — NIEWYDANY.

To port **istniejącego kodu źródłowego** [`tools/single_repository_scope_gate.py`](../../../tools/single_repository_scope_gate.py) z testami [`tools/test_single_repository_scope_gate.py`](../../../tools/test_single_repository_scope_gate.py), a nie rekonstrukcja na podstawie opisu. Oryginały Python zachowano.

Zakres portu: Odczyt innych repozytoriów jest dopuszczony; zapis wyłącznie do mojealterego/Knowledge-projects przez roboczą gałąź knowledge/, policy/, feature/ lub fix/. PR i merge wymagają pull_base=main i poprawnego pull_head. Bezpośredni zapis do main jest blokowany.

Kontrola zmian: Rust używa ścisłego serde deny_unknown_fields i limitu długości gałęzi 160 bajtów. Reguły operacji, prefiksy branchy oraz odmowa cross-repo write odpowiadają źródłowemu Python. Oryginał nie był interceptor GitHub, Rust również nim nie jest.

Nie skopiowano tajnych kluczy, nie uruchomiono serwerów OMAs/OMAt/OSTc, nie wykonano rzeczywistych operacji na zewnętrznych systemach. Rozszerzenia wymagają osobnych testów, audytu i dokumentacji.
