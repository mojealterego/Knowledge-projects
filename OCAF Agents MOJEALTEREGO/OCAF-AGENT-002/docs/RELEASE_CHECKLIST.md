# VIGILIS SRE — OCAF-AGENT-002

**Nazwa własna:** VIGILIS SRE · **Numer ewidencyjny:** 002 · **Numer seryjny:** `AM-SRE-VIG-2026-000002` · **Agent ID:** `AM-OCAF-AI-0002` · **Nr katalogowy:** `AM-OCAF-SRE-002` · **Certyfikat:** `CERT-AM-2026-0002` — REZERWACJA, NIEWYDANY.

**Stan:** `INCOMPLETE_DEVELOPMENT`. Lokalny kod Rust do celu: diagnostyka incydentów infrastruktury. Nie jest to kompletna migracja projektu źródłowego ani aktywny produkt OMAs/OMAt/OSTc.

## Kryteria odbioru

| Warunek | Stan |
|---|---|
| Unikatowa nazwa, serial, Agent ID, katalog i folder | REGISTERED DEVELOPMENT |
| Kod Rust `incident.triage`, testy i config | IMPLEMENTED, CI do potwierdzenia |
| Specyfikacja, instrukcja, bezpieczeństwo, serwis, migracja, cert record | DOCUMENTED DEVELOPMENT SCOPE |
| Rzeczywista integracja OMAs agent server i OMAt tool server | NOT IMPLEMENTED |
| Weryfikowany ruch Zenoh TLS/mTLS i wielohost | NOT IMPLEMENTED |
| OSTc — szyfrowany odczyt plików źródłowych | BLOCKED: pełna specyfikacja nie dostarczona |
| Kubernetes API readback, Log/Prometheus, Nebius/Nemotron/Tavily/NeMo, bezpieczny sandbox i faktyczne remediacje | NOT IMPLEMENTED |
| Podpis i digest binarny + SBOM/reproducible build | NOT AVAILABLE |
| Testy awarii/recovery, tenant isolation, authz | NOT RUN |
| Niezależny audyt bezpieczeństwa i prawny | NOT RUN |
| Prawdziwy wizerunek i logo, certyfikat, podpis operatora | DRAFT / NOT ISSUED |
| Formalna aktywacja | DENIED BY LIFECYCLE |

Nie przyznawaj `PRODUCTION APPROVED` tylko na podstawie testu jednostkowego, nazwy, serialu, CI lub dokumentacji. Kryteria całości wymagają dowodów infrastruktury i OSTc.
