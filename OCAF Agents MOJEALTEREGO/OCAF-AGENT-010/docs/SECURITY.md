# Model zagrożeń i zabezpieczenia — ZIP WARDEN

**ZIP WARDEN** · ewidencja **010** · serial `AM-ARC-ZWD-2026-000010` · Agent ID `AM-OCAF-AI-0010` · katalog `AM-OCAF-ARC-010` · certyfikat `CERT-AM-2026-000010` (**rezerwacja; niewydany**).

## Reguła integralności
W wersji OCAF nazwy archiwalnych plików nigdy nie są ujawniane. Flagę include_names=false wybiera host na sztywno, a klient nie może włączyć jej w JSON. Wynik oznacza jedynie INVENTORIED_NOT_SEMANTICALLY_REVIEWED.

| Zagrożenie | Lokalna kontrola | Wymóg produkcyjny |
|---|---|---|
| Model wpisuje fałszywe dane wejściowe | strict schema, testy negatywne | niezależny issuer i E2E |
| Zafałszowanie nazwy/serialu | spójność JSON/TOML/stałych Rust i registry | podpis OMAs związany z RPC |
| Ucieczka danych do sieci | sieć wyłączona, brak wysyłania danych | Zenoh mTLS i dowody tenant isolation |
| Nadużycie OMAt | lokalny tool bez side effects | podpisane scoped granty |
| Dostęp do niezaszyfrowanego źródła | `source_access.enabled=false` i DENY | specyfikacja OSTc, audyt i vectors |
| Podstawiony payload | limity rozmiaru i dane walidowane | sandbox, fuzzing, niezależna recenzja |
| Zastąpienie testów prawdziwym dowodem | wynik jasno opisany jako offline | realne źródła i podpisy |
| Nieprawdziwy certyfikat | issued=false i Active DENY | zewnętrzny odbiór |

## Ograniczenia szczególne
Oryginał Python ogranicza ZIP do 500 członków, 750 MiB na element, 3 GiB łącznie. Port w Rust działa wyłącznie na bajtach już w pamięci i dodatkowo ogranicza źródłowy ZIP do 16 MiB, plik do 64 MiB oraz łącznie do 256 MiB. Do lokalnego OCAF dopuszcza tylko 1 MiB binarnego archiwum przesyłanego jako hex. Zabezpieczenie ścieżek Rust odrzuca nie-ASCII zamiast stosować Unicode casefold, co jest celową rozbieżnością.

Pozytywny wynik funkcji nie nadaje autoryzacji do wykonania żadnej komendy, wyszukiwania w sieci, przypisywania tożsamości osobie ani wydania certyfikatu. Dostęp OMAs, OMAt i OSTc NIE jest jeszcze realnym połączeniem sieciowym.
