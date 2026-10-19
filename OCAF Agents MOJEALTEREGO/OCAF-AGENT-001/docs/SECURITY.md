# Model zagrożeń i polityka bezpieczeństwa — WDA AURELION

**Instancja:** 001 / `AM-WDA-AUR-2026-000001-DE338AC20B42`. Materiał: code review aktualnego Rust i zachowany historyczny Node/PWA; **brak formalnego niezależnego pentestu**.

| Powierzchnia | Ryzyko | Zabezpieczenie istniejące | Dalsze warunki |
|---|---|---|---|
| Wejście `goal` | długi tekst, znaki sterujące, próba obejścia identity lock | limit 1024 bajty UTF-8, deny false, JSON schema, testy | budżety CPU i rate limit w hoście |
| Rejestr agentów | podszycie się pod numer 001, serial lub nazwę | spójna konfiguracja, unikalność w JSON rejestrze, testy | podpisane poświadczenie OMAs, wiązanie RPC |
| OMAs | podszycie się pod agenta i przekierowanie wiadomości | tylko format tras; network disabled | tożsamość podpisana, mTLS, nonce, fencing, niezależny audyt |
| OMAt | nieautoryzowane uruchomienie narzędzia | jedynie lokalny manifest `photo.plan`; bez płatnych efektów | podpisane uprawnienia, scopes, idempotency keys |
| Zenoh | przechwycenie/transit lub routing injection | walidacja segmentów, bez sesji sieciowej | uwierzytelniony TLS i izolacja tenantów |
| OSTc | nieuprawniony odczyt lub odszyfrowanie źródeł | `source_access.enabled=false`; zawsze deny | pełna specyfikacja OSTc, test vectors, implementacja, audyt i klucze |
| Legacy Node | brak authz płatnych Adobe API | niezainstalowany w produkcji, izolacja do audytu | pełen przegląd, limity body, authn/authz, bezpieczne ścieżki |
| Dane obrazowe | prywatność i prompt injection | Rust obecnie nie czyta zdjęć | klasyfikacja, redakcja, audyt dostawców i retencja |
| Łańcuch dostaw | git dependency, słabe śledzenie artefaktów | pin `rev`; testy CI | SBOM, SLSA/provenance, digest, review polityk OCAF |
| Lifecycle | fałszywe `ReleaseEvidence` | `ACTIVE` odrzucone niezależnie od pól | dowód z niezależnego wystawcy, weryfikacja podpisu |
| Dokumenty handlowe | omyłkowa dystrybucja niewydanego agenta | `certificate_issued=false`, `production_approved=false` | operator acceptance, przegląd prawny |

## Decyzje projektowe
- Numer seryjny nie jest hasłem, tokenem, kluczem ani podpisem — sam nie uwierzytelnia wywołań. Identyfikatory nie zastępują certyfikacji.
- Pełny `OSTc` jest **celowo nierozwijany** bez źródłowej specyfikacji. Nie wdrażamy prowizorycznego szyfrowania.
- Nie uruchamiać publicznie `legacy-pwa` ani nie podawać jej kluczy API. W przyszłym wdrożeniu niezbędne są operator approval, limity płatnych efektów, maskowanie sekretów, audit trace.
- Raporty CI dokumentują tylko konkretny zakres testów. Produkcyjny audyt niezależny, tenant isolation i bezpieczeństwo sieci **nie były weryfikowane**.
