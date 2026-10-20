# Bezpieczeństwo i granice zaufania — NEXUS CONSENT

**NEXUS CONSENT** · ewidencja **013** · serial `AM-COM-NEX-2026-000013` · Agent ID `AM-OCAF-AI-0013` · katalog `AM-OCAF-COM-013` · certyfikat `CERT-AM-2026-0013` **NIEWYDANY**.

**Inwariant:** accepted=true jest wyłącznie zgodą planistyczną, nie pozwoleniem na aktywację radia, połączenie Zenoh, lokalizację GPS ani skanowanie sieci. has_physical_location_evidence=false i device_contacted=false dla wszystkich wejść.

| Ryzyko | Lokalna ochrona | Wymagany dowód produkcyjny |
|---|---|---|
| Deklaracja modelu podana jako autentyczny grant/dowód | flagi niezależnej weryfikacji/execution stale false | niezależny issuer i podpisane receipts |
| Podrobiony serial i Agent ID | statyczne wartości Rust, rejestr, TOML fail-closed | podpisana tożsamość OMAs RPC |
| Nieznany input/prompt injection | `deny_unknown_fields`, limit JSON, brak executor | sandbox, fuzzing i policy enforcement |
| Sieć Zenoh lub zdalny OMAt bez mTLS | tylko lokalny adapter; network false | real mTLS, tenant isolation i host acceptance |
| Odczyt źródeł bez OSTc | `source_access.enabled=false`; odmowa w runtime | spec, wektory i niezależny audyt kryptografii |
| Fałszywy certyfikat | `certificate_issued=false`, `Active` denied | podpisane wydanie, SBOM, operator |
| Szkodliwy cel łączności lub fałszywy grant | wymagana zgoda obu stron i negatywne flagi | niezależna kontrola autoryzacji przed efektem |

**Szczególne ograniczenia:** Aktywny pairing, odbiór zgody za pomocą kryptograficznej tożsamości, pomiary sygnału/położenia, rzeczywisty radio stack, izolacja sieci, testy sprzętu. Wynik `accepted/permitted` nigdy nie zastępuje zgody na rzeczywisty side effect.
