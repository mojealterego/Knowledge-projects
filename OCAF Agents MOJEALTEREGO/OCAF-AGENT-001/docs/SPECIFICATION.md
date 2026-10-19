# Specyfikacja funkcjonalna i techniczna — WDA AURELION

## 1. Metryka
Nazwa własna: **WDA AURELION**. Rodzina: WDA Photo Agent. Numer ewidencyjny: **001**. Serial: `AM-WDA-AUR-2026-000001-DE338AC20B42`. Agent ID: `AM-OCAF-AI-0001`. Numer katalogowy: `AM-OCAF-WDA-001`. Wersja: `0.2.1-dev`; stan: `INCOMPLETE_DEVELOPMENT`. Projekt P116, repo `mojealterego/Knowledge-projects`.

## 2. Cel
Docelowo agent wspierający kontrolowaną obróbkę fotografii z zachowaniem tożsamości osób, struktury twarzy i integralności materiału źródłowego. **Aktualny zrealizowany zakres** to wyłącznie lokalny planer słowny `photo.plan`.

## 3. Interfejs aktualny
- Input JSON dla OCAF: `{"goal":"adjust highlights","identity_lock":true}`. `goal`: string, po `trim` niepusty, maksymalnie 1024 bajty UTF-8, bez znaków sterujących. Nieprzewidziane pola niedozwolone.
- Output JSON: `{"agent_id":"AM-OCAF-AI-0001","status":"plan_only","identity_lock":true,"workflow":[...]}`. Pole `workflow` zawiera pięć planowanych kroków, **nie zmodyfikowany obraz**.
- CLI: `cargo run --manifest-path ".../Cargo.toml" -- "adjust highlights"`.
- Side effects: brak zapisu, brak przetwarzania zdjęć, brak sieci, brak wywołań płatnych i brak źródłowych uprawnień. `cost_sats=0` w manifeście narzędzia jest deklaracją lokalną, nie cennikiem komercyjnym.

## 4. Komponenty i funkcje
| Element | Stan | Wynik |
|---|---|---|
| Rust planner `photo.plan` | IMPLEMENTED | tekstowa sekwencja kroków |
| Identity lock i size validation | IMPLEMENTED | odrzucenie niezgodnych danych |
| Nazwa unikatowa + seryjny + manifest | IMPLEMENTED | spójna ewidencja i konfiguracja |
| Lokalne ToolExecutor/ToolManifest | IMPLEMENTED/TESTED | rejestracja `photo.plan` w OCAF testowym |
| Logiczne trasy OMAs vs OMAt | IMPLEMENTED LOCAL | kontrakt przestrzeni nazw |
| Produkcyjny serwer OMAs | NOT IMPLEMENTED | brak |
| Produkcyjny serwer OMAt | NOT IMPLEMENTED | brak |
| Transport Zenoh aktywny i mTLS | NOT IMPLEMENTED | brak |
| OSTc / szyfrowany dostęp do źródeł | BLOCKED | specyfikacja niedostarczona |
| Dekodowanie obrazów i edycja w Rust | NOT IMPLEMENTED | brak |
| Adobe Firefly/OpenAI API i OAuth | NOT IMPLEMENTED | brak |
| Aplikacja na telefon z Rust backend | NOT IMPLEMENTED | legacy PWA niegotowe produkcyjnie |
| Podpisanie, wydanie certyfikatu, akceptacja operatora | NOT RUN | brak |

## 5. Platformy i zależności
Rust 2021; pakiet `wda_photo_agent` 0.2.1-dev; opcjonalna zależność do wersji OCAF z ustalonym `rev` w Cargo.toml; `serde`, `serde_json`, `toml`, `async-trait`, `tokio`; sprawdzany przez GitHub Actions na `ubuntu-latest`. Android, iOS, Windows, macOS, WASI i Firecracker **nie zostały przetestowane** dla nowego wydania.

## 6. Parametry, jakość, gwarancje
Czas `timeout_ms=5000` w deklaracji OCAF nie stanowi dowodu SLA. Brak pomiaru p95/p99, niezależnego audytu, środowiska wielohostowego, RTO/RPO i kontroli jakości fotograficznej. Nie formułować gwarancji prawnych, ubezpieczeniowych ani finansowych.

## 7. Oferta i wycena
Produkt w stanie developerskim: **CENA NIEUSTALONA / BRAK ZATWIERDZONEJ OFERTY**. Modele sprzedaży, dzierżawy i najmu mają odrębne, niewiążące szkice w `docs/commercial/`. Minimalna dzierżawa 3 miesiące, minimalny najem 7 dni. Wycena wymaga określenia licencji, zakresu usług, VAT, kosztów dostawców, SLA i odpowiedzialności.

## 8. Definition of Done
Zobacz [RELEASE_CHECKLIST](RELEASE_CHECKLIST.md) i [TRACEABILITY_MATRIX](TRACEABILITY_MATRIX.md). Samo wydanie numeru seryjnego lub pozytywne testy jednostkowe **nie oznaczają gotowości agenta**.
