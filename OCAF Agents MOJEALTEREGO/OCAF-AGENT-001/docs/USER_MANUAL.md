# Instrukcja użytkownika / operatora — OCAF-AGENT-001

## Instalacja
1. Użyj zatwierdzonego SHA z repozytorium `Knowledge-projects`.
2. Sprawdź `cargo fmt`, `cargo test --all-features`, `cargo clippy --all-targets --all-features -D warnings`; w szczególności testy odmowy odblokowania tożsamości, rozdzielenia OMAs/OMAt i OSTc.
3. Nie podawaj kluczy API ani danych klienta do `legacy-pwa/`. Legacy jest zachowane do audytu; nie uruchamiaj publicznie.
4. `config/agent.toml` musi zachować `network_enabled=false` i `source_access.enabled=false`.

## Wykorzystanie
```sh
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-001/Cargo.toml" -- "adjust highlights"
```
Wynik to lokalny plan bez zmian w fotografii. Integracja narzędzia OCAF: `photo.plan` z JSON `{"goal":"adjust highlights","identity_lock":true}` wyłącznie w lokalnym/testowym host OCAF, nie udostępniaj go przez anonimowy endpoint.

## Role usług
**OMAs** — serwer agentów, **OMAt** — serwer narzędzi, **Zenoh** — warstwa komunikacyjna. Uwierzytelnianie po stronie autorytatywnego hosta, nie na podstawie adresu Zenoh.

## Obsługa incydentu
Przy naruszeniu autoryzacji zatrzymaj integrację hosta, zabezpiecz trace/correlation ID bez danych osobowych, odmów wznowienia przed analizą. Akcje płatne nie są włączone. Restart planera jest bezstanowy; nie implikuje recovery OMAs.

## Certyfikacja i szyfrowanie źródeł
`certification/CERTIFICATE_RECORD.md` jest rejestrem wewnętrznym, nie dokumentem podpisanym. OSTc pozostaje PENDING; bez pełnej specyfikacji i audytu dostęp agenta do plików źródłowych pozostaje zablokowany. Nie deklarować gotowości produkcyjnej.

## Materiały dodatkowe
`docs/SECURITY.md`, `docs/TEST_EVIDENCE.md`, `docs/MIGRATION.md`, `docs/commercial/`.
