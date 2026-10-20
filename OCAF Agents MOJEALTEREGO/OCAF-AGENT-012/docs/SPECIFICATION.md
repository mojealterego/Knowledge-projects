# Charakterystyka i zakres — ZIP WARDEN

**ZIP WARDEN** · ewidencja **012** · serial `AM-ARC-ZWD-2026-000012` · Agent ID `AM-OCAF-AI-0012` · katalog `AM-OCAF-ARC-012` · certyfikat `CERT-AM-2026-000012` (**rezerwacja; niewydany**).

## Wejście
Nazwa wywołania: `archive.audit_zip`.

```json
{"archive_hex":"<HEX OF ZIP BYTES>"}
```

## Wyjście
`AuditReport { source_zip_sha256, status, member_count, total_uncompressed_bytes, duplicate_content_hashes, files[], privacy_names_included=false, semantic_review_performed=false, archive_extracted=false }`.

## Implementacja
Hash SHA-256 całego ZIP i rozpakowanej treści każdego pliku, lista rozszerzeń i rozmiarów, wykrywanie zduplikowanej treści, blokada traversal i niebezpiecznych ścieżek, case-insensitive duplikatów, dowolnych linków symbolicznych, nieobsługiwanych zaszyfrowanych elementów i bomb kompresyjnych.

Oryginał Python ogranicza ZIP do 500 członków, 750 MiB na element, 3 GiB łącznie. Port w Rust działa wyłącznie na bajtach już w pamięci i dodatkowo ogranicza źródłowy ZIP do 16 MiB, plik do 64 MiB oraz łącznie do 256 MiB. Do lokalnego OCAF dopuszcza tylko 1 MiB binarnego archiwum przesyłanego jako hex. Zabezpieczenie ścieżek Rust odrzuca nie-ASCII zamiast stosować Unicode casefold, co jest celową rozbieżnością.

## Niezrealizowane funkcje
ekstrakcja/analiza znaczenia dokumentów, deduplikacja semantyczna, obsługa dużych strumieniowych archiwów 3GiB, szyfrowane ZIP, pełny Unicode casefold, zewnętrzny podpisany raport. Produkcyjne OMAs/OMAt, Zenoh z TLS i OSTc pozostają osobnymi zadaniami. Wyniki analizy danych przekazanych przez użytkownika nie potwierdzają stanu realnego środowiska ani certyfikacji.

## Oferta i cennik
Ceny jednostkowej i rynkowej **nie zatwierdzono**; umowy mają stan DRAFT, bez autentycznych stron, polis, SLA i podpisów. Certyfikat `CERT-AM-2026-000012` zarezerwowany, **niewydany**.
