# Instrukcja użytkownika — ZIP WARDEN

**ZIP WARDEN** · ewidencja **010** · serial `AM-ARC-ZWD-2026-000010` · Agent ID `AM-OCAF-AI-0010` · katalog `AM-OCAF-ARC-010` · certyfikat `CERT-AM-2026-000010` (**rezerwacja; niewydany**).

## Pierwsze uruchomienie
Z katalogu checkout `Knowledge-projects`, po instalacji Rust stable, sprawdź pakiet:
```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-010/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-010/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-010/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-010/Cargo.toml" -- demo
```

CLI `demo` używa wyłącznie danych testowych, nie skanuje dysku/OSINT, nie wykonuje sieci ani zmian w repozytorium.

## Lokalny OCAF
`OcafRuntime::register(zenoh::manifest(), AgentTool)` dla jednostkowego adaptera bez nazw archiwalnych.

Wywołanie `archive.audit_zip`:
```json
{"archive_hex":"<HEX OF ZIP BYTES>"}
```

W wersji OCAF nazwy archiwalnych plików nigdy nie są ujawniane. Flagę include_names=false wybiera host na sztywno, a klient nie może włączyć jej w JSON. Wynik oznacza jedynie INVENTORIED_NOT_SEMANTICALLY_REVIEWED.

Nie przekazuj sekretów, haseł, lokalizacji osób ani wrażliwych danych do testów. Wiadomość o odmowie oznacza celowe zabezpieczenie, nie wymóg obejścia polityki.

## Diagnostyka
Niezgodność serialu / TOML → `InvalidConfiguration`. Niewłaściwe archiwum/dowody → `ToolError::Validation`. Źródła OSTc → `SourceEncryptionNotImplemented`. Aktywacja → `DeploymentNotAuthorized`. Zachowaj commit i run ID testów, nie przepisuj danych wrażliwych do logów.
