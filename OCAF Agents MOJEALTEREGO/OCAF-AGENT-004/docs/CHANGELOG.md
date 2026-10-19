# Historia zmian — AXIOM FORGE

## 0.1.0-dev — 2026-10-20

- Utworzono własny egzemplarz i numer seryjny: `AM-ENG-AXF-2026-000004`, niezależny od ewidencji 004.
- Wdrożono Rust `graph.review` z deterministyczną walidacją wejścia, bez zewnętrznych efektów.
- Rozdzielono role OMAs server / OMAt server od transportu Zenoh (bez sieci).
- Dodano lokalny OCAF adapter ToolManifest/ToolExecutor, testy negatywne tożsamości, serwisu i bramek lifecycle.
- Dostęp do kodu OSTc jest wyłączony do czasu dostarczenia pełnej specyfikacji i audytu.
- Dodano instrukcje, bezpieczeństwo, ewidencję, projekty umów i niewydany rekord certyfikacyjny.
- Funkcje rzeczywiste wykonywanie workflow, zewnętrzne silniki build/test, niezależna promocja i rollback NIE SĄ ZAIMPLEMENTOWANE.
