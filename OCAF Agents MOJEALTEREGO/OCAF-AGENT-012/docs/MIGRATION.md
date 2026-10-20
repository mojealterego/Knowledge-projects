# Proweniencja i migracja — FIDES GRANTIS

**FIDES GRANTIS** · ewidencja **012** · serial `AM-GRT-FID-2026-000012` · Agent ID `AM-OCAF-AI-0012` · katalog `AM-OCAF-GRT-012` · certyfikat `CERT-AM-2026-0012` **NIEWYDANY**.

Źródłem kodu jest istniejący [`tools/program_claim_gate.py`](../../../tools/program_claim_gate.py) oraz [testy referencyjne](../../../tools/test_program_claim_gate.py). W Rust napisano samodzielne procedury oceny, nie migrację aktywnej sieciowej usługi. Nie zmieniono oryginalnych plików.

**Portowane reguły:** Sprawdza wyłącznie repo Knowledge-projects, działania research/draft, brak credentiali, zakres deklarowanych kredytów i tokenów programu Alibaba AI Catalyst, historyczny status General Learning Hacks i wymaganie zweryfikowanej kwalifikowalności przy tworzeniu szkicu.

**Jawne odchylenia:** Ustalony świadomie schemat JSON z opcjonalnymi liczbami całkowitymi, czas z niezależnego hosta; odrzuca nieznane pola, a bool nie jest liczbą całkowitą. Informacje o programach są ograniczone do zapisanych reguł źródłowego Python — nie stanowią aktualnego potwierdzenia ich dostępności.

**Niekompletne:** Weryfikacja bieżących regulaminów i kwalifikowalności; prawdziwy grant issuer, konta programów, API zewnętrznych dostawców, rzeczywiste wnioski, podpisany dostęp. Integracja rzeczywistych systemów wymaga odrębnej zgody i dowodów.
