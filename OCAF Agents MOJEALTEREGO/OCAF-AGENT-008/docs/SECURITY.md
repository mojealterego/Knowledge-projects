# Model zagrożeń — DGM SENTINEL

**Nazwa własna:** DGM SENTINEL · **Numer ewidencyjny:** 008 · **Serial:** `AM-DGM-SNT-2026-000008` · **Agent ID:** `AM-OCAF-AI-0008` · **Katalog:** `AM-OCAF-DGM-008` · **Certyfikat:** `CERT-AM-2026-0008` — NIEWYDANY.

**Zasada naczelna:** Zatwierdzone kandydatury, potwierdzone CI, receipt faz, rollback i source provenance muszą być wstrzyknięte przez zaufanego hosta jako PromotionTrust. Samo rollback_ref_verified=true lub source_provenance_verified=true w prośbie modelu nie ma autorytatywnej mocy.

| Atak / błąd | Kontrola aktualna | Wymagana bramka produkcyjna |
|---|---|---|
| Model deklaruje własną zgodę i dowody | host-owned approval registry w `AgentTool` | podpisany issuer, revocation i audit |
| Fałszywy serial i ID | sztywne stałe Rust i `agent.toml`, testy | powiązanie certyfikatu z RPC |
| Złośliwy nieznany payload | serde `deny_unknown_fields` i maks. długość adaptera | fuzzing, isolation policy |
| Przejęcie omas/omat route | tylko walidacja logicznego keyspace; network OFF | niezależne TLS/mTLS, tenant partitioning |
| Niedostępne źródła OSTc | `source_access.enabled=false` i DENY | szczegółowa specyfikacja, vectors i audyt |
| Uznanie wyniku review za zgodę operacyjną | `execution_authorized=false` i brak implementacji wykonawcy | operator-controlled signed authorization |
| Niezależna akceptacja bez realnego dowodu | mapa zaufanego hosta jest umownym wejściem w testach | prawdziwy zewnętrzny weryfikator |
| Odwzorowanie Python bez testów | nowy port + jednostki oryginału | differential property tests i independent audit |
| Łańcuch dostaw Git rev | przypięta rewizja OCAF | SCA, SBOM, signed binary, licencje |

**Specjalne ryzyko:** Rust 008 dodaje wymóg hostowych zatwierdzeń receiptów czterech faz, źródła i rollbacku ponad pierwotne pola proposal w Python. Zaostrza format integration branch i ogranicza rozmiar wejścia. Pozostała lista kontroli pochodzi z oryginalnego Python. Nie oznacza to ścisłej bitowej równoważności.

Nie oznaczać żadnego certyfikatu jako wydanego i nie pozwalać na `Active` przed niezależnym podpisem operatora i rzeczywistymi dowodami.
