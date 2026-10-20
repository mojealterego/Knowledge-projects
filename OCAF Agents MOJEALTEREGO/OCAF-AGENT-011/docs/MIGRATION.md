# Pochodzenie i migracja — PROVENANT EVIDENCE

**PROVENANT EVIDENCE** · ewidencja **011** · serial `AM-PRV-EVD-2026-000011` · Agent ID `AM-OCAF-AI-0011` · katalog `AM-OCAF-PRV-011` · certyfikat `CERT-AM-2026-000011` (**rezerwacja; niewydany**).

Użyto istniejącego narzędzia Python `tools/osint_provenance_gate.py` z testami `tools/test_osint_provenance_gate.py` jako zweryfikowanego źródła funkcjonalnego. Stworzono nową, ograniczoną implementację w `src/agent.rs`, nie skasowano oryginału i nie uznano kompletnego projektu repo za ukończony.

**Zakres faktycznie przeniesiony:** Zachowuje odrębne etykiety simulated/observed/verified; observed wymaga scope_id, source_ref, observed_at oraz hash SHA-256; verified dodatkowo wymaga wiarygodnego identyfikatora weryfikatora. Status odpowiedzi HTTP nigdy nie świadczy o istnieniu konkretnej osoby lub konta.

**Granice i różnice:** W Pythonie do utworzenia verified wystarczało niepuste verifier_ref. Port Rust wymaga dodatkowo, by identyfikator należał do trusted_verifiers dostarczanych poza żądaniem przez hosta. Waliduje rozmiar claim oraz pól odniesienia, a oddzielny adapter OCAF limituje JSON do 8192 bajtów. To wzmocniona, nie bitowo równoważna wersja modułu Python.

**Pozostaje:** samodzielna identyfikacja źródeł w sieci, rozpoznawanie prawdziwości treści, signed verifier receipts, E2E OSINT, runtime do pozyskiwania danych, formalny certyfikat źródła; także OMAs/OMAt/Zenoh i OSTc. Nie wprowadzono zmian w oryginalnym Python ani nie wywołano zewnętrznego systemu.
