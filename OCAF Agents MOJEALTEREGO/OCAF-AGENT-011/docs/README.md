# LIMES GIT — OCAF-AGENT-011

**Nazwa:** LIMES GIT · **Ewidencja:** 011 · **Serial:** `AM-GIT-LIM-2026-000011` · **Agent ID:** `AM-OCAF-AI-0011` · **Katalog:** `AM-OCAF-GIT-011` · **Certyfikat:** `CERT-AM-2026-0011` — NIEWYDANY.

**Status `INCOMPLETE_DEVELOPMENT` — `production_approved=false`, `certificate_issued=false`.**

## Cel, pochodzenie i ograniczenia

Lokalne badanie operacji GitHub przed dopuszczeniem, z rozdzieleniem operacji odczytu od zapisu i rygorystycznym ograniczeniem zapisów do Knowledge-projects. Rzeczywisty kod źródłowy: [`tools/single_repository_scope_gate.py`](../../../tools/single_repository_scope_gate.py); jego oryginalne testy: [`tools/test_single_repository_scope_gate.py`](../../../tools/test_single_repository_scope_gate.py). Źródła Python pozostają w repo bez modyfikacji.

Odczyt innych repozytoriów jest dopuszczony; zapis wyłącznie do mojealterego/Knowledge-projects przez roboczą gałąź knowledge/, policy/, feature/ lub fix/. PR i merge wymagają pull_base=main i poprawnego pull_head. Bezpośredni zapis do main jest blokowany.

**Granica bezpieczeństwa:** Wynik permitted=true oznacza jedynie lokalny preflight. Agent nie wykonuje połączeń GitHub, nie ma tokenu, nie podpisuje merge, nie autoryzuje wywołania; execution_authorized=false i api_called=false.

## Uruchomienie

```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-011/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-011/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-011/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-011/Cargo.toml" -- demo
```

To lokalny kod Rust, a nie faktycznie działające serwery OMAs/OMAt. Zenoh udostępnia w tej wersji jedynie kontrolowane *key expressions*, bez zdalnej sesji. OSTc ma `source_access.enabled=false` i odmowę dostępu do źródeł.

## Dokumentacja

[ARCHITECTURE](ARCHITECTURE.md) · [SPECIFICATION](SPECIFICATION.md) · [USER_MANUAL](USER_MANUAL.md) · [SECURITY](SECURITY.md) · [SERVICE](SERVICE.md) · [IDENTITY_AND_SERIAL](IDENTITY_AND_SERIAL.md) · [PYTHON_PARITY](PYTHON_PARITY.md) · [MIGRATION](MIGRATION.md) · [DEPENDENCIES](DEPENDENCIES.md) · [TRACEABILITY_MATRIX](TRACEABILITY_MATRIX.md) · [RELEASE_CHECKLIST](RELEASE_CHECKLIST.md) · [TEST_EVIDENCE](TEST_EVIDENCE.md) · [CHANGELOG](CHANGELOG.md) · [CERTIFICATE_RECORD](../certification/CERTIFICATE_RECORD.md), [SALE-DRAFT](commercial/SALE-DRAFT.md), [LEASE-DRAFT](commercial/LEASE-DRAFT.md), [RENT-DRAFT](commercial/RENT-DRAFT.md).
