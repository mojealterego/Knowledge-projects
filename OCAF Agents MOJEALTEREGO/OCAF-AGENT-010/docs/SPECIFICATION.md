# Charakterystyka techniczna — VERITAS TRACE

**Nazwa:** VERITAS TRACE · **Ewidencja:** 010 · **Serial:** `AM-OSN-VRT-2026-000010` · **Agent ID:** `AM-OCAF-AI-0010` · **Katalog:** `AM-OCAF-OSN-010` · **Certyfikat:** `CERT-AM-2026-0010` — NIEWYDANY.

**Cel:** Lokalna walidacja metryki pochodzenia dowodu OSINT i prawidłowe oznaczanie treści symulowanych, obserwowanych oraz deklarowanych jako zweryfikowane.

**Narzędzie:** `osint.provenance_record`. Wyniki przeznaczone do oceny przez operatora, nie do wywołania bezpośrednich skutków ubocznych.

Wejście `EvidenceInput`: `claim`, `origin` (simulated/observed/verified), opcjonalne `scope_id`, `source_ref`, `observed_at`, `verifier_ref`, `source_sha256`. Obserwacja wymaga pełnego śladu, a verified dodatkowo referencji weryfikatora. `ui_badge` rozróżnia trzy stany w języku polskim. Wynik `EvidenceRecord` zawiera `independently_verified=false` niezależnie od klasyfikacji, bo nie ma niezależnego połączenia z dostawcą ani verifierem. Metoda `classify_http_username_status` odmawia wnioskowania o istnieniu konta nawet przy HTTP 200.

**Zachowanie:** Weryfikacja wymagalności scope_id, source_ref, observed_at, sha256 dla observed/verified, verifier_ref dla verified; status HTTP nigdy nie dowodzi istnienia konta.

**Granice:** Oznaczenie verified opisuje jedynie przekazaną dokumentację; brak niezależnego poświadczenia źródła, stąd independently_verified=false i source_contacted=false.

Niezależny audyt, podpisy, OSTc, rzeczywiste sieciowe OMAs/OMAt/Zenoh, multi-host i operator acceptance pozostają niewykonane. Cena i wycena brak.
