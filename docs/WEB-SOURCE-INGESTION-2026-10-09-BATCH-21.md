# Partia 21 — Zed Guild, Railway Template Bounties, WordPress płatne wtyczki (2026-10-09)

**Repozytorium docelowe do ZAPISU: TYLKO `mojealterego/Knowledge-projects`.** Wszystkie 3 przekazane linki i wszystkie dodatkowe źródła dostawców są **wyłącznie do odczytu**. Baza: `main@9a0389db8b48520619d7d1ae3d64f87661dea25b`. Nie instalowano ani nie kupowano wtyczek, nie tworzono kont, nie zgłaszano bounty, nie wykonywano PR w repo zewnętrznych.

## Rejestr 3 URL

| # | Źródło użytkownika | Stan odczytu | Co faktycznie wiadomo | Właściciel |
|---|---|---|---|---|
| 1 | https://github.com/orgs/zed-industries/projects/74 | **PUBLIC_BOARD_SHELL_ONLY**, bez dynamicznego stanu kart | Oficjalna strona Zed Guild pokazuje program prac nad edytorem i listę kwestii/issue. Stan 3 sprawdzonych numerów odczytano niezależnie przez publiczne GitHub Issues API. **Nie zweryfikowano pełnej listy kart, przydziału, prawa użytkownika do udziału ani nagrody.** | P100, P59, P72, P47 |
| 2 | https://github.com/orgs/railwayapp/projects/2 | **PUBLIC_BOARD_SHELL_ONLY**, widoczny tytuł „Template Bounties” bez pełnego statusu kart | Repo `railwayapp/templates` oficjalnie mówi o szablonach i bounties. Oficjalny serwis Station i dokumentacja Railway pokazują mechanizm wynagrodzeń, lecz odrębne przykłady starych zadań mają status **Solved**. **Nie potwierdzono obecnie otwartego bounty z tej konkretnej tablicy**. | P33, P56, P121, P47 |
| 3 | https://wordpress.com/plugins/browse/paid/mojealteregopl.wordpress.com | **AUTH_REQUIRED / SITE_ACCOUNT_UNVERIFIED** | Oficjalna pomoc WordPress.com (przegląd 2026-10-05) opisuje instalowanie wtyczek na płatnych planach i osobno płatne pluginy Marketplace, ale brak dostępu do zalogowanego konta, aktywnej taryfy czy katalogu produktów dla tej witryny. **Nie potwierdzono instalacji, aktualnej ceny ani uprawnień.** | P45, P56, P47 |

## Znacząco nowa wiedza po ponownej weryfikacji

### Zed — rzeczywiście otwarte i zamknięte zadania (odczyt API 2026-10-09)
- [Zed issue #51333](https://github.com/zed-industries/zed/issues/51333), „Gemini CLI external agent does not start when using Gemini's sandbox”: **OPEN**, tagi `state:reproducible`, `area:ai/gemini`, `reach:many users`, `severity:S3`; publiczny issue od 2026-03-11, ostatnia aktualizacja 2026-07-09. **Możliwy cel analizy/dokumentacji**, nie potwierdzone płatne zlecenie.
- [Zed issue #65199](https://github.com/zed-industries/zed/issues/65199), „Agent Panel: read-only profile loops on identical tool calls — 171× read_file on one path, 195 calls, 0 edits, ~10.3M tokens, never stopped”: **OPEN**, `state:needs triage`; utworzony 2026-10-05. To powiązany przypadek regresji kontroli pętli agentów. **Źródłowe liczby są opisem zgłoszenia**, nie rezultatem uruchomionego przez nas benchmarku.
- [Zed issue #65205](https://github.com/zed-industries/zed/issues/65205), „webrtc-sys match absence”: **CLOSED**, reason `not_planned`, aktualizacja 2026-10-07 — przykład, dlaczego treść z publicznej listy / tablicy może mieć już nieaktualny status.

Oficjalny [Zed Guild](https://zed.dev/community/guild) opisuje 12-tygodniowe cohorty, 5–10 h/tydzień, ścieżki Repro Specialist/Bug Basher/Feature Shipper i mentoring w Rust. Na stronie nadal widnieje harmonogram **zakończony we wrześniu 2026** i etykieta `Cohort #2 in Progress`, co tworzy **nieaktualną/niespójną wskazówkę o bieżącej rekrutacji**. Nie przedstawiam udziału, wypłaty, możliwości zapisania się dziś ani zaproszenia jako potwierdzonych.

**Implementacja w Knowledge-projects:** `tools/agent_tool_loop_guard.py` — lokalny kontrakt chroniący przed zbyt wieloma identycznymi wywołaniami bez **zewnętrznego, zaufanego** postępu oraz przed przekroczeniem liczby wywołań/tokenów. **Nie jest patchem w Zed; nie został włączony do żadnego zewnętrznego IDE**.

### Railway — prace nad szablonami i warunki nagród

Repo [railwayapp/templates](https://github.com/railwayapp/templates) potwierdza program template bounties i wymogi poprawnego publicznego repo usług. Tworzenie i publikacja szablonu odbywa się obecnie przez UI Railway, a nie wyłącznie przez wystawienie issue w repo. Repo wskazuje poprawność zależności usług, zmiennych środowiskowych, health checks, katalogów głównych i publicznie dostępnego kodu jako ważne aspekty.

[Railway Station Bounties](https://station.railway.com/bounties) i [dokumentacja bounties](https://docs.railway.com/community/bounties) rozdzielają publikację zadań i formalne zatwierdzenie rozwiązania/wypłaty. **Przykłady historyczne, nie otwarte oferty**: [NodeBB template](https://station.railway.com/questions/template-request-node-bb-b54455b9) i [GPT OSS](https://station.railway.com/templates/template-request-gpt-oss-12dbfa91) mają status **Solved** oraz opis nagrody **$150**; nie stanowią dostępnych dziś 2×$150.

`RailwayTemplateEvidence` → oryginalny link zlecenia, data/status i wymagania, potwierdzony budżet testowy/infra, licencja i autorstwo, publiczny repo usług, persistent volume, startup/health, sekrety poza GitHub, rzeczywiste testy i zaakceptowane zgłoszenie. **Nie wdrażano usług i nie poniesiono kosztów chmury**.

### WordPress.com — premium katalog ≠ zakup

[Oficjalna pomoc instalowania pluginów](https://wordpress.com/support/plugins/install-a-plugin/) (last reviewed 2026-10-05) informuje, że aktualne płatne plany Personal/Premium/Business/Commerce oferują instalowanie wtyczek, ale na bezpłatnym planie instalacja wymaga zmiany planu. Dostęp do określonych funkcji na **starych/legacy planach** bywa inny — nie potwierdzono stanu konta `mojealteregopl.wordpress.com`.

**Płatna wtyczka** to osobny koszt/checkout: oficjalna procedura „Purchase and activate”, rozliczenie miesięczne/roczne, zachowanie subskrypcji. Obowiązkowo: najpierw sprawdź **istniejące darmowe/wbudowane funkcje** WordPress.com, kompatybilność, licencję, uprawnienia dostępu, budżet oraz zgodę właściciela na zakup; żadna wymieniona oferta nie jest zatwierdzona.

**To nie jest informacja, że wtyczka jest już zainstalowana lub że właściciel ma płatny plan.** Nie odczytano listy konkretnych płatnych pluginów przeznaczonych dla witryny i nie wykonywano checkout.

## Konkretnie utworzone pliki i ograniczenia

- `tools/agent_tool_loop_guard.py` + `tools/test_agent_tool_loop_guard.py`: offline ochrona przed identycznymi wywołaniami bez zaufanego postępu, limity token/call, deterministyczna blokada i restart tylko zewnętrzny; testy jednostkowe.
- `tools/external_opportunity_evidence_gate.py` + `tools/test_external_opportunity_evidence_gate.py`: sprawdzanie daty, typu i źródła statusu „open” oraz **wyłącznego repo do zapisu Knowledge-projects**; katalog, zamknięty issue i WordPress premium page nigdy nie oznaczają gotowej płatnej pracy. **Nie pobiera automatycznie bounty i nie weryfikuje wypłaty.**
- Rozszerzenia kanoniczne: P33, P45, P47, P56, P59, P72, P100, P121.
- **Nowe projekty numerowane: 0** — P56 ma już warstwę komercjalizacji i poszukiwania szans; P100/P59/P72 rozwój agentów i IDE; P33/P121 szablony/hosting; P45 WordPress/content commerce.

CI i odczyt `main` wymagane przed oznaczeniem wdrożenia jako ukończonego.
