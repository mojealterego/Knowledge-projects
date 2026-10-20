# Porównanie Python → Rust — ZIP WARDEN

**ZIP WARDEN** · ewidencja **010** · serial `AM-ARC-ZWD-2026-000010` · Agent ID `AM-OCAF-AI-0010` · katalog `AM-OCAF-ARC-010` · certyfikat `CERT-AM-2026-000010` (**rezerwacja; niewydany**).

Źródło kodu: [`tools/source_bundle_audit.py`](../../../tools/source_bundle_audit.py), testy referencyjne [`tools/test_source_bundle_audit.py`](../../../tools/test_source_bundle_audit.py). Oryginały pozostawiono bez zmian.

## Portowane zachowania
Hash SHA-256 całego ZIP i rozpakowanej treści każdego pliku, lista rozszerzeń i rozmiarów, wykrywanie zduplikowanej treści, blokada traversal i niebezpiecznych ścieżek, case-insensitive duplikatów, dowolnych linków symbolicznych, nieobsługiwanych zaszyfrowanych elementów i bomb kompresyjnych.

## Jawne odstępstwa
Oryginał Python ogranicza ZIP do 500 członków, 750 MiB na element, 3 GiB łącznie. Port w Rust działa wyłącznie na bajtach już w pamięci i dodatkowo ogranicza źródłowy ZIP do 16 MiB, plik do 64 MiB oraz łącznie do 256 MiB. Do lokalnego OCAF dopuszcza tylko 1 MiB binarnego archiwum przesyłanego jako hex. Zabezpieczenie ścieżek Rust odrzuca nie-ASCII zamiast stosować Unicode casefold, co jest celową rozbieżnością.

## Regresje
Referencyjny Python unittest sprawdza kod oryginalny; Rust unit i testy domenowe sprawdzają odrębny port. Dodatkowy `ocaf_contract` testuje granicę wywołań modelowych i odrzucanie niepodpisanych zdalnych peerów. Przejście obu pakietów nie jest dowodem formalnej pełnej równoważności Python/Rust. Wyniki CI i źródłowy SHA wpisane w `TEST_EVIDENCE.md`.
