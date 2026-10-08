# P124 — Adult Safety Support Evidence Navigation Lab

**Status:** PROPOSED / ARCHITECTURE / NOT DEPLOYED

## Why a new project instead of P36
The 394-page 2017 doctoral study by Agnieszka Filipek concerns adults experiencing domestic abuse, methods of institutional and informal support and the lived limits of help processes. P36 studies **influence and manipulation risks**, P90 automates organizational SOPs, and P32 manages investigative provenance; none owns **trauma-informed, privacy-safe assistance resource navigation in the Polish social/health/legal environment**. P124 has a separate end user, constraints, expert review and release boundary. The thesis's purposive 2016 survey (`n=152`, Białystok 97 / Łomża 31 / Suwałki 24) is not current national prevalence evidence.

## Product scope and anti-goals
Build a **voluntary information and verified-resource discovery tool** for adults seeking to understand available support categories, without asking for disclosure. Explicit non-goals: automated diagnosis, behavior/guilt scoring, recording alleged perpetrators, covert tracking, contacting institutions without permission, third-party impersonation, legal predictions, clinical treatment, violence risk forecasting or autonomously filing reports.

## Information architecture
```text
SELF-DIRECTED CONTEXT / NO ACCOUNT REQUIRED
   ↓
OPTIONAL SUPPORT CATEGORY SELECTION (no sensitive profile)
   ↓
PRIMARY-SOURCE VERIFIED SERVICE DIRECTORY
   ↓
ELIGIBILITY + ACCESSIBILITY + LAST-CHECKED DATE
   ↓
USER-INITIATED PROFESSIONAL CONTACT OUTSIDE THE MODEL
   ↓
PRIVACY-PRESERVING END SESSION
```

## Typed contracts
- `ServiceReference` (jurisdiction, service type, provider, primary-source URL, last verified time, reviewer credentials, restrictions, fallback).
- `SafeSession` (opt-in only, transient state, no automatic logs, no silent notifications or shareable history).
- `RiskReview` (shared-device discovery risk, abusive partner access, unanticipated alerts, language clarity, false assurances, failure to load current services).
- `SourceLimitations` distinguishes 2016 purposive-survey finding from current service information and legally binding rules.

## Priority releases / tests
- P0: current Polish institution/hotline and law checking with qualified safeguarding reviewers; privacy/security and adverse-outcome threat modeling.
- P0: entirely offline prototype using **synthetic** service records until verified; no actual contact capability.
- P1: usability/accessibility evaluation; user-triggered delete/exit; no persistent telemetry.
- P1: automatic stale-directory rejection and human-maintained update workflow.
- P2: verified external integration only after consent, regulatory and safety approval.

## Safety invariants
Never promise anonymity on a shared device; avoid hidden histories, stalking-tool functionality or push reminders; never use a 2017 law citation as current law; every emergency/help option requires current official verification. System does not infer whether a person is suffering abuse and never blames a person for their response to coercion.

## Evidence and related owners
[Source and protection boundary](../../docs/knowledge-base/2026-10-08-adult-safeguarding-evidence-boundary.md) · [Ingestion batch 13](../../docs/KNOWLEDGE-INGESTION-2026-10-08-BATCH-13.md) · P36 influence defenses · P90 procedural assurance · P47 registry.
