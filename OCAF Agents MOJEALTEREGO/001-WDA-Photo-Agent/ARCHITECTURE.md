# Architektura agent 001

```text
OPERATOR → [autoryzacja w OCAF Runtime] → ToolManifest photo.plan
  → Rust ToolExecutor → Rust plan(goal, identity_lock=true) → plan_only

Legacy Node/PWA (separately preserved, untrusted) — NOT CONNECTED
Paid image analysis / Adobe adapters — NOT IMPLEMENTED IN RUST
OMAs V2 signed RPC / Zenoh — NOT DEPLOYED
OSTc — NOT IMPLEMENTED
```

Źródło prawdziwych interfejsów: `mojealterego/ocaf-runtime` SHA `52d4f597e0b481678f90cf2073b721ad2e99e253`: `src/executor/mod.rs`, `src/registry/mod.rs`, `src/runtime/mod.rs`, `src/omas_signed.rs`. Pole `ExecutionContext.agent_id` nie stanowi dowodu tożsamości; zewnętrzny gateway musi zweryfikować podpis RPC, tenant i uprawnienia przed wykonaniem. Prace wykonawcze nie dotyczą ocaf-runtime: repo tylko do odczytu.
