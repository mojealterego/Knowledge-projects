# Bezpieczeństwo i granice zaufania — FIDES GRANTIS

**FIDES GRANTIS** · ewidencja **012** · serial `AM-GRT-FID-2026-000012` · Agent ID `AM-OCAF-AI-0012` · katalog `AM-OCAF-GRT-012` · certyfikat `CERT-AM-2026-0012` **NIEWYDANY**.

**Inwariant:** Wynik permitted=true jest lokalną decyzją walidatora; nie potwierdza otrzymania pieniędzy ani grantów, nie składa wniosku, nie uruchamia obliczeń i nie sprawdza na żywo stanu programów. Czas UTC pochodzi od hosta, nie z modelowego JSON.

| Ryzyko | Lokalna ochrona | Wymagany dowód produkcyjny |
|---|---|---|
| Deklaracja modelu podana jako autentyczny grant/dowód | flagi niezależnej weryfikacji/execution stale false | niezależny issuer i podpisane receipts |
| Podrobiony serial i Agent ID | statyczne wartości Rust, rejestr, TOML fail-closed | podpisana tożsamość OMAs RPC |
| Nieznany input/prompt injection | `deny_unknown_fields`, limit JSON, brak executor | sandbox, fuzzing i policy enforcement |
| Sieć Zenoh lub zdalny OMAt bez mTLS | tylko lokalny adapter; network false | real mTLS, tenant isolation i host acceptance |
| Odczyt źródeł bez OSTc | `source_access.enabled=false`; odmowa w runtime | spec, wektory i niezależny audyt kryptografii |
| Fałszywy certyfikat | `certificate_issued=false`, `Active` denied | podpisane wydanie, SBOM, operator |
| Szkodliwy cel łączności lub fałszywy grant | reject po zmianie statusu na awarded i po próbie submit | niezależna kontrola autoryzacji przed efektem |

**Szczególne ograniczenia:** Weryfikacja bieżących regulaminów i kwalifikowalności; prawdziwy grant issuer, konta programów, API zewnętrznych dostawców, rzeczywiste wnioski, podpisany dostęp. Wynik `accepted/permitted` nigdy nie zastępuje zgody na rzeczywisty side effect.
