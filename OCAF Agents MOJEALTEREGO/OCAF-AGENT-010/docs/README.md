# VERITAS TRACE — OCAF-AGENT-010

**Nazwa:** VERITAS TRACE · **Ewidencja:** 010 · **Serial:** `AM-OSN-VRT-2026-000010` · **Agent ID:** `AM-OCAF-AI-0010` · **Katalog:** `AM-OCAF-OSN-010` · **Certyfikat:** `CERT-AM-2026-0010` — NIEWYDANY.

**Status `INCOMPLETE_DEVELOPMENT` — `production_approved=false`, `certificate_issued=false`.**

## Cel, pochodzenie i ograniczenia

Lokalna walidacja metryki pochodzenia dowodu OSINT i prawidłowe oznaczanie treści symulowanych, obserwowanych oraz deklarowanych jako zweryfikowane. Rzeczywisty kod źródłowy: [`tools/osint_provenance_gate.py`](../../../tools/osint_provenance_gate.py); jego oryginalne testy: [`tools/test_osint_provenance_gate.py`](../../../tools/test_osint_provenance_gate.py). Źródła Python pozostają w repo bez modyfikacji.

Weryfikacja wymagalności scope_id, source_ref, observed_at, sha256 dla observed/verified, verifier_ref dla verified; status HTTP nigdy nie dowodzi istnienia konta.

**Granica bezpieczeństwa:** Oznaczenie verified opisuje jedynie przekazaną dokumentację; brak niezależnego poświadczenia źródła, stąd independently_verified=false i source_contacted=false.

## Uruchomienie

```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-010/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-010/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-010/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-010/Cargo.toml" -- demo
```

To lokalny kod Rust, a nie faktycznie działające serwery OMAs/OMAt. Zenoh udostępnia w tej wersji jedynie kontrolowane *key expressions*, bez zdalnej sesji. OSTc ma `source_access.enabled=false` i odmowę dostępu do źródeł.

## Dokumentacja

[ARCHITECTURE](ARCHITECTURE.md) · [SPECIFICATION](SPECIFICATION.md) · [USER_MANUAL](USER_MANUAL.md) · [SECURITY](SECURITY.md) · [SERVICE](SERVICE.md) · [IDENTITY_AND_SERIAL](IDENTITY_AND_SERIAL.md) · [PYTHON_PARITY](PYTHON_PARITY.md) · [MIGRATION](MIGRATION.md) · [DEPENDENCIES](DEPENDENCIES.md) · [TRACEABILITY_MATRIX](TRACEABILITY_MATRIX.md) · [RELEASE_CHECKLIST](RELEASE_CHECKLIST.md) · [TEST_EVIDENCE](TEST_EVIDENCE.md) · [CHANGELOG](CHANGELOG.md) · [CERTIFICATE_RECORD](../certification/CERTIFICATE_RECORD.md), [SALE-DRAFT](commercial/SALE-DRAFT.md), [LEASE-DRAFT](commercial/LEASE-DRAFT.md), [RENT-DRAFT](commercial/RENT-DRAFT.md).
