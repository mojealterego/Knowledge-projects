# Evolucja portfolio — partia 14 / 2026-10-08

## Właściciel źródła → faktyczna zmiana kanoniczna
| Klaster źródeł | Projekty | Zakres |
|---|---|---|
| AURA 60 kart / termochrom / NFC | **P50, P71** | pełny manifest + production QC + 30 missing instances + consent/gambling safeguards |
| GCG adwersarze modeli | **P60, P108** | defensywny threat-model/evaluation harness; bez gotowych payloadów jailbreak |
| OmniCore Omega / AI zamiast kodu | **P09, P26, P80** | zweryfikowane safety-fallback jądra, suwerenny model uprawnień, signed build/gate |
| Agentic SOP / 2 różne pliki z tym samym tekstem OmniCore | **P90, P115** | bezpieczne wywoływanie zadań / role broker / receipts |
| Siwak strategic competences | **P66** | competency mapping, ekonomiczna wartość jako hipoteza, weryfikacja niezależna |
| Dubisz lexical / seksualizacja | **P75** | rozdzielanie znaczeń/rejestrów, provenance debaty, no intent guess |
| ASUS 2020 notebook manual | **P26** | hardware-doc provenance, model/firmware verification, evidence preservation |
| Numeracja/dedupe | **P47** | preflight full tree, source text equivalence vs byte identity |

**Wersjonowanie:** dokument `docs/KNOWLEDGE-INGESTION-2026-10-08-BATCH-14.md` przechowuje hash 10 źródeł. **Nowych numerowanych projektów: 0**, ponieważ wszystkie wiązki są już objęte konkretnymi właścicielami. Nie powstaje drugi „AURA Foundry”, „GCG Lab” ani „OmniCore OS” bez osobnej granicy produktu.

## Realnie wykonany kod
Dodano `tools/aura_deck_gate.py` i `tools/test_aura_deck_gate.py`, **11 testów unittest pomyślnych lokalnie**. Walidator fail-closed, **nie uzupełnia 30 brakujących kart** i nie sugeruje manipulacyjnych/uzależniających funkcji. Zmiany dodatkowo obejmują opcjonalną konfigurację GitHub Actions uruchamiającą narzędziowe testy Python bez zewnętrznych API — jej wynik po commicie musi zostać odczytany, nie zakładany.

## Wymagane dowody przed oznaczeniem wdrożenia
- AURA: pełne 60 kart z unikalnymi ID, zasady, temperatura, faktura, alergie/zapach, NFC zgoda, dostępność, próby drukarskie i koszty.
- GCG: wyłącznie kontrolowana ewaluacja modeli ze zgodą właściciela, raport ASR/robustness bez payloadów szkodliwych.
- OmniCore: zbudowane binaria/Rust no_std, QEMU testy boot i awaryjnego fallbacku, benchmarki.
- ASUS: aktualny dokument zgodny z dokładnie ustalonym modelem/firmware, brak domyślnie destrukcyjnego resetu.
- SOP: rzeczywiste wykonanie przez narzędzia i niezależna weryfikacja.
- Strategia/język: weryfikacja czasu źródeł i niezależne wyniki, unikanie nadinterpretacji.

**Stany:** analiza wiedzy i edycja dokumentów — EXECUTED; lokalne testy walidatora — PASSED; druk, fizyczny produkt, kernel, live GCG, chmura, prawdziwe maszyny i aktualne informacje dostawców — NOT VERIFIED/NOT EXECUTED.
