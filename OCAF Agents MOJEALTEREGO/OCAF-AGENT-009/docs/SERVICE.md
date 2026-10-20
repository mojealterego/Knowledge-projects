# Serwis / lifecycle — CLOUD FISCALIS

**Nazwa własna:** CLOUD FISCALIS · **Numer ewidencyjny:** 009 · **Serial:** `AM-CLD-FIS-2026-000009` · **Agent ID:** `AM-OCAF-AI-0009` · **Katalog:** `AM-OCAF-CLD-009` · **Certyfikat:** `CERT-AM-2026-0009` — NIEWYDANY.

1. Porównaj wpis JSON globalnego rejestru z konfiguracją TOML, manifestem i kodem.
2. Uruchom testy Rust i oryginalny Python z konkretnym commit SHA; zachowaj run ID i logi.
3. Zweryfikuj `network_enabled=false` i `source_access.enabled=false`.
4. Przed każdym użyciem zweryfikuj niezależną autoryzację hosta; CloudApproval powstaje w niezależnym systemie polityki właściciela/hosta i jest przekazywane do AgentTool; pole modelu explicit_external_side_effect_approval nie stanowi IAM, rozliczenia ani zgody na rzeczywiste wykonanie.
5. Błąd to odmowa z listą powodów, nie pozwolenie na obejście kontroli.
6. W krytycznym incydencie zachowuj logi bez danych wrażliwych, wyłącz adapter przed zmianą polityk.

Stan `Draft`, `Tested`, `Staged` to lokalna ewidencja. `Active` zawsze DENIED, nie ma rzeczywistego revoke na serwerze OMAs. SLA, koszt utrzymania, gwarancja, 24/7 i polisa pozostają NIEUSTALONE.
