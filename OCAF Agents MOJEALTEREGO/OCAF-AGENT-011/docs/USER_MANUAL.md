# Instrukcja użytkownika — PROVENANT EVIDENCE

**PROVENANT EVIDENCE** · ewidencja **011** · serial `AM-PRV-EVD-2026-000011` · Agent ID `AM-OCAF-AI-0011` · katalog `AM-OCAF-PRV-011` · certyfikat `CERT-AM-2026-000011` (**rezerwacja; niewydany**).

## Pierwsze uruchomienie
Z katalogu checkout `Knowledge-projects`, po instalacji Rust stable, sprawdź pakiet:
```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-011/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-011/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-011/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-011/Cargo.toml" -- demo
```

CLI `demo` używa wyłącznie danych testowych, nie skanuje dysku/OSINT, nie wykonuje sieci ani zmian w repozytorium.

## Lokalny OCAF
`OcafRuntime::register(zenoh::manifest(), AgentTool)` dla `AgentTool::new(trusted_verifiers)`.

Wywołanie `evidence.review_provenance`:
```json
{"claim":"DNS returned address","origin":"observed","scope_id":"case123","source_ref":"res:primary","observed_at":"2026-10-20T10:00:00Z","source_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}
```

Niezależny rejestr weryfikatorów musi być dostarczony przez operatora do instancji AgentTool. Sam ciąg znaków verifier_ref ani deklaracja pochodzenia w JSON nie stanowi dowodu prawdziwości. Etykieta WERYFIKACJA UDOKUMENTOWANA nadal wymaga przejrzenia faktycznych dowodów.

Nie przekazuj sekretów, haseł, lokalizacji osób ani wrażliwych danych do testów. Wiadomość o odmowie oznacza celowe zabezpieczenie, nie wymóg obejścia polityki.

## Diagnostyka
Niezgodność serialu / TOML → `InvalidConfiguration`. Niewłaściwe archiwum/dowody → `ToolError::Validation`. Źródła OSTc → `SourceEncryptionNotImplemented`. Aktywacja → `DeploymentNotAuthorized`. Zachowaj commit i run ID testów, nie przepisuj danych wrażliwych do logów.
