# Specyfikacja techniczna — NEXUS CONSENT

**NEXUS CONSENT** · ewidencja **013** · serial `AM-COM-NEX-2026-000013` · Agent ID `AM-OCAF-AI-0013` · katalog `AM-OCAF-COM-013` · certyfikat `CERT-AM-2026-0013` **NIEWYDANY**.

## Funkcja domenowa

Mapuje osiem protokołów — IrDA, BLE, NFC, UWB, Wi-Fi Direct, LoRaWAN, Li-Fi i Zenoh — na opisowe etykiety transportu, zasięgu i sposobu parowania. Wymaga celu owned_device_transfer/authorized_testbed/offline_comparison, zgody właściciela i drugiej strony, zakazu pasywnego śledzenia, skanowania, nieuprawnionej publikacji i fałszywych roszczeń globalnej geolokalizacji.

## Interfejs

Narzędzie `communication.review_link`. Wejście `LinkPlan`: `protocol`, `purpose`, `endpoint_owner_authorized`, `counterparty_opted_in`, `passive_tracking`, `network_scan`, `claims_global_device_location`, `cloud_publication`. Wynik `LinkVerdict`: `accepted`, `reasons[]`, `has_physical_location_evidence=false`, `execution_authorized=false`, `device_contacted=false`.

## Kryteria odmowy i dowody

accepted=true jest wyłącznie zgodą planistyczną, nie pozwoleniem na aktywację radia, połączenie Zenoh, lokalizację GPS ani skanowanie sieci. has_physical_location_evidence=false i device_contacted=false dla wszystkich wejść. Port zachowuje jakościowe opisy protokołów i wszystkie przyczyny odmowy z Python. Opcjonalne bool pozwalają wiernie odwzorować brak pól w Python; Rust serde dodatkowo odrzuca nieznane pola i typy inne niż dozwolone.

## Niezaimplementowane

Aktywny pairing, odbiór zgody za pomocą kryptograficznej tożsamości, pomiary sygnału/położenia, rzeczywisty radio stack, izolacja sieci, testy sprzętu. OMAs, OMAt i Zenoh sieciowe, OSTc, podpis certyfikatu, operator acceptance i produkcyjne podpisy.
