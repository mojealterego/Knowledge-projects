# Architektura — VERITAS TRACE

**Nazwa:** VERITAS TRACE · **Ewidencja:** 010 · **Serial:** `AM-OSN-VRT-2026-000010` · **Agent ID:** `AM-OCAF-AI-0010` · **Katalog:** `AM-OCAF-OSN-010` · **Certyfikat:** `CERT-AM-2026-0010` — NIEWYDANY.

```text
Host OCAF Runtime (lokalny)
    |
    +-- OMAs [serwer agentów: NOT DEPLOYED]
    +-- OMAt [serwer narzędzi: NOT DEPLOYED]
    +-- Zenoh [kontrolowane trasy omas/v1 i omat/v1, sieć OFF]
    +-- Rust ToolExecutor osint.provenance_record
    |        +-- src/agent.rs: provenance validation
    |        +-- immutable output: no external execution
    +-- OSTc [zaszyfrowany dostęp do źródeł: DENY]
```

`src/lib.rs` zabrania unsafe Rust; `agent.rs` implementuje domenę, `zenoh.rs` zawiera lokalny manifest/tool adapter OCAF i routy logiczne, `security.rs` sprawdza stałe ewidencyjne i zakaz sieci/źródeł, `lifecycle.rs` odrzuca stan `Active`, `main.rs` uruchamia wyłącznie demo.

**Rzeczywista zasada funkcjonalna:** Weryfikacja wymagalności scope_id, source_ref, observed_at, sha256 dla observed/verified, verifier_ref dla verified; status HTTP nigdy nie dowodzi istnienia konta.

**Trust boundary:** Oznaczenie verified opisuje jedynie przekazaną dokumentację; brak niezależnego poświadczenia źródła, stąd independently_verified=false i source_contacted=false. Modelowy JSON nie tworzy podpisu ani mocy wykonawczej. Test niepodpisanego peera na lokalnym OCAF nie zastępuje właściwego TLS-bound OMAs RPC, autoryzacji OMAt i hostów produkcyjnych.

Braki: Autentykacja niezależnego weryfikatora, retrieval rzeczywistych obserwacji, podpisany digest, ochrona danych osobowych, testy zewnętrznych źródeł i polityk uprawnień. Certyfikat i podpisane wydanie nie istnieją.
