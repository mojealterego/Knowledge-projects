# PROVENANT EVIDENCE — OCAF-AGENT-011

**PROVENANT EVIDENCE** · ewidencja **011** · serial `AM-PRV-EVD-2026-000011` · Agent ID `AM-OCAF-AI-0011` · katalog `AM-OCAF-PRV-011` · certyfikat `CERT-AM-2026-000011` (**rezerwacja; niewydany**).

**Status produktu:** `INCOMPLETE_DEVELOPMENT` · `certificate_issued=false` · `production_approved=false`.

## Faktyczna funkcja
etykietowanie pochodzenia i statusu dowodów bez samodzielnego OSINT ani wyszukiwania kont osób. Port istniejącego [`tools/osint_provenance_gate.py`](../../../tools/osint_provenance_gate.py) z porównaniem oryginalnych [testów Python](../../../tools/test_osint_provenance_gate.py). Zachowuje odrębne etykiety simulated/observed/verified; observed wymaga scope_id, source_ref, observed_at oraz hash SHA-256; verified dodatkowo wymaga wiarygodnego identyfikatora weryfikatora. Status odpowiedzi HTTP nigdy nie świadczy o istnieniu konkretnej osoby lub konta.

## Granica działania
Niezależny rejestr weryfikatorów musi być dostarczony przez operatora do instancji AgentTool. Sam ciąg znaków verifier_ref ani deklaracja pochodzenia w JSON nie stanowi dowodu prawdziwości. Etykieta WERYFIKACJA UDOKUMENTOWANA nadal wymaga przejrzenia faktycznych dowodów.

**Zależności i odstępstwa od oryginału:** W Pythonie do utworzenia verified wystarczało niepuste verifier_ref. Port Rust wymaga dodatkowo, by identyfikator należał do trusted_verifiers dostarczanych poza żądaniem przez hosta. Waliduje rozmiar claim oraz pól odniesienia, a oddzielny adapter OCAF limituje JSON do 8192 bajtów. To wzmocniona, nie bitowo równoważna wersja modułu Python.

## Lokalny test
```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-011/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-011/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-011/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-011/Cargo.toml" -- demo
```

Adapter `evidence.review_provenance` rejestruje tylko lokalnego `ToolExecutor`. OMAs — osobny serwer agentów, OMAt — osobny serwer narzędzi, Zenoh — docelowy transport. **Żadna usługa sieciowa nie jest włączona**. Dostęp do kodu źródłowego OSTc celowo zablokowany do uzyskania specyfikacji i audytu.

## Dokumenty własne
[ARCHITECTURE](ARCHITECTURE.md) · [SPECIFICATION](SPECIFICATION.md) · [USER_MANUAL](USER_MANUAL.md) · [SECURITY](SECURITY.md) · [SERVICE](SERVICE.md) · [IDENTITY_AND_SERIAL](IDENTITY_AND_SERIAL.md) · [PYTHON_PARITY](PYTHON_PARITY.md) · [DEPENDENCIES](DEPENDENCIES.md) · [MIGRATION](MIGRATION.md) · [TRACEABILITY_MATRIX](TRACEABILITY_MATRIX.md) · [TEST_EVIDENCE](TEST_EVIDENCE.md) · [RELEASE_CHECKLIST](RELEASE_CHECKLIST.md) · [CHANGELOG](CHANGELOG.md) · [CERTIFICATE_RECORD](../certification/CERTIFICATE_RECORD.md) · [SALE](commercial/SALE-DRAFT.md) · [LEASE](commercial/LEASE-DRAFT.md) · [RENT](commercial/RENT-DRAFT.md).

**Niezrealizowane:** samodzielna identyfikacja źródeł w sieci, rozpoznawanie prawdziwości treści, signed verifier receipts, E2E OSINT, runtime do pozyskiwania danych, formalny certyfikat źródła. Nie przedstawiać tej wersji jako kompletnego autonomicznego agenta ani certyfikowanego produktu.
