# Instrukcja użytkownika — CUSTOS ADMISSION

**CUSTOS ADMISSION** · nr ewidencyjny **006** · nr seryjny `AM-SEC-CUS-2026-000006` · Agent ID `AM-OCAF-AI-0006` · katalog `AM-OCAF-SEC-006` · certyfikat `CERT-AM-2026-0006` — **NIEWYDANY**.

## Uruchomienie offline

W checkout `mojealterego/Knowledge-projects` z dostępnym Rust stable:

```sh
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-006/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-006/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-006/Cargo.toml" -- demo
```

Testy nie ładują sekretów i nie uruchamiają zewnętrznych narzędzi. CLI drukuje status demonstracji.

## Wywołanie przez lokalny OCAF

Zarejestruj `zenoh::manifest()` i `AgentTool` w `OcafRuntime`. Schemat JSON:

```json
{"manifest":{"artifact_sha256":"<64 hex chars>","capabilities":["parse_document"],"sandbox_profile":"isolated_no_network_read_only","timeout_seconds":30,"expires_at":"2026-10-22T00:00:00Z","external_side_effects":false},"artifact_hex":"<hex encoded bytes>"}
```

Zewnętrzny host przed konstrukcją AgentTool przekazuje niezależne zatwierdzenia mapowane SHA-256 → capability i czas UTC. NIE umieszczaj approvals w wiadomości modelu ani w manifeście. Wynik accepted oznacza WYŁĄCZNIE zgodność offline; nie wykonuje artefaktu.

## Błędy i serwis

Modyfikacja TOML/serialu: `InvalidConfiguration`; błędny payload: `ToolError::Validation`; brak OSTc: `SourceEncryptionNotImplemented`; próba uruchomienia Active: `DeploymentNotAuthorized`. Zaloguj SHA, numer job, nie zapisuj sekretów ani danych wrażliwych. Brak działania network/revoke w tej wersji.
