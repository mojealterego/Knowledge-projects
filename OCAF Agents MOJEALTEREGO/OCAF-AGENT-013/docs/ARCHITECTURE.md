# Architektura — NEXUS CONSENT

**NEXUS CONSENT** · ewidencja **013** · serial `AM-COM-NEX-2026-000013` · Agent ID `AM-OCAF-AI-0013` · katalog `AM-OCAF-COM-013` · certyfikat `CERT-AM-2026-0013` **NIEWYDANY**.

```text
Trusted local operator/host
    |
    v
OCAF Runtime (local in-process ToolExecutor)
    |
    +--> communication.review_link / Rust `src/agent.rs`
    +--> OMAs agent server: NOT DEPLOYED
    +--> OMAt tool server: NOT DEPLOYED
    +--> Zenoh: route validation only / network OFF
    +--> OSTc: source_access.enabled=false / DENY
```

## Moduły

- `src/agent.rs`: Mapuje osiem protokołów — IrDA, BLE, NFC, UWB, Wi-Fi Direct, LoRaWAN, Li-Fi i Zenoh — na opisowe etykiety transportu, zasięgu i sposobu parowania. Wymaga celu owned_device_transfer/authorized_testbed/offline_comparison, zgody właściciela i drugiej strony, zakazu pasywnego śledzenia, skanowania, nieuprawnionej publikacji i fałszywych roszczeń globalnej geolokalizacji.
- `src/zenoh.rs`: rozdzielone logiczne przestrzenie `omas/v1` oraz `omat/v1`, lokalny manifest JSON i adapter OCAF z limitem 4096 B; **brak sieci**.
- `src/security.rs`: walidacja stałej tożsamości, serialu, config TOML i zakazu uruchomienia połączeń.
- `src/lifecycle.rs`: `Active` zawsze zabroniony do formalnego zatwierdzenia.
- `src/main.rs`: lokalne demo, brak zewnętrznych efektów.

**Host trust boundary:** accepted=true jest wyłącznie zgodą planistyczną, nie pozwoleniem na aktywację radia, połączenie Zenoh, lokalizację GPS ani skanowanie sieci. has_physical_location_evidence=false i device_contacted=false dla wszystkich wejść.

Produkcyjne signed OMAs RPC, OMAt scoped grants, TLS-bound Zenoh, OSTc i testy niezależnych hostów pozostają niezaimplementowane.
