# Serwis i lifecycle — ARGUS MOBILE

**Nazwa własna:** ARGUS MOBILE · **Ewidencja:** 005 · **Numer seryjny:** `AM-MOB-ARG-2026-000005` · **Agent ID:** `AM-OCAF-AI-0005` · **Katalog:** `AM-OCAF-MOB-005` · **Nr certyfikatu:** `CERT-AM-2026-0005` (rezerwacja, niewydany).

## Kontrole serwisowe

1. Zweryfikuj zgodność z globalnym `AGENT_IDENTITY_REGISTRY.json`: nazwa, ewidencja, ID, serial i katalog.
2. Odpal pełne `cargo fmt --check`, `cargo test --all-features`, `cargo clippy -D warnings` i CLI smoke.
3. Upewnij się, że lokalna konfiguracja nie uruchamia network ani source access OSTc.
4. `Draft → Tested → Staged` nie tworzy uprawnień; `Active` jest zawsze zabronione.
5. W trybie incydentu zbierz nieosobowe dowody z logów, identyfikator commita oraz run ID; nie kopiuj sekretów.
6. Zatrzymaj dopuszczenie wydania przy niezgodnej nazwie, serialu, braku podpisanego artefaktu lub złamaniu polityki.

## Rollback i wycofanie

Wersja jest odtwarzana po konkretnym SHA źródeł. Formalne odwołanie certyfikatu, wycofanie tożsamości OMAs/OMAt i OSTc nie jest zaimplementowane, ponieważ nie wydano ani kluczy, ani certyfikatu. Nie ma zadeklarowanego SLA, RTO/RPO, gwarancji 24/7 czy polisy.
