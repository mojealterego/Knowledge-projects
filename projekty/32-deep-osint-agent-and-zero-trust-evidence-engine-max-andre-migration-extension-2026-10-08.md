# P32 extension — scoped Andre OSINT identity migration (2026-10-08)

Reference: `docs/knowledge-base/2026-10-08-andre-migration-assurance.md`.

## Scope
The source targets a named family (Nexus-Eye, Omega Infinity, Ghost Protocol, Vantage Point, Aether). This extension is a **migration proposal** for a confirmed owner-approved OSINT product; it is not a command to rebrand P32, the entire portfolio or other repositories.

## Required deliverables
- Reverse-reference graph of Python imports, JS exports, prompt text, user-facing UI, API/CLI identifiers, reports, persisted data and tests.
- Explicit `MigrationEntry` for each external-facing identifier with owner, compatibility alias, test and rollback.
- Persona policy preserving authorization, defensive OSINT scope, source attribution and privacy.
- Versioned artifact filenames, schema migration, import/runtime regression tests and accessibility checks.
- Audit of third-party integrations and truthful automation network identification; no stealth masquerading.

## Prohibited shortcuts
Blind find-and-replace; changing identifiers without import tests; breaking report readers; adding covert-operation features under "OPSEC"; silently changing named brands in unrelated repos.

## Gates
Verified codebase/brand owner; complete inventory; compatibility plan; passing tests; review; reversible deployment and authoritative postcondition readback.

State: proposal, not executed rename.
