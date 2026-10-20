# Instrukcja użytkownika — VIGIA LOOP

**VIGIA LOOP** · nr ewidencyjny **007** · nr seryjny `AM-OPS-VIG-2026-000007` · Agent ID `AM-OCAF-AI-0007` · katalog `AM-OCAF-OPS-007` · certyfikat `CERT-AM-2026-0007` — **NIEWYDANY**.

## Uruchomienie offline

W checkout `mojealterego/Knowledge-projects` z dostępnym Rust stable:

```sh
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-007/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-007/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-007/Cargo.toml" -- demo
```

Testy nie ładują sekretów i nie uruchamiają zewnętrznych narzędzi. CLI drukuje status demonstracji.

## Wywołanie przez lokalny OCAF

Zarejestruj `zenoh::manifest()` i `AgentTool` w `OcafRuntime`. Schemat JSON:

```json
{"tool":"read_file","args_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","estimated_tokens":1}
```

Zewnętrzny host tworzy jeden AgentTool na sesję z Limits, obserwuje wywołania bez resetowania guard przy każdym żądaniu i dostarcza postęp WYŁĄCZNIE po autorytatywnym readback. W żadnym modelowym JSON nie może pojawić się trusted_progress_version. Wynik allowed oznacza tylko zaliczenie bramki limitu, a nie prawo do wykonania.

## Błędy i serwis

Modyfikacja TOML/serialu: `InvalidConfiguration`; błędny payload: `ToolError::Validation`; brak OSTc: `SourceEncryptionNotImplemented`; próba uruchomienia Active: `DeploymentNotAuthorized`. Zaloguj SHA, numer job, nie zapisuj sekretów ani danych wrażliwych. Brak działania network/revoke w tej wersji.
