# Serwis i lifecycle — VERITAS TRACE

**Nazwa:** VERITAS TRACE · **Ewidencja:** 010 · **Serial:** `AM-OSN-VRT-2026-000010` · **Agent ID:** `AM-OCAF-AI-0010` · **Katalog:** `AM-OCAF-OSN-010` · **Certyfikat:** `CERT-AM-2026-0010` — NIEWYDANY.

Przed użyciem porównać `config/agent.toml`, `config/agent-manifest.json`, stałe Rust i centralny `AGENT_IDENTITY_REGISTRY.json`. Zweryfikować źródłowe testy Python oraz aktualne wyniki Rust, formatowania, Clippy i CLI według commit SHA/run ID.

Jeśli wystąpi naruszenie walidacji lub niepożądana zgoda, wynik traktować jako incydent: zatrzymać użycie adaptera, zachować zanonimizowane wejście/wynik i dokładny commit. **Nie zmieniać flagi Active ani certyfikatu bez autentycznego odbioru.**

Lifecycle `Draft → Tested → Staged` nie uruchamia sieci ani nie upoważnia do produkcji. `Active` zawsze zabroniony. Serwisowe SLA, RTO/RPO, ubezpieczenie, koszt hostingu i zasady wycofania tożsamości sieciowych pozostają nieustalone.
