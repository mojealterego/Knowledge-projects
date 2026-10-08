# Partia 20 — ODYN-AI, Hermes Agent, NousResearch i dostawcy inferencji (2026-10-09)

## Procedura i dostęp do źródeł

Źródła przekazane przez użytkownika: repozytoria własne `mojealterego/ODYN-AI`, `Knowledge-projects`, `PLANY-I-POST-PY--W-REPOZYTORIACH` i profil; oficjalne dokumenty Nous Hermes Agent / AgentSkills / Honcho; osiem usług/portali modelowych; 3 issues `astral-sh/uv`; Android F-Droid i repozytorium wydań; `computer-use-linux` / `hermesclaw`; 80 **unikalnych** adresów repozytoriów `NousResearch/*` (w pytaniu `nomos` i `cline` powtórzono), dokumenty CLI/security/Agent Skills także powtórzone.

**Zakres faktycznie zbadany:** każdy z 80 adresów NousResearch ma osobno potwierdzoną metadane istnienia przez publiczne `GET /repos/NousResearch/{name}`. **Nie** oznacza to sklonowania i audytu 80 drzew ani uruchomienia benchmarków. Wysokopriorytetowe dokumenty Hermes/Termux/security/tool gateway/Agent Skills/Honcho i niektóre repozytoria przeczytano bardziej szczegółowo. Własny ODYN-AI sprawdzono na aktualnej **gałęzi domyślnej `codex/termux-five-goals`**, commit `df56169a5f472ca4156e8f8a77a75b89baa93733`, 6966 elementów drzewa, w tym `android/README.md`, `website/docs/getting-started/termux.md`, testy, aplikacja Android; odrębna gałąź `codex/odyn-ai` nie jest obecnie gałęzią domyślną. Drugie repozytorium planów ma aktywną gałąź `main`; zachowano jego zawartość bez modyfikacji.

[Pełny indeks 80 repozytoriów wraz z klasyfikacją i stanem metadanych](UPSTREAM-NOUSRESEARCH-REPOSITORY-CATALOG-2026-10-09.md).

## Ustalenia o najwyższej wadze

### 1. Android: różnica między natywnym APK i instalacją przez Termux

**Oficjalna dokumentacja Nous** `https://hermes-agent.nousresearch.com/docs/getting-started/termux` na dzień przeglądu informuje jednoznacznie: **"Termux is currently broken"**. Dla arm64 opisuje osobne repo APT `https://hermes-assets.nousresearch.com/releases/termux/stable`, `hermes-stable`, GPG fingerprint `C572 B5FD D1A2 9CCF A9A9 12B6 840B 0848 E139 156D`, standardowy prefiks `/data/data/com.termux/files/usr` i brak systemd. Nie jest to ścieżka instalacyjna skryptu glibc Linux. Podany obecny stan blokuje obietnicę działającego Termux „od ręki”; nie jest dowodem awarii **odrębnego** aplikacyjnego APK MobileFork.

Własne `ODYN-AI/android/README.md` rzeczywiście opisuje natywne Android `com.mobilefork.hermesagent`, F-Droid, GitHub releases, import GGUF i LiteRT-LM. Własne `ODYN-AI/website/docs/getting-started/termux.md` opisuje wcześniejszy source/venv install jako testowany. Nie wolno tych instrukcji traktować jako potwierdzenia zgodności aktualnego oficjalnego pakietu APT.

**Rzeczywista zmiana w ODYN-AI:** przygotowano osobną, niezmieniającą domyślnej gałęzi PR `integration/odyn-hermes-termux-upstream-safety-2026-10-09` z lokalnym `scripts/odyn_upstream_compat_gate.py`, 17 testami, osobnym CI i poprawionymi ostrzeżeniami w obu dokumentach. Kod nie łączy się z telefonem, nie sprawdza podpisu GPG kryptograficznie i **nie instaluje Hermesa**. [PR ODYN #17](https://github.com/mojealterego/ODYN-AI/pull/17). Stan merge/CI należy udokumentować osobno przy końcowym readback.

### 2. Trzy issues uv to problemy AV na Windows, nie naprawy Androida

`astral-sh/uv#13553` — raport heurystycznego Bitdefender na Windows 11 z 2025, zamknięty. `#15011` — Defender kwarantannuje `uvw.exe` w ręcznie pobranym pakiecie Windows, issue zamknięte jako duplikat; `#10079` — Windows + Bitdefender blokuje instalator PowerShell, zamknięte „not planned”. **Żaden nie dokumentuje poprawki instalacji uv na Termux**. Nie wyłączać AV w odpowiedzi na te zgłoszenia; nie rekomendować obejścia podpisu repo APT.

### 3. Hermes agent: narzędzia i bezpieczeństwo

Oficjalne przewodniki `/docs/user-guide/features/{tools,skills,memory,mcp,cron,context-files}`, `/docs/user-guide/security`, `/docs/user-guide/cli`, `/docs/developer-guide/architecture` rozdzielają: agenta/LLM, narzędzia i Toolsets, Skills (`SKILL.md`/AgentSkills), pamięć lokalną/Honcho, MCP z niezależnym uwierzytelnieniem, Cron/sesję, messaging i context-files. Skills są dokumentacją z progresywnym ładowaniem, **nie automatycznymi uprawnieniami shell/plików ani podpisanym wykonywalnym kodem**. Nie zakładać, że wszystkie opcjonalne extras uruchamiają się w Android/Termux.

**Tool Gateway nie jest darmowy:** oficjalny opis mówi o płatnej subskrypcji Nous Portal i zużyciu kredytów pay-as-you-use dla web/image/TTS/browser. Nie deklarować, że aktywacja Hermes automatycznie zapewnia bezpłatne bezlimitowe przeglądanie. Można użyć własnych kluczy tylko jeśli użytkownik je ma i jawnie autoryzuje; **żadnych tokenów nie publikować w repozytorium**.

### 4. Honcho v3 jako opcjonalna warstwa pamięci

`plastic-labs/honcho` + `honcho.dev/docs/v3/documentation/introduction/overview` opisują pamięć dla agentów i modelowanie długiego kontekstu. Poprawny adapter `MemoryBackend` wymaga oddzielenia `local`, `Honcho`, tenant/session/person, źródła, czasu i zgody na eksport; polityki usunięcia, ponownej zgody i weryfikacji krytycznych faktów. Żaden „psychologiczny” wniosek o użytkowniku nie powinien stać się faktem bez własnego źródła i korekty. Nie kopiować prywatnych rozmów do hostowanej pamięci bez jawnej zgody i retencji.

### 5. Dostawcy modeli / koszty

Źródła: `nousresearch.com`, `portal.nousresearch.com`, `novita.ai`, `build.nvidia.com`, `platform.xiaomimimo.com/token-plan`, `chat.z.ai`, `platform.kimi.ai`, `minimax.io`, `huggingface.co`. Są to **różne** powierzchnie — API inferencyjne, chat UI, konto/abonament, katalog modeli, GPU hosting, HF Hub. Nie każdy link do chatu jest adresem OpenAI-compatible API; nie każdy „free” oznacza dostęp do narzędzi, integracji lub darmowy transfer danych. Pricing/region/limity i format endpointów są zmienne; konkretnego API key, subskrypcji ani dostępnych kredytów **nie odczytano z kont**. Rekomendowany `ProviderCapabilityProbe` ma testować modele/streaming/tools/schema/context/fallback na koncie **po** autoryzacji i limicie kosztu, zamiast wybierać dostawcę po nazwie platformy.

### 6. Research ecosystem: oddzielić runtime, badania, legacy i forki

80 repozytoriów Nous jest zgodnych z 6 grupami tematycznymi (runtime/governance, trening/RL, benchmarki, inferencja/networking, twórczość, projekty poboczne). API potwierdziło `NousResearch/atropos` jako **`archived=true`** (ostatni push 2026-07-04). Ważne jest również rozpoznanie forków: m.in. `Megatron-LM`, `Megatron-Bridge`, `Liger-Kernel`, `OpenShell`, `litellm`, `ollama`, `nous-llama.cpp` są forkami; nie można zakładać aktualnej zgodności z upstreamem ani przenosić licencji bez sprawdzenia.

**Kandydaci do integracji w ODYN:** `hermes-agent-self-evolution` (DSPy+GEPA: staged skill/code scoring), `hermes-compression-eval` (retention probes), `hermes-example-plugins` (plugin surface, **przykłady**, nie wbudowane core), `agent-governance-toolkit` (policy pattern), `OpenShell` (sandbox pattern). Research-only: `atropos`, `tinker-atropos`, `torchtitan`, `Megatron-LM`, `DeepEP`, `nccl-tests`, `nanotron`, `DisTrO` — wymagają infrastruktury treningowej, nie są gotowymi Android pluginami. Tooling benchmark: `lighteval`, `openai-evals`, `lm-eval-harness`, `hermes-compression-eval`, writing benches; odrębne metryki i licencje. Nie uruchamiać automatycznie `llm-abliteration` do obchodzenia zasad zewnętrznych dostawców.

### 7. Odniesienia portfolio właściciela

`Knowledge-projects` już zawiera P17, P29, P37, P47, P59, P72, P87, P100, P108, P114, P115, P119, P121 i inne. `PLANY-I-POST-PY--W-REPOZYTORIACH` ma osobne plany/priorytety (np. `Agent-Android`, `Googleskills`, `Lemon-termux`). **Nie kopiowano tego drugiego repo** ani nie uznano list planów za wykonanie. Nie powstaje drugi „Hermes OS” ani nowy numer bez odrębnej definicji produktu.

## Praktyczna decyzja wdrożeniowa

- Zmiany w **ODYN-AI**: read-only signed APT preflight + testy/CI/dokumentacja. Jest to kod, ale **nie** instalator ani aktualizacja samego Hermes core.
- Zmiany w **Knowledge-projects**: kanoniczne projekty + katalog 80 repo + [offline gate adopcji upstream](../tools/hermes_upstream_adoption_gate.py) wymagający rewizji, legalnego użycia, zgody właściciela, testów i aktualności przy próbie runtime adopcji.
- Nie zmieniano gałęzi domyślnej; integracje do Cloud/Nous/HF/MiMo/Kimi/MiniMax/Z.ai nie wykonują żadnych płatnych działań.
- Ocena własności telefonu i rzeczywistej instalacji: **NOT TESTED**. Wywołania funkcji/modeli, twierdzenia o benchmarkach, poziom kont i „wszystko zainstalowane”: **NOT VERIFIED**.

## Sprostowanie zakresu prac — 2026-10-09

**Korekta nadrzędna:** użytkownik upoważnił agenta do rozbudowy **jednego repozytorium docelowego: `mojealterego/Knowledge-projects`**. Wykonanie zmian w dodatkowym `ODYN-AI` było **błędem**, choć wcześniejszy raport odnotowuje je jako fakt historyczny. Nie kontynuować tej ścieżki ani jej nie traktować jako standardowego przepływu. Wykorzystanie ODYN-AI i Hermes ogranicza się do **odczytu, analizy i integracji wiedzy oraz prototypów wyłącznie w Knowledge-projects**. [Pełne sprostowanie](REPOSITORY-SCOPE-ERRATUM-2026-10-09.md).
