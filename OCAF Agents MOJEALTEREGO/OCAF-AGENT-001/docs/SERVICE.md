# Serwis, utrzymanie i cykl życia — WDA AURELION

## Metryka
Ewidencja **001**; serial **AM-WDA-AUR-2026-000001-DE338AC20B42**; techniczne ID `AM-OCAF-AI-0001`. Egzemplarz w fazie DEVELOPMENT, certyfikat niewydany. Wsparcie handlowe 24/7 / ubezpieczenie / SLA: **niezamówione i niezatwierdzone**.

## Stany implementacji
`src/lifecycle.rs` definiuje `Draft`, `Validated`, `Tested`, `Staged`, `Active`, `Paused`, `Retired`. Dozwolone przejścia lokalne:
```text
Draft -> Validated -> Tested -> Staged
  |           |         |       |
  +-----------+---------+-------+--> Retired
Staged -X-> Active (ZABLOKOWANE)
Active -> Paused -> Active (gdyby serwer dopuścił aktywację; dziś niemożliwe)
```
Pola `ReleaseEvidence` są danymi przekazanymi przez klienta i **nie mogą poświadczać poprawności**. `Active` zawsze odrzucane; w rzeczywistym produkcyjnym lifecycle wymagany niezależny podpis i potwierdzone artefakty.

## Operacje testowe
1. Odczytaj z rejestru `AGENT_IDENTITY_REGISTRY.json` nazwę i serial; porównaj z TOML i manifestem.
2. Uruchom Rust fmt, testy, Clippy i CLI smoke według `USER_MANUAL.md`.
3. Prowadź ewidencję SHA commita, CI run ID oraz wyników negatywnych testów `source_access`/auth.
4. Nie aktywuj transportu Zenoh i nie podłączaj OMAs/OMAt bez formalnej polityki zaufania.
5. Przy awarii zachowaj logi bez sekretów; oddziel fałszywe żądania od rzeczywistych błędów domeny.
6. Przerwij publikację, jeśli certyfikat jest niewydany, artifact digest nieznany lub numer seryjny niezgodny.

## Aktualizacje i rollback
Zmiany kodu przez PR, testy i przegląd; nie zmieniaj serialu bez wpisu migracyjnego. Cofnięcie do konkretnego SHA powinno być zatwierdzone przez operatora; nie uruchamiano testów data-migration/rollback na niezależnych hostach. Historyczny JS jest nieutwardzony i **nie stanowi produkcyjnego fallbacku**.

## Revoke / retire
`Retired` jest stanem modelu lokalnego, nie rzeczywistym odebraniem klucza OMAs czy uprawnienia OMAt. Po wdrożeniu polityk wymaga potwierdzonego revoke we wszystkich serwerach, wygaszenia tokenów, protokołu retencji, protokołu niszczenia/bezpiecznego eksportu danych i niezależnego audytu.

## Dokumenty i kryteria
`certification/CERTIFICATE_RECORD.md`, `docs/TEST_EVIDENCE.md`, `docs/RELEASE_CHECKLIST.md`, `docs/SECURITY.md`. Nie przyznawać statusu `PRODUCTION APPROVED` bez wszystkich bramek.
