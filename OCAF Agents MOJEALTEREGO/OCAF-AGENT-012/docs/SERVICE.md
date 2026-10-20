# Obsługa serwisowa — ZIP WARDEN

**ZIP WARDEN** · ewidencja **012** · serial `AM-ARC-ZWD-2026-000012` · Agent ID `AM-OCAF-AI-0012` · katalog `AM-OCAF-ARC-012` · certyfikat `CERT-AM-2026-000012` (**rezerwacja; niewydany**).

1. Porównaj niezmienniki identyfikacji `AM-ARC-ZWD-2026-000012`, 012, `AM-OCAF-AI-0012` w centralnym rejestrze, TOML i manifeście.
2. Uruchom Rust unit/integration/security, formatowanie, Clippy i źródłowy Python unittest z SHA.
3. Skontroluj, że transport Zenoh `network_enabled=false`, dostęp OSTc `enabled=false`, lifecycle Active DENIED.
4. Potwierdź brak path w modelowym JSON i brak ekstrakcji plików, również przy błędnych i skompresowanych ZIP.
5. Zapisuj tylko zanonimizowane identyfikatory incydentów, sha commita i run ID; nie utrwalaj jawnych sekretów i nazw archiwów.
6. Nie zatwierdzaj wydania ani sprzedaży przed signed build, SBOM, audytem i operator acceptance.

Nie ma produkcyjnego rollback, SLA, revocation PKI, 24/7 support ani systemu zarządzania certyfikatami. Certyfikat nadal niewydany.
