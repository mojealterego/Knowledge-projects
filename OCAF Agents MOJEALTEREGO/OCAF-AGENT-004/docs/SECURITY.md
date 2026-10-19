# Zabezpieczenia i model zagrożeń — AXIOM FORGE

**Nazwa własna:** AXIOM FORGE · **Ewidencja:** 004 · **Numer seryjny:** `AM-ENG-AXF-2026-000004` · **Agent ID:** `AM-OCAF-AI-0004` · **Katalog:** `AM-OCAF-ENG-004` · **Nr certyfikatu:** `CERT-AM-2026-0004` (rezerwacja, niewydany).

## Powierzchnie ryzyka i obowiązujące blokady

| Ryzyko | Stan obecny | Bramka docelowa |
|---|---|---|
| Prompt injection i dane wejściowe modelu | Wejścia walidowane, nie są autoryzacją | źródłowy trust boundary + niezależna polityka |
| Podszycie się pod nazwę i numer seryjny | fail-closed TOML + global registry test | podpisana tożsamość OMAs/RPC |
| Routing między tenantami | kontrola segmentów, brak transmisji | mTLS + izolacja hostów/tenantów |
| Nadużycie OMAt | tylko lokalny read-only `graph.review` | niezależne podpisane capability grant |
| Nieuprawniony dostęp do źródeł | `source_access.enabled=false`, metoda DENY | pełna specyfikacja OSTc, vectors, review |
| Potwierdzenie wydania przez sam model | `Stage::Active` zawsze DENY | operator acceptance, signed evidence |
| Niezaufana specyfikacja i dane wejściowe | `verified=false`; Model lub graf zadania nie może sam nadać uprawnień do zapisu, deploymentu i promocji wydania. | niezależny readback i recenzja |
| Zależność git od OCAF core | pin `rev`, ale nie pełna proweniencja | SBOM, reprodukowalność, supply-chain |

## Zasada operacyjna

Agent działa wyłącznie jako lokalny analizator bez zapisów i bez praw uprzywilejowanych. Wyniki nie są dowodami stanu realnego systemu. **Nie dodawać pozornego szyfrowania** zamiast OSTc; `certification` pozostaje niepodpisany. W przypadku naruszenia granicy uprawnień poprawkę należy potwierdzić testem negatywnym.
