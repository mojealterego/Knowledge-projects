# Instrukcja obsługi — LIMES GIT

**Nazwa:** LIMES GIT · **Ewidencja:** 011 · **Serial:** `AM-GIT-LIM-2026-000011` · **Agent ID:** `AM-OCAF-AI-0011` · **Katalog:** `AM-OCAF-GIT-011` · **Certyfikat:** `CERT-AM-2026-0011` — NIEWYDANY.

1. Użyj Rust stable i źródeł `mojealterego/Knowledge-projects`. Nie wpisuj tokenów ani sekretów do demo.
2. Uruchom polecenia `cargo fmt`, `cargo test --all-features` i `cargo clippy --all-targets --all-features -- -D warnings` według [README](README.md).
3. Wprowadź dane dla `github.scope_review` przez lokalny OCAF `AgentTool`, a nie przez nieautoryzowany zdalny serwer.
4. Oceń wynik, pamiętając: **Wynik permitted=true oznacza jedynie lokalny preflight. Agent nie wykonuje połączeń GitHub, nie ma tokenu, nie podpisuje merge, nie autoryzuje wywołania; execution_authorized=false i api_called=false.**
5. Przy odmowie zachowaj numer zadania i konkretne `reasons`, bez sekretów i danych osobowych. Niezgodny serial → `InvalidConfiguration`; źródła OSTc → `SourceEncryptionNotImplemented`; żądanie Active → `DeploymentNotAuthorized`.

## Przykładowe wywołanie lokalne

```sh
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-011/Cargo.toml" -- demo
```

Uruchomienie nie kontaktuje się z Internetem i nie wykonuje zmian w GitHub.

## Dane, bezpieczeństwo i odpowiedzialność

Odczyt innych repozytoriów jest dopuszczony; zapis wyłącznie do mojealterego/Knowledge-projects przez roboczą gałąź knowledge/, policy/, feature/ lub fix/. PR i merge wymagają pull_base=main i poprawnego pull_head. Bezpośredni zapis do main jest blokowany. To tylko przegląd danych zadeklarowanych przez wywołującego. Zewnętrzne zasoby wymagają osobnej zgody i poświadczeń operatora. Ograniczenia i tryb serwisowy: `SECURITY.md`, `SERVICE.md`, `RELEASE_CHECKLIST.md`.
