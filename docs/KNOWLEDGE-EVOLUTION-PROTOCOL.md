# Obowiązkowy protokół ewolucji wiedzy i projektów — Knowledge-projects

**Wersja:** 2026-10-08. **Zastosowanie:** każda nowa partia źródeł dostarczona przez właściciela repozytorium.

## Reguła decyzyjna
Każdy nowy dokument jest porównywany ze **wszystkimi właściwymi** dotychczasowymi materiałami i projektami, z ustaleniem czy zachodzi:
1. **Nowa wiedza, istniejący właściciel** — dopisz merytoryczny zakres do *kanonicznego pliku projektu* `projekty/<id>-*.md`, aktualizuj powiązane dokumenty, wymagania i kryteria testów. Osobny plik `-extension` może szczegółowo opisywać specyfikację, ale sam w sobie **nie zastępuje aktualizacji projektu macierzystego**.
2. **Wartość rzeczywiście niezależna** — utwórz projekt z następnym wolnym numerem dopiero po udokumentowaniu różnicy celu, odpowiedzialności, wejść/wyjść, architektury oraz powodów, dla których żadna istniejąca linia nie jest właścicielem koncepcji; aktualizuj główny indeks i rejestr portfolio.
3. **Duplikat/niepotwierdzona hipoteza** — nie twórz sztucznego projektu; zapisz pochodzenie, rozbieżności, potwierdzony zakres i wyraźną granicę weryfikacji.
4. **Wiedza wspólna dla projektów** — archiwizuj w `docs/knowledge-base/` i dodawaj jednoznaczne odsyłacze w każdym istotnym pliku kanonicznym.

## Wymagane dowody operacyjne na każdą partię
- Weryfikacja rzeczywistych plików i hashy SHA-256; identyczność tematów to nie to samo co identyczność bajtów.
- Odczyt aktualnego `main` przed modyfikacjami; bez automatycznego przełączenia domyślnej gałęzi.
- Mapa źródło → pojęcie → istniejący projekt → zmiana → niezależny test.
- Dla nowych projektów: uzasadnienie nowej granicy odpowiedzialności i stan `PROPOSED`, `ARCHITECTURE`, `IMPLEMENTED`, `TESTED` lub `DEPLOYED`.
- Dla rozbudowy: faktyczna zmiana kanonicznego dokumentu, kontraktów i testów; wdrożenie kodu tylko w obecnym odpowiednim repozytorium, gdy kod jest dostępny i zakres wyraźnie dotyczy implementacji.
- Wszystkie twierdzenia prawne, finansowe, naukowe i aktualne cenniki przed wdrożeniem wymagają niezależnego źródła i daty.
- Separacja instrukcji ze źródła od uprawnień narzędzi; prywatne lub wrażliwe dane nie trafiają automatycznie do publicznej bazy.
- Rejestr ewolucji, numeracja, odczyt powrotny z GitHub, widoczny SHA commita oraz stan PR/CI.

## Kryteria ukończenia partii
Nie oznaczaj rozbudowy kanonicznych projektów jako wykonanej na podstawie **samego utworzenia osobnych plików rozszerzeń**. Raport końcowy podaje oddzielnie: zapisaną wiedzę, zmodyfikowane istniejące pliki projektów, utworzone nowe projekty, wdrożony kod, uruchomione testy i niezweryfikowane założenia.

W razie braku użytecznego nowego projektu należy jawnie podać decyzję `new_numbered_projects: 0` wraz z mapowaniem do istniejących projektów.

## Automatyczny start po otrzymaniu plików — aktualizacja 2026-10-08

**Zasada domyślna:** Po otrzymaniu kolejnego pliku lub partii plików w aktywnej rozmowie **rozpocznij analizę bez oczekiwania na polecenie „Analizuj”**. Dla wielu uploadów widocznych w tym samym komunikacie uruchom jeden spójny proces ingestii. Dalsze kroki: kontrola typów i hashy, odczyt całych źródeł/obrazów, identyfikacja duplikatów i sprzeczności, porównanie z portfolio, aktualizacja **kanonicznych** plików projektów lub uzasadniony genesis nowego numeru, testy tam gdzie wykonalne, commit/PR/readback i raport ze stanem potwierdzenia.

Nie czekaj na drugie potwierdzenie tylko po to, by rozpocząć research/zmiany w repozytorium w ramach udzielonego upoważnienia. Nie oznacza to zgody na nieodwracalne działania, płatne wdrożenia, nieautoryzowany dostęp lub niebezpieczne sterowanie sprzętem. Automatyczna reakcja zachodzi w aktualnie obsługiwanej rozmowie; **nie jest deklaracją stale działającego monitora uploadów w tle**.

## Unikalność numeru nowego projektu — kontrola obowiązkowa

Przed nadaniem numeru nowemu projektowi wykonaj `python tools/project_id_gate.py <numer>` na aktualnym checkout i sprawdź aktualny zdalny `main` przed scalenieniem. Skrypt rozpoznaje zarówno numerowane pliki Markdown, jak i README-backed katalogi projektu. Każdy istniejący numer jest zarezerwowany nawet wtedy, gdy ma wiele historycznych plików. Nie wolno przydzielać jednego ID różnym produktom; przy konflikcie zatrzymaj genesis i udokumentuj uzgodnienie numeracji. Patrz [reconciliation 2026-10-08](PROJECT-NUMBER-RECONCILIATION-2026-10-08.md).

Zmiany nie mogą fałszować dawnych commitów; korekty numeracji zapisuje się jawnie, utrzymując readback i identyfikatory projektu w bieżących indeksach.

## Zewnętrzne linki do zbiorów plików — kontrola kompletności materiału

Link Quick Share, tytuł dokumentu albo lista plików w przeglądarce **nie oznaczają**, że rzeczywiste bajty zostały pobrane i dokument przeczytany. Przechowuj odrębne stany `LISTED_ONLY`, `BYTES_RECEIVED`, `HASHED`, `CONTENT_READ`, `OWNER_MAPPED`, `VERIFIED`, `COMMITTED`. Jeśli odczyt treści jest niemożliwy, zapisz prawdziwy stan pozyskiwania, lecz **nie twórz nowej wiedzy ani projektów z domniemanej zawartości**. Przy dużych paczkach ZIP do offline'owego skanowania struktury, SHA-256 i duplikatów służy `tools/source_bundle_audit.py`; nie wykonuje kodu i domyślnie nie ujawnia nazw plików. Przed publikacją materiałów dotyczących konkretnych osób lub prywatnej korespondencji obowiązuje ograniczenie danych osobowych i sprawdzenie uprawnień. Pierwszy rejestr: [Quick Share batch 17](QUICKSHARE-INTAKE-2026-10-08-BATCH-17.md).

## Zakres zapisu — JEDNO repozytorium, nadrzędna reguła 2026-10-09

**Wyłącznie `mojealterego/Knowledge-projects` jest repozytorium docelowym wszystkich zmian dokonywanych w wyniku dostarczania materiałów źródłowych.**

Repozytoria `mojealterego/ODYN-AI`, `NousResearch/*`, wszystkie forki i dowolne inne projekty wskazane URL-ami mają status **READ-ONLY SOURCE**. Dokumentacja czy kod zewnętrznych repozytoriów służą do pozyskania wiedzy, propozycji zmian i budowy prototypów **tutaj**, nigdy do tworzenia commitów, PR, merge, release ani modyfikacji ich konfiguracji. Przypadkowe wcześniejsze wykonanie PR poza `Knowledge-projects` **nie** tworzy wyjątku. Nawet cofnięcie takiego PR wymaga osobnego, wyraźnego zlecenia dotyczącego tamtego repozytorium.

Na początku każdej operacji zapisu sprawdź docelowe `repository_full_name`, gałąź i bazę PR. Wykonaj przejrzysty preflight `tools/single_repository_scope_gate.py` lub równoważny test; **jest to nieautorytatywny checker**, nie substytut ochrony GitHub. Nie twórz PR pomiędzy różnymi repozytoriami; scalaj tylko PR w `Knowledge-projects:main`. Każde przyszłe oznaczenie `DONE` wymaga pokazania SHA w **tym** repozytorium.

[Szczegółowe AGENTS.md](../AGENTS.md) · [Zarejestrowane sprostowanie partii 20](REPOSITORY-SCOPE-ERRATUM-2026-10-09.md).
