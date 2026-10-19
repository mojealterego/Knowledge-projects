# Architektura — AXIOM FORGE

**Nazwa własna:** AXIOM FORGE · **Ewidencja:** 004 · **Numer seryjny:** `AM-ENG-AXF-2026-000004` · **Agent ID:** `AM-OCAF-AI-0004` · **Katalog:** `AM-OCAF-ENG-004` · **Nr certyfikatu:** `CERT-AM-2026-0004` (rezerwacja, niewydany).

## Topologia logiczna (nie produkcyjne połączenia)

```text
Operator
   |
   v
OCAF Runtime (lokalny host testowy)
   +-- OMAs : SERWER AGENTÓW — NIEPODŁĄCZONY
   +-- OMAt : SERWER NARZĘDZI — NIEPODŁĄCZONY
   +-- Zenoh: TRANSPORT — ROUTE KEY ONLY
   +-- AXIOM FORGE : graph.review -- Rust domain axiom_forge_agent
   +-- OSTc: DENY SOURCE ACCESS (SPEC PENDING)
```

## Kontrakt domeny

Wykrywanie duplikatów identyfikatorów, brakujących zależności, samoodwołań i cykli; deterministyczne uporządkowanie topologiczne; zero komend, mutacji i połączeń z GitHub.

Przekazywany obiekt wejściowy nie stanowi niezależnie uwierzytelnionego dowodu. Wyjście narzędzia: `GraphReview: status=structurally_valid|blocked, task_count, order[], issues[], execution_authorized=false, verified=false`. Model lub graf zadania nie może sam nadać uprawnień do zapisu, deploymentu i promocji wydania.

## Moduły

| Plik | Odpowiedzialność |
|---|---|
| `src/agent.rs` | domenowa, deterministyczna analiza i stałe identyfikatorów |
| `src/zenoh.rs` | walidowane key expressions `omas/v1/` / `omat/v1/` i lokalny adapter `ToolExecutor` |
| `src/security.rs` | konfiguracja tożsamości, transport wyłączony, brak OSTc, walidacja segmentów |
| `src/lifecycle.rs` | automatyczne odmowy stanu `Active`, lokalne fazy deweloperskie |
| `src/main.rs` | odizolowany demonstracyjny CLI bez dostępu do urządzenia i sieci |
| `tests/` | testy negatywne, tożsamości i OCAF integration na testowym runtime |

## Brakujące składniki

Produkcyjne OMAs podpis/tożsamość RPC, OMAt podpisany capability lease, Zenoh TLS/mTLS i izolacja hostów, OSTc zaszyfrowany dostęp do źródeł, niezależny audyt, urządzenia/środowiska docelowe, podpisane artefakty i operator acceptance. Nie wdrożono połączeń sieciowych ani zdalnych side effects.
