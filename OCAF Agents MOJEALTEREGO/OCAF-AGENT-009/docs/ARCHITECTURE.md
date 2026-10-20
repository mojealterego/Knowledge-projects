# Architektura — CLOUD FISCALIS

**Nazwa własna:** CLOUD FISCALIS · **Numer ewidencyjny:** 009 · **Serial:** `AM-CLD-FIS-2026-000009` · **Agent ID:** `AM-OCAF-AI-0009` · **Katalog:** `AM-OCAF-CLD-009` · **Certyfikat:** `CERT-AM-2026-0009` — NIEWYDANY.

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
          +--- cloud.review_budget -> Rust src/agent.rs [OFFLINE]
          +--- OSTc source gate [DENY; SPEC PENDING]
```

## Komponenty

`src/agent.rs` — Walidacja ośmiu kategorii produktów i akcji, strict decimal-string USD przez rust_decimal, request_id, project_scope, allowlist grantów, budżetu i zakazu sekretów w promptach. Dokumentacja publiczna wyłącznie przy zerowym koszcie i publicznej klasyfikacji.

`src/zenoh.rs` — lokalny `ToolManifest/ToolExecutor` OCAF; namespace `omas/v1/` i `omat/v1/` to rozdzielne role logiczne, nie połączenia. `src/security.rs` — niezmienna tożsamość z TOML, wyłączony network, OSTc DENY, walidacja tras. `src/lifecycle.rs` — przejście do `Active` bezwarunkowo zabronione. `src/main.rs` — deterministyczny pokaz CLI.

## Host i źródło zaufania

CloudApproval powstaje w niezależnym systemie polityki właściciela/hosta i jest przekazywane do AgentTool; pole modelu explicit_external_side_effect_approval nie stanowi IAM, rozliczenia ani zgody na rzeczywiste wykonanie.

Wartości podawane w JSON przez klienta są niezaufane. Pole wynikowe `CloudVerdict { allowed, reasons[], execution_authorized=false, charge_incurred=false }`. Zdalny podpis OMAs i autoryzacja OMAt nie zostały implementowane; test `AuthRequired` świadczy jedynie o odrzuceniu niepodpisanego peera w lokalnym OCAF.

## Brakujące elementy

Google Cloud IAM auth, billing/quota API, zaszyfrowane sekrety, wykonanie działań, niezależnie podpisane granty i interfejs dla użytkownika. Multi-host recovery, sandbox, podpisy, SBOM, zgodność produkcyjna i operator acceptance nie zostały zweryfikowane.
