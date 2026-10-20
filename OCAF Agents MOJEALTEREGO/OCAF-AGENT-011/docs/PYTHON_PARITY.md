# Zgodność zakresu Python / Rust — LIMES GIT

**Nazwa:** LIMES GIT · **Ewidencja:** 011 · **Serial:** `AM-GIT-LIM-2026-000011` · **Agent ID:** `AM-OCAF-AI-0011` · **Katalog:** `AM-OCAF-GIT-011` · **Certyfikat:** `CERT-AM-2026-0011` — NIEWYDANY.

Źródło: [`tools/single_repository_scope_gate.py`](../../../tools/single_repository_scope_gate.py). Testy: [`tools/test_single_repository_scope_gate.py`](../../../tools/test_single_repository_scope_gate.py).

**Portowane reguły:** Odczyt innych repozytoriów jest dopuszczony; zapis wyłącznie do mojealterego/Knowledge-projects przez roboczą gałąź knowledge/, policy/, feature/ lub fix/. PR i merge wymagają pull_base=main i poprawnego pull_head. Bezpośredni zapis do main jest blokowany.

**Jawne różnice:** Rust używa ścisłego serde deny_unknown_fields i limitu długości gałęzi 160 bajtów. Reguły operacji, prefiksy branchy oraz odmowa cross-repo write odpowiadają źródłowemu Python. Oryginał nie był interceptor GitHub, Rust również nim nie jest.

Oryginalne pliki Python pozostają niezmienione. Uruchomienie obu niezależnych zestawów testów nie jest dowodem formalnej równoważności wszystkich możliwych wejść. Nie ma integracji produkcyjnych ani uprawnień do działań sieciowych. Dalsze kroki to property-based tests i golden fixtures dla trudnych przypadków.
