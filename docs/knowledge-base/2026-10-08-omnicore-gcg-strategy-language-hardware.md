# Batch 14 — wielodziedzinowa synteza dowodowa (2026-10-08)

## OmniCore: trzy wizje i jedna para identycznych tekstów
`Automatyzacja Tworzenia Oprogramowania z AI.pdf` oraz wariant `_260105_120906.pdf` mają **inne SHA-256, lecz identyczny wyekstrahowany tekst** na 11 stronach. Źródła proponują sfederowane role Cursor/Windsurf/Devin, Rust `no_std`, SASOS, probabilistyczny scheduler, AI Foundry, PUI i QEMU. `Architektura Systemu AI OmniCore Omega (1).pdf` i `Architektura AI Zastępująca Statyczny Kod (1).pdf` to powiązane, ale odrębne opracowania (10 i 11 stron), z CrewAI/Vertex/Godot/3DGS i twierdzeniem o zastąpieniu deterministycznych mechanizmów probabilistycznymi.
- **Niezmiennik:** tylko deterministyczna, zweryfikowana warstwa nadzoruje pamięć, przerwania, uprawnienia i urządzenia; LLM/ML wolno proponować działania, ale nie może bez testu narzucać zmian w Ring 0.
- SASOS nie jest automatycznie izolacją bezpieczeństwa; brak granic adresowych wymaga dodatkowych formalnych warunków.
- Learned scheduling ma sens jako kontrolowany eksperyment z fallback do bezpiecznego planisty; mierzyć throughput, tail-latency, jitter, starvation, fairness, zasoby i awarie.
- Sugerowane narzędzia/funkcje/cenniki zależą od wersji; materiał PDF nie jest instalacją ani pomiarem.

## GCG: defensywne testy odporności
`Atak GCG na Modele Językowe.pdf` (9 stron) opisuje gradientową optymalizację dyskretnych sufiksów, transferowalność i potencjalne obejścia zachowania alignment. Raport OmniCore również postuluje GCG dla obchodzenia zabezpieczeń usług zewnętrznych; ten wariant **nie jest przyjętą funkcjonalnością produktu**. Istniejące `docs/knowledge-base/gcg-adversarial-attack-and-defense.md`, P60 i P108 już obejmują domenę.
- `AdversarialEvalRun`: wersja modelu, tokenizer, dataset opisowy/bezpieczny, threat class, budżet prób, zakreślona zgoda, wyniki odmów/niepożądanych działań, niezależny verifier, regressions.
- Traktować wzorce ataków jako kontrolowane zasoby testowe, bez rozpowszechniania operacyjnych payloadów lub automatyzacji jailbreaków zewnętrznych usług.
- Narzędzia są autoryzowane **poza** promptem; odmienna odpowiedź językowa modelu nie może automatycznie uruchomić komendy.

## Automatyzacja projektów: SOP → rzeczywiste wykonanie
`Automatyzacja Projektu z AI_ Instrukcja i Wykonani....pdf` (11 stron) rozdziela procesyzację, SOP jako opis procedury i delegowanie do agentów. Właściciele P90 i P115 odróżniają **deklarowane** kroki w opisie od faktycznych receiptów/artefaktów narzędzi. Przed przejściem do implementacji wymagane są typowane I/O, niezależne kryteria akceptacji, zgody, rollback, ograniczenie budżetu, potwierdzony commit i readback.

## Kompetencje strategiczne i ich pomiar
Kamila Siwak, `Rola kluczowych kompetencji organizacji w realizacji strategii biznesu`, e-mentor nr 5(62), 2015 (7 PDF stron), na podstawie literatury i pięciu IDI z właścicielami przedsiębiorstw w 2014 bada rolę niematerialnych zdolności i trudność wyceny. W P66 obiekt `CapabilityEvidence` rozdziela faktyczną zdolność od deklaracji, przewagi strategicznej i hipotetycznej wartości finansowej; unikalność i rzadkość **muszą być wykazane** na porównywalnym rynku.

## Polski dyskurs i wieloznaczność
Artykuł `Seks – seksualizm – seksualizacja – seksualizować`, „Poradnik Językowy”, 2019, 3 strony, DOI 10.33896/PorJ.2019.7.12, analizuje leksykalne znaczenia, przemieszczenie rejestrów i możliwość polityczno-retorycznej zmiany znaczenia słów w debacie publicznej. Używać jako **przykładu leksykograficznego** dla P75, z datą i autorstwem, nie jako automatycznego etykietowania intencji poszczególnych mówców czy jednoznacznego rozstrzygnięcia sporu.

## Manual ASUS PL16622 (kwiecień 2020) i dowody sprzętowe
`Asus UX581 ZenBook Pro Duo Laptop.pdf`: 92-stronicowa polska instrukcja notebooka z zasadami bezpieczeństwa, POST/BIOS, funkcjami odzyskiwania, FAQ i prawami autorskimi. **Model UX581 jest w nazwie pliku uploadu**, nie jest jednoznacznie potwierdzony poprzez ekstrakcję treści całej broszury; nie wolno inferować właściwego BIOS/komponentów konkretnego laptopa, własności ani dostępności urządzenia.
- `HardwareManualEvidence`: producent, `document_ref`, data, model w treści, **model w nazwie uploadu**, `claimed_compatibility`, `device_serial_verified=false`, `firmware_verified=false`.
- Przy bezpieczeństwie/incydentach nie zalecać automatycznego Windows Reset/recovery/BIOS flash, które mogłyby usuwać dowody, bez zatwierdzonej procedury zabezpieczenia danych. Modele sprzętowe i aktualne instrukcje weryfikować u producenta.
