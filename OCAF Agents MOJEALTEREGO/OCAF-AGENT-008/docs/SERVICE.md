# Serwis / lifecycle — DGM SENTINEL

**Nazwa własna:** DGM SENTINEL · **Numer ewidencyjny:** 008 · **Serial:** `AM-DGM-SNT-2026-000008` · **Agent ID:** `AM-OCAF-AI-0008` · **Katalog:** `AM-OCAF-DGM-008` · **Certyfikat:** `CERT-AM-2026-0008` — NIEWYDANY.

1. Porównaj wpis JSON globalnego rejestru z konfiguracją TOML, manifestem i kodem.
2. Uruchom testy Rust i oryginalny Python z konkretnym commit SHA; zachowaj run ID i logi.
3. Zweryfikuj `network_enabled=false` i `source_access.enabled=false`.
4. Przed każdym użyciem zweryfikuj niezależną autoryzację hosta; Zatwierdzone kandydatury, potwierdzone CI, receipt faz, rollback i source provenance muszą być wstrzyknięte przez zaufanego hosta jako PromotionTrust. Samo rollback_ref_verified=true lub source_provenance_verified=true w prośbie modelu nie ma autorytatywnej mocy.
5. Błąd to odmowa z listą powodów, nie pozwolenie na obejście kontroli.
6. W krytycznym incydencie zachowuj logi bez danych wrażliwych, wyłącz adapter przed zmianą polityk.

Stan `Draft`, `Tested`, `Staged` to lokalna ewidencja. `Active` zawsze DENIED, nie ma rzeczywistego revoke na serwerze OMAs. SLA, koszt utrzymania, gwarancja, 24/7 i polisa pozostają NIEUSTALONE.
