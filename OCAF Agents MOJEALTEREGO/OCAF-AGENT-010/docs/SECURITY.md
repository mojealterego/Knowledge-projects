# Model zagrożeń — VERITAS TRACE

**Nazwa:** VERITAS TRACE · **Ewidencja:** 010 · **Serial:** `AM-OSN-VRT-2026-000010` · **Agent ID:** `AM-OCAF-AI-0010` · **Katalog:** `AM-OCAF-OSN-010` · **Certyfikat:** `CERT-AM-2026-0010` — NIEWYDANY.

**Najważniejsze:** Oznaczenie verified opisuje jedynie przekazaną dokumentację; brak niezależnego poświadczenia źródła, stąd independently_verified=false i source_contacted=false.

| Ryzyko | Kontrola obecna | Bramka docelowa |
|---|---|---|
| Przedstawianie treści modelu jako dowodu lub zgody | wynik lokalny nie oznacza niezależnej weryfikacji/wykonania | podpisany issuer i readback |
| Błędny numer, nazwa lub tenant | Rust constants i TOML fail closed; test rejestru | kryptograficzna tożsamość RPC |
| JSON injection / unknown fields | serde deny_unknown_fields, rozmiar ograniczony | niezależny fuzzing i authz |
| Obejście granicy narzędzia | brak uprawnień do modyfikacji i sieci | OMAt capability scoped grants |
| Sieciowy podszywający się agent | wyłączony transport, lokalny test AuthRequired | prawdziwy Zenoh/TLS/mTLS |
| Odczyt źródeł bez OSTc | trwała odmowa, żadnych fikcyjnych kluczy | implementacja i audyt OSTc |
| Fałszywy certyfikat | `certificate_issued=false`, `Active` DENIED | signed release i operator acceptance |
| Nieukończony produkt opisany jako pełny | jawny zakres lokalny i niewydany certyfikat | formalny audyt wdrożenia |

## Scenariusze specyficzne

Weryfikacja wymagalności scope_id, source_ref, observed_at, sha256 dla observed/verified, verifier_ref dla verified; status HTTP nigdy nie dowodzi istnienia konta. Autentykacja niezależnego weryfikatora, retrieval rzeczywistych obserwacji, podpisany digest, ochrona danych osobowych, testy zewnętrznych źródeł i polityk uprawnień.

Wersja developerska nie ma poświadczeń i nie może być wdrażana jako kontrola egzekwująca operacje zewnętrzne.
