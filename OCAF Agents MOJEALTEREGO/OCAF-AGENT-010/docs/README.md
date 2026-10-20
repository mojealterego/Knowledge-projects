# ZIP WARDEN — OCAF-AGENT-010

**ZIP WARDEN** · ewidencja **010** · serial `AM-ARC-ZWD-2026-000010` · Agent ID `AM-OCAF-AI-0010` · katalog `AM-OCAF-ARC-010` · certyfikat `CERT-AM-2026-000010` (**rezerwacja; niewydany**).

**Status produktu:** `INCOMPLETE_DEVELOPMENT` · `certificate_issued=false` · `production_approved=false`.

## Faktyczna funkcja
inwentaryzacja archiwów ZIP bez ekstrakcji i bez ujawniania nazw w odpowiedziach OCAF. Port istniejącego [`tools/source_bundle_audit.py`](../../../tools/source_bundle_audit.py) z porównaniem oryginalnych [testów Python](../../../tools/test_source_bundle_audit.py). Hash SHA-256 całego ZIP i rozpakowanej treści każdego pliku, lista rozszerzeń i rozmiarów, wykrywanie zduplikowanej treści, blokada traversal i niebezpiecznych ścieżek, case-insensitive duplikatów, dowolnych linków symbolicznych, nieobsługiwanych zaszyfrowanych elementów i bomb kompresyjnych.

## Granica działania
W wersji OCAF nazwy archiwalnych plików nigdy nie są ujawniane. Flagę include_names=false wybiera host na sztywno, a klient nie może włączyć jej w JSON. Wynik oznacza jedynie INVENTORIED_NOT_SEMANTICALLY_REVIEWED.

**Zależności i odstępstwa od oryginału:** Oryginał Python ogranicza ZIP do 500 członków, 750 MiB na element, 3 GiB łącznie. Port w Rust działa wyłącznie na bajtach już w pamięci i dodatkowo ogranicza źródłowy ZIP do 16 MiB, plik do 64 MiB oraz łącznie do 256 MiB. Do lokalnego OCAF dopuszcza tylko 1 MiB binarnego archiwum przesyłanego jako hex. Zabezpieczenie ścieżek Rust odrzuca nie-ASCII zamiast stosować Unicode casefold, co jest celową rozbieżnością.

## Lokalny test
```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-010/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-010/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-010/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-010/Cargo.toml" -- demo
```

Adapter `archive.audit_zip` rejestruje tylko lokalnego `ToolExecutor`. OMAs — osobny serwer agentów, OMAt — osobny serwer narzędzi, Zenoh — docelowy transport. **Żadna usługa sieciowa nie jest włączona**. Dostęp do kodu źródłowego OSTc celowo zablokowany do uzyskania specyfikacji i audytu.

## Dokumenty własne
[ARCHITECTURE](ARCHITECTURE.md) · [SPECIFICATION](SPECIFICATION.md) · [USER_MANUAL](USER_MANUAL.md) · [SECURITY](SECURITY.md) · [SERVICE](SERVICE.md) · [IDENTITY_AND_SERIAL](IDENTITY_AND_SERIAL.md) · [PYTHON_PARITY](PYTHON_PARITY.md) · [DEPENDENCIES](DEPENDENCIES.md) · [MIGRATION](MIGRATION.md) · [TRACEABILITY_MATRIX](TRACEABILITY_MATRIX.md) · [TEST_EVIDENCE](TEST_EVIDENCE.md) · [RELEASE_CHECKLIST](RELEASE_CHECKLIST.md) · [CHANGELOG](CHANGELOG.md) · [CERTIFICATE_RECORD](../certification/CERTIFICATE_RECORD.md) · [SALE](commercial/SALE-DRAFT.md) · [LEASE](commercial/LEASE-DRAFT.md) · [RENT](commercial/RENT-DRAFT.md).

**Niezrealizowane:** ekstrakcja/analiza znaczenia dokumentów, deduplikacja semantyczna, obsługa dużych strumieniowych archiwów 3GiB, szyfrowane ZIP, pełny Unicode casefold, zewnętrzny podpisany raport. Nie przedstawiać tej wersji jako kompletnego autonomicznego agenta ani certyfikowanego produktu.
