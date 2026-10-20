# Model zagrożeń — CLOUD FISCALIS

**Nazwa własna:** CLOUD FISCALIS · **Numer ewidencyjny:** 009 · **Serial:** `AM-CLD-FIS-2026-000009` · **Agent ID:** `AM-OCAF-AI-0009` · **Katalog:** `AM-OCAF-CLD-009` · **Certyfikat:** `CERT-AM-2026-0009` — NIEWYDANY.

**Zasada naczelna:** CloudApproval powstaje w niezależnym systemie polityki właściciela/hosta i jest przekazywane do AgentTool; pole modelu explicit_external_side_effect_approval nie stanowi IAM, rozliczenia ani zgody na rzeczywiste wykonanie.

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

**Specjalne ryzyko:** Rust 009 zachowuje reguły plan-only ze źródłowego Python, ale dodatkowo odrzuca secrets_in_prompt nawet w ścieżce public_docs, w której Python kończył pracę przed tą kontrolą. JSON cost musi być stringiem — schema odrzuca liczby i booleans przed wywołaniem domeny. Zakres maksymalnego kosztu to precyzyjna arytmetyka decimal.

Nie oznaczać żadnego certyfikatu jako wydanego i nie pozwalać na `Active` przed niezależnym podpisem operatora i rzeczywistymi dowodami.
