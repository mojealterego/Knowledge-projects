# Analiza 19 linków — GitHub MCP / Agent Apps / Gemini / gry dla par
**Partia 19, 2026-10-09.** Baza GitHub: `main@4c4c7e8a01a219101815487260e8a2f91d13b449`.

**Status:** analiza publicznych stron i ograniczeń dostępności, korelacja z istniejącymi właścicielami projektów. **Żadna aplikacja Marketplace, serwer MCP, konto gier ani płatna usługa nie została zainstalowana lub połączona.** Link do reklamy/platformy nie jest upoważnieniem do OAuth, instalowania kodu ani uruchomienia skanów.

## Rejestr 19 źródeł
| # | URL wejściowy / nazwa | Dostępność / istota | Decyzja |
|---|---|---|---|
| 1 | `https://luma.com/codex-community?utm_source=oaidevs` | PUBLIC: kalendarz spotkań Codex/hackathonów; zapisy/statusy zmienne | P100/P47 — tylko źródło do aktualnych wydarzeń, brak rejestracji |
| 2 | `https://github.com/marketplace?type=apps&category=agent-apps` | PUBLIC: kategoria GitHub Agent Apps w public preview, użycie wg dokumentacji zależy od planu Copilot | P72/P100/P108 — katalog dostawców |
| 3 | `https://github.com/mcp/uk.sadiqoon/sakh` | PUBLIC: SAKH research corpus, read-only 7 tools, OAuth 2.1, prywatny korpus arab./pers.; wydawca Sadiqoon | P19/P29 — źródłowy RAG z ograniczeniem perspektywy korpusu |
| 4 | `https://github.com/mcp/tools.structura/structura` | REGISTRY_UNRESOLVED; powiązana publiczna dokumentacja `structura.tools`: modelowanie domen i generowanie kodu; nie mylić z projektem Minecraft | P87/P28/P100 — model-first codegen |
| 5 | `https://github.com/mcp/tools.apricot/apricot` | REGISTRY_UNRESOLVED; publiczny repo `metadevpro/apricot-mcp` opisuje SysML2 i OAuth; edycja/validacja nie wszystkie dostępne przez zewn. MCP | P87/P28 — modele SysML2, precondition/permission |
| 6 | `https://github.com/mcp/md.traveler/mcp` | REGISTRY_UNRESOLVED; oficjalna docs `docs.traveler.md/mcp` i 8 scoped tools, profile/trip memory, **brak cen/rezerwacji/wyszukiwarki** | P29/P114/P72 — pamięć podróżna, nie agent rezerwacji |
| 7 | `https://github.com/marketplace/actions/instructvault` | PUBLIC: InstructVault Action v0.7.1, Git-native prompt YAML/JSON, validate/lint/test/bundle | P90/P100 — deterministyczny prompt QA |
| 8 | `https://github.com/marketplace/miro-github-connector` | DIRECT_PAGE_UNRESOLVED; alternatywnie `github.com/apps/miro` potwierdza połączenie Copilot↔Miro przez identity-bound access | P100/P72 — weryfikuj konkretną aplikację przed instalacją |
| 9 | `https://github.com/marketplace/packfiles-agent` | PUBLIC: planowanie/diagnostyka migracji GitHub; wymaga aktywnego Packfiles Warp i Copilot | P102/P121 — migracja i uzasadnione koszty |
| 10 | `https://github.com/marketplace/endor-labs-agenthq-plugin` | PUBLIC: darmowa Developer Edition do publicznej analizy zależności/CVE, SCA/SAST oferta zależna od planu | P108/P72 — SCA i pochodzenie zależności |
| 11 | `https://github.com/marketplace/launchdarkly-agent` | PUBLIC: flagi funkcjonalne i AI Config, zmienia konfiguracje środowiska | P121/P72 — staged rollout z zatwierdzeniem |
| 12 | `https://github.com/marketplace/octopus-deploy-intelligence-agent` | PUBLIC: odczyt wdrożeń, release/runbook i mutacje via token Octopus | P121/P90 — deployment state/readback |
| 13 | `https://github.com/marketplace/bright-security-agent` | PUBLIC: DAST na izolowanych lokalnych targetach, demo/trial ma ograniczenia | P108/P72 — skanowanie tylko posiadanych celów |
| 14 | `https://github.com/marketplace/miro-agent-app` | PUBLIC: tablice/diagramy, architektura PR, możliwość design→code; OIDC i zgody użytkownika | P100/P72 — ślad projektowy; bez automatycznej publikacji |
| 15 | `https://github.com/marketplace/sonarqube-agent` | PUBLIC: SonarQube issue/quality gates/remediation w PR, może tworzyć commity | P108/P72/P100 — developer QA i branched fixes |
| 16 | `https://gemini.google/gemini-drops/` | PUBLIC: dziennik zmian/nowości Gemini; strona dynamiczna i lokalizowana | P17/P21 — obserwacja wersji i dostępności bez domniemanego abonamentu |
| 17 | `https://privegame.com/pl` | PUBLIC: Privé opisuje niezależne odpowiedzi dwóch dorosłych i ujawnianie wyłącznie wspólnych odpowiedzi; deklarowane wyniki ankiet/statystyki dostawcy są niezweryfikowane | **P122 CHEMIA** — privacy-first mutual matching |
| 18 | `https://loveplay.io/pl/games/` | DIRECT_PAGE_UNAVAILABLE; oficjalny `loveplay.io/pl/` i blog produktowy dostępne, **zawierają sprzeczne opisy** trybu parowania z dwóch telefonów kontra jedno urządzenie | **P122 CHEMIA** — warianty rozgrywki i kontrakt dowodowy |
| 19 | `https://modernlove.pl/gry-erotyczne-online` | PUBLIC: komercyjny przegląd kategorii gier dla dorosłych (2025), nie niezależna walidacja bezpieczeństwa/rynku | P122/P50 — otoczenie konkurencyjne i jawna autonomia |

## Sprawdzone różnice i niepewności

- **GitHub Agent Apps** = aplikacje partnerów działające w GitHub/Copilot Agent HQ, **nie** automatycznie wtyczki dostępne i zainstalowane w ChatGPT. GitHub wskazuje public preview i płatne plany Copilot. Każdy dostawca może wymagać dodatkowej usługi, planu, dostępu do repozytoriów czy OAuth. Nawet wyświetlany „Free” nie oznacza całkowicie darmowego użycia. Dotyczy zwłaszcza Packfiles Warp, Bright 14-day trial i Octopus.
- **MCP registry**: SAKH jest opisany bezpośrednio w podanym widoku; dla Structura, Apricot i Traveler bezpośrednie podane strony nie były możliwe do odczytu w interfejsie web, więc ich możliwości są powiązane z **oddzielną publiczną dokumentacją dostawcy**. Nie twierdzimy, że podane listingi są aktywne, zainstalowane albo mają identyczne `tools/list`.
- **SAKH**: wycinkowy korpus poglądów jednej osoby/polityczno-religijnej tradycji, przydatny do badań *tego korpusu*. Dokumentowane cytaty nie stanowią niezależnej weryfikacji historycznej ani prawnej; własność korpusu jest zastrzeżona przez wydawcę.
- **Traveler.md**: to pamięć preferencji i planowanych podróży, a nie lista aktualnych hoteli, cen, rezerwacji czy dostępności.
- **Privé**: deklaracje prywatności, liczby użytkowników i skuteczności są twierdzeniami producenta, **nie audytem**. Oddzielne odpowiedzi i ujawnianie tylko wspólnych preferencji jest wartościowym wzorcem. Wspólny temat **nie daje zgody na rzeczywiste działanie**.
- **LovePlay**: opis główny twierdzi, że gra na **jednym telefonie** jest docelowa i gra nie wymaga drugiego urządzenia, podczas gdy materiał rankingowy w tym samym serwisie opisuje dwuosobowe konta i synchronizację na dwóch telefonach dla wszystkich gier. To **nieuzgodniona sprzeczność źródeł producenta**; nie przepisujemy jej jako gotowej funkcji CHEMIA.
- **Modern Love**: artykuł zakupowo-redakcyjny obejmuje visual novel, symulatory i kategorie gier online. Efekty psychologiczne i prywatność są hasłami marketingowymi, bez dostarczonego niezależnego eksperymentu.

## Implementacja i właściciele
**Nowe projekty numerowane: 0.** P122 już jest grą 18+ z kontrolą zgody; P100/P72/P108/P121 obejmują integrację i audyt GitHub agent apps; P19/P29/P87/P114 modelowanie i korpusy. Nie powielamy tych zakresów.

Rzeczywiście dodano **dwa moduły czystego Python + testy**:
1. `tools/marketplace_agent_review_gate.py` — ocena planowanych integracji GitHub Marketplace według zewnętrznej zgody właściciela, zakresu repo/zdolności, czasu, izolacji skanu i braku eksportu danych. **Nie instaluje app.**
2. `projekty/122-chemia-consent-aware-intimate-two-player-game/consent_intersection.py` — nietrwałe obliczenie przecięcia dwóch jawnie zaakceptowanych zestawów identyfikatorów tematów, blokowanie niezaakceptowanych, wycofanie sesji; **bez zapisu odpowiedzi, bez weryfikacji wieku i bez sieci**.

CI workflow musi potwierdzić testy; nie raportować jako zainstalowanego Miro/SonarQube/Octopus, aktywnego SAKH/Traveler MCP lub działającego produktu mobilnego. Nie opublikowano rzeczywistych prywatnych odpowiedzi osób.
