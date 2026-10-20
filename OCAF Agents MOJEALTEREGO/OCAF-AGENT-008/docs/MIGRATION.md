# Wydzielenie narzędzia Python — DGM SENTINEL

**Nazwa własna:** DGM SENTINEL · **Numer ewidencyjny:** 008 · **Serial:** `AM-DGM-SNT-2026-000008` · **Agent ID:** `AM-OCAF-AI-0008` · **Katalog:** `AM-OCAF-DGM-008` · **Certyfikat:** `CERT-AM-2026-0008` — NIEWYDANY.

Zbadano rzeczywisty kod [`tools/dgm_cycle_gate.py`](../../../tools/dgm_cycle_gate.py) oraz jego testy [`tools/test_dgm_cycle_gate.py`](../../../tools/test_dgm_cycle_gate.py), a następnie napisano odrębną implementację Rust w `src/agent.rs`. Nie jest to migracja pełnego serwera OMAs, OMAt, Zenoh ani OSTc. Oryginały pozostały bez zmian.

Rust 008 dodaje wymóg hostowych zatwierdzeń receiptów czterech faz, źródła i rollbacku ponad pierwotne pola proposal w Python. Zaostrza format integration branch i ogranicza rozmiar wejścia. Pozostała lista kontroli pochodzi z oryginalnego Python. Nie oznacza to ścisłej bitowej równoważności.

Wynik portu to lokalny analizator bez skutków ubocznych i bez wywołań zewnętrznych. Moduł nie zakłada posiadania rzeczywistej zgody właściciela, dostępu do cloud, instancji FIrecracker czy wydanego certyfikatu.
