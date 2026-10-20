# Macierz portu Python do Rust — NEXUS CONSENT

**NEXUS CONSENT** · ewidencja **013** · serial `AM-COM-NEX-2026-000013` · Agent ID `AM-OCAF-AI-0013` · katalog `AM-OCAF-COM-013` · certyfikat `CERT-AM-2026-0013` **NIEWYDANY**.

Oryginały [`tools/communication_link_policy.py`](../../../tools/communication_link_policy.py) i [`tools/test_communication_link_policy.py`](../../../tools/test_communication_link_policy.py) pozostają bez zmian.

## Zachowane zachowania

Mapuje osiem protokołów — IrDA, BLE, NFC, UWB, Wi-Fi Direct, LoRaWAN, Li-Fi i Zenoh — na opisowe etykiety transportu, zasięgu i sposobu parowania. Wymaga celu owned_device_transfer/authorized_testbed/offline_comparison, zgody właściciela i drugiej strony, zakazu pasywnego śledzenia, skanowania, nieuprawnionej publikacji i fałszywych roszczeń globalnej geolokalizacji.

## Różnice semantyczne i ograniczenia

Port zachowuje jakościowe opisy protokołów i wszystkie przyczyny odmowy z Python. Opcjonalne bool pozwalają wiernie odwzorować brak pól w Python; Rust serde dodatkowo odrzuca nieznane pola i typy inne niż dozwolone.

Oryginalne Python unittest uruchamia się w osobnym kroku CI: `python3 -m unittest discover -s tools -p 'test_communication_link_policy.py' -v`. Fakt, że zestawy Rust i Python niezależnie przechodzą, nie stanowi pełnego dowodu równoważności semantycznej. Do certyfikacji wymagane golden fixtures i property-based differential tests.
