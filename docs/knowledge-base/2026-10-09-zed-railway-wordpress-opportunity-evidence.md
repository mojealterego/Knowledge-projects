# Zed Guild / Railway Template Bounties / WordPress premium — synteza techniczna

**Źródła i fakty kontrolne:** [rejestr trzech linków partii 21](../WEB-SOURCE-INGESTION-2026-10-09-BATCH-21.md).

## 1. Reguły danych o okazjach do współpracy i wynagrodzeniach

Oddzielne pola `OpportunityEvidence`: `provider, user_url, actual_task_url, source_kind, observed_at, status_from_trusted_source, last_updated, expected_work, skill_requirements, geographic_plan_constraints, compensation_offered, payout_verified, local_project_owner, allowable_actions`.

**Zed Guild**: projekt GitHub #74 ma publiczną powłokę bez stabilnego stanu kart. Oficjalna witryna `zed.dev/community/guild` publikuje listę issue i historyczny harmonogram. Publiczny GitHub Issues REST może potwierdzić **stan konkretnego issue**, ale nie autoryzuje użytkownika do wydawania kodu w cudzym repo ani nie potwierdza wynagrodzenia. `zed-industries/zed#65199` jest ważny jako przypadek przeciążenia pętli narzędzi: 171 `read_file` bez postępu, 195 wywołań i źródłowo raportowane około 10.3 miliona tokenów. Specjalny offline guard P59/P72 ogranicza takie wzorce. `#51333` (Gemini sandbox external agent) był otwarty przy odczycie API 2026-10-09; `#65205` był zamknięty.

**Railway**: projekt GitHub #2 ma tytuł „Template Bounties”, ale tablica bez czytelnych kart nie jest dowodem otwartych zadań. Repo Railway Templates i Railway Station zapewniają dodatkowe oficjalne informacje. Rozróżniaj `BOUNTY_LISTED` → `TASK_STATUS_OPEN_IN_SOURCE` → `IMPLEMENTED_TESTED` → `SUBMITTED` → `ACCEPTED` → `PAID`. Nie przechodź do `PAID` z samego `Solved` ani z reklamy $150; w pierwszej kolejności wymagaj aktualnego konkretnego zlecenia, zgody właściciela, aktualnego terms, testów i potwierdzenia wypłaty. Szablon do publicznego repo: publiczny kod, health checks, persystencja, bez twardo zakodowanych sekretów, przykładowe środowisko, określenie kosztów.

**WordPress.com**: ścieżka `/plugins/browse/paid/<site>` jest widokiem wymagającym dostępu do konta, a nie listą już kupionych/włączonych wtyczek. Aktualna pomoc 2026-10-05 opisuje plug-iny na płatnych planach, możliwe rozbieżności starych planów i osobny premium checkout. W `PluginDecisionEvidence` przechowuj `feature_needed, current_builtin_solution, plugin_slug, vendor, third_party_permission, site_plan_entitlement_verified, compatible_runtime, license, recurring_charge, explicit_payment_approval`. Rozważ bezpłatne funkcje jako pierwsze. Żaden zakup/włączenie nie jest wykonywany na podstawie URL.

## 2. Implementacje w jedynym repozytorium

### P59/P72: `AgentToolLoopGuard`
```text
MODEL PROPOSES TOOL CALL
 → TRUSTED HOST supplies tool name, SHA256 of args and true progress version
 → BEFORE tool call: capped global call count/token budget
 → SAME TOOL + SAME ARGUMENTS + NO VERIFIED PROGRESS repeated > N?
 → HARD DENY + HALT; independent reset only by trusted owner
```
Ta demonstracja nie patchuje Zed i nie łączy się z żadnym runtime. `trusted_progress_version` musi pochodzić z rzeczywistego potwierdzenia stanu przez niezależny host, a nie z komentarza językowego modelu. Przed szerokim użyciem trzeba uwzględnić celowe ponowienia przy awarii, monitoring false positives i rzeczywiste koszty. Ochrona dotyczy jedynie prostego powtarzania identycznych żądań; nie wykrywa każdej pętli semantycznej.

### P56/P47: `external_opportunity_evidence_gate`
Przyjmuje wersjonowaną obserwację z zaufanego źródła `GitHub Issues/Station/publicdocs`, jej datę, status i proponowaną czynność. Odrzuca próby zapisu poza `mojealterego/Knowledge-projects`, traktowanie board shell jako konkretnego zadania, zamknięte/niepotwierdzone issue, nieaktualne dane, opłacenie pluginu czy zewnętrzne wysłanie oferty. **Brak automatycznej autoryzacji, pobierania nowych danych lub przyznawania nagród**: to offline kwalifikator źródeł.

## 3. Zakres i warunki dalszego rozwoju

**Zrobione w partii:** potwierdzone źródła, aktualizacje 8 kanonicznych projektów, dwa realne moduły statyczne + testy. **NIE zrobiono:** udział w Guild, PR do Zed, publiczny Railway template/deploy, podłączenie konta Railway, zakup WordPress, pobranie danych konta użytkownika, faktyczny watch w tle.

Nowy numer projektu byłby duplikatem P56/P33/P59/P72/P100/P121; zostały rozszerzone istniejące odpowiedzialności. Do każdej automatycznej przyszłej procedury potrzeba jawnego harmonogramu, dostępu do oficjalnych zasobów, zgody na ewentualną aplikację/checkout oraz prawdziwych telemetrii CI/revenue.
