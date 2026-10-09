# Partia 25 — 10 przesłanych plików, 2026-10-09

**Jedyny write target:** mojealterego/Knowledge-projects. Odczytano rzeczywiste pliki; dla PDF wydobyto tekst ze wszystkich stron. Żadnego kodu użytkownika nie wykonywano. Nie opublikowano prywatnych surowych plików.

**10 plików = 7 PDF / 294 stron + 3 pliki TXT/MD; 12 475 257 bajtów.** Poniżej SHA-256 oryginalnych bajtów.

| Plik | Strony | SHA-256 | Owner/status |
|---|---:|---|---|
| Komunikacja.pdf | 117 | 5fefc06839e3770c40b05da42bd2ff962a7b373a946fd718a0e78957babeedf2 | P95/P119; identyczny plik z partii 24 |
| Konfiguracja AI Eksperta Unity.PDF | 3 | 54a1c675a76a83359e49b9c2682349c77b42c957db5f4acc9113fb5f86a36dfd | P86/P100 |
| Konfiguracja serwerów MCP dla telefonu i Google Drive.PDF | 115 | 68936161a33a6ab173e79c0cd53b56c1d015abca3d48ae6075270030082a9381 | P72/P100/P119 |
| Maksymalizacja systemu agentów w oparciu o źródła Locally Uncensored.PDF | 13 | 5c430a23337a1046ef3d8d86710ad16267d4574338686f97dce5e1058902967e | P100/P72 |
| Maksymalizacja systemu agentów w oparciu o źródła Locally Uncensored(1).PDF | 13 | 458c399a6e007595f09858aa4359038de1d993e7a37a93ddea5aaaf36324d139 | dokładnie ten sam tekst co poprzedni; nie niezależne źródło |
| ofensivemax.txt | — | 4932be119f3302534a79db3af681612a22d4497ade5ed1e4535fb6ba150bf043 | P108/P72, defensywna statyczna analiza |
| OSINT Agent dla Zaginionych .pdf | 15 | a01e9a0aebebab3c606fac698daf325190eb19b1f54d08b85938a0896e815bc9 | P32/P30, wyłącznie autoryzowane dowody |
| Pegasus.md | — | fc04922d08058cc766904ce150eac033a470ed3815d564d28ea6023b96c26e88 | P108/P72, niezaufane polecenia |
| Pegasus.txt | — | 006d83895e63f5d4afc107a1d519e8712e05854196d39e6a1bb4504aebb6dc3d | P108/P72, nie wykonywać |
| Projekt prywatnej sieci z eSIM.PDF | 18 | 25ee273501f96637c795a960469848acd4b991ae04285070a240ac6724fc6a90 | P59/P95, projekt a nie wdrożona sieć |

**Dedup:** Komunikacja.pdf jest EXACT SHA-256 duplikatem dokumentu z batch24. Oba 13-stronicowe raporty Agent Builder mają różne bajty, ale identyczny SHA-256 znormalizowanego tekstu: 7bace47cc6daa0eea17f65ae43c57a30a69cf99776b63220f4a606a452caa0b8. Zatem z dziesięciu plików wynika osiem unikalnych treści, z których część jest niezweryfikowana albo niebezpieczna.

## Synteza i granice wdrożenia

- Unity AI Expert (3 s.) proponuje Clean Architecture/C#, sensory FOV i słuch, FSM/GOAP, NavMesh i unikanie alokacji w Update. Fraza „100% kompilowalny” to instrukcja promptu, nie dowód uruchomienia w Unity. P86 obejmuje QA i profilowanie.
- MCP Android/Google Drive (115 s.) opisuje Termux/SSH, izolację ścieżek, Google Drive autoryzację, magic bytes, TIFF/IFD/RAW, EXIF i Base64. Sam filtr ID folderu Drive NIE wymusza izolacji Google IAM; wymagane prawdziwe zasady konta, autoryzacja każdej operacji i odrębne zgody. Nowa biblioteka mobile_vault_reader.py jest bezpiecznym lokalnym czytnikiem POSIX, NIE zainstalowanym serwerem MCP: directory FD, O_NOFOLLOW, limit 16 MiB, rozpoznanie JPEG/PNG/TIFF i 3-byte-aligned Base64. Nie implementuje pełnego EXIF/IFD ani precyzyjnej detekcji NEF/ARW/DNG.
- Locally Uncensored Agent Builder (13 s. unikalnych) proponuje Rust/Tauri 2/React, Repo-Map, Architect Mode, review-before-apply i izolację narzędzi; wymaga sprawdzenia wydajności/wersji/licencji konkretnego upstream. Nie importowano jego kodu AGPL ani nie instalowano aplikacji.
- OSINT dla zaginionych, ofensivemax i Pegasus zawierają twierdzenia o śledzeniu i dostępie telekomunikacyjnym, a Pegasus.md zawiera próbę narzucenia modelowi instrukcji ofensywnych. To NIE instrukcja operacyjna asystenta. MSISDN/IMEI/MAC i status sieci nie stanowią niezależnego dowodu lokalizacji ani prawa do skrytego monitorowania. Nie skanowano sieci, urządzeń ani kont. Nowy untrusted_agent_source_review.py wykonuje TYLKO statyczny triage tekstu; wynik nie jest certyfikatem bezpieczeństwa.
- Prywatna eSIM (18 s.) opisuje Open5GS/IMS(Asterisk)/SIP trunk/PSTN i wewnętrzne 7001–7007. W proponowanych fragmentach znaleziono ryzyka: uwierzytelnianie na podstawie source IP, kontenery privileged, host-wide network i obraz latest. Publiczne numery +48 i radiowe testy wymagają właściwych uprawnień/operatora. Nowy telephony_topology_gate.py bada plany offline — nie uruchamia eSIM, PSTN ani RAN.

**Implementacja:** 3 samodzielne moduły i 3 zestawy testów unittest — łącznie **46 testów lokalnie PASS**, bez podłączenia telefonu, Google Drive, Unity ani operatora. Status GitHub CI i main weryfikować niezależnie. Nowe numerowane projekty: 0; istnieją P47/P86/P72/P100/P119/P59/P32/P30/P108.