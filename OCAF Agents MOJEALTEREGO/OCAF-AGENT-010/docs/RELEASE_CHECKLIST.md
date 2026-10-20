# Kryteria ukończenia i wydania — VERITAS TRACE

**Nazwa:** VERITAS TRACE · **Ewidencja:** 010 · **Serial:** `AM-OSN-VRT-2026-000010` · **Agent ID:** `AM-OCAF-AI-0010` · **Katalog:** `AM-OCAF-OSN-010` · **Certyfikat:** `CERT-AM-2026-0010` — NIEWYDANY.

| Zakres | Status |
|---|---|
| Odrębny folder, nazwa, serial, katalog, Agent ID, TOML, manifest | IMPLEMENTED DEV |
| Kod domenowy Rust i lokalny OCAF ToolExecutor | IMPLEMENTED; TESTED LOCAL — RUN 39422100249 |
| Testy bezpieczeństwa i negatywne | IMPLEMENTED; TESTED LOCAL — RUN 39422100249 |
| Testy oryginalnego Python i porównanie kontraktu | TESTED LOCAL — RUN 39422100249 |
| Dokumentacja, instrukcja, cert record i projekty trzech umów | DOCUMENTED DEV |
| Produkcyjny OMAs z podpisaną tożsamością RPC | NOT IMPLEMENTED |
| Produkcyjny OMAt z odrębną autoryzacją narzędzi | NOT IMPLEMENTED |
| Zenoh mTLS, tenant isolation i niezależne hosty | NOT IMPLEMENTED |
| OSTc — zaszyfrowany dostęp do kodu źródłowego | BLOCKED BY SPEC |
| Autentykacja niezależnego weryfikatora, retrieval rzeczywistych obserwacji, podpisany digest, ochrona danych osobowych, testy zewnętrznych źródeł i polityk uprawnień. | NOT IMPLEMENTED |
| SBOM, signed artifact, formalny operator acceptance | NOT COMPLETED |
| Rzeczywisty certyfikat | NOT ISSUED |

Nie wydawać numeru `CERT-AM-2026-0010` jako autentycznego certyfikatu. Sam PASS w CI nie jest formalnym odbiorem.
