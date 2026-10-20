# Porównanie Python → Rust — PROVENANT EVIDENCE

**PROVENANT EVIDENCE** · ewidencja **011** · serial `AM-PRV-EVD-2026-000011` · Agent ID `AM-OCAF-AI-0011` · katalog `AM-OCAF-PRV-011` · certyfikat `CERT-AM-2026-000011` (**rezerwacja; niewydany**).

Źródło kodu: [`tools/osint_provenance_gate.py`](../../../tools/osint_provenance_gate.py), testy referencyjne [`tools/test_osint_provenance_gate.py`](../../../tools/test_osint_provenance_gate.py). Oryginały pozostawiono bez zmian.

## Portowane zachowania
Zachowuje odrębne etykiety simulated/observed/verified; observed wymaga scope_id, source_ref, observed_at oraz hash SHA-256; verified dodatkowo wymaga wiarygodnego identyfikatora weryfikatora. Status odpowiedzi HTTP nigdy nie świadczy o istnieniu konkretnej osoby lub konta.

## Jawne odstępstwa
W Pythonie do utworzenia verified wystarczało niepuste verifier_ref. Port Rust wymaga dodatkowo, by identyfikator należał do trusted_verifiers dostarczanych poza żądaniem przez hosta. Waliduje rozmiar claim oraz pól odniesienia, a oddzielny adapter OCAF limituje JSON do 8192 bajtów. To wzmocniona, nie bitowo równoważna wersja modułu Python.

## Regresje
Referencyjny Python unittest sprawdza kod oryginalny; Rust unit i testy domenowe sprawdzają odrębny port. Dodatkowy `ocaf_contract` testuje granicę wywołań modelowych i odrzucanie niepodpisanych zdalnych peerów. Przejście obu pakietów nie jest dowodem formalnej pełnej równoważności Python/Rust. Wyniki CI i źródłowy SHA wpisane w `TEST_EVIDENCE.md`.
