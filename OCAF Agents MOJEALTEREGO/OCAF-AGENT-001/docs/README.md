# WDA AURELION — OCAF-AGENT-001

**Nazwa własna:** WDA AURELION · **Rodzina:** WDA Photo Agent · **Numer ewidencyjny:** 001  
**Agent ID:** `AM-OCAF-AI-0001` · **Numer seryjny:** `AM-WDA-AUR-2026-000001-DE338AC20B42`  
**Numer katalogowy:** `AM-OCAF-WDA-001` · **Certyfikat:** `CERT-AM-2026-0001` — NIEWYDANY

**Stan wydania: INCOMPLETE_DEVELOPMENT** · `production_approved=false` · OCAF/OMAs/OMAt pełny E2E: NOT VERIFIED · OSTc: PENDING SPECIFICATION.

## Cel i zakres
WDA AURELION jest własną nazwą agenta fotograficznego powiązanego z [projektem P116](../../../projekty/116-wda-photo-agent-max/README.md). Tożsamość egzemplarza jest trwale przypisana do wpisu 001, ale **001 nie jest nazwą ani numerem seryjnym**. Historyczne `WDA Photo Agent` oznacza rodzinę produktu.

**Działająca część:** lokalny deterministyczny planer obróbki zdjęć `photo.plan`, bez dostępu do zdjęć, sieci ani usług płatnych; walidacja `identity_lock`; Rust 2021; lokalny OCAF `ToolExecutor`; ograniczone trasy logiczne OMAs/OMAt; wyłączenie źródeł do czasu OSTc. Nie zaimplementowano rzeczywistej obróbki zdjęć w Rust ani uruchomienia serwerów.

## Architektura
- **OCAF Runtime:** framework wykonawczy, autoryzacja i kontrole hosta.
- **OMAs:** niezależny **serwer agentów**, docelowa kontrola tożsamości, cyklu życia i koordynacji.
- **OMAt:** niezależny **serwer narzędzi**, docelowo delegowane capabilities i autoryzacja narzędzi.
- **Zenoh:** transport, nie zastępuje OMAs/OMAt, uruchamianie sieciowe WYŁĄCZONE w agencie.
- **OSTc:** przyszła kryptografia plików źródłowych. Brak kompletnej specyfikacji i implementacji; dostęp DENIED.

## Użytkowanie aktualnej wersji
```sh
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-001/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-001/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-001/Cargo.toml" -- "adjust highlights"
```
Wynik: tylko tekstowy, odwracalny plan. Wywołanie `photo.plan` przyjmuje `goal` (niepusty UTF-8, max 1024 bajty) i `identity_lock=true`, odrzuca inne tryby. Nazwa własna i numer seryjny są weryfikowane przez konfigurację.

## Dokumentacja (obowiązująca)
| Dokument | Zawartość |
|---|---|
| [IDENTITY_AND_SERIAL](IDENTITY_AND_SERIAL.md) | rozdzielenie nazwy, seryjnego, katalogowego, ID i numeru certyfikatu |
| [ARCHITECTURE](ARCHITECTURE.md) | topologia i kontrakty OMAs/OMAt/Zenoh/OCAF/OSTc |
| [SPECIFICATION](SPECIFICATION.md) | zakres, warunki wejścia i wyjścia, ograniczenia |
| [USER_MANUAL](USER_MANUAL.md) | uruchomienie, komendy, reakcja na błędy |
| [SECURITY](SECURITY.md) | model zagrożeń i wymagania fail-closed |
| [SERVICE](SERVICE.md) | serwis, lifecycle, incydenty i wycofanie |
| [TRACEABILITY_MATRIX](TRACEABILITY_MATRIX.md) | wymóg → plik → test → status |
| [DEPENDENCIES](DEPENDENCIES.md) | zależności, ścieżka dostaw i SBOM |
| [RELEASE_CHECKLIST](RELEASE_CHECKLIST.md) | jawne blokady wydania i odbioru |
| [TEST_EVIDENCE](TEST_EVIDENCE.md) | dowody z CI i granice testów |
| [MIGRATION](MIGRATION.md) | przeniesienie źródeł P116 |
| [CHANGELOG](CHANGELOG.md) | wersje zmian |

Rejestr identyfikatorów: [AGENT_IDENTITY_REGISTRY.json](../../AGENT_IDENTITY_REGISTRY.json). Rejestr formalnego certyfikatu: [CERTIFICATE_RECORD.md](../certification/CERTIFICATE_RECORD.md). Osobne wzory handlowe: [SALE](commercial/SALE-DRAFT.md), [LEASE](commercial/LEASE-DRAFT.md), [RENT](commercial/RENT-DRAFT.md). Historyczne źródła: `legacy-pwa/`, zachowane jako materiał do bezpiecznej migracji.

**Dokumentacja nie poświadcza ukończenia produktu.** Nie wdrażać jako agent certyfikowany i nie wystawiać komercyjnie przed protokołem odbioru.
