# Macierz portu Python do Rust — FIDES GRANTIS

**FIDES GRANTIS** · ewidencja **012** · serial `AM-GRT-FID-2026-000012` · Agent ID `AM-OCAF-AI-0012` · katalog `AM-OCAF-GRT-012` · certyfikat `CERT-AM-2026-0012` **NIEWYDANY**.

Oryginały [`tools/program_claim_gate.py`](../../../tools/program_claim_gate.py) i [`tools/test_program_claim_gate.py`](../../../tools/test_program_claim_gate.py) pozostają bez zmian.

## Zachowane zachowania

Sprawdza wyłącznie repo Knowledge-projects, działania research/draft, brak credentiali, zakres deklarowanych kredytów i tokenów programu Alibaba AI Catalyst, historyczny status General Learning Hacks i wymaganie zweryfikowanej kwalifikowalności przy tworzeniu szkicu.

## Różnice semantyczne i ograniczenia

Ustalony świadomie schemat JSON z opcjonalnymi liczbami całkowitymi, czas z niezależnego hosta; odrzuca nieznane pola, a bool nie jest liczbą całkowitą. Informacje o programach są ograniczone do zapisanych reguł źródłowego Python — nie stanowią aktualnego potwierdzenia ich dostępności.

Oryginalne Python unittest uruchamia się w osobnym kroku CI: `python3 -m unittest discover -s tools -p 'test_program_claim_gate.py' -v`. Fakt, że zestawy Rust i Python niezależnie przechodzą, nie stanowi pełnego dowodu równoważności semantycznej. Do certyfikacji wymagane golden fixtures i property-based differential tests.
