# Lifecycle i serwis — FIDES GRANTIS

**FIDES GRANTIS** · ewidencja **012** · serial `AM-GRT-FID-2026-000012` · Agent ID `AM-OCAF-AI-0012` · katalog `AM-OCAF-GRT-012` · certyfikat `CERT-AM-2026-0012` **NIEWYDANY**.

1. Odczytaj identyfikację w TOML, manifest, stałe Rust i globalny rejestr; wartości muszą być identyczne.
2. Uruchom testy jednostkowe, OCAF integration, original Python unittest, fmt, strict clippy, CLI; zachowaj SHA/run ID.
3. Upewnij się, że `network_enabled=false` oraz `source_access.enabled=false`; `Active` zawsze denied.
4. Przy incydencie zatrzymaj użycie lokalnego adaptera, zanotuj wyjściowe `reasons[]` i commit, nie kopiuj credentiali.
5. Zmianę polityki rozpocznij od testów RED, następnie implementacja i niezależny review.

Nie są wdrożone serwerowa revocation, certyfikacja, RTO/RPO, SLA, monitoring multi-host ani kryptograficzna wymiana kluczy OSTc.
