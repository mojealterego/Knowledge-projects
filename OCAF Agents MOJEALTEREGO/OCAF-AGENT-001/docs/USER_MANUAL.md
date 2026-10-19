# Instrukcja obsługi — WDA AURELION (Agent 001)

**Identyfikacja:** `WDA AURELION`, serial `AM-WDA-AUR-2026-000001-DE338AC20B42`, Agent ID `AM-OCAF-AI-0001`. Aktualny tryb: wyłącznie PLAN-ONLY.

## 1. Przygotowanie
Wymagany jest host z Rust stable i Git. Nie należy używać środowiska produkcyjnego ani podpinać sekretów. Z repozytorium wybierz zweryfikowany commit i przejdź do katalogu projektu `Knowledge-projects`. Upewnij się, że `config/agent.toml` nie ma aktywnej sieci ani źródeł.

## 2. Test uruchomienia
```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-001/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-001/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-001/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-001/Cargo.toml" -- "adjust highlights"
```
CLI identyfikuje agenta nazwą i serialem oraz zwraca pięć tekstowych zaleceń. **Nie przetwarza obrazu**.

## 3. Wywołanie OCAF
Zaufany host testowy rejestruje `zenoh::manifest()` oraz `WdaPlanExecutor` pod nazwą `photo.plan`. Payload: `{"goal":"adjust highlights","identity_lock":true}`. Wynik to JSON `status="plan_only"`, `identity_lock=true` i `workflow`. Pola inne niż `goal`, `identity_lock` powodują odmowę. Przestrzenie logiczne `omas/v1/` i `omat/v1/` **nie są działającymi serwerami**.

## 4. Błędy i odmowy
- `IdentityUnlockDenied` — zadano `identity_lock=false`; żądanie odrzucone.
- `InvalidGoal` — cel pusty, ma znaki sterujące lub przekracza limit 1024 bajtów.
- `InvalidConfiguration` — zmieniono tożsamość, nazwę, serial, sieć, ustawienia OSTc albo parametry bezpieczeństwa.
- `SourceEncryptionNotImplemented` — nie udostępniamy źródeł przed implementacją OSTc.
- `ReleaseGateNotSatisfied` — aktywacja nadal niemożliwa, nawet z zadeklarowanymi pozytywnymi flagami.

## 5. Zasady operatora
Wstrzymaj wdrożenie, jeżeli serial nie zgadza się z rejestrem, manifestem i TOML. Zachowuj commit SHA, identyfikator CI i logi bez sekretów. Aktualizacje identyfikatorów wymagają formalnego wpisu w rejestrze; nie używać serialu jako danych uwierzytelniających. Nie uruchamiać legacy PWA publicznie.

## 6. Wsparcie, incydenty, wycofanie
Przy wykryciu nieautoryzowanego dostępu zatrzymaj ewentualny host, zablokuj zewnętrzne integracje i dokumentuj zdarzenie. Revoke/retire w tym wydaniu są wyłącznie deklaracjami lokalnego lifecycle, a nie rzeczywistym unieważnieniem w OMAs. Zobacz `SERVICE.md`, `SECURITY.md` i `RELEASE_CHECKLIST.md`.

## 7. Ograniczenia
W obecnej wersji nie ma Adobe Firefly, OpenAI vision, edycji obrazu, klientów mobilnych Rust, signed OMAs/OMAt RPC ani OSTc. Pełna obsługa fotograficzna i wydanie certyfikatu są jeszcze **NIEUKOŃCZONE**.
