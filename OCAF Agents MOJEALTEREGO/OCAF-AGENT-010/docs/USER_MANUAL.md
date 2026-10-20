# Instrukcja obsługi — VERITAS TRACE

**Nazwa:** VERITAS TRACE · **Ewidencja:** 010 · **Serial:** `AM-OSN-VRT-2026-000010` · **Agent ID:** `AM-OCAF-AI-0010` · **Katalog:** `AM-OCAF-OSN-010` · **Certyfikat:** `CERT-AM-2026-0010` — NIEWYDANY.

1. Użyj Rust stable i źródeł `mojealterego/Knowledge-projects`. Nie wpisuj tokenów ani sekretów do demo.
2. Uruchom polecenia `cargo fmt`, `cargo test --all-features` i `cargo clippy --all-targets --all-features -- -D warnings` według [README](README.md).
3. Wprowadź dane dla `osint.provenance_record` przez lokalny OCAF `AgentTool`, a nie przez nieautoryzowany zdalny serwer.
4. Oceń wynik, pamiętając: **Oznaczenie verified opisuje jedynie przekazaną dokumentację; brak niezależnego poświadczenia źródła, stąd independently_verified=false i source_contacted=false.**
5. Przy odmowie zachowaj numer zadania i konkretne `reasons`, bez sekretów i danych osobowych. Niezgodny serial → `InvalidConfiguration`; źródła OSTc → `SourceEncryptionNotImplemented`; żądanie Active → `DeploymentNotAuthorized`.

## Przykładowe wywołanie lokalne

```sh
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-010/Cargo.toml" -- demo
```

Uruchomienie nie kontaktuje się z Internetem i nie wykonuje zmian w GitHub.

## Dane, bezpieczeństwo i odpowiedzialność

Weryfikacja wymagalności scope_id, source_ref, observed_at, sha256 dla observed/verified, verifier_ref dla verified; status HTTP nigdy nie dowodzi istnienia konta. To tylko przegląd danych zadeklarowanych przez wywołującego. Zewnętrzne zasoby wymagają osobnej zgody i poświadczeń operatora. Ograniczenia i tryb serwisowy: `SECURITY.md`, `SERVICE.md`, `RELEASE_CHECKLIST.md`.
