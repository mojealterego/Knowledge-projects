# Pochodzenie i port — VERITAS TRACE

**Nazwa:** VERITAS TRACE · **Ewidencja:** 010 · **Serial:** `AM-OSN-VRT-2026-000010` · **Agent ID:** `AM-OCAF-AI-0010` · **Katalog:** `AM-OCAF-OSN-010` · **Certyfikat:** `CERT-AM-2026-0010` — NIEWYDANY.

To port **istniejącego kodu źródłowego** [`tools/osint_provenance_gate.py`](../../../tools/osint_provenance_gate.py) z testami [`tools/test_osint_provenance_gate.py`](../../../tools/test_osint_provenance_gate.py), a nie rekonstrukcja na podstawie opisu. Oryginały Python zachowano.

Zakres portu: Weryfikacja wymagalności scope_id, source_ref, observed_at, sha256 dla observed/verified, verifier_ref dla verified; status HTTP nigdy nie dowodzi istnienia konta.

Kontrola zmian: Port Rust wzmacnia walidację limitami długości i znakami sterującymi, dodaje jawne dwa pola independently_verified=false/source_contacted=false. Python tego nie raportował, lecz jego dokumentacja stwierdza brak niezależnego dowodu. Status HTTP w Rust przyjmuje u16 zamiast dynamicznego int Python; niezmiennik 100..599 zostaje zachowany.

Nie skopiowano tajnych kluczy, nie uruchomiono serwerów OMAs/OMAt/OSTc, nie wykonano rzeczywistych operacji na zewnętrznych systemach. Rozszerzenia wymagają osobnych testów, audytu i dokumentacji.
