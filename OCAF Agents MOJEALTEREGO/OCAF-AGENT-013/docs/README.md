# NEXUS CONSENT — OCAF-AGENT-013

**NEXUS CONSENT** · ewidencja **013** · serial `AM-COM-NEX-2026-000013` · Agent ID `AM-OCAF-AI-0013` · katalog `AM-OCAF-COM-013` · certyfikat `CERT-AM-2026-0013` **NIEWYDANY**.

**STATUS: INCOMPLETE_DEVELOPMENT | `certificate_issued=false` | `production_approved=false`.**

## Cel i granica funkcji

analiza zgody stron i dopuszczalnych zastosowań technologii komunikacyjnych bez użycia łączności. Mapuje osiem protokołów — IrDA, BLE, NFC, UWB, Wi-Fi Direct, LoRaWAN, Li-Fi i Zenoh — na opisowe etykiety transportu, zasięgu i sposobu parowania. Wymaga celu owned_device_transfer/authorized_testbed/offline_comparison, zgody właściciela i drugiej strony, zakazu pasywnego śledzenia, skanowania, nieuprawnionej publikacji i fałszywych roszczeń globalnej geolokalizacji.

Kod oparto na rzeczywistym [`tools/communication_link_policy.py`](../../../tools/communication_link_policy.py) i jego [testach Python](../../../tools/test_communication_link_policy.py). accepted=true jest wyłącznie zgodą planistyczną, nie pozwoleniem na aktywację radia, połączenie Zenoh, lokalizację GPS ani skanowanie sieci. has_physical_location_evidence=false i device_contacted=false dla wszystkich wejść.

## Kontrole lokalne

```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-013/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-013/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-013/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-013/Cargo.toml" -- demo
```

Działające funkcje lokalne Rust: `src/agent.rs` oraz lokalny OCAF ToolExecutor w `src/zenoh.rs`. OMAs jest oddzielnym docelowym **serwerem agentów**, OMAt **serwerem narzędzi**, natomiast Zenoh pozostaje wyłączonym transportem sieciowym (tylko klucze tras). OSTc nie ma jeszcze zweryfikowanej specyfikacji; dostęp do źródeł jest zabroniony.

Dokumenty: [ARCHITECTURE](ARCHITECTURE.md), [SPECIFICATION](SPECIFICATION.md), [USER_MANUAL](USER_MANUAL.md), [SECURITY](SECURITY.md), [SERVICE](SERVICE.md), [IDENTITY_AND_SERIAL](IDENTITY_AND_SERIAL.md), [PYTHON_PARITY](PYTHON_PARITY.md), [DEPENDENCIES](DEPENDENCIES.md), [MIGRATION](MIGRATION.md), [RELEASE_CHECKLIST](RELEASE_CHECKLIST.md), [TRACEABILITY_MATRIX](TRACEABILITY_MATRIX.md), [TEST_EVIDENCE](TEST_EVIDENCE.md), [CHANGELOG](CHANGELOG.md), [CERTIFICATE_RECORD](../certification/CERTIFICATE_RECORD.md), [SALE-DRAFT](commercial/SALE-DRAFT.md), [LEASE-DRAFT](commercial/LEASE-DRAFT.md), [RENT-DRAFT](commercial/RENT-DRAFT.md).
