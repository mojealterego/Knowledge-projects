# Zgodność zakresu Python / Rust — VERITAS TRACE

**Nazwa:** VERITAS TRACE · **Ewidencja:** 010 · **Serial:** `AM-OSN-VRT-2026-000010` · **Agent ID:** `AM-OCAF-AI-0010` · **Katalog:** `AM-OCAF-OSN-010` · **Certyfikat:** `CERT-AM-2026-0010` — NIEWYDANY.

Źródło: [`tools/osint_provenance_gate.py`](../../../tools/osint_provenance_gate.py). Testy: [`tools/test_osint_provenance_gate.py`](../../../tools/test_osint_provenance_gate.py).

**Portowane reguły:** Weryfikacja wymagalności scope_id, source_ref, observed_at, sha256 dla observed/verified, verifier_ref dla verified; status HTTP nigdy nie dowodzi istnienia konta.

**Jawne różnice:** Port Rust wzmacnia walidację limitami długości i znakami sterującymi, dodaje jawne dwa pola independently_verified=false/source_contacted=false. Python tego nie raportował, lecz jego dokumentacja stwierdza brak niezależnego dowodu. Status HTTP w Rust przyjmuje u16 zamiast dynamicznego int Python; niezmiennik 100..599 zostaje zachowany.

Oryginalne pliki Python pozostają niezmienione. Uruchomienie obu niezależnych zestawów testów nie jest dowodem formalnej równoważności wszystkich możliwych wejść. Nie ma integracji produkcyjnych ani uprawnień do działań sieciowych. Dalsze kroki to property-based tests i golden fixtures dla trudnych przypadków.
