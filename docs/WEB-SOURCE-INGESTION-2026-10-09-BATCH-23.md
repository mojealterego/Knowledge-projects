# Partia 23 — siedem stron GitHub MCP Registry (2026-10-09)

**Zakres:** wyłącznie `mojealterego/Knowledge-projects` do zapisu. Linki publicznego katalogu to materiały źródłowe, **nie** zgoda na instalowanie aplikacji, serwerów, konfigurację OAuth, płatność ani nadawanie dostępu do innych repozytoriów.

## Wejście i realna kompletność

Przetworzono **wszystkie siedem URL użytkownika**: [1](https://github.com/mcp?page=1), [2](https://github.com/mcp?page=2), [3](https://github.com/mcp?page=3), [4](https://github.com/mcp?page=4), [5](https://github.com/mcp?page=5), [6](https://github.com/mcp?page=6), [7](https://github.com/mcp?page=7).

**Odczyt dynamicznej strony z 2026-10-09**: wyświetlany katalog deklarował **394 serwery MCP łącznie**, a każda z przekazanych stron zawierała 30 wpisów. Zatem zinwentaryzowano **210 / 394** opisów (7×30), zaś pozostałe **184** wpisy nie należały do żądanego zakresu stron 1–7; **nie udawaj audytu całego katalogu**.

Wcześniejsze statyczne wycinki wyników wyszukiwarki pokazywały 164/210 pozycji (różne starsze snapshoty). Obecna liczba 394 pochodzi bezpośrednio z 7 odczytanych stron przeglądarki w tej sesji; to liczba z **momentu odczytu**, a nie stała gwarantowana przez GitHub.

**[Pełny indeks 210 pozycji: JSON](mcp-registry/2026-10-09-github-mcp-pages-1-7.json)**. Każda pozycja ma stronę, pozycję 1–30, tytuł, opis **z widocznej karty**, dokładny URL listingowy, stan weryfikacji `PUBLIC_LISTING_DESCRIPTION_UNVERIFIED`. Przy dwóch kartach ekstrakcja strony nie ujawniła tytułu: **strona 3 poz. 27** (`Vortx-AI/emem`) i **strona 5 poz. 18** (`adkit/ads`), więc zachowano `title=null` i opis bez dopowiadania nazwy. Wszystkie **210 adresów listingów są odrębne**.

**Status epistemiczny:** widać wyłącznie *opisy deklarowane przez wydawców i linki katalogu*. Nie otwierano i nie instalowano 210 serwerów indywidualnie, nie badano ich kodu, licencji, polityki retencji, żądanych uprawnień, aktualnej ceny, `tools/list` ani kompatybilności z ChatGPT. Wpis w GitHub MCP Registry nie oznacza, że produkt jest zainstalowany ani połączony z kontem użytkownika.

## Przydatne wzorce dla istniejących projektów

| Wzorzec / listingi ze źródła | Kanoniczny właściciel i wartość | Warunek przed prawdziwą integracją |
|---|---|---|
| Markitdown, Imagesorcery, Google AI Search MCP, Microsoft Learn | **P47/P29/P57** — wejście PDF/audio/obraz, rozdział konwersji od dowodów | licencja, prywatność źródeł, jakość ekstrakcji, oryginalne cytaty |
| Context7, Serena, Sourcegraph, Kotlin & Java Library Sources, DeepWiki | **P100/P87/P125** — repo search, biblioteki, kontekst API i Kotlin/Java | przypięty upstream, repo scope, brak samowolnej modyfikacji kodu |
| Playwright, Chrome DevTools MCP, Cypress Cloud, mabl, Wopee, Argus Testing | **P33/P100/P72** — QA i UI regression z czytelnymi dowodami | tylko kontrolowane środowisko/testowy target, jawny user consent |
| Netdata, Sentry, Dynatrace, Logfire, PagerDuty, Shipbook, Octopus Deploy | **P121/P59** — obserwowalność i reakcje na incydenty | rozdzielenie READ od mutacji, rate limits, blasty, koszty |
| GitHub, Azure DevOps, GitLab, Terraform, StackQL, Control Plane, Vercel, Neon | **P72/P121/P47** — orkiestracja i infra | *tylko Knowledge-projects jako write target*; konta zewn. read-only |
| SonarQube, Snyk, Sonatype, Codacy, StackHawk, Black Duck, CrowdStrike | **P108/P72** — kategorie SAST/DAST/SCA i raporty | tylko upoważnione skany, nie dowód, że repo jest bezpieczne |
| Basic Memory, PMB AI, ContextStream, Selvedge, XMemo, memo, Claude FAF | **P114/P29** — pamięć i decyzje | bez automatycznego przesyłania prywatnych treści, TTL, usuwanie |
| Hugging Face, Chroma, pgEdge, Elasticsearch, DBHub, Supabase, MongoDB | **P29/P37/P114** — modele/vector+DB | tenant isolation, read-only tools, query limits, aktualne IAM |
| draw.io, Figma, Miro, Lucid, Mobbin, Anima | **P100/P33** — specyfikacja/design-to-code | źródła/licencje, odrębne zgody na publikację |
| Stripe, Zapier, DSers, Avalara, Easyship, Longbridge, Atom.com | **P56/P45** — automatyzacja handlu/rozliczeń | żadne płatności/transakcje/zakupy bez osobnej zgody |
| WordPress WP Agent, Wix, Webflow, Vercel, Hostinger | **P45/P33** — publikacja i strony | osobne uprawnienia do konta i domeny |
| gamedev.pl, Unity MCP | **P86/P50** — prototypy gier | licencje, build i testy, brak zewnętrznych zapisów |
| fittok, Scholar Sidekick, Semantic Scholar, Rigour, Topos | **P59/P77/P114** — selekcja kontekstu, błędne cytowania, kontrola jakości | miara skuteczności dopiero po testach |

Powyższe grupowanie jest **oceną możliwości wynikających z opisu**, nie weryfikacją zintegrowanego endpointu. Modele finansowe/komunikacyjne, zdalny shell/desktop i funkcje publikowania mają najwyższy priorytet izolacji; deklaracje bezpiecznego działania wydawców nie są dowodami.

## Oficjalne zasady GitHub / MCP

- [GitHub Docs: korzystanie z MCP w Copilot IDE](https://docs.github.com/en/copilot/how-tos/provide-context/use-mcp-in-your-ide/extend-copilot-chat-with-mcp?tool=vscode) opisuje instalację do **GitHub Copilot/IDE** i autoryzację OAuth; **to nie jest automatyczna instalacja do ChatGPT**.
- [GitHub Docs: własny rejestr](https://docs.github.com/en/copilot/how-tos/administer-copilot/manage-mcp-usage/configure-mcp-registry) opisuje public-preview prywatny registry z endpointami `GET /v0.1/servers` i `GET /v0.1/servers/{name}/versions/...`; odradza poleganie na preview registry jako głównej blokadzie bezpieczeństwa. Wbudowane polityki i `managed-settings.json` są ważniejsze niż zaufanie do samych kart.
- [Oficjalne MCP Registry](https://github.com/modelcontextprotocol/registry) jest osobnym serwisem metadanych ze schematem `server.json`; lista katalogowa GitHub `/mcp` nie jest z automatu podpisanym `server.json` ani weryfikacją `tools/list`.

## Co faktycznie zaimplementowano

1. **JSON indeks 210 listingów** z numerami stron i adresami, odróżniający `source-cards` od zainstalowanych wtyczek.
2. **`tools/mcp_registry_catalog.py`** — offline walidacja kompletności 7×30, URL, duplikatów, braku tytułów, statusów niezweryfikowania; ostrożna heurystyka *kategorii do wyszukiwania*, nie oceny uprawnień. Sekcja `evaluate_integration()` odmawia prób instalacji/połączeń/płatności, braku consent lub prób zapisu poza Knowledge-projects. **Nie komunikuje się z katalogiem i nie instaluje MCP.**
3. **`tools/test_mcp_registry_catalog.py`** — 22 testy stdlib i rzeczywisty fixture 210 rekordów pod GitHub CI.
4. Ewolucja istniejących kanonicznych projektów. **Nowe projekty numerowane: 0** — brak nowej, odrębnej linii produktu; MCP tool registry to rozwinięcie właścicieli P47/P72/P100.
5. Żadne konta zewnętrzne, płatne API, wtyczki, niestandardowe instalacje, subskrypcje, urządzenia ani inne repozytoria nie są modyfikowane.

**Następny krok:** dla priorytetowych serwerów sprawdzić indywidualne źródła, model uprawnień, licensing, wersje, host compatibility i realny tool manifest po odrębnym upoważnieniu. Przed pierwszym użyciem mechanizmów tworzących skutki wymagać niezależnego zatwierdzenia.
