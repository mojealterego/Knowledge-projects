# Project 116 — WDA Photo Agent MAX

## Status
PROPOSED → ARCHITECTURE BASELINE → IMPLEMENTATION INGESTED 2026-09-11

## Mission
WDA Photo Agent MAX is a mobile-first photographic art-direction and AI post-production runtime. It separates forensic observation, identity preservation, production planning, generative execution, quality gates and versioned evidence.

## Why this is distinct
P100 owns the general NeXus AI developer environment and its MCP/desktop control plane. P114 owns the reusable cognitive memory/verification substrate. P116 owns the **domain-specific photographic production runtime**: visual diagnosis → identity/structure lock → retouch plan → controlled generative operation → quality verification.

## Source implementation ingested
The supplied WDA 2.0 implementation contains:

- forensic image analysis through the OpenAI Responses API;
- identity, pose and proportion preservation as first-class constraints;
- WDA conversational orchestration;
- Adobe Firefly Generate and Generative Fill integration;
- painted generative masks;
- Image 5 model selection;
- local browser session history;
- PWA/service-worker/camera support;
- server-side API credentials;
- quality-gate output;
- non-destructive workflow language.

The supplied files are an implementation witness, not proof of production readiness. The uploaded server expects static assets below `public/` and a package-level `npm start`, while the upload contained the files flat and did not include `package.json`. The normalized project therefore defines the required runtime packaging and hardening boundary.

## Canonical architecture

```text
PHOTO / OPERATOR
      ↓
INPUT VALIDATION
      ↓
FORENSIC VISION ANALYSIS
      ↓
IDENTITY + STRUCTURE LOCK
      ↓
DIRECTOR PLAN
      ↓
FIREFLY PROMPT / OPERATION CONTRACT
      ↓
CAPABILITY + POLICY GATE
      ↓
ADOBE GENERATION / FILL
      ↓
PROVIDER READBACK
      ↓
QUALITY GATE
      ↓
VERSIONED SESSION EVIDENCE
```

## Core contracts

### 1. Analysis contract

Required fields:

- `subject`
- `identity_lock[]`
- `composition`
- `lighting`
- `skin_texture`
- `color`
- `background`
- `retouch_plan[]`
- `firefly_prompt`
- `risks[]`
- `quality_checks[]`

The analysis model must distinguish visible observation from recommendation and must not invent invisible details.

### 2. Identity lock

The default workflow preserves:

- facial identity;
- facial structure;
- body proportions;
- pose;
- wardrobe;
- authentic texture.

A generative operation is not permitted to silently relax the identity lock.

### 3. Execution boundary

```text
PROPOSAL ≠ EXECUTION

LLM ANALYSIS
    ↓
PRODUCTION PROMPT
    ↓
AUTHORIZED FIREFLY CALL
    ↓
PROVIDER RESPONSE
    ↓
RESULT URL / ARTIFACT
    ↓
QUALITY GATE
```

The UI must never claim that a generative edit succeeded solely because a prompt was generated or a button was clicked.

## MCP / agent integration

P116 can be exposed to the broader agent fabric through typed capabilities such as:

- `photo.inspect`
- `photo.analyze`
- `photo.build_prompt`
- `photo.create_mask`
- `photo.generate`
- `photo.generative_fill`
- `photo.quality_gate`
- `photo.export_evidence`

Each capability must declare side effects, required scope, credential class, cost/time budget and verification method. MCP exposure is a capability boundary, not authorization.

## Security

- API credentials remain server-side.
- Browser code never receives provider secrets.
- Request bodies are bounded.
- Requests are rate-limited.
- Static file serving is path-contained under `public/`.
- Security headers are emitted for browser responses.
- Provider errors are surfaced without converting them into false success states.
- Hidden endpoint discovery, credential extraction, authentication bypass and CAPTCHA-token harvesting are outside the architecture.
- Uploaded source images are treated as user-controlled data and are not implicitly trusted as instructions.

## Internationalization

Native PL/EN support is required across:

- UI labels;
- buttons;
- placeholders;
- status messages;
- health/error responses;
- quality-gate messaging.

The selected language is persisted locally and transmitted to the server using `X-WDA-Language`.

## UX / mobile architecture

- camera capture and gallery upload are first-class input paths;
- the source image is visually separated from generated output;
- masking uses a dedicated canvas layer;
- session history is version-oriented rather than destructive;
- analysis, mask, history and studio are separate task surfaces;
- reduced-motion behavior should be added before production release;
- WCAG 2.2 AA/AAA compliance remains a verification target, not an assumed property.

## Integration with repository architecture

- **P100** — general MCP, desktop, model and multimodal orchestration shell.
- **P114** — memory, provenance, verification and reversible state substrate.
- **P72** — cross-framework assurance and postcondition verification.
- **P97** — multimedia production lineage.
- **P57** — multimodal intelligence.
- **P40** — provider-neutral agentic execution fabric.

## Verification program

1. Unit-test analysis schema conformance.
2. Reject malformed/oversized image payloads.
3. Verify identity-lock fields cannot disappear from analysis responses.
4. Verify missing OpenAI/Adobe credentials fail closed.
5. Verify provider calls are never represented as successful before readback.
6. Verify static-path traversal is rejected.
7. Verify rate limiting and security headers.
8. Verify PL/EN switching without page reload.
9. Verify mask coordinate scaling across mobile and desktop viewports.
10. Verify source image remains unchanged through analysis and reset flows.
11. Verify generated result/history entries are separately versioned.
12. Verify Firefly Generate and Fill adapters against current provider contracts before release.
13. Add browser E2E tests for upload → analyze → prompt → generate → quality gate.
14. Add adversarial tests for prompt injection embedded in image metadata or model output.

## Definition of Done

P116 advances beyond architecture baseline only when:

- the runtime package installs and starts reproducibly;
- PL/EN coverage is complete;
- provider adapters pass current API contract tests;
- identity-lock regression tests pass;
- all consequential provider calls have authoritative readback;
- security/path/rate-limit tests pass;
- generated artifacts have deterministic session lineage;
- quality-gate failures prevent silent promotion;
- mobile and desktop E2E suites pass.

## Evidence classification

- supplied WDA source code: **OBSERVED / SOURCE-DERIVED**;
- normalized packaging and hardening: **IMPLEMENTATION TARGET**;
- provider capability/version compatibility: **CURRENT-DOCS-DEPENDENT**;
- visual quality claims: **UNVERIFIED until benchmarked**;
- autonomous photographic judgment: **PROPOSED capability, not established fact**.


---

## Wydzielenie agenta 001 z projektu P116 — 2026-10-19

P116 pozostaje kanonicznym projektem i roadmapą. Kod WDA `source-v2/` został przeniesiony zachowawczo do [`OCAF Agents MOJEALTEREGO/001-WDA-Photo-Agent/legacy-pwa`](../../OCAF%20Agents%20MOJEALTEREGO/001-WDA-Photo-Agent/legacy-pwa/). Rust + manifest + dokumentacja + projekt certyfikatu: [Agent 001](../../OCAF%20Agents%20MOJEALTEREGO/001-WDA-Photo-Agent/README.md). Integracja usług fotografii z produkcyjnym OCAF Runtime pozostaje **PARTIAL**, nie zweryfikowano E2E.
