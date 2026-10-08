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
