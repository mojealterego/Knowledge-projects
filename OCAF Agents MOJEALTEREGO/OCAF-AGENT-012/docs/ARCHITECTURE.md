# Architektura — FIDES GRANTIS

**FIDES GRANTIS** · ewidencja **012** · serial `AM-GRT-FID-2026-000012` · Agent ID `AM-OCAF-AI-0012` · katalog `AM-OCAF-GRT-012` · certyfikat `CERT-AM-2026-0012` **NIEWYDANY**.

```text
Trusted local operator/host
    |
    v
OCAF Runtime (local in-process ToolExecutor)
    |
    +--> program.claim_review / Rust `src/agent.rs`
    +--> OMAs agent server: NOT DEPLOYED
    +--> OMAt tool server: NOT DEPLOYED
    +--> Zenoh: route validation only / network OFF
    +--> OSTc: source_access.enabled=false / DENY
```

## Moduły

- `src/agent.rs`: Sprawdza wyłącznie repo Knowledge-projects, działania research/draft, brak credentiali, zakres deklarowanych kredytów i tokenów programu Alibaba AI Catalyst, historyczny status General Learning Hacks i wymaganie zweryfikowanej kwalifikowalności przy tworzeniu szkicu.
- `src/zenoh.rs`: rozdzielone logiczne przestrzenie `omas/v1` oraz `omat/v1`, lokalny manifest JSON i adapter OCAF z limitem 4096 B; **brak sieci**.
- `src/security.rs`: walidacja stałej tożsamości, serialu, config TOML i zakazu uruchomienia połączeń.
- `src/lifecycle.rs`: `Active` zawsze zabroniony do formalnego zatwierdzenia.
- `src/main.rs`: lokalne demo, brak zewnętrznych efektów.

**Host trust boundary:** Wynik permitted=true jest lokalną decyzją walidatora; nie potwierdza otrzymania pieniędzy ani grantów, nie składa wniosku, nie uruchamia obliczeń i nie sprawdza na żywo stanu programów. Czas UTC pochodzi od hosta, nie z modelowego JSON.

Produkcyjne signed OMAs RPC, OMAt scoped grants, TLS-bound Zenoh, OSTc i testy niezależnych hostów pozostają niezaimplementowane.
