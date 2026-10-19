# Changelog

## 0.2.0-dev (2026-10-19)
- Nowy kanoniczny `OCAF-AGENT-001/`; 27 starych blob SHA zachowanych, manifest zaktualizowany do nowej architektury; historyczny JS zachowany 8/8.
- Główny crate Rust i moduły `agent.rs`, `zenoh.rs`, `security.rs`, `lifecycle.rs`; testy w `tests/integration.rs` i `tests/security.rs`.
- OMAs = osobny serwer agentów, OMAt = osobny serwer narzędzi; adresowanie Zenoh jako kontrakt lokalny; sieciowe integracje nadal nieuruchomione.
- OSTc dla zaszyfrowanego dostępu źródeł **PENDING**; dostęp zablokowany.
- `ACTIVE` niedostępny nawet przy sfałszowanych booleanach poświadczeń.
- [CI SUCCESS, 12/12 testów](https://github.com/mojealterego/Knowledge-projects/actions/runs/39203233792).

## 0.1.0-dev (2026-10-19)
- Nadano numer ewidencyjny 001, wyodrębniono WDA z P116; Rust plan-only i adapter OCAF.
- Produkcyjne podpisanie i formalny odbiór nadal niewykonane.
