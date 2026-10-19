# Architektura — WDA Agent 001 (Rust / OCAF / OMAs / OMAt / Zenoh / OSTc)

```text
OPERATOR
   │
   ▼
OCAF Runtime (policy engine + verifiable execution)
   ├── OMAs — SERVER AGENTÓW
   │     └── identity / agent lifecycle / registered agents
   │
   ├── Zenoh — transport wiadomości (external, off by default)
   │
   └── OMAt — SERVER NARZĘDZI
         └── photo.plan (Rust ToolExecutor, read-only)
                     │
                     ▼
                PhotoPlan result

ŹRÓDŁA AGENTA → OSTc gate [PENDING SPEC / DENY]
```

W tym wydaniu `src/zenoh.rs` implementuje deterministyczne adresowanie logiczne OMAs i OMAt oraz adapter `OcafRuntime::register(ToolManifest, ToolExecutor)`. **Nie otwiera sesji sieciowej** i nie jest serwerem OMAs ani OMAt. Transport produkcyjny wymaga oddzielnego, zweryfikowanego gateway z TLS, autoryzacją i podpisem OMAs związanym z tożsamością RPC. `tenant` i nazwy tras są walidowane, jednak trasa nie poświadcza tożsamości.

`src/security.rs` waliduje statyczną konfigurację i odrzuca próby włączenia źródeł/Zenoh przez samą zmianę TOML. Docelowy mechanizm OSTc musi uzyskać dostarczony projekt protokołu, test vectors, bezpieczną implementację, model zagrożeń oraz niezależny audyt. Nie zastępujemy go innym algorytmem.

`src/lifecycle.rs` egzekwuje stany DRAFT→VALIDATED→TESTED→STAGED→ACTIVE/PAUSED/RETIRED. Produkcyjna aktywacja wymaga pozytywnych bramek: CI, uwierzytelnienia OMAs, OMAt authz, dostępu OSTc i formalnej akceptacji operatora. Wymagane są niezależne dowody poza deklaracją samego modelu.

**Klient fotograficzny legacy** w `legacy-pwa/` zachowany jako nieutwardzone źródło do migracji; nie jest częścią CLI Rust. Schematy: `schemas/`; kontrakt wejścia `goal` i `identity_lock=true`; wyjście `plan_only`.

**Aktualna blokada wydania:** `Lifecycle::transition(Active)` jest zawsze odrzucane, nawet gdy klient poda same `true` w strukturze `ReleaseEvidence`. Prawdziwe poświadczenia muszą być weryfikowane przez niezależnego operatora OMAs/OMAt i ukończoną warstwę OSTc, zanim kod będzie mógł obsługiwać aktywację.
