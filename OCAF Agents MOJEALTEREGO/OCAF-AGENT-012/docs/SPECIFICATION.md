# Specyfikacja techniczna — FIDES GRANTIS

**FIDES GRANTIS** · ewidencja **012** · serial `AM-GRT-FID-2026-000012` · Agent ID `AM-OCAF-AI-0012` · katalog `AM-OCAF-GRT-012` · certyfikat `CERT-AM-2026-0012` **NIEWYDANY**.

## Funkcja domenowa

Sprawdza wyłącznie repo Knowledge-projects, działania research/draft, brak credentiali, zakres deklarowanych kredytów i tokenów programu Alibaba AI Catalyst, historyczny status General Learning Hacks i wymaganie zweryfikowanej kwalifikowalności przy tworzeniu szkicu.

## Interfejs

Narzędzie `program.claim_review`. Wejście `ProgramClaim`: `repository`, `action`, `credential_in_payload`, `program`, `credits_usd`, `tokens`, `status`, `company_eligibility_checked`. Timestamp UTC do review dostarczany jest **przez hosta** i nie istnieje jako pole żądania modelu. Wynik `ProgramVerdict`: `permitted`, `reasons[]`, `funding_awarded_verified=false`, `external_submission_executed=false`, `execution_authorized=false`.

## Kryteria odmowy i dowody

Wynik permitted=true jest lokalną decyzją walidatora; nie potwierdza otrzymania pieniędzy ani grantów, nie składa wniosku, nie uruchamia obliczeń i nie sprawdza na żywo stanu programów. Czas UTC pochodzi od hosta, nie z modelowego JSON. Ustalony świadomie schemat JSON z opcjonalnymi liczbami całkowitymi, czas z niezależnego hosta; odrzuca nieznane pola, a bool nie jest liczbą całkowitą. Informacje o programach są ograniczone do zapisanych reguł źródłowego Python — nie stanowią aktualnego potwierdzenia ich dostępności.

## Niezaimplementowane

Weryfikacja bieżących regulaminów i kwalifikowalności; prawdziwy grant issuer, konta programów, API zewnętrznych dostawców, rzeczywiste wnioski, podpisany dostęp. OMAs, OMAt i Zenoh sieciowe, OSTc, podpis certyfikatu, operator acceptance i produkcyjne podpisy.
