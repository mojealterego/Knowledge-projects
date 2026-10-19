# Agent 001 — WDA Photo Agent / OCAF Runtime

**Numer ewidencyjny: 001** · Agent ID: **AM-OCAF-AI-0001** · powiązany projekt: [P116](../../projekty/116-wda-photo-agent-max/README.md).

**Stan:** PARTIAL / DRAFT / NOT PRODUCTION READY. Ewidencja agenta, nie ukończony certyfikat ani wdrożenie. Data: 2026-10-19.

## Wydzielenie z katalogu projektów

Osiem oryginalnych plików kodu i dokumentacji `source-v2/` przeniesiono **bez modyfikacji blob SHA** do [`legacy-pwa/`](legacy-pwa/); pierwotna ścieżka zachowała dokument przekierowujący. Projekt P116 i jego architektura pozostają w `projekty/`.

Obecny serwer Node w `legacy-pwa/` **nie jest bezpieczny do publicznego wdrożenia**: nieograniczony rozmiar body, brak operator authz dla płatnych wywołań Adobe, nieutwardzone statyczne ścieżki. Kod zachowany do audytu i testów porównawczych.

## OCAF

Nowa implementacja `rust-agent/` zapewnia deterministyczną, bezpieczną funkcję `photo.plan` (plan bez efektów ubocznych), z adapterem `ToolExecutor` i `ToolManifest` OCAF pod flagą `ocaf-integration`. Nie ma produkcyjnego podłączenia OMAs, analizy wizji, Adobe ani OSTc. Weryfikacja kompilacji i CI obowiązkowa przed oznaczeniem TESTED.

## Sześć odrębnych dokumentów

1. [Certyfikat identyfikacyjny — projekt niewydany](documentation/01-CERTIFICATE-DRAFT.md)
2. [Charakterystyka / specyfikacja / wycena](documentation/02-SPECIFICATION.md)
3. [Instrukcja obsługi](documentation/03-OPERATIONS.md)
4. [Umowa sprzedaży — projekt](documentation/04-SALE-DRAFT.md)
5. [Umowa dzierżawy — projekt](documentation/05-LEASE-DRAFT.md)
6. [Umowa najmu — projekt](documentation/06-RENT-DRAFT.md)

Pozostałe: [manifest](agent-manifest.json), [architektura](ARCHITECTURE.md), [zagrożenia](SECURITY.md), [testy i dowody](TEST_EVIDENCE.md), [procedury](SERVICE.md), [changelog](CHANGELOG.md).
