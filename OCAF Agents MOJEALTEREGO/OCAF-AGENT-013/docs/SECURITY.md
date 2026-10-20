# Model zagrożeń i zabezpieczenia — PROVENANT EVIDENCE

**PROVENANT EVIDENCE** · ewidencja **013** · serial `AM-PRV-EVD-2026-000013` · Agent ID `AM-OCAF-AI-0013` · katalog `AM-OCAF-PRV-013` · certyfikat `CERT-AM-2026-000013` (**rezerwacja; niewydany**).

## Reguła integralności
Niezależny rejestr weryfikatorów musi być dostarczony przez operatora do instancji AgentTool. Sam ciąg znaków verifier_ref ani deklaracja pochodzenia w JSON nie stanowi dowodu prawdziwości. Etykieta WERYFIKACJA UDOKUMENTOWANA nadal wymaga przejrzenia faktycznych dowodów.

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
W Pythonie do utworzenia verified wystarczało niepuste verifier_ref. Port Rust wymaga dodatkowo, by identyfikator należał do trusted_verifiers dostarczanych poza żądaniem przez hosta. Waliduje rozmiar claim oraz pól odniesienia, a oddzielny adapter OCAF limituje JSON do 8192 bajtów. To wzmocniona, nie bitowo równoważna wersja modułu Python.

Pozytywny wynik funkcji nie nadaje autoryzacji do wykonania żadnej komendy, wyszukiwania w sieci, przypisywania tożsamości osobie ani wydania certyfikatu. Dostęp OMAs, OMAt i OSTc NIE jest jeszcze realnym połączeniem sieciowym.
