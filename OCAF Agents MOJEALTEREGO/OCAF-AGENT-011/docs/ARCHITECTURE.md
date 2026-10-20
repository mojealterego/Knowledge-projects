# Architektura — LIMES GIT

**Nazwa:** LIMES GIT · **Ewidencja:** 011 · **Serial:** `AM-GIT-LIM-2026-000011` · **Agent ID:** `AM-OCAF-AI-0011` · **Katalog:** `AM-OCAF-GIT-011` · **Certyfikat:** `CERT-AM-2026-0011` — NIEWYDANY.

```text
Host OCAF Runtime (lokalny)
    |
    +-- OMAs [serwer agentów: NOT DEPLOYED]
    +-- OMAt [serwer narzędzi: NOT DEPLOYED]
    +-- Zenoh [kontrolowane trasy omas/v1 i omat/v1, sieć OFF]
    +-- Rust ToolExecutor github.scope_review
    |        +-- src/agent.rs: single-repository scope preflight
    |        +-- immutable output: no external execution
    +-- OSTc [zaszyfrowany dostęp do źródeł: DENY]
```

`src/lib.rs` zabrania unsafe Rust; `agent.rs` implementuje domenę, `zenoh.rs` zawiera lokalny manifest/tool adapter OCAF i routy logiczne, `security.rs` sprawdza stałe ewidencyjne i zakaz sieci/źródeł, `lifecycle.rs` odrzuca stan `Active`, `main.rs` uruchamia wyłącznie demo.

**Rzeczywista zasada funkcjonalna:** Odczyt innych repozytoriów jest dopuszczony; zapis wyłącznie do mojealterego/Knowledge-projects przez roboczą gałąź knowledge/, policy/, feature/ lub fix/. PR i merge wymagają pull_base=main i poprawnego pull_head. Bezpośredni zapis do main jest blokowany.

**Trust boundary:** Wynik permitted=true oznacza jedynie lokalny preflight. Agent nie wykonuje połączeń GitHub, nie ma tokenu, nie podpisuje merge, nie autoryzuje wywołania; execution_authorized=false i api_called=false. Modelowy JSON nie tworzy podpisu ani mocy wykonawczej. Test niepodpisanego peera na lokalnym OCAF nie zastępuje właściwego TLS-bound OMAs RPC, autoryzacji OMAt i hostów produkcyjnych.

Braki: Rzeczywiste przechwytywanie wszystkich zapisów GitHub, integracja z uprawnieniami i tokenem o ograniczonym zakresie, weryfikacja stanu PR/branch z API, polityka podpisów i operator acceptance. Certyfikat i podpisane wydanie nie istnieją.
