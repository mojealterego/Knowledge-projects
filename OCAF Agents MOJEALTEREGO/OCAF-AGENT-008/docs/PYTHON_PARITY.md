# Porównanie oryginału Python z Rust — DGM SENTINEL

**Nazwa własna:** DGM SENTINEL · **Numer ewidencyjny:** 008 · **Serial:** `AM-DGM-SNT-2026-000008` · **Agent ID:** `AM-OCAF-AI-0008` · **Katalog:** `AM-OCAF-DGM-008` · **Certyfikat:** `CERT-AM-2026-0008` — NIEWYDANY.

Oryginał: [`tools/dgm_cycle_gate.py`](../../../tools/dgm_cycle_gate.py). Testy referencyjne: [`tools/test_dgm_cycle_gate.py`](../../../tools/test_dgm_cycle_gate.py). Nie zmieniono oryginalnych plików Python.

## Zachowane reguły

Strukturalna kontrola 40-znakowych commit SHA, izolowanej nazwy gałęzi integration/, 4 faz ModelScout, TechRecon, Strategist, DGM_Core, listy required_checks, UTC expiry oraz zakazu skutków ubocznych.

## Celowe odchylenia i wzmocnienia

Rust 008 dodaje wymóg hostowych zatwierdzeń receiptów czterech faz, źródła i rollbacku ponad pierwotne pola proposal w Python. Zaostrza format integration branch i ogranicza rozmiar wejścia. Pozostała lista kontroli pochodzi z oryginalnego Python. Nie oznacza to ścisłej bitowej równoważności.

## Walidacja

CI ma oddzielnie uruchamiać `cargo test --all-features`, `cargo clippy -D warnings`, Rustfmt i `python3 -m unittest discover -s tools -p 'test_dgm_cycle_gate.py' -v`. Testy oryginału służą potwierdzeniu referencji, ale nie dowodzą pełnej równoważności całej przestrzeni wejścia. Dopiero niezależne golden fixtures i testy różnicowe mogą zwiększyć poziom zaufania.

**Żaden test nie uruchamia sieciowego OMAs/OMAt/OSTc ani nie nadaje uprawnień do wykonywania operacji zewnętrznych.**
