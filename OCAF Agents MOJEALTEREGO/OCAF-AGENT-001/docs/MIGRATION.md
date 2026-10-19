# Relokacja P116 → Agent 001

Punkt wejścia `b6930a2c63a3c19d32890768f9aec44e4334eee8`. Każdy pierwotny blob `projekty/116-wda-photo-agent-max/source-v2/**` ma odpowiednik `OCAF Agents MOJEALTEREGO/001-WDA-Photo-Agent/legacy-pwa/**` z identycznym Git blob SHA; kompletność powinna zostać automatycznie sprawdzona po commicie. P116 zostaje projektem produktu i oryginalna dokumentacja architektury nie jest usuwana. W miejscu historycznego `README.md` pozostaje odsyłacz kompatybilności. Wszelkie historyczne dokumenty odnoszące się do dawnej lokalizacji zachowano dla audytu.

Pełny port JS provider API do Rust **nie został wykonany** — obecny Rust realizuje tylko offline `photo.plan`. Nie przedstawiać tego jako zakończonego przeniesienia wszystkich funkcji do OCAF Runtime.


## Zapis po uruchomieniu CI

Dowód kompilacji adaptera OCAF: [run 39198615847](https://github.com/mojealterego/Knowledge-projects/actions/runs/39198615847), commit `d8a03875b4ac7b3c7e4b3dd3361306541a440a`. Uruchomiono 6 testów (3 domenowe, 3 integracyjne), wynik PASS. Nie jest to test produkcyjnej usługi WDA.
