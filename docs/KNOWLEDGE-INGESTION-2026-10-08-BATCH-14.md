# Automatyczna ingestia partii 14 — 2026-10-08

**Uruchomienie:** przesłano 10 plików PDF; zgodnie z protokołem Knowledge-projects analiza rozpoczęła się bez czekania na „Analizuj”.
**Repozytorium:** `mojealterego/Knowledge-projects`; przyjęta baza GitHub `main@d3d0fafa82f8374211ace48db858dae20cc04c31`.
**Metoda:** odczyt tekstu wszystkich 10 plików i ich liczby stron, SHA-256 surowych bajtów, porównanie treści dwóch edycji OmniCore, kontrola wcześniejszej bazy wiedzy i właścicieli projektów. Nie kopiowano podręcznika ASUS ani cudzych publikacji do publicznego GitHuba.

## Manifest źródeł
| # | Przesłany plik | Liczba stron | SHA-256 | Projekt | Decyzja |
|---:|---|---:|---|---|---|
| 1 | `art_34-39_Siwak.pdf` | 7 | `fd653d67638e1be2194a00c7a0b9774326c585b0967dc90e77c2361619e28b41` | P66 | TOPIC_REVISIT: 2015 strategy/competence, 5 interviews |
| 2 | `articles-2083591.pdf.pdf` | 3 | `8463795411943a90215973cc5e8cc7648e1d32942cb19e9b3752162ffca662aa` | P75/P36 | NEW_LINGUISTIC_DELTA: semantic drift of terms about sexuality |
| 3 | `AURA_ Specyfikacja Kart i Strategia Wdrożenia (202....pdf` | 5 | `bf5b133ef68da2af59c23a77a6ab040cd924416d4134b06679ddd95f1ba4d730` | P50/P71 | TOPIC_REVISIT + PRODUCTION_GAP: 60-card count missing 30 definitions |
| 4 | `Automatyzacja Projektu z AI_ Instrukcja i Wykonani....pdf` | 11 | `42c1bf390d38db2e42f69eab9d636c0912634bb7b33c4362e8d403a0c53bbe97` | P90/P115 | TOPIC_REVISIT: SOP->execution and verification receipts |
| 5 | `Automatyzacja Tworzenia Oprogramowania z AI_260105_120906.pdf` | 11 | `421c39438e3fd47ade76f98e9e7aaf4bb8e10301da8dc22d4b65888c79c4f108` | P09/P26/P115 | BYTE_DIFFERENT_TEXT_EQUIVALENT: OmniCore agent architecture |
| 6 | `Architektura Systemu AI OmniCore Omega (1).pdf` | 10 | `0098444c0ba770f93fc5ed1f24487bb2423776386bde5bac7b531ac180ceb614` | P09/P26/P80 | THREAT_BOUNDARY: stochastic kernel, affect inference, GCG claim |
| 7 | `Architektura AI Zastępująca Statyczny Kod (1).pdf` | 11 | `8a1adf76b393b22888a1885bdce283224cc474ae9cd213c2e372a17f1cab850b` | P09/P80 | RESEARCH: learned scheduler/driver/PUI not proven replacement for deterministic safety |
| 8 | `Asus UX581 ZenBook Pro Duo Laptop.pdf` | 92 | `ab36cf06e6d82e1cefeae2c7937d01a5cf4c724ec1c9eb78e881a860de92fe4b` | P26/P09 | HARDWARE_MANUAL_2020: generic ASUS notebook manual; exact UX581 configuration not established |
| 9 | `Automatyzacja Tworzenia Oprogramowania z AI.pdf` | 11 | `935112485ef91fb70dd81a93b0d9752f0f70edddd8e12dae068a56787a0841c1` | P09/P26/P115 | EXACT_NORMALIZED_TEXT_DUPLICATE of edition 260105; distinct file bytes |
| 10 | `Atak GCG na Modele Językowe.pdf` | 9 | `66c8ea3f595613e8c169d65bc37e394ac348836010697938d8dd234dbff32bee` | P60/P108 | TOPIC_REVISIT: GCG defensive threat/evaluation |

**Razem: 10 PDF / 170 stron.** „Automatyzacja Tworzenia Oprogramowania z AI” ma dwa różne pliki o różnych SHA-256, lecz identycznym tekście po wydobyciu i normalizacji białych znaków (współczynnik porównania **1,0**); liczymy je jako jedno świadectwo treściowe, nie dwa niezależne dowody. Pozostałe tematy AURA, GCG, kluczowe kompetencje, learned kernel oraz SOP były już częściowo lub znacząco objęte wcześniejszym korpusem, co potwierdzają istniejące pliki `docs/knowledge-base/`.

## Kluczowe rozbieżności i braki źródeł
1. **AURA:** deklarowane 60 kart; opisano **dwie** z czterech domen w wartościach 1–10 (20 instancji) i trzy klasy anomalii: `mirror=4`, `venom=4`, `black_swan=2` (10 instancji). **30 instancji pozostaje niezdefiniowanych** w szczegółowej specyfikacji; nie są opisane dwie domeny. Nie wolno ich dopowiadać na podstawie sugestii generatora. Dodatkowo termochrom deklarowany jest jako 29°C w sekcji architektury, a 26–27°C w sekcji produkcji. Brak wykonanych prototypów/QC, cen i potwierdzonych ofert wytwórców. Opis „ukrytych komend”/gamblingowych pętli nie jest zweryfikowaną psychologią ani dopuszczalnym celem projektowym.
2. **ASUS:** 92-stronicowy polski podręcznik producenta, oznaczenie `PL16622`, pierwsze wydanie kwiecień 2020. Nazwa `UX581` jest w nazwie uploadu, lecz sam parsowalny tekst podręcznika ma treści ogólne i nie potwierdza konfiguracji konkretnego egzemplarza, numeru seryjnego, modułów pamięci, dysków czy pochodzenia urządzenia. Instrukcje odzyskiwania Windows są historią dokumentacji, **nie poleceniem resetu na urządzeniu wymagającym zabezpieczenia dowodów**.
3. **GCG:** źródła opisują ataki na alignment i transfer między modelami; raport OmniCore proponuje ofensywne obchodzenie ograniczeń. Korpus jest **wyłącznie materiałem do defensywnego model threat/evaluation**, nie mechanizmem łamania ochrony dostawców.
4. **Strategia:** badanie Siwak opiera się na pięciu pogłębionych wywiadach (2014) w określonych branżach; nie jest reprezentatywnym pomiarem przedsiębiorstw w 2026.
5. **Język:** trzystronicowy tekst leksykologiczny Dubisza (2019) odróżnia znaczenia słów `seks`, `seksualizm`, `seksualizacja`, `seksualizować` i analizuje retoryczne przesunięcia znaczeń. To przedmiot analizy języka, a nie rozstrzygnięcie sporów społecznych.
6. **Agentic SOP / OmniCore:** koncepcje autonomii i prognozy wyższości AI są **source claims**, wymagają implementacji i benchmarków; model sam nie nadaje uprawnień, a generowany raport nie jest dowodem wykonania narzędzi.

## Decyzja projektowa
- Zaktualizowano istniejących właścicieli: **P09, P26, P47, P50, P60, P66, P71, P75, P80, P90, P108, P115**.
- **Nowych projektów numerowanych: 0**, gdyż P50/P71 od dawna obejmują AURA, P60/P108 GCG, P09/P26/P80 learned kernel, P90/P115 orkiestrację, P66 strategię, P75 polszczyznę.
- Utworzono walidator `tools/aura_deck_gate.py`, który **nie wymyśla brakujących kart**, wymaga jawności efektów i pełnej struktury 60 kart, w tym deklarowanych kwot anomalii, oraz 11 testów `unittest` wykonanych lokalnie.

## Status wykonania
Dokumentacja i kontrakty: zapisano w repozytorium w tym PR. Walidator/testy: kod dodany oraz testy lokalne przeprowadzone. Druk próbny, zachowanie farby, kompatybilność NFC, działanie fizycznej gry, kernel QEMU, eksperymenty GCG, dostępność producentów, wydajność OmniCore, zgodność UX581 z konkretnym sprzętem: **nieprzeprowadzone**.
