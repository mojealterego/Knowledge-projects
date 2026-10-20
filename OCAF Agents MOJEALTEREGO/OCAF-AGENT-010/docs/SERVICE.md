# Obsługa serwisowa — ZIP WARDEN

**ZIP WARDEN** · ewidencja **010** · serial `AM-ARC-ZWD-2026-000010` · Agent ID `AM-OCAF-AI-0010` · katalog `AM-OCAF-ARC-010` · certyfikat `CERT-AM-2026-000010` (**rezerwacja; niewydany**).

1. Porównaj niezmienniki identyfikacji `AM-ARC-ZWD-2026-000010`, 010, `AM-OCAF-AI-0010` w centralnym rejestrze, TOML i manifeście.
2. Uruchom Rust unit/integration/security, formatowanie, Clippy i źródłowy Python unittest z SHA.
3. Skontroluj, że transport Zenoh `network_enabled=false`, dostęp OSTc `enabled=false`, lifecycle Active DENIED.
4. Potwierdź brak path w modelowym JSON i brak ekstrakcji plików, również przy błędnych i skompresowanych ZIP.
5. Zapisuj tylko zanonimizowane identyfikatory incydentów, sha commita i run ID; nie utrwalaj jawnych sekretów i nazw archiwów.
6. Nie zatwierdzaj wydania ani sprzedaży przed signed build, SBOM, audytem i operator acceptance.

Nie ma produkcyjnego rollback, SLA, revocation PKI, 24/7 support ani systemu zarządzania certyfikatami. Certyfikat nadal niewydany.
