# Serwis i lifecycle — LIMES GIT

**Nazwa:** LIMES GIT · **Ewidencja:** 011 · **Serial:** `AM-GIT-LIM-2026-000011` · **Agent ID:** `AM-OCAF-AI-0011` · **Katalog:** `AM-OCAF-GIT-011` · **Certyfikat:** `CERT-AM-2026-0011` — NIEWYDANY.

Przed użyciem porównać `config/agent.toml`, `config/agent-manifest.json`, stałe Rust i centralny `AGENT_IDENTITY_REGISTRY.json`. Zweryfikować źródłowe testy Python oraz aktualne wyniki Rust, formatowania, Clippy i CLI według commit SHA/run ID.

Jeśli wystąpi naruszenie walidacji lub niepożądana zgoda, wynik traktować jako incydent: zatrzymać użycie adaptera, zachować zanonimizowane wejście/wynik i dokładny commit. **Nie zmieniać flagi Active ani certyfikatu bez autentycznego odbioru.**

Lifecycle `Draft → Tested → Staged` nie uruchamia sieci ani nie upoważnia do produkcji. `Active` zawsze zabroniony. Serwisowe SLA, RTO/RPO, ubezpieczenie, koszt hostingu i zasady wycofania tożsamości sieciowych pozostają nieustalone.
