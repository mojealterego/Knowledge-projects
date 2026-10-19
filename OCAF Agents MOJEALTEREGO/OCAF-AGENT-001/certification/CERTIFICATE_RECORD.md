# Rekord certyfikacyjny — WDA AURELION

**STATUS: NIEWYDANY PROJEKT DOKUMENTU — NOT CERTIFIED / NOT PRODUCTION APPROVED**

## Metryka egzemplarza (część ewidencyjna)
| Pole | Treść |
|---|---|
| Nazwa unikatowa | **WDA AURELION** |
| Rodzina produktu | WDA Photo Agent / projekt P116 |
| Numer ewidencyjny | **001** |
| Numer seryjny (PRZYDZIELONY) | **AM-WDA-AUR-2026-000001-DE338AC20B42** |
| Agent ID | **AM-OCAF-AI-0001** |
| Numer katalogowy (PRZYDZIELONY) | **AM-OCAF-WDA-001** |
| Numer certyfikatu (REZERWACJA) | **CERT-AM-2026-0001** |
| Wersja źródeł | **0.2.1-dev**, agent nieukończony |
| Repozytorium | `mojealterego/Knowledge-projects` |
| Katalog | `OCAF Agents MOJEALTEREGO/OCAF-AGENT-001/` |
| Wpis rejestru | `OCAF Agents MOJEALTEREGO/AGENT_IDENTITY_REGISTRY.json` |
| Data nadania identyfikatorów (ewidencyjna) | 2026-10-19 |
| Data wydania certyfikatu | **BRAK** |
| Wystawca, podpis, pieczęć | **BRAK / DO WERYFIKACJI** |
| Prawdziwa fotografia 35×45 mm i logo | **NIEDOSTARCZONE / NIEGENEROWANE** |
| Podpisany artifact digest/SBOM | **BRAK** |
| Operator acceptance | **NIE PRZEPROWADZONO** |

## Rozdzielenie znaczeń
Seria jest nadana ewidencyjnie — nie jest certyfikatem ani poświadczeniem bezpieczeństwa. Odrębny numer certyfikatu `CERT-AM-2026-0001` pozostaje nieaktywną rezerwacją; nie oznacza wydania. Poprzedni szkic z rezerwacją `AM-OMAS-2026-0001` zachowano w [archiwum](legacy-certificate-draft.md) jako dokument historyczny, **nie obowiązujący seryjny**.

## Faktycznie zweryfikowane dotąd
Poprzednie wydanie strukturalne przeszło [run 39203631208](https://github.com/mojealterego/Knowledge-projects/actions/runs/39203631208), commit `bfe879ec2670006b0126cce81dac1a03b0ba29e7`, 12/12 testów. To dowód **starej wersji 0.2.0-dev**, nie wydania 0.2.1-dev. Bieżący wynik CI należy dopisać po weryfikacji.

## Brakujące bramki certyfikacji
- rzeczywiste podpisanie artefaktu i zweryfikowany digest;
- pełny port funkcji fotograficznych i testy jakości;
- OMAs podpis/tożsamość RPC i OMAt autoryzacja;
- Zenoh TLS, multi-host isolation/recovery;
- OSTc szyfrowany dostęp do źródeł (specyfikacja nie dostarczona);
- SBOM, audyt bezpieczeństwa, legal review i podpisany odbiór operatora.

Ograniczenie kodowe: `ACTIVE` zawsze DENIED. **Nie jest wydany żaden certyfikat autentyczności ani odbioru**. Następne czynności zgodnie z [RELEASE_CHECKLIST](../docs/RELEASE_CHECKLIST.md).

## Pola do późniejszego formalnego wypełnienia
Miejsce i data wydania: `[BRAK]`; właściciel prawny: `[DO WERYFIKACJI]`; osoba akceptująca: `[BRAK]`; podpis i pieczęć: `[BRAK]`; digest i podpis builda: `[BRAK]`; protokół odbioru: `[BRAK]`.


## Dowód spójności metryki technicznej (nie certyfikacji)

[Run 39324839309 — SUCCESS](https://github.com/mojealterego/Knowledge-projects/actions/runs/39324839309), 16 testów Rust PASS dla `3fb99da89cc1d7b7ca50277440b8d6ab29888b5d`. Testy obejmują zgodność nazwy i numeru seryjnego oraz negatywną próbę ich podmiany. Nadal **nie nadano podpisu certyfikującego**.
