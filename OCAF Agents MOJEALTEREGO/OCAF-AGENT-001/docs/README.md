# OCAF-AGENT-001 — WDA Photo Agent

**Ewidencja:** 001 · **Agent ID:** `AM-OCAF-AI-0001` · **Pochodzenie:** [P116](../../../projekty/116-wda-photo-agent-max/README.md).

**Status:** development, local plan-only tested historically. `production_approved=false`, certyfikat formalny niewydany.

## Docelowy podział usług

- **OCAF Runtime** — framework i autorytatywne środowisko wykonawcze.
- **OMAs** — odrębny **serwer agentów** (identyfikacja, cykl życia, komunikacja/koordynacja agentów).
- **OMAt** — odrębny **serwer narzędzi** (katalog narzędzi i egzekwowanie uprawnień).
- **Zenoh** — magistrala komunikacji; nie nadaje samodzielnie uprawnień i nie zastępuje OMAs/OMAt.
- **OSTc** — docelowa warstwa szyfrowanego dostępu każdego agenta do plików źródłowych. **Specyfikacja w toku, nieimplementowane**; dostęp do źródeł przez agenta zablokowany do niezależnej walidacji.

## Struktura

`Cargo.toml`, `src/{main,lib,agent,zenoh,security,lifecycle}.rs`, `tests/{integration,security}.rs`, `config/agent.toml`, `docs/{README,ARCHITECTURE,USER_MANUAL}.md`, `certification/CERTIFICATE_RECORD.md`. Dodatkowo zachowano całą historię, wzory umów i legacy PWA.

## Komendy

```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-001/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-001/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-001/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-001/Cargo.toml" -- "adjust highlights"
```

CLI tylko lokalnie planuje obróbkę. Brak operacji w Adobe/OpenAI, brak obsługi zdjęć w Rust, brak aktywnego listenera Zenoh; ten moduł weryfikuje trasowanie i implementuje `ToolExecutor` dla OCAF. **Nie deklarować integracji OMAs/OMAt po sieci ani produkcyjnego OSTc** na podstawie samej struktury.

## Indeks
[Architektura](ARCHITECTURE.md) · [Instrukcja użytkownika](USER_MANUAL.md) · [Bezpieczeństwo](SECURITY.md) · [Testy](TEST_EVIDENCE.md) · [Historia](MIGRATION.md) · [Rejestr certyfikatu](../certification/CERTIFICATE_RECORD.md) · [Umowy](commercial/).
