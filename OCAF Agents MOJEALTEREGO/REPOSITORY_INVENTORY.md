# Inwentaryzacja struktury Git — pierwsza migawka

Data: 2026-10-19. Metoda: odczyt kompletnych drzew domyślnych gałęzi GitHub API z `recursive=1`. Odczyt drzewa **nie** oznacza audytu zawartości każdego pliku.

| Repozytorium | Obserwowany commit main | Elementy | Pliki (blob) | Katalogi (tree) | Git tree listing |
|---|---|---:|---:|---:|---|
| `mojealterego/Knowledge-projects` | `3218a158cf030b8db954055cec864d0a428e3b03` | 745 | 714 | 31 | [JSON](KNOWLEDGE_PROJECTS_TREE.json) |
| `mojealterego/ocaf-runtime` | `52d4f597e0b481678f90cf2073b721ad2e99e253` | 623 | 548 | 75 | [JSON](OCAF_RUNTIME_TREE.json) |

## Knowledge-projects — rozkład według pierwszego segmentu ścieżki

- `.github`: 3 elementów drzewa
- `AGENTS.md`: 1 elementów drzewa
- `README.md`: 1 elementów drzewa
- `assets`: 2 elementów drzewa
- `docs`: 454 elementów drzewa
- `projekty`: 239 elementów drzewa
- `tools`: 45 elementów drzewa

W `projekty/` zaobserwowano **215 plików** (wliczając rozszerzenia, README i kod), a w `tools/` **44 plików**. Wśród ścieżek projektowych wykryto 130 kandydatów przez **heurystykę nazewniczą** (słowa agent, swarm, orchestrat, omni, runtime, mcp, builder, tool); to **nie jest liczba agentów** i nie dowodzi ich implementacji.

## Kontrole wstępne

1. Istnieje [AGENTS.md](../AGENTS.md) ograniczający zapisy wyłącznie do Knowledge-projects.
2. Istnieje [protokół ewolucji wiedzy](../docs/KNOWLEDGE-EVOLUTION-PROTOCOL.md), [portfolio](../projekty/README.md), [historyczne rejestry portfolio](../docs/PORTFOLIO-REGISTRY-2026-09-09.yaml) i [katalog MCP](../docs/mcp-registry/2026-10-09-github-mcp-pages-1-7.json).
3. W repozytorium referencyjnym istnieją `Cargo.toml`, `src/`, `tests/`, `README.md`, `ROADMAP.md` i workflow CI; odnotowano ich ścieżki, lecz nie uruchomiono tutaj testów.
4. Referencyjny [ROADMAP.md](https://github.com/mojealterego/ocaf-runtime/blob/52d4f597e0b481678f90cf2073b721ad2e99e253/ROADMAP.md) deklaruje 333 etapy główne, 64 techniczne i 10 produktów Agents Builder (407 pozycji). Jest to stan **dokumentacji**, a nie osobno potwierdzona produkcyjność.
5. Zabezpieczenie: w tej partii nie wykonano `git mv` ani zmian importów. Przeniesienia dopiero po przeglądzie właścicieli modułów, relacji importów, testów i historii.

### Następny zakres dowodowy

Pełny audyt zawartości 714 plików docelowych i 548 referencyjnych, wykrycie rzeczywistych manifestów agentów, granic zależności, uruchomienie testów i ocena CI są nadal **NOT RUN/UNVERIFIED**. Migawki drzew zachowują ścieżki, typ, rozmiar i Git object SHA; służą do wznawiania prac, nie do potwierdzania bezpieczeństwa.
