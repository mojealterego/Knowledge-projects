# Obsługa serwisowa — PROVENANT EVIDENCE

**PROVENANT EVIDENCE** · ewidencja **011** · serial `AM-PRV-EVD-2026-000011` · Agent ID `AM-OCAF-AI-0011` · katalog `AM-OCAF-PRV-011` · certyfikat `CERT-AM-2026-000011` (**rezerwacja; niewydany**).

1. Porównaj niezmienniki identyfikacji `AM-PRV-EVD-2026-000011`, 011, `AM-OCAF-AI-0011` w centralnym rejestrze, TOML i manifeście.
2. Uruchom Rust unit/integration/security, formatowanie, Clippy i źródłowy Python unittest z SHA.
3. Skontroluj, że transport Zenoh `network_enabled=false`, dostęp OSTc `enabled=false`, lifecycle Active DENIED.
4. Zweryfikuj źródło host-owned rejestru weryfikatorów; bez niezależnego wpisu Verified musi zostać odrzucone.
5. Zapisuj tylko zanonimizowane identyfikatory incydentów, sha commita i run ID; nie utrwalaj jawnych sekretów i nazw archiwów.
6. Nie zatwierdzaj wydania ani sprzedaży przed signed build, SBOM, audytem i operator acceptance.

Nie ma produkcyjnego rollback, SLA, revocation PKI, 24/7 support ani systemu zarządzania certyfikatami. Certyfikat nadal niewydany.
