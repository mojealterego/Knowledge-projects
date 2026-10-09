# P125 — test plan responsive Android launcher / Googlebook

**Stan:** MANUAL QA REQUIRED — brak Android SDK/emulator/device build w tej partii. Testy Python w GitHub Actions są **static source checks**, nie testy UI.

**Źródło:** [Android Developers, 2026-09-22](https://android-developers.googleblog.com/2026/09/adaptive-development-scale-app-googlebook.html). Obecny prototyp jest oparty o Android Views; artykuł rekomenduje Compose/Navigation 3, których **jeszcze nie implementowano**.

## Matryca

| Test | Warunek / krok | Kryterium zaliczenia |
|---|---|---|
| A01 | Android emulator okno 390dp; start po wyborze HOME | Tylko lista aplikacji, szczegóły ukryte |
| A02 | Okno 600–839dp | Lista pojedyncza, przewijanie bez kolizji |
| A03 | Okno >=840dp | Lista po lewej i panel informacji po prawej |
| A04 | Przeciąganie granicy freeform 700→1000→700dp | Widok przełącza się bez ponownego tworzenia przycisków; zachowana lista |
| A05 | Zmiana orientacji urządzenia | Brak przepełnienia/czarnych obszarów |
| A06 | Długa lista 100+ aplikacji | Przewijanie i klawiaturowy focus bez utraty stanu |
| A07 | Klawiatura, D-pad, Enter/Space | Fokus osiąga aplikacje i uruchamia tylko wskazaną |
| A08 | Trackpad / mysz — kliknięcie | Wyłącznie wybrana aplikacja jest uruchamiana |
| A09 | TalkBack i skalowanie czcionek 200% | Etykiety dostępności poprawne, bez uciętych krytycznych kontrolek |
| A10 | Oszczędzanie/odtwarzanie stanu po process recreation | Ostatnia etykieta wyboru odtworzona |
| A11 | Brak aplikacji widocznych przez PackageManager | Komunikat o pustym katalogu |
| A12 | Aplikacja docelowa odinstalowana po załadowaniu listy | Toast błędu i brak crash |
| A13 | Android Default apps → Home app | Systemowy wybór launchera i powrót do poprzedniego HOME możliwe |
| A14 | Android 11+ package visibility | Brak potrzeby QUERY_ALL_PACKAGES i innych szczególnych pozwoleń |
| A15 | Tryb bez sieci/telemetrii | Launcher w pełni lokalny, brak uprawnień INTERNET |
| A16 | Rozmiar okna i density inne niż fizyczny ekran | Breakpoint zależy od szerokości okna w dp |
| A17 | Wymuszona wysoka/niska gęstość / kontrast | Czytelne odstępy i widoczny fokus |
| A18 | Wielookienkowość Googlebook, multi-instance | **NOT IMPLEMENTED / NIE ZALICZONO** — wymaga projektowania i testów |
| A19 | HandoffActivityData / Continue On | **NOT IMPLEMENTED** — wymaga zgody, kontraktu stanu i urządzeń |
| A20 | Navigation 3 / Compose list-detail | **NOT IMPLEMENTED** — obecny dowód to Android Views |
| A21 | Google Play Googlebook badge | **NOT VERIFIED** — program ma osobne kryteria |

## Dowody akceptacyjne

Dla prawdziwych A01–A17 zachować: commit SHA, konfigurację AVD/urządzenia, Android API level, density, wynik Gradle assembleDebug, screenshot przed/po resize, logcat z testu, wynik TalkBack/klawiatury i status PASS/FAIL. Testy A18–A21 pozostają niezrealizowane do momentu osobnego planu.

**Nie używać płatnych chmur, fizycznego telefonu użytkownika ani konta Google Play bez upoważnienia.**
