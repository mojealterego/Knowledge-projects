# Kryteria odbioru — AXIOM FORGE

**AXIOM FORGE** / ewidencja **004** / serial `AM-ENG-AXF-2026-000004` / ID `AM-OCAF-AI-0004` / katalog `AM-OCAF-ENG-004` / certyfikat `CERT-AM-2026-0004` **NIEWYDANY**.

| Zakres | Stan |
|---|---|
| Własna nazwa i serial niezależne od numeru 004 | REGISTERED DEVELOPMENT |
| Struktura Rust, TOML/JSON, pełna dokumentacja developerska | IMPLEMENTED; CI required |
| Funkcja lokalna `graph.review` i testy negatywne | IMPLEMENTED; wynik w TEST_EVIDENCE |
| Zweryfikowany globalny rejestr nazw i seriali | TESTS; nie podpisany |
| Produkcyjny serwer OMAs i podpis związany z RPC | NOT IMPLEMENTED |
| Produkcyjny serwer OMAt i realne granty narzędzi | NOT IMPLEMENTED |
| Zenoh network, TLS/mTLS, multi-host isolation | NOT IMPLEMENTED |
| Szyfrowany dostęp do kodu przez OSTc | BLOCKED / SPEC PENDING |
| rzeczywiste wykonywanie workflow, zewnętrzne silniki build/test, niezależna promocja i rollback | NOT IMPLEMENTED |
| Supply-chain SBOM, reprodukowalny build, artifact digest i podpis | NOT COMPLETED |
| Niezależny audyt bezpieczeństwa, formalny odbiór operatora | NOT COMPLETED |
| Autentyczny wizerunek 35×45 mm, logo, podpis i certyfikat | NOT ISSUED |
| `Active` | DENIED IN CODE |

## Próg ukończenia

Wymagany podpisany pakiet `source SHA + executable digest + SBOM + OMAs/OMAt/OSTc evidence + tests T01–T15 + independent audit + operator acceptance`. Numery seryjne i pozytywne CI jednostkowe **nie zastępują** tych dowodów.
