# Tożsamość rejestrowa — WDA AURELION

## Rozdzielenie pól
| Pole | Wartość | Znaczenie |
|---|---|---|
| Nazwa unikatowa agenta | **WDA AURELION** | własna nazwa egzemplarza w rejestrze |
| Rodzina produktowa / alias historyczny | WDA Photo Agent | pochodzenie produktu P116, nie nazwa ewidencyjna |
| Numer ewidencyjny | **001** | pozycja w rejestrze agentów |
| Agent ID | **AM-OCAF-AI-0001** | tożsamość techniczna API |
| Numer seryjny | **AM-WDA-AUR-2026-000001-DE338AC20B42** | przydzielony numer identyfikacyjny tego egzemplarza; nie certyfikat |
| Numer katalogowy | **AM-OCAF-WDA-001** | identyfikator oferty/katalogu, nie seryjny |
| Numer certyfikatu | **CERT-AM-2026-0001** | numer zarezerwowanego, **niewydanego** certyfikatu |
| Wersja implementacji | 0.2.1-dev | niezatwierdzone wydanie developerskie |
| Status | INCOMPLETE_DEVELOPMENT | nie ukończono portu, integracji i odbioru |

## Algorytm identyfikatora seryjnego
Numer seryjny nadano w lokalnym rejestrze `AGENT_IDENTITY_REGISTRY.json`. Sufiks `DE338AC20B42` to pierwsze 12 znaków zapisu szesnastkowego SHA-256 UTF-8 z kanonicznego tekstu:
```text
MOJEALTEREGO|Knowledge-projects|OCAF-AGENT-001|AM-OCAF-AI-0001|WDA AURELION|P116|2026-10-19
```
Pełny format: `AM-WDA-AUR-2026-000001-` + sufiks. To sposób administracyjnego nadania stabilnego numeru, **nie podpis kryptograficzny, nie dowód autentyczności artefaktu i nie gwarancja globalnej unikatowości na świecie**.

## Reguły dla wszystkich następnych agentów
Przed nadaniem nowego wpisu sprawdź rejestr `AGENT_IDENTITY_REGISTRY.json` i istniejące nazwy/numery; odrzuć duplikat bez względu na wielkość liter. Przydziel osobno unikatową nazwę i numer seryjny, nr ewidencyjny, `agent_id`, nr katalogowy i status certyfikatu. Przy wydaniu wersji produkcyjnej dołącz niezależny podpisany digest binarium, tożsamość OMAs i potwierdzenie OSTc. Nie modyfikuj nadanego numeru seryjnego po podpisaniu egzemplarza; w razie wydania nowego egzemplarza utwórz odrębny wpis.

## Migracja identyfikatora historycznego
W archiwalnym szkicu certyfikatu figurowała **rezerwacja** `AM-OMAS-2026-0001`. Nigdy nie była wpisana do wykonawczej konfiguracji agenta jako numer seryjny. Obecnym **przydzielonym** identyfikatorem egzemplarza jest `AM-WDA-AUR-2026-000001-DE338AC20B42`. Stary szkic zachowano bez zmian dla śladu audytowego.

**Certyfikat pozostaje niewydany**, pole właściciela i podpisy pozostają niepotwierdzone. Numer seryjny jest zarejestrowany, lecz nie poświadcza ukończenia produktu.
