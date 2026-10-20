# Pochodzenie i migracja — ZIP WARDEN

**ZIP WARDEN** · ewidencja **012** · serial `AM-ARC-ZWD-2026-000012` · Agent ID `AM-OCAF-AI-0012` · katalog `AM-OCAF-ARC-012` · certyfikat `CERT-AM-2026-000012` (**rezerwacja; niewydany**).

Użyto istniejącego narzędzia Python `tools/source_bundle_audit.py` z testami `tools/test_source_bundle_audit.py` jako zweryfikowanego źródła funkcjonalnego. Stworzono nową, ograniczoną implementację w `src/agent.rs`, nie skasowano oryginału i nie uznano kompletnego projektu repo za ukończony.

**Zakres faktycznie przeniesiony:** Hash SHA-256 całego ZIP i rozpakowanej treści każdego pliku, lista rozszerzeń i rozmiarów, wykrywanie zduplikowanej treści, blokada traversal i niebezpiecznych ścieżek, case-insensitive duplikatów, dowolnych linków symbolicznych, nieobsługiwanych zaszyfrowanych elementów i bomb kompresyjnych.

**Granice i różnice:** Oryginał Python ogranicza ZIP do 500 członków, 750 MiB na element, 3 GiB łącznie. Port w Rust działa wyłącznie na bajtach już w pamięci i dodatkowo ogranicza źródłowy ZIP do 16 MiB, plik do 64 MiB oraz łącznie do 256 MiB. Do lokalnego OCAF dopuszcza tylko 1 MiB binarnego archiwum przesyłanego jako hex. Zabezpieczenie ścieżek Rust odrzuca nie-ASCII zamiast stosować Unicode casefold, co jest celową rozbieżnością.

**Pozostaje:** ekstrakcja/analiza znaczenia dokumentów, deduplikacja semantyczna, obsługa dużych strumieniowych archiwów 3GiB, szyfrowane ZIP, pełny Unicode casefold, zewnętrzny podpisany raport; także OMAs/OMAt/Zenoh i OSTc. Nie wprowadzono zmian w oryginalnym Python ani nie wywołano zewnętrznego systemu.
