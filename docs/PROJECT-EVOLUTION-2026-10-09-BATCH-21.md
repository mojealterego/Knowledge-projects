# Ewolucja portfolio — partia 21 / 2026-10-09

**Jedyny target zapisów:** `mojealterego/Knowledge-projects` — zgodnie z [AGENTS.md](../AGENTS.md). Wszystkie 3 źródła są read-only, bez zapisów do Zed, Railway czy WordPress.com.

| Kanoniczny projekt | Udokumentowane zmiany | Stan wdrożenia |
|---|---|---|
| P33 App Delivery | Railway template wymagania: publiczny kod, health/persistence, licencje, koszt i zaakceptowanie rozwiązania | ARCHITECTURE |
| P45 Content Commerce | WordPress paid plugin vs plan, built-in/freemium alternatywy, kompatybilność i koszty | ARCHITECTURE |
| P47 Portfolio Integrity | 3 źródła były wcześniej rejestrowane jako shell; sprawdzono alternatywne oficjalne źródła i rzeczywiste API Zed | VERIFIED SOURCE DIFF |
| P56 Commercialization | Zed otwarty issue ≠ płatne bounty; Railway historyczne Solved $150 ≠ aktualna oferta; premium plugin = potencjalny koszt | ARCHITECTURE+CODE |
| P59 Execution Fabric | Zed #65199 no-progress agent 171 `read_file`, preflight limitów wywołań/tokenów i pauza | ARCHITECTURE+CODE |
| P72 Assurance | trusted postcondition/readback, no-progress/retry guard, deny claims of unverified bounty | ARCHITECTURE+CODE |
| P100 NeXus IDE | Zed Guild program i aktualne GitHub issue #51333/#65199 OPEN i #65205 CLOSED, integracja guard w przyszłości | ARCHITECTURE |
| P121 InfraSentinel | Railway deployment, koszty szablonu, rollback, payout status nieimplikowany przez przykład | ARCHITECTURE |

**Nowe projekty numerowane: 0**. Wszystkie nowe koncepcje należą do istniejących projektów.

## Faktyczny kod
1. `tools/agent_tool_loop_guard.py` + `tools/test_agent_tool_loop_guard.py`: 12 testów; statyczny limit pętli i zużycia zasobów; **nie jest uruchomionym Zed adapterem**.
2. `tools/external_opportunity_evidence_gate.py` + `tools/test_external_opportunity_evidence_gate.py`: 18 testów; kwalifikacja zaufanego aktualnego statusu i ograniczenie zapisu do Knowledge-projects; **nie jest internetowym scraperem ani systemem wypłat**.
3. Konkretny publiczny readback statusów Zed wykonano przez GitHub API w trakcie analizy; do testowania algorytmów wykorzystano wyłącznie syntetyczne fixtures, bez logowania do kont zewnętrznych.

## Ograniczenia
Brak dostępu do dynamicznych elementów tablic projektowych, prywatnego katalogu płatnych wtyczek użytkownika, salda Railway/WordPress, faktycznej wypłaty i kont Zed Guild. Nie wykonywano instalacji, deploy, zewnętrznych PR, wpłat ani zapisów poza Knowledge-projects. Odbiór GitHub Actions wymaga sprawdzenia rzeczywistego wyniku run.
