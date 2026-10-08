# Batch 15 — mobile home UI, content products, agentic builders, ODYN reports and integrity

**All source names, byte SHA-256 hashes and limits:** [batch 15](../KNOWLEDGE-INGESTION-2026-10-08-BATCH-15.md). All findings are **SOURCE_DERIVED** unless explicitly marked as code/test evidence.

## A. Android AI launchers → new P125 product
10-page 2025–26 ranking proposes Smart Launcher 6, Niagara, KISS/Sentient-like experimental designs, user-behavior categories, local NPU/LLM and permission-driven context. The source conflates intelligent recommendations with permission to read private notifications/location. HOME selection is an Android OS-mediated user choice; feature claims, versions and model performance need current Android documentation and emulator checks.

P125 is the launcher **surface**, not privileged automation: list launchable apps, user tap, offline UI, explicit HOME intent; behavior history and notifications remain opt-in future features. P105 is authorized app/UI execution; P119 is local AI engine; P08 is high-level personal OS. `MainActivity.kt` is source scaffolding, **no APK built**.

## B. AI digital products → P45/P56, not a new business
The input PDF has **9 image-only pages**, presenting text fragments from chapters 8–10. Specifically: niche email newsletter templates, a five-message welcome sequence as a generator idea, Google Docs formatting, Etsy/Gumroad/Shopify/Teachers Pay Teachers distribution and generic bundle pricing/feedback. It cannot support claims from unseen chapters 1–7. Monetization is a hypothesis; copyright and template originality checks, opt-in marketing and honest customer reporting apply. Build a reusable `DigitalProductOffer` with owner rights, user value, editable fields, source_refs, consent_email_marketing, channel rules, price test, measurement and refund policy.

## C. Digital credit systems → fail-closed trust boundaries
24-page report details risks of trusting localStorage/IndexedDB/UI/JavaScript state, proxy-modified requests, parameter tampering and server race/ledger replay. Treat this strictly as **defensive threat material** for P72/P70/P56. Real balance is derived from atomic server-authoritative events with idempotency key, scoped identity, checked authorization, independent reconciliation and rate limits. Client displays, altered API payloads and decoded claims never become authoritative monetary state. No code or instructions for unauthorized manipulation of third-party applications should be published.

## D. OmniStack AI → existing app-builder ownership
34-page report compares Convex Chef, Wasp MAGE, Open Lovable, Open Design, CodinIT.dev, December, Dyad and Bolt.diy. It suggests four layers: sandbox workspace, agent swarm/planning, design/security/test enforcers and rollback-aware deployment. `Architect → Tests → Generation → Shadow QA → Review` is an architecture proposal. Report-simulated supervisor/CI logs and "test passed" paragraphs must not be treated as actual GitHub receipts. Vendor claims and deprecation require checking actual upstream versions and licenses before adapter work. P33 is delivery/control plane, P100 developer UX, P115 orchestrator.

## E. ODYN/Nexus MAS feasibility series → already-owned P114/P115/P72
The five PDFs are versioned reports about a common Nexus core; `(2)` and `(3)` have identical normalized text despite distinct SHA-256. Reported ingredients: bitemporal store/CoALA, dual GGUF inference, GoT/Reflexion, tool synthesis, Skills, HermesClaw, Linux computer use; later versions add ROS Noetic AlterEgo bridge, Jevbridge MCP, Hermes Function Calling, latency budgets, strict input validation, semantic clustering, failure-memory, semantic caching and throwaway Python scripts.

**Critical risk:** unreviewed `importlib` dynamic loading and temporary Python execution are remote-code-execution **surfaces**, not isolation. Source language saying "safe" or "implemented" does not establish actual containment. Formal admission must hash the artifact, source approvals **outside the model**, enforce short expiration and explicit read-only capabilities, then pass code to a separately verified isolation runtime. Our Python `agent_tool_admission_gate.py` performs **admission metadata evaluation only; it never executes a script**. ROS physical action needs additional physical safety/human supervision. Hermes JSON formatting only improves structural interoperability; it cannot guarantee factual correctness or permission.

## F. "Walter White ASI" chemical-cyber persona → P91/P32/P60 safety
The 8-page PDF is a proposed persona prompt/skills table, not proof of a superintelligent model. It mixes toxicology, OSINT/OPSEC and spyware/zero-click references, some with aggressive self-declared surveillance and "99.1% flawless" claims, and malformed example JSON. Extract **only safe data schema lessons**: parse correctness, signed provenance, domain boundaries, legal scope, human expert review, limitations and prohibition of hidden monitoring or dangerous chemical assistance. No independent evidence of scientific validation, real surveillance capability, or access to sensitive telemetry was supplied.

## Evidence chain
```text
PDF BYTES/HASH → SOURCE TEXT/IMAGE AND VERSION
 → EXISTING OWNER OR NEW PRODUCT BOUNDARY
 → THREAT/LEGAL/PRIVACY CAPABILITY GATES
 → ACTUAL CODE + TESTS (IF AVAILABLE)
 → GITHUB CI/READBACK OR "NOT VERIFIED"
```
