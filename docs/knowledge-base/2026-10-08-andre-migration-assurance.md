# Andre OSINT naming migration — compatibility and assurance study, 2026-10-08

Source: `Zmiana Nazwy Projektu na Andre_ Manualnie i AI.pdf`.
Owner: P32 Deep OSINT Agent, with links to P30 Evidence Control, P100 development/security tooling, P108 adversarial testing.

## Decision
The source proposes renaming the **specific** Nexus-Eye / Omega Infinity / Ghost Protocol / Vantage Point / Aether OSINT lineage to "Andre". This is a scoped, proposed migration. It does **not** authorize renaming the entire `Knowledge-projects` portfolio, other repos or the user's established ODYN/OMEGA brands. The PDF refers to source code from other documents; availability of those exact codebases has not been established in this repository.

## Identity is more than branding
A safe migration inventories:
- Python class symbols, constructors, imports, module paths and plugin names.
- React/JS component exports, routes, HTML titles, UI strings, Canvas labels and tests.
- System prompts, persona identity and typed agent output conventions.
- Artifact filenames, archive paths, report metadata and provenance IDs.
- Package identifiers, external integrations, CI labels, schemas, cache keys and persisted state.
- Telemetry, log correlation IDs and network User-Agent identifiers.

Do **not** change declared network identity to impersonate a browser, evade provider access controls or conceal unauthorized scanning; identify legitimate automation truthfully.

## Migration plan
```text
INVENTORY + OWNER APPROVAL
 -> reference graph / externally consumed identifiers
 -> migration manifest with before/after names
 -> compatibility aliases / explicit deprecation
 -> staged implementation branch
 -> static imports + UI test + data schema + prompt consistency
 -> permissions and evidence boundary tests
 -> rollback plan and diff review
 -> deployment ONLY after appropriate approval
```

## Versioned data contract
```yaml
MigrationEntry:
  old_identifier: string
  new_identifier: string
  kind: symbol|module|ui|prompt|artifact|schema|api|telemetry
  owner_project: P32
  file_path: string
  compatibility_strategy: alias|versioned_api|data_migration|documentation_only
  dependent_consumers: []
  verification_evidence: []
  rollback_method: string
  review_status: proposed
```

## Hard gates
1. A complete, actual inventory of affected code, not source-PDF file names alone.
2. No changed public API/export or serialized key without compatibility or migration strategy.
3. Persona/prompt changes must preserve model/tool authorization and evidence provenance.
4. The public OSINT boundary stays explicitly authorized; no stealth/evasion-by-default.
5. No global automated rename without per-module regression testing and backup.
6. Asset filenames and report schema are backwards-compatible or migrated with tests.
7. Rebranding may proceed only when target codebase and brand decision are confirmed.

Status: **architectural migration plan**; source code was not renamed in this ingestion.
