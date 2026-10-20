# Proweniencja i migracja — NEXUS CONSENT

**NEXUS CONSENT** · ewidencja **013** · serial `AM-COM-NEX-2026-000013` · Agent ID `AM-OCAF-AI-0013` · katalog `AM-OCAF-COM-013` · certyfikat `CERT-AM-2026-0013` **NIEWYDANY**.

Źródłem kodu jest istniejący [`tools/communication_link_policy.py`](../../../tools/communication_link_policy.py) oraz [testy referencyjne](../../../tools/test_communication_link_policy.py). W Rust napisano samodzielne procedury oceny, nie migrację aktywnej sieciowej usługi. Nie zmieniono oryginalnych plików.

**Portowane reguły:** Mapuje osiem protokołów — IrDA, BLE, NFC, UWB, Wi-Fi Direct, LoRaWAN, Li-Fi i Zenoh — na opisowe etykiety transportu, zasięgu i sposobu parowania. Wymaga celu owned_device_transfer/authorized_testbed/offline_comparison, zgody właściciela i drugiej strony, zakazu pasywnego śledzenia, skanowania, nieuprawnionej publikacji i fałszywych roszczeń globalnej geolokalizacji.

**Jawne odchylenia:** Port zachowuje jakościowe opisy protokołów i wszystkie przyczyny odmowy z Python. Opcjonalne bool pozwalają wiernie odwzorować brak pól w Python; Rust serde dodatkowo odrzuca nieznane pola i typy inne niż dozwolone.

**Niekompletne:** Aktywny pairing, odbiór zgody za pomocą kryptograficznej tożsamości, pomiary sygnału/położenia, rzeczywisty radio stack, izolacja sieci, testy sprzętu. Integracja rzeczywistych systemów wymaga odrębnej zgody i dowodów.
