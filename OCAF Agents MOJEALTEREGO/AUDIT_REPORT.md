# Raport audytu wstępnego — OCAF Knowledge Architect

**Stan:** INITIAL INVENTORY / PARTIAL; **data:** 2026-10-19; **evidence:** drzewa Git na commitach zapisanych w REPOSITORY_INVENTORY.md, cztery pliki referencyjne i jedna dyrektywa.

## Ustalenia potwierdzone

- OBSERVED: pełne listy ścieżek `mojealterego/Knowledge-projects` i `mojealterego/ocaf-runtime` zostały pobrane (`3218a158cf030b8db954055cec864d0a428e3b03`, `52d4f597e0b481678f90cf2073b721ad2e99e253`).
- OBSERVED: `Knowledge-projects/AGENTS.md` i `docs/KNOWLEDGE-EVOLUTION-PROTOCOL.md` definiują ograniczenie zapisu wyłącznie do `Knowledge-projects`.
- OBSERVED: w `ocaf-runtime` istnieją manifest Rust, moduły runtime, testy i CI oraz kanoniczna Roadmapa; to nie potwierdza przejścia poszczególnych bramek produkcyjnych.
- OBSERVED: w `Knowledge-projects` znajdują się historyczne portfolio/rejestry, narzędzia Python i specyfikacje rozmaitych systemów; heurystyka nazw **nie** rozdziela automatycznie aplikacji, agentów i rojów.

## Otwarte luki i ryzyko

| ID | Luka / ryzyko | Waga | Dowód wymagany | Status |
|---|---|---|---|---|
| G01 | Mieszanie projektu z uruchamialnym agentem | HIGH | analiza entrypointów i zależności per kandydat | UNVERIFIED |
| G02 | Przeniesienia mogą zerwać importy, testy, referencje | HIGH | dependency graph + `git mv` + testy | NOT RUN |
| G03 | Brak referencyjnych wyników OMAs signed RPC, jobs, fenced effects | CRITICAL for release | CI run ID, logi, negatywne testy na rzeczywistej infrastrukturze | NOT RUN |
| G04 | Korpus narzędzi OMAt nie został dostarczony w tej partii | HIGH | oryginalne pliki 333+ pozycji i SHA | BLOCKED |
| G05 | OSTc bez dostarczonej specyfikacji kryptograficznej i niezależnego audytu | CRITICAL for deployment claims | model zagrożeń, test vectors, przegląd niezależny | UNVERIFIED |
| G06 | Brak dowodów formalnej certyfikacji agentów | CRITICAL for legal/commercial release | wydanie, odbiór, polityki licencji i opinia prawna | NOT RUN |

## Macierz testów T01–T15

Źródło SRC-004: T01 Manifest, T02 Rust, T03 signed OMAs, T04 Tenant isolation, T05 Jobs authz, T06 Fenced effects, T07 SIGKILL/restart, T08 Multihost, T09 Tools sandbox, T10 Secrets, T11 OSTc vectors, T12 Supply chain, T13 Observability, T14 Lifecycle, T15 Operator acceptance.

**Dla wszystkich T01–T15 status tej partii = NOT RUN.** Nie mylić wcześniejszych deklaracji oraz historycznych raportów repozytorium z nowymi wynikami testów. Implementacja i produkcja nie zostały poświadczone w tej partii.

## Materiały komercyjne

SRC-002/SRC-003 to **brief koncepcyjny** obejmujący sześć rodzajów dokumentów; przykładowy cennik, numery, certyfikaty, SLA i ubezpieczenie nie są zatwierdzone. Dalsza praca wymaga rzeczywistej walidacji technicznej i prawnej; nie publikować danych kontaktowych ze źródła ani fikcyjnych podpisów.
