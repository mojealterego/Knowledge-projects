# 32 — Deep OSINT Agent & Zero-Trust Evidence Engine MAX

## Cel projektu

Zbudować defensywnego agenta OSINT, który przechodzi od prostego skryptu enumeracyjnego do **rekurencyjnego, dowodowego systemu analitycznego** z wyraźną granicą między zbieraniem danych, wnioskowaniem i działaniem.

Materiały Deep OSINT opisują przejście od pasywnego zbierania do rekurencyjnego procesu, w którym każdy punkt danych staje się potencjalnym `pivot point`; podkreślają również konieczność zdefiniowania celu, zakresu oraz ram prawnych i etycznych przed zbieraniem danych.

## Canonical investigation loop

```text
TARGET / QUESTION
      ↓
SCOPE + AUTHORIZATION + LEGAL BOUNDARY
      ↓
PASSIVE COLLECTION
      ↓
NORMALIZATION
      ↓
EVIDENCE GRAPH
      ↓
PIVOT SELECTION
      ↓
NEW COLLECTION
      ↓
CORRELATION / ANALYSIS
      ↓
CONFIDENCE + COUNTEREVIDENCE
      ↓
REPORT / ALERT
```

## Source-derived baseline: Deep OSINT Framework

Dostarczony framework implementuje klasę `DeepOSINT` z modułami generowania zapytań wyszukiwawczych, crawl/extraction, analizy EXIF, skanowania portów oraz raportowania HTML. Źródło pokazuje ekstrakcję e-maili, telefonów i linków społecznościowych z HTML, analizę `model`, `make`, `datetime_original`, GPS i software w EXIF oraz sprawdzanie typowych portów 21/22/25/53/80/443/3306/8080. `DeepOSINT` jest traktowany jako adapter kolektorów, nie jako system dowodowy.

Framework zawiera również generowanie wyszukiwań dotyczących publicznych dokumentów, paneli logowania, otwartych katalogów, plików konfiguracyjnych, wyników społecznościowych i treści z serwisów paste. W systemie produkcyjnym takie zapytania są klasyfikowane jako **high-risk discovery patterns** i mogą być wykonywane wyłącznie w dozwolonym zakresie oraz z zachowaniem polityki prywatności i autoryzacji; nie stanowią domyślnej ścieżki eksfiltracji ani pozyskiwania danych uwierzytelniających.

## Influence / relationship graph — new corpus extension

Dostarczony materiał Deepsearch rozszerza OSINT o szczegółową **mapę powiązań / sieć wpływów**. Model projektu przyjmuje cztery główne klasy relacji:

1. **Osoby** — mentorzy, protegowani, sojusznicy, rywale i inni kluczowi współpracownicy.
2. **Struktury biznesowe** — bezpośrednie i pośrednie powiązania kapitałowe oraz wspólne rady/zarządy.
3. **Sfera polityczna i publiczna** — funkcje publiczne, role partyjne oraz publicznie udokumentowane wsparcie inicjatyw politycznych.
4. **Powiązania rodzinne** — relacje rodzinne tylko wtedy, gdy są odpowiednio udokumentowane, wraz z rolami krewnych w organizacjach biznesowych/publicznych.

Każda krawędź musi być typowana i posiadać własną proweniencję:

```yaml
RelationshipEdge:
  subject:
  predicate:
  object:
  valid_from:
  valid_to:
  source:
  source_type:
  evidence:
  confidence:
  counterevidence:
  status: observed|reported|inferred|verified|disputed
```

Samo współwystępowanie nazwisk, wspólna lokalizacja lub pojedynczy wynik wyszukiwarki nie wystarcza do ustanowienia relacji.

## Investigative chronology

Nowy korpus raportowy wnosi dodatkowy wzorzec: rozbudowaną chronologię zdarzeń, w której występują bezpośrednie obserwacje, relacje osób trzecich, dokumenty, interpretacje i zarzuty. Project 32 normalizuje takie materiały do atomowych rekordów:

```yaml
Event:
  event_id:
  date_or_interval:
  actor:
  action:
  location:
  claim_type: direct_observation|third_party_report|documented_record|allegation
  source:
  corroboration:
  contradiction:
  confidence:
```

System nie może usuwać różnicy pomiędzy `allegation`, `third_party_report` i `direct_observation` podczas generowania podsumowania.

## Negative-finding protocol

Raporty dochodzeniowe mogą zawierać ważne wyniki negatywne: wielokrotne wyszukiwania, kontrolę list instytucjonalnych oraz analizę rejestrów mogą nie wykazać relacji.

Project 32 koduje taki wynik jako:

```text
SEARCH SCOPE
 + SOURCE SET
 + TIME WINDOW
 + QUERY SPACE
 + NEGATIVE RESULT
 = DOCUMENTED NEGATIVE FINDING
```

Nigdy:

`NO_RESULT → PROOF_OF_NONEXISTENCE`.

Brak śladu jest informacją o obserwowalności w określonym zbiorze źródeł, a nie absolutnym dowodem braku relacji.

## Source hierarchy and entity resolution

Dla relacji korporacyjnych, publicznych i instytucjonalnych system preferuje źródła pierwotne oraz oficjalne rekordy, następnie dokumenty organizacyjne, rejestry publiczne, wiarygodne relacje współczesne i dopiero później źródła wtórne lub społecznościowe.

Tożsamość osoby jest rozwiązywana na podstawie wielu atrybutów: nazwy, roli, organizacji, geografii, okresu oraz innych niezależnych danych. Dwóch kandydatów o tym samym imieniu i nazwisku nie wolno automatycznie scalać.

## Evidence object

Każde znalezisko jest normalizowane do:

```yaml
Evidence:
  id:
  observed_at:
  source:
  source_type:
  target:
  claim:
  raw_reference:
  extracted_data:
  provenance:
  freshness:
  reliability:
  confidence:
  collection_mode: passive|active
  authorization_scope:
  counterevidence:
  transformations:
```

`Evidence != Claim != Hypothesis != Recommendation`.

## Passive-first policy

Domyślnie agent zaczyna od pasywnego rozpoznania. Aktywne testy są dozwolone wyłącznie po spełnieniu:

```text
explicit_scope
+ target_ownership_or_authorization
+ operation_allowlist
+ rate_limit
+ audit_logging
```

Brak autoryzacji oznacza przejście do trybu pasywnego albo odmowę.

## Tool fabric

Klasyczne moduły pozostają capability-scoped tools:

```text
capability_id
subject_id
target_scope
operation
expiry
rate_limit
provenance
approval_state
```

Narzędzie nie dziedziczy automatycznie uprawnień od agenta.

## Collector contract

```yaml
Collector:
  id:
  version:
  input_schema:
  output_schema:
  collection_mode: passive|active
  required_capabilities: []
  target_constraints: []
  rate_limit:
  timeout:
  provenance_fields: []
  evidence_quality:
  failure_modes: []
```

Minimalny wynik kolektora musi zawierać `observed_at`, `source`, `target`, `raw_reference` i identyfikator kolektora. Kolektor nie może samodzielnie zmienić `target_scope`.

## Weaknesses of the baseline script

- scraping bez snapshotu/proweniencji nie jest dowodem wysokiej jakości;
- regexy generują false positives/negatives;
- pojedynczy rekord nie potwierdza tożsamości podmiotu;
- EXIF jest obserwacją metadanych, nie automatycznym dowodem miejsca wykonania zdjęcia;
- active socket scan wymaga autoryzacji i kontroli zakresu;
- raport HTML nie ma kryptograficznego łańcucha pochodzenia ani modelu counterevidence;
- brak wersjonowanego stanu dochodzenia utrudnia wznowienie i audyt.

## Deep OSINT recursion governor

Agent wybiera kolejny pivot na podstawie:

`information_gain × relevance × confidence_gap × freshness / cost`

Zatrzymanie następuje, gdy pytanie zostało odpowiednio pokryte, marginal information gain spadł poniżej progu, pozostałe pivots wymagają nieautoryzowanego działania, przekroczono budżet albo pojawiła się sprzeczność wymagająca człowieka.

## Evidence quality gate

```text
RAW OBSERVATION
      ↓
PARSING / NORMALIZATION
      ↓
SOURCE + TIME + TARGET VALIDATION
      ↓
DUPLICATE / CONFLICT CHECK
      ↓
COUNTEREVIDENCE SEARCH
      ↓
RELIABILITY + FRESHNESS SCORE
      ↓
CLAIM ELIGIBILITY
```

Reguła nadrzędna: brak dowodu nie jest dowodem braku, a brak spójności nie jest automatycznie dowodem oszustwa.

## MCP + Agent Skills architecture

Remote MCP jest osobną domeną zaufania. Model może otrzymywać narzędzia zewnętrzne, ale narzędzie pozostaje capability-scoped i musi przejść niezależną autoryzację oraz walidację wyniku.

```text
MODEL
 ↓
TOOL DISCOVERY
 ↓
ALLOWED-CAPABILITY FILTER
 ↓
AUTHORIZATION
 ↓
MCP CALL
 ↓
OUTPUT VALIDATION
 ↓
PROVENANCE NORMALIZATION
 ↓
EVIDENCE GRAPH
```

## UI / evidence workstation

Proponowany interfejs:

```text
LEFT: target / scope / permissions
CENTER: evidence + influence graph
RIGHT: claim / relationship inspector
BOTTOM: provenance / chronology / raw source
```

Stan biznesowy pozostaje na serwerze; stan UI jest efemeryczny.

## Adversarial evaluation

Test suite obejmuje:

- fałszywe korelacje;
- spoofed profiles;
- poisoned public data;
- prompt injection w stronach i dokumentach;
- sprzeczne źródła;
- stale data;
- unicode/homoglyph obfuscation;
- wymuszenie aktywnego skanowania poza zakresem;
- fałszywe lub zmanipulowane EXIF;
- źródła zmieniające treść po pobraniu;
- kolektory próbujące rozszerzyć `target_scope`;
- kolektory zwracające dane bez wymaganych provenance fields;
- błędne scalanie osób na podstawie samego nazwiska;
- fałszywe krawędzie wpływu wynikające wyłącznie ze współwystępowania.

Celem jest sprawdzenie, czy agent potrafi powiedzieć **„brak wystarczających dowodów”** zamiast generować pozorną pewność.

## Resumability

Każda faza dochodzenia zapisuje checkpoint:

```yaml
InvestigationCheckpoint:
  investigation_id:
  state:
  scope_version:
  evidence_graph_version:
  completed_collectors: []
  pending_pivots: []
  blocked_actions: []
  unresolved_conflicts: []
  budget:
  last_updated:
```

Wznowienie nie może automatycznie odziedziczyć wygasłych uprawnień. Capability grants są ponownie walidowane przy wykonaniu.

## Integracja

Project 32 integruje Projects 06, 12, 15, 19, 25, 26, 27, 29 i 30 oraz nową wiedzę z Deep OSINT, Deepsearch i metodologii analizy dochodzeniowej.

## Definition of Done

- typed investigation graph;
- typed influence/relationship edges;
- provenance dla każdego twierdzenia i relacji;
- passive-first execution;
- authorization-aware active tools;
- evidence/reasoning separation;
- collector contracts;
- evidence quality gates;
- negative-finding protocol;
- entity-resolution states;
- chronology/event model;
- resumable investigations;
- adversarial evaluation;
- audytowalny raport końcowy.

---

## Integracja wiedzy — 2026-10-08: propozycja migracji tożsamości OSINT „Andre”

**Status:** SPECIFIED / REBRANDING NOT EXECUTED; źródło: `Zmiana Nazwy Projektu na Andre_ Manualnie i AI.pdf`.
**Specyfikacja:** [P32 Andre migration extension](32-deep-osint-agent-and-zero-trust-evidence-engine-max-andre-migration-extension-2026-10-08.md); [dokument wymagań migracyjnych](../docs/knowledge-base/2026-10-08-andre-migration-assurance.md).

### Granica projektu
„Andre” jest docelową **propozycją** wspólnej tożsamości konkretnej rodziny narzędzi Nexus-Eye/Omega Infinity/Ghost Protocol/Vantage Point/Aether. Nazwa nie jest automatycznie zmieniana w tym repozytorium, ODYN AI, innych systemach OMEGA ani zewnętrznych projektach.

### Obowiązkowa procedura rozbudowy
1. `IdentityInventory`: zinwentaryzuj rzeczywiste klasy/importy Python, moduły React, CLI/API, prompty, artefakty raportów i identyfikatory utrwalone.
2. `MigrationManifest`: dla każdego symbolu podaj `old_identifier`, `new_identifier`, `owner_project`, zależności, test zgodności, alias lub migrację, plan odwrócenia.
3. `CompatibilityLayer`: zachowuj aliasy/importy i wersjonowanie raportów; nie zmieniaj kluczy baz ani endpointów bez procedury migracyjnej.
4. `AgentIdentityPolicy`: odrębnie wersjonuj nazwę/personę i uprawnienia narzędzi, bez obchodzenia zasad dostępu albo nieprawdziwej identyfikacji klienta sieciowego.
5. `RegressionHarness`: uruchom testy importów, generowania raportów, UI, serializacji, historii śledczej, granic autoryzacji i odczytu artefaktów sprzed migracji.

### Warunki wejścia do implementacji
Zidentyfikowany kod źródłowy, zgoda właściciela na zakres zmiany nazwy, raport zależności, plan kompatybilności oraz zestaw testów. Same fragmenty kodu wymienione w PDF nie dowodzą istnienia plików w tym repozytorium. Dotychczasowy zakres defensywnego OSINT i ochrony dowodów nie ulega rozszerzeniu na nieautoryzowane zbieranie danych.

---

## Integracja wiedzy — batch 11 (2026-10-08): VANTAGE POINT provenance and UI security

Źródło: `VANTAGE POINT(1).pdf` (16 stron kodu React/Firebase w postaci obrazów; **nie uruchomiono programu**). Wcześniejsza dokumentacja o VANTAGE istnieje w korpusie Knowledge Base.

**Nowe wymaganie `OSINTObservation`:**
```yaml
OSINTObservation:
  target_type: DOMAIN|IP|USER
  authorized_scope_ref: null
  tenant_ref: null
  data_origin: simulated|observed|verified
  source_refs: []
  observed_at: null
  verification_state: pending
  display_label: null
```

Na obrazie strony 10 kod zawiera `simulateScan`: wyniki wytwarzane przez opóźnienie i fikcyjne dane **nigdy nie mogą udawać prawdziwych danych OSINT**. Również losowo rysowane krawędzie Canvas nie są zweryfikowaną siatką powiązań. Wdrożyć etykietę „SIMULATED”, provenance także w eksporcie, reguły Firestore izolujące tenantów, test anonimowego uwierzytelnienia, cleanup `onSnapshot`/resize/`requestAnimationFrame`, etykiety dostępności oraz rozróżnienie UI state od backend truth.

**Testy:** zabroniony odczyt innego tenant, brak fałszywego awansowania mock→verified, brak fałszywego przypisania relacji, UI unmount bez wiszących subskrypcji, wymagana zgoda na aktywne kolektory. Brak automatycznego skanowania rzeczywistych systemów.

---

## 2026-10-08 — batch 13: 4-page OSINT script hardening and evidence claims

Source: `Agent OSINT.pdf`. The uploaded Python text contains zero-width formatting characters, direct `http://ip-api.com` calls, imitated browser UA, DNS resolution, redirect-following HTTP HEAD and HTTP 200/404 username heuristics. These patterns are **source risks**, not accepted production design or proof of identity.

### Upgraded tool/evidence controls
1. `ScopeTicket`: explicit owner/authorized target types, bounds, expiry, privacy retention, tool allowlist and request rate caps. Active queries require pre-authorized scope outside model text.
2. `TransportPolicy`: approved provider HTTPS, redirects restricted to permitted hosts, private/link-local/metadata IP destinations denied and DNS re-resolution checked before transport (SSRF controls). No UA spoofing for evasion.
3. `ClaimClassifier`: `simulated | observed | verified` with mandatory source hash, timestamp and independent verifier reference; HTTP status alone yields `inconclusive_http_response`, never "username belongs to person".
4. `GeolocationBoundary`: API IP-to-region data must never be asserted as a physical location or identity of a person; backend provider/precision and timestamps recorded.
5. `ArchiveNormalizer`: remove invisible PDF formatting artifacts before code review; never execute PDF-extracted source code blindly.
6. `DemoSeparation`: AETHER and VANTAGE visual demos remain unequivocally simulated; offline test data cannot be silently promoted to verified results.

**Executable evidence in this batch:** `tools/osint_provenance_gate.py` (pure data, zero network calls) and `tools/test_osint_provenance_gate.py` (8 locally passing tests). These tests check provenance invariants, **not scanner correctness, privacy compliance or real OSINT findings**.

## 2026-10-09 — Batch 24: uploaded Seeker reports

Five Seeker-focused PDFs (one SPEC OPS Python draft, three closely overlapping cognitive-architecture versions and one global hardware-awareness report) were fully received, hashed and analyzed. The 17-, 18- and 19-page versions share substantial text; they are not independent witness evidence. Retain a defensive, consent-checked evidence workflow: independently authorized case scope, original source SHA-256, owner-supplied documents, provenance, uncertainty, contradiction checks, audit trail and human review. Hardware identifiers and network metadata must never by themselves establish the identity or live physical coordinates of any person. Do not automatically query private telecommunications or social accounts, probe third parties, or deploy background location tracking. The new offline tools/seeker_evidence_scope_gate.py rejects intrusive operations and unverifiable claims; it does not query a live service or establish actual case legitimacy. Technical assertions and example code in Gemini-produced PDFs are proposals, not verified tests. The source inventory with exact hashes and textual overlap is docs/UPLOADED-PDF-INGESTION-2026-10-09-BATCH-24.md.
