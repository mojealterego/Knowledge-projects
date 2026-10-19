# Architektura — ARGUS MOBILE

**Nazwa własna:** ARGUS MOBILE · **Ewidencja:** 005 · **Numer seryjny:** `AM-MOB-ARG-2026-000005` · **Agent ID:** `AM-OCAF-AI-0005` · **Katalog:** `AM-OCAF-MOB-005` · **Nr certyfikatu:** `CERT-AM-2026-0005` (rezerwacja, niewydany).

## Topologia logiczna (nie produkcyjne połączenia)

```text
Operator
   |
   v
OCAF Runtime (lokalny host testowy)
   +-- OMAs : SERWER AGENTÓW — NIEPODŁĄCZONY
   +-- OMAt : SERWER NARZĘDZI — NIEPODŁĄCZONY
   +-- Zenoh: TRANSPORT — ROUTE KEY ONLY
   +-- ARGUS MOBILE : ui.proposal_assess -- Rust domain argus_mobile_agent
   +-- OSTc: DENY SOURCE ACCESS (SPEC PENDING)
```

## Kontrakt domeny

Ocena deklarowanej zgody i typu akcji bez dostępu do telefonu; stała blokada DELETE, SEND i GRANT_PERMISSION; pozostałe propozycje wymagają weryfikacji przez rzeczywisty Android UI runtime.

Przekazywany obiekt wejściowy nie stanowi niezależnie uwierzytelnionego dowodu. Wyjście narzędzia: `ActionAssessment: status=blocked|requires_grant|operator_review, execution_authorized=false, readback_required=true, verified=false, reasons[]`. Dane snapshotu, w tym teksty aplikacji i operator_grant, nie są niezależnie zweryfikowane i nie upoważniają do działania.

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
