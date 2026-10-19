# Charakterystyka / specyfikacja / zastosowania i wycena — 001

**Cel:** asystent kierunku artystycznego fotografii z preservation-lock. Rozdziela planowanie od zewnętrznych skutków ubocznych. Projekt P116 opisuje analizę wizji i Adobe Firefly; **w Rust dostępny jest tylko offline `photo.plan`**.

**Kontrakt:** wejście `goal` 1–1024 bajtów UTF-8 i `identity_lock:true`; wyjście `agent_id,status=plan_only,identity_lock,workflow[]`. Rust 2021, OCAF ToolExecutor; no network, cost=0, no permissions. Parametry docelowe: 128 MB RAM / 1 CPU / 5000 ms, niepotwierdzona egzekucja zasobów hosta.

**Komponenty:** OCAF Runtime = uruchamianie/kontrola; OMAs = planowany next-gen serwer; OMAt = przyszłe API providerów; OSTc = niewdrożona/niezweryfikowana Kryptografia Topologiczna. Separacja tenantów i signed RPC niezweryfikowane E2E.

**Unikalność:** wewnętrzna kompozycja, nie stwierdzono absolutnej wyłączności rynkowej. Cykl: draft→build→test→sign→operator acceptance→activate→retire. Brak końcowej legal/prawnej klasyfikacji agentów.

**Ceny:** Standard/Serwis 24/7/Premium z ubezpieczeniem dla sprzedaży, dzierżawy (min. 3 mies.) i najmu (min. 7 dni) — **DO WYCENY, NIEZATWIERDZONE**, brak gwarancji ubezpieczeniowych. Konieczne TCO, VAT, licencje, RTO/RPO, limity usług i zakres odpowiedzialności.
