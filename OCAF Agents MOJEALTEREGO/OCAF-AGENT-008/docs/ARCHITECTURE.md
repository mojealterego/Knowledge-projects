# Architektura — DGM SENTINEL

**Nazwa własna:** DGM SENTINEL · **Numer ewidencyjny:** 008 · **Serial:** `AM-DGM-SNT-2026-000008` · **Agent ID:** `AM-OCAF-AI-0008` · **Katalog:** `AM-OCAF-DGM-008` · **Certyfikat:** `CERT-AM-2026-0008` — NIEWYDANY.

```text
Zaufany host / Operator
   |
   +--> External evidence policy authority [not implemented in this repo]
   |                  |
   |                  v
   +--> OCAF Runtime ToolExecutor (lokalny test)
          |
          +--- OMAs: serwer AGENTÓW [EXTERNAL, NOT DEPLOYED]
          +--- OMAt: serwer NARZĘDZI [EXTERNAL, NOT DEPLOYED]
          +--- Zenoh keyspace [VALIDATION ONLY; NETWORK DISABLED]
          +--- dgm.assess_promotion -> Rust src/agent.rs [OFFLINE]
          +--- OSTc source gate [DENY; SPEC PENDING]
```

## Komponenty

`src/agent.rs` — Strukturalna kontrola 40-znakowych commit SHA, izolowanej nazwy gałęzi integration/, 4 faz ModelScout, TechRecon, Strategist, DGM_Core, listy required_checks, UTC expiry oraz zakazu skutków ubocznych.

`src/zenoh.rs` — lokalny `ToolManifest/ToolExecutor` OCAF; namespace `omas/v1/` i `omat/v1/` to rozdzielne role logiczne, nie połączenia. `src/security.rs` — niezmienna tożsamość z TOML, wyłączony network, OSTc DENY, walidacja tras. `src/lifecycle.rs` — przejście do `Active` bezwarunkowo zabronione. `src/main.rs` — deterministyczny pokaz CLI.

## Host i źródło zaufania

Zatwierdzone kandydatury, potwierdzone CI, receipt faz, rollback i source provenance muszą być wstrzyknięte przez zaufanego hosta jako PromotionTrust. Samo rollback_ref_verified=true lub source_provenance_verified=true w prośbie modelu nie ma autorytatywnej mocy.

Wartości podawane w JSON przez klienta są niezaufane. Pole wynikowe `PromotionDecision { admitted, reasons[], execution_authorized=false, branch_mutated=false }`. Zdalny podpis OMAs i autoryzacja OMAt nie zostały implementowane; test `AuthRequired` świadczy jedynie o odrzuceniu niepodpisanego peera w lokalnym OCAF.

## Brakujące elementy

podpisane prawdziwe CI attestation, weryfikacja git refs z GitHub, operator approvals na serwerze, rzeczywistego uruchamiania etapów DGM i rollback. Multi-host recovery, sandbox, podpisy, SBOM, zgodność produkcyjna i operator acceptance nie zostały zweryfikowane.
