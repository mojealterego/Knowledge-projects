# Projektowa ewolucja — partia 20 / 2026-10-09

## Dwa repozytoria docelowe

- **ODYN-AI** (`mojealterego/ODYN-AI`): bazowe `codex/termux-five-goals@df56169a5f472ca4156e8f8a77a75b89baa93733`; bez zmiany gałęzi domyślnej utworzono [PR #17](https://github.com/mojealterego/ODYN-AI/pull/17) zawierający `scripts/odyn_upstream_compat_gate.py`, `tests/test_odyn_upstream_compat_gate.py`, `.github/workflows/odyn-upstream-compat.yml` oraz ostrzeżenia w dwóch kanonicznych dokumentach Android/Termux. **17 testów stdlib** w dedykowanym CI; PR wymaga osobnego wyniku/merge-readback.
- **Knowledge-projects**: import 80 metadanych repozytoriów NousResearch, synteza dokumentacji i dostawców, [`tools/hermes_upstream_adoption_gate.py`](../tools/hermes_upstream_adoption_gate.py), testy oraz aktualizacje kanonicznych projektów. Nie zapisano źródeł plików wag ani kodu z obcych repozytoriów bez licencji/zgody.

## Projekty kanoniczne do rozbudowy

| Właściciel | Konkretna funkcja / nowa wiedza | Co jest rzeczywistym kodem |
|---|---|---|
| P17 Model Router | źródłowe możliwości/limits/fallback Nous Portal, Novita, NVIDIA, MiMo, Z.ai, Kimi, MiniMax, Hugging Face; oddzielna autoryzacja | procedura/kontrakt, **bez model invocation** |
| P29 Research | Honcho V3 + evidence memory, repo/corpus retrieval, bitemporal rights | kontrakt; brak Honcho konta |
| P37 local inference | stare forki llama.cpp/Ollama, speculative decoding, device-specific Termux/GGUF compatibility | kontrakt, bez model weights |
| P47 registry | 80 repo repo-metadata verified, `atropos archived=true`, repo status and fork preflight | source ledger/CI |
| P59 autonomous execution | Tool Gateway paid, no silent bill, cron/gateway lifecycle separate from Termux foreground | architektura |
| P72 agent assurance | upstream pin, signed repository, git revision, independent grant, eval sandbox, rollback | **offline gate + tests** |
| P87 compiler | source guardrails for generated skills/AgentSkills standard, no model prompt self-authority | kontrakt |
| P100 IDE | Hermes CLI, ACP, skills, MCP, optional computer-use-linux and HermesClaw under bounded permissions | kontrakt |
| P108 security | security docs, OpenShell, agent governance, sandbox/stranger-message defenses, eval baseline | kontrakt |
| P114 memory | Honcho vs local memory, consent, deletion and source independent corroboration | kontrakt |
| P115 agent orchestrator | staged self-evolution DSPy/GEPA, metrics, compressed memory probe, safe PR promotion | kontrakt+gate |
| P119 Android runtime | native MobileFork APK vs currently broken official Nous Termux APT; release status | **code in ODYN PR** |
| P121 SRE | providers' token plans, paid Tool Gateway, cloud GPU provisioning approval | kontrakt |
| P100/P115/P37 | evaluation/training repos including archived Atropos and stale forks, no runtime claim | repo catalog |
| P126 game server | benchmark/simulation artifacts can be research references, no production game code inferred | status unchanged |

**Nowe numerowane projekty: 0.** P115/P119/P37/P72/P114 już obejmują właściwe granice; drugi ODYN/Hermes gateway nie ma niezależnego produktu.

## Rzeczywista implementacja

**`Knowledge-projects/tools/hermes_upstream_adoption_gate.py`** — wyłącznie lokalny policy gate, używa pinowanej SHA, faktycznie zewnętrznego review, licencji, daty, statusu archived/fork, zgody właściciela, CI; **17 testów**. Nie jest cyfrowym podpisem, autoryzatorem GitHub ani uruchomioną piaskownicą.

**`ODYN-AI/scripts/odyn_upstream_compat_gate.py`** — APT podpisany `hermes-stable`, standardowe Termux arm64/API>=24, 72h fresh independent release status, explicit owner approval; `BROKEN` blokuje. Testy/CI. **Nie** jest instalatorem Android i **nie** dowodzi, że upstream został naprawiony.

### Ograniczenia wykonania
Przyjęte twierdzenia publicznych stron są oznaczone jako właściwości oferowane przez dostawcę, nie jako wynik własnego testu integracyjnego. Nie wydano pieniędzy ani nie pozyskano kluczy/tokenów, nie przebudowano Android APK, nie sprawdzono urządzenia, nie uruchomiono treningu/replikacji/ML inference. Większość z 80 repozytoriów potwierdzono na poziomie metadanych, **nie pełnego audytu kodu**.

## Erratum — wyłącznie jeden cel zapisu

Nagłówek „Dwa repozytoria docelowe” opisuje wcześniejszą, **błędną decyzję operacyjną**. Wolno modyfikować wyłącznie `mojealterego/Knowledge-projects`. Scalony PR #17 w `mojealterego/ODYN-AI` był działaniem poza zakresem i nie stanowi wzorca dla kolejnych partii. Dalsze ulepszenia Android/Termux/Hermes zapisuj jako kod, testy i specyfikacje w `Knowledge-projects`; repo źródłowe pozostaje tylko do odczytu. [Sprostowanie](REPOSITORY-SCOPE-ERRATUM-2026-10-09.md).
