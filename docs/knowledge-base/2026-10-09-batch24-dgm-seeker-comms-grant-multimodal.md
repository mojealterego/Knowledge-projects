# Wiedza implementacyjna — 262 strony: DGM / Seeker / granty / komunikacja / lokalne wideo

## Seeker: obróbka dowodów, nie technologia ukrytego namierzania

**Źródła:** Seeker SPEC OPS (10 s.), 3 podobne iteracje kognitywne (17–19 s.) i Global Seeker (12 s.). P32 posiada już źródłowy schemat `scope → collection → normalization → evidence graph → contradiction/corroboration → report`; P30 ma zarządzanie dowodami. Dołożona granica: `EvidenceOrigin{simulated,observed,verified}` + autoryzowany zakres, własność materiałów, SHA256, data, stanowisko niezależnego weryfikatora, zakaz przypisywania fizycznej lokalizacji na podstawie prefiksu MAC/OUI/IMEI.

**Własny bezpieczny prototyp:** `tools/seeker_evidence_scope_gate.py`. Nie gromadzi danych o osobach, nie robi zapytań HLR, nie skanuje sieci, nie posiada transportu; kwalifikuje tylko odpersonalizowane dowody właściciela/autoryzowanego analityka do dalszego przeglądu. Format logu musi unikać numerów MSISDN/IMEI/MAC w jawnej postaci. Wyróżnić `observed_at` oraz `valid_at`; brak danych to `UNKNOWN`, nie `located`.

**Ocena jakości źródła:** pisanie w PDF „kompletna implementacja” nie czyni kodu uruchamialnym. Zawiera przykłady z niekompletnymi sygnaturami i pseudointerfejsami; przed adaptacją wymaga rekonstrukcji pakietów, pinów bibliotek, testów typów i wersji.

## DGM: autonomiczna ewolucja z ograniczoną sprawczością

**Źródło:** `Architektura Autonomicznego Systemu DGM.PDF` (41 s.). Wzorzec koncepcyjny:
```text
ModelScout (tylko źródła publiczne)
  + TechRecon (przepisy/reguły/prawa/licencja)
      ↓ evidence pack with SHA and rights
Strategist (porównanie do kanonicznych właścicieli Pxx)
      ↓ tested candidate plan
DGM_Core (tylko Knowledge-projects, gałąź robocza)
      ↓ test, test, test and compare regressions
Independent CI / approval / rollback / GitHub main readback
```
P59/P115 utrzymują jeden sterowany przepływ. Już istniejące `tools/dgm_cycle_gate.py` należy wykorzystać jako gate propozycji, zamiast powielać lub obchodzić go. 33+33 to lista życzeń i wymaga oddzielnych testów użyteczności/źródeł/unikalności, nie 66 automatycznie wdrożonych produktów. GitLab i Hugging Face w dokumencie to integracje kandydackie bez kont/praw do automatycznych commitów lub pobierania wag modeli.

## Finansowanie: dokument źródłowy kontra oficjalny operator

Alibaba Catalyst: **UP TO** 120k kredytów oraz 2 mld tokenów, nie bezwarunkowe dotacje gotówkowe ani automatyczne uprawnienia. Operator wymaga firm, profilu AI i dostępnej strony oraz wyklucza pewne wydatki. Plan wydatków = draft; nie wolno wypełniać fikcyjnego `Cloud ID`, wymyślać gotowego PoC lub deklarować „Alpha” bez dowodu. Bazy programowe P39/P34/P56. `tools/program_claim_gate.py` umożliwia offline ocenę maksymalnych obietnic, warunków i dat zakończonego hackathonu. **Nie składa formularza ani nie przyznaje kredytów**.

General Learning Hacks: fizyczny hackathon Hongkong 19–20.09.2026, deadline z regulaminu **09:00 HKT 20.09.2026** = **01:00 UTC**; dokument ostrzegał o alternatywnej 10:00. Konkurs był zamknięty 09.10.2026 (oficjalna strona Devpost). „Fresh code” dopuszcza frameworki i wcześniejszy projekt tylko z dokładnym wykazaniem rozdziału nowego kodu. Przy ocenianiu możliwości udziału nie podawać nieaktualnej instrukcji „wyślij teraz”.

## Komunikacja: transport to nie śledzenie

**Źródło:** `Komunikacja.pdf` (117 s.), bez importowania niezweryfikowanych specyfikacji jako standardów. Zachowuj rozdział:
- IRDA/CIR, Li-Fi/FSO — medium optyczne, wymogi widoczności/toru i warunków;
- BLE/NFC/UWB — lokalne protokoły radiowe, funkcje od wykrywania po zbliżeniowe połączenie i ranging, nie prawna zgoda na odnajdywanie obcych;
- Wi-Fi Direct / Wi-Fi / mesh — lokalna/punktowa łączność i routingi, nie dokładny adres fizyczny urządzenia;
- LoRaWAN / satelity — transfer na odległość z ograniczeniami pasma/energia/licencje;
- **Zenoh** — warstwa komunikacyjna danych/pubsub/query, nie fizyczny standard radiowy.

Stworzono `tools/communication_link_policy.py` z 8 przykładowymi identyfikatorami protokołów. Offline plan dopuszcza własne urządzenia i zgodne stanowiska testowe; żaden protokół sam nie stanowi dowodu GPS/IMEI ani podstawy do tajnego skanowania. Dokładne osiągi, zasięg, przepustowość i precyzja należą do empirii sprzętowej i lokalnej konfiguracji.

## Locally Uncensored: legalny adapter scen + render

**Źródło:** `Integracja API z Locally Uncensored.PDF` (18 s.). Przydatna jest separacja `Local Director Prompt → ShotPlan JSON → media provider adapter → async render job → source asset metadata → editorial QA`; ComfyUI i lokalny Wan to kandydaci. Z dokumentu **nie przenosimy** procedur przejmowania sesji, wydobywania ciasteczek ani wywoływania ukrytych endpointów Google Flow. Google Flow i oficjalne programowe Gemini API Veo mają odmienne modele uprawnień i cennik. Dla P97/P22 preferowana legalna integracja przez **dokumentowane Gemini API Veo** i kwalifikację kosztów, o ile użytkownik osobno udostępni odpowiednie poświadczenia. Kod i licencja AGPL-3.0 dla Locally Uncensored są twierdzeniami dokumentu wymagającymi niezależnej weryfikacji licencji konkretnej wersji, nie automatycznie zgodą na import kodu.

## Rezultaty i kryterium faktu

Rzeczywiste artefakty tej partii: SHA/podział źródeł, merytoryczne wpisy we właścicielach Pxx, trzy offline policy gates i testy. **Nie uruchomiono** NLP LangGraph/Neo4j/Qdrant, żadnego live API, Google Flow, Alibaba GPU, Android/GitLab, Bluetooth skanera, Find Hub, aplikacji mobilnej ani produkcji filmu. Każdy następny claim wymaga właściwych dowodów wykonania.
