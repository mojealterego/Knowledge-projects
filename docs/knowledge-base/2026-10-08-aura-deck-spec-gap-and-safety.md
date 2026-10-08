# AURA: The Collective — luka specyfikacji 60 kart / ocena produkcyjna (2026-10-08)

**Źródło:** `AURA_ Specyfikacja Kart i Strategia Wdrożenia (202....pdf` (5 stron). **Istniejący właściciele:** P50 (mechanika gry i doświadczenie), P71 (fizyczna produkcja), P36/P40 (bezpieczeństwo poznawcze), P47 (rejestr). Jest to odrębna marka/wersja gry **wewnątrz** odpowiedzialności P50 i P71, a nie podstawa do duplikowania ich platformy. Historia źródła: `docs/knowledge-base/aura-physical-interface-safety-and-production.md`.

## Kontrola spójności
| Pozycja | Twierdzenie w PDF | Co można faktycznie ustalić |
|---|---|---|
| Wielkość talii | 60 kart | plan, brak kompletnego wykazu 60 jednoznacznych ID |
| Domeny | 4 domeny | szczegółowo podano tylko Instynkt i Pustka, wartości 1–10 |
| Domena Instynktu | 1–10 | 10 kart według wartości, nazwy grupowe 1–3, 4–6, 7–9, 10 |
| Domena Pustki | 1–10 | 10 kart według wartości, nazwy grupowe 1–3, 4–6, 7–9, 10 |
| Anomalie | Zwierciadło 4, Toksyczność 4, Czarny Łabędź 2 | 10 kart |
| Niezdefiniowane elementy | brak dalszego wykazu | **30 instancji brakujących do deklarowanych 60** |
| Temperatura termochromu | 29°C / 26–27°C | **sprzeczne wymagania**, wymagana decyzja i test tolerancji |
| NFC | ukryty chip w opakowaniu uruchamia web/audio | brak specyfikacji prywatności/URL/zgody i testów |
| Materiały | 330gsm Black Core, UV, termochrom, cold foil, zapach, soft touch | propozycje, bez fabrycznej walidacji, bezpieczeństwa kontaktowego i kosztów |
| Rezultaty marketingowe | wielokrotnie wyższa pamięć/„dopaminowa pętla” | niepotwierdzone źródłem jako uniwersalna wielkość, bez eksperymentu |

## Ulepszenie architektury
```text
AURA SOURCE + RIGHTS + PROVENANCE
  → 60-CARD DOMAIN/ANOMALY MANIFEST (NO GUESSED CARDS)
  → RULES + DISCLOSED EFFECTS / USER CONSENT
  → PREPRESS MATERIAL BILL / TEMPERATURE DECISION
  → PHYSICAL SAFETY / SCENT / NFC / ACCESSIBILITY REVIEW
  → SAMPLE PROTOTYPE + QA / AGE APPROPRIATENESS
  → PRICE / COST / SUPPLIER QUOTE VERIFICATION
  → SMALL PILOT / OPT-IN PLAYTEST → RELEASE
```

**Bezpieczeństwo:** w źródle są instrukcje „omijania krytycznego myślenia” i zachęty do hazardu. Traktujemy je jako **ryzyko projektowe**, a nie instrukcje do powielenia. Wynik interfejsu, gra, reklama oraz dźwięk NFC muszą być dobrowolne, jawne i odwracalne; nagrody losowe nie są projektowane do wywoływania uzależnienia. Brak nieoczekiwanego nagrywania i profilowania uczestników.

**Wykonana kontrola danych:** `tools/aura_deck_gate.py` wymaga 60 kart, czterech nazwanych domen, kwot 4/4/2 dla klas anomalii, unikalnych ID, jawności efektów i braku covert/gambling-pressure design, opt-in NFC. `test_aura_deck_gate.py` tworzy jawnie **syntetyczną** kompletną talię testową; `UNSPECIFIED_A/B` nie reprezentują prawdziwych nazw domen. Zaliczenie testów walidatora **nie oznacza**, że źródłowa AURA ma kompletne karty ani że istnieje produkt fizyczny.
