# Migracja produktu P116 do agenta WDA AURELION

**Nazwy i pochodzenie:** projekt macierzysty P116 pozostaje w `projekty/116-wda-photo-agent-max`. Jego historyczny projekt `WDA Photo Agent` został przypisany do nowej tożsamości ewidencyjnej `WDA AURELION` (001, serial `AM-WDA-AUR-2026-000001-DE338AC20B42`), bez zmiany jego rodzinnej nazwy w dokumentacji źródłowej.

## Ślad relokacji
1. Bazowy punkt inwentaryzacji `b6930a2c63a3c19d32890768f9aec44e4334eee8`.
2. Historyczne 8 plików `projekty/116-wda-photo-agent-max/source-v2/**` zostało przeniesionych do `OCAF Agents MOJEALTEREGO/OCAF-AGENT-001/legacy-pwa/**` zachowując ich blob SHA.
3. Rust crate znajduje się bezpośrednio w `OCAF-AGENT-001/`, nie w dawnym `001-WDA-Photo-Agent/rust-agent`.
4. Oryginalne skrypty i wcześniejsze szkice certyfikatów utrzymano w archiwum, aby nie tracić pochodzenia.
5. Archiwalna rezerwacja serialu `AM-OMAS-2026-0001` została zastąpiona przydzielonym numerem `AM-WDA-AUR-2026-000001-DE338AC20B42`; `IDENTITY_AND_SERIAL.md` objaśnia sposób nadania.

## Co nie zostało zmigrowane
- działająca interpretacja/edycja zdjęć z użyciem modeli obrazowych lub dostawców Adobe/OpenAI;
- kompletna aplikacja mobilna oparta na Rust;
- bezpieczne publiczne API, authz i limity oraz rzeczywisty klient web produkcyjny;
- serwery agentów OMAs i narzędzi OMAt, transport Zenoh z podpisem i mTLS;
- OSTc i polityka szyfrowanego odczytu źródeł.

Zapis `photo.plan` jest lokalnym **nowym fragmentem kodu**, nie pełnym portem historycznego produktu. Nie należy oznaczać migracji jako `COMPLETE` przed testami jakości, bezpieczeństwa i ewidencji.
