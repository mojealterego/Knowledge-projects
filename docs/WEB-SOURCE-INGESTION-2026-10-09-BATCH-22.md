# Źródło — Googlebook adaptive development / deduplikacja partii 22

**Data analizy:** 2026-10-09. **Jedyny cel modyfikacji:** `mojealterego/Knowledge-projects`.

**Dokument:** [Android Developers Blog — Land your apps on Googlebook with adaptive development](https://android-developers.googleblog.com/2026/09/adaptive-development-scale-app-googlebook.html), **22 września 2026**, autorzy Fahd Imtiaz, Loryn Hairston. `?m=1` to wariant wyświetlania tego **samego** artykułu. Ten URL był już podany i zinwentaryzowany w **partii 18** (`docs/WEB-SOURCE-INGESTION-2026-10-08-BATCH-18.md`). To **nie jest nowa, niezależna przesłanka**, więc nie zliczamy go jako odkrycia wymagającego nowego projektu.

## Co mówi tekst źródłowy

Artykuł opisuje Googlebook jako kategorię laptopów ze wspólną podstawą Androida. Jako elementy dobrego doświadczenia wskazuje:
1. **Adaptacyjne interfejsy**: więcej niż skalowanie rozciągniętej komórkowej kolumny — pane reflow, list/detail/supporting panes, layout reagujący na **szerokość bieżącego okna** (nie fizycznego ekranu) podczas freeform resize.
2. **Frameworki i narzędzia**: Jetpack Compose, Navigation 3 z `ListDetailSceneStrategy` i `SupportingPaneSceneStrategy`, Grid/FlexBox; MediaQuery/Styles opisane jako **eksperymentalne/nadchodzące**, więc nie przedstawiaj jako stabilnego API.
3. **Wiele sposobów wejścia**: touch, mysz/trackpad, hover/right click, klawiatura i pomoc skrótów; czytelna typografia, jawne cele kliknięć i dostępność.
4. **Wiele okien i instancji**: użytkownik może pracować równolegle; drag-and-drop między oknami, stylizacja caption bar z zachowaniem kontrolek systemowych.
5. **Continue On**: przesłanie kontekstu zadania przez `HandoffActivityData`, nieograniczone do ślepego przesyłania wrażliwych danych; wymaga prawdziwej integracji i zgody.
6. **Testy i dystrybucja**: emulator desktop w Android Studio Canary, wieloinstancyjność, urządzenia wejściowe, wskazówki Android CLI/adaptive skill; Google Play badging oraz Apps Experience Program to **programy o osobnych kryteriach**.

Nie ma w tekście implementacji specyficznej dla `Sovereign Launcher`, kodu gotowego APK, potwierdzonego wpisu do programu Play ani dowodu kompatybilności Samsung S24 Ultra lub konkretnego procesora desktop.

## Faktycznie nowa implementacja w portfolio

W partii 18 do P125 dodano tylko **matrycę planowanych testów**, a `MainActivity.kt` pozostał statycznym pionowym widokiem. Teraz źródło wykorzystano do **modyfikacji kodu** w istniejącym `projekty/125-sovereign-contextual-android-launcher`:

- `MainActivity.kt`: natywny `View.onSizeChanged` bieżącego okna → szerokość dp → kompaktowa lista lub **lista + panel informacyjny przy >=840dp**. Wykorzystuje klasyczne `LinearLayout`, `ScrollView` i `TextView`, **nie deklaruje wdrożenia Compose/Navigation 3**.
- Odświeżenie orientacji i udziału paneli po zmianie szerokości bez odtwarzania całej listy aplikacji; etykieta ostatnio wybranej aplikacji odtwarzana z `savedInstanceState`.
- Kontrola czytelności przycisków, minimalny rozmiar 48dp, focus/keyboard/D-pad i etykiety dostępności; polskie napisy przeniesione do `res/values/strings.xml`.
- `AndroidManifest.xml`: `android:resizeableActivity="true"` dla HOME. Żadnych uprawnień dostępu do lokalizacji, Internetu, UsageStats, powiadomień, Accessibility ani kontaktów.
- **Uruchomienie aplikacji następuje wyłącznie po świadomej aktywacji przycisku przez użytkownika.**
- `tools/test_googlebook_adaptive_launcher.py`: statyczne testy deklarowanych invariantów, **nie emulator / build / test na Googlebook**.
- `android/ADAPTIVE-QA.md`: rozpisane scenariusze ręcznego sprawdzenia.

**Ograniczenia:** klasyczne Views nie implementują Navigation 3, przekazywania HandoffActivityData, natywnych menu kontekstowych, drag-and-drop, multi-instance ani oficjalnego badging. `onSizeChanged` kod musi przejść realny Android build, kontrolę działania oraz testy dostępności; test tekstowego kontraktu nie dowodzi poprawności renderowania. Brak urządzenia, emulatora, podpisanego APK, certyfikacji i dystrybucji Play.

**Dalszy etap:** uruchomić Gradle/Android SDK w upoważnionym środowisku, wykonać manualny test matrycy, usunąć błędy wykryte przy multi-window, rozważyć Compose/Navigation 3 jako **osobny przyszły migracyjny PR**, włączyć CI build, potem odrębnie projektować Continue On z bezpieczną obsługą identyfikatorów/zgody. Nie należy włączać wysyłania prywatnych danych tylko dlatego, że producent opisał tę możliwość.
