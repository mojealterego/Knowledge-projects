#  Knowledge Base

This directory is the durable project archive for documentation and repository knowledge extracted for building OpenAI-based agents, MCP integrations, coding agents, RAG systems, realtime/voice agents, safety layers, and evaluation harnesses.

## Current status

The repository is documentation-first. Source-derived knowledge is archived here before it is used as implementation guidance.

## Materials currently absorbed

- OpenAI Plugins / Apps architecture: Skills, MCP servers, optional UI, packaging, testing, submission, metadata optimization, security/privacy.
- MCP server design: tools, schemas, annotations, structured results, auth, OAuth 2.1, skills import, deployment and review requirements.
- MCP Apps / ChatGPT UI bridge: shared MCP Apps standard, `window.openai` extensions, UI resources, state, file handling, CSP, rendering patterns.
- Checkout API concepts and constraints.
- OpenAI API / Responses API fundamentals, model selection, prompts/instructions, function calling and structured outputs.
- Agents SDK fundamentals: Agent definitions, tools, MCP, handoffs, agents-as-tools, sessions, running agents, results/state, guardrails and human review, tracing.
- Tool Search and Programmatic Tool Calling.
- Models/providers and transport configuration.
- TypeScript Agents SDK quickstart and configuration.
- Sandbox Agents architecture and execution model.
- Sandbox concepts: `SandboxAgent`, `Manifest`, capabilities, sandbox sessions, `RunState`, `sessionState`, snapshots, lifecycle ownership, skills/memory/compaction, filesystem/shell, mounts, permissions, path grants, credential boundaries, exposed ports and sandbox clients.
- Sandbox client options: Unix local, Docker, hosted providers, session ownership, resume/snapshot semantics, materialization limits and mount credential safety.

## Repository-derived knowledge

- `openai/codex`, `openai/symphony`, `openai/plugins`, `openai/openai-mcpkit` and `openai/openai-apps-sdk-examples` and other high-value OpenAI repositories have been mined for reusable engineering patterns.
- The consolidated repository map is in `docs/knowledge-base/openai-org-repository-knowledge.md`.
- Earlier extractions from `openai/whisper`, `openai/evals`, `openai/openai-python`, `openai/tiktoken`, and `openai/openai-node` are archived separately.

## Cross-platform knowledge

- Official/public technical material for OpenAI developer/platform documentation, MCP, IBM Cloud, Activepieces, FlutterFlow, Expo, Replit, Render, Railway and Vercel has been analyzed for reusable architecture and production patterns.
- The consolidated cross-platform extraction is in `docs/knowledge-base/platforms-openai-mcp-cloud-builders.md`.
- `docs/knowledge-base/mcp-and-agent-platforms-second-wave.md` captures the second-wave MCP, framework and low-code agent analysis.
- `docs/knowledge-base/ai-app-builders-2026.md` captures the AI web/mobile app-builder and agentic-development ecosystem, including coding agents, visual builders, enterprise AI workflow platforms, and agent infrastructure.
- The latest extraction emphasizes separation between product builders, agent runtimes, workflow engines, tools/MCP, sandboxes, deployment substrates and observability/evaluation systems.

## Key architectural themes

1. Agent reasoning is only one layer: production systems also need explicit tools, state/memory, authorization, safety, sandboxing, observability and evaluation.
2. Side effects require stronger controls than read-only retrieval.
3. Context should be incremental, bounded and task-relevant.
4. Typed schemas should bridge probabilistic model output and deterministic application logic.
5. Specialize agents when instruction/tool/policy surfaces materially diverge.
6. Treat evaluations and traces as part of the runtime lifecycle, not merely release-time tests.
7. Keep authoritative business state server-side and treat UI/widget state as ephemeral.
8. Separate deterministic workflows from model-directed agent loops.
9. Isolate development, preview/staging, production and untrusted execution environments.
10. Treat credentials, deployment and infrastructure mutation as explicit security-sensitive capabilities.
11. Separate knowledge from capability: Skills teach the method; tools/MCP provide access or side effects.
12. Treat infrastructure as typed state that can be inspected, planned, validated, approved, deployed and observed.
13. Build recovery into the runtime through retries, rollback, staged changes, resumability and durable orchestration.
14. MCP is a capability integration protocol, not an agent framework; keep protocol, orchestration and infrastructure concerns separate.
15. Prefer hybrid systems: model-directed reasoning where choice is needed, deterministic workflows where the path is known.
16. A schema should be treated as an executable contract whose validity must remain aligned with runtime parameters.
17. AI app builders reduce time-to-first-application but do not eliminate software architecture, testing, security, state management or deployment boundaries.
18. AI coding environments and visual builders should preserve an escape hatch to inspect, test, export, version and operate the resulting application as ordinary software.
19. Human review should be modeled as a resumable runtime state transition for high-impact operations rather than as an informal UI confirmation.
20. Agent hosting is a separate architectural layer from agent design; builders, runtimes and execution substrates should remain independently replaceable.

## Source principle

The supplied documentation and repository source are treated as the primary evidence for this project. Later synthesis should verify exact details against archived source materials rather than guessing from memory. Where this knowledge base summarizes repository or web content, it should not be read as a claim that every file on every listed platform has been exhaustively analyzed.


## Ingestion — 2026-10-08, 10 uploaded source files

Five existing subjects from Iteration 16 were recognized without duplicating project ownership. New source-derived analyses: [Gemini inference optimization](2026-10-08-gemini-performance-evidence.md), [visual influence defense](2026-10-08-visual-influence-defense.md), [Andre migration assurance](2026-10-08-andre-migration-assurance.md), [public-assembly historical law reference](2026-10-08-assembly-law-reference-poland.md), and [HTML mistaken for audio](2026-10-08-voice-preview-html-validation.md). Full provenance/sha256 manifest: [batch ingestion log](../KNOWLEDGE-INGESTION-2026-10-08-BATCH-10.md); [project evolution](../PROJECT-EVOLUTION-2026-10-08-BATCH-10.md).

These are source-derived research and architecture notes, not verified current law, provider prices, experimental neuroscience, legal deployment readiness, a code migration, or confirmed financial results.

## 2026-10-08 — batch 11: automatyczna ingestia 10 plików

[Manifest źródeł SHA-256](../KNOWLEDGE-INGESTION-2026-10-08-BATCH-11.md) ·
[Synteza OmniCore GCP, CCR UE5, VANTAGE, AI](2026-10-08-corpus-cloud-ccr-vantage-ai-2026-batch11.md) ·
[Bezpieczna ekstrakcja instrukcji VICTRIX](2026-10-08-gas-appliance-manual-safety-boundary.md) ·
[Ograniczony fragment strategii marki](2026-10-08-brand-strategy-excerpt-notes.md) ·
[Decyzje projektowe P24/P31/P32/P66/P89/P123](../PROJECT-EVOLUTION-2026-10-08-BATCH-11.md).

Dwa identyczne bajtowo raporty monetizacji nie są ponownie promowane do projektów. PDF VANTAGE zawiera zrzuty kodu i skaner symulowany; fragment książki nie reprezentuje całej publikacji. P123 jest domenowo nowym projektem referencyjnym, bez możliwości sterowania gazem.

## 2026-10-08 — batch 12: persona, history, OmniCore MVP, LLM optimization, hidden commands, SOP and Apeiron

- [Input hashes / owner map / source boundaries](../KNOWLEDGE-INGESTION-2026-10-08-BATCH-12.md)
- [Interdisciplinary synthesis and evidence caveats](2026-10-08-corpus-persona-omnis-models-history-influence-sop-apeiron.md)
- [Canonical project evolution and no-duplicate genesis decision](../PROJECT-EVOLUTION-2026-10-08-BATCH-12.md)
- [P122 CHEMIA versus P123 gas project identity reconciliation](../PROJECT-NUMBER-RECONCILIATION-2026-10-08.md)

Updated P01, P11, P19, P23, P36, P47, P80, P87, P90, P91. No new concept ID allocated to previously owned source domains.

## Batch 13 — 2026-10-08: AI systems, OSINT, influence, marketing and safeguarding

- [10 sources, file hashes, deduplication and project decisions](../KNOWLEDGE-INGESTION-2026-10-08-BATCH-13.md).
- [Interdisciplinary source-derived analysis](2026-10-08-corpus-agents-osint-marketing-influence-and-safeguarding.md).
- [Adult domestic-abuse support: survey limitations, privacy and professional safeguards](2026-10-08-adult-safeguarding-evidence-boundary.md).
- [Canonical project evolution / new P124](../PROJECT-EVOLUTION-2026-10-08-BATCH-13.md).
- [AETHER prototype](aether-nexus-omega-interface-prototype-analysis.md) remains a simulated browser demonstration; no actual kernel/secure-link capability demonstrated.

No copyrighted books or case histories republished. 2014 marketing data, 2016 abuse-survivor survey and 2017 legal descriptions are historical sources, not evidence of current laws or services.

## Batch 14 — 2026-10-08: AURA, GCG, OmniCore, strategy and linguistic evidence

- [Source SHA-256 manifest and exact-text duplicate findings](../KNOWLEDGE-INGESTION-2026-10-08-BATCH-14.md).
- [AURA 60-card gap assessment, thermochrom contradiction and user-autonomy requirements](2026-10-08-aura-deck-spec-gap-and-safety.md).
- [Defensive GCG, verifiable OmniCore, SOP, strategic competences, Polish lexical ambiguity and ASUS manual limits](2026-10-08-omnicore-gcg-strategy-language-hardware.md).
- [Canonical-project evolution report](../PROJECT-EVOLUTION-2026-10-08-BATCH-14.md).

No new numbered project: existing P50/P71, P60/P108, P09/P26/P80, P90/P115, P66 and P75 own the relevant capabilities. Source limitations are not silently promoted to verified behavior.

## 2026-10-08 — batch 15: Android launchers, app builders, credit integrity and ODYN evidence

- [Source manifest / 10 PDFs / 146 pages / SHA-256 / near-duplicate feasibility reports](../KNOWLEDGE-INGESTION-2026-10-08-BATCH-15.md).
- [Source-grounded technical synthesis and safety boundaries](2026-10-08-corpus-mobile-builders-security-odyn.md).
- [Canonical project update report and new P125](../PROJECT-EVOLUTION-2026-10-08-BATCH-15.md).
- [P125 Kotlin HOME launcher](../../projekty/125-sovereign-contextual-android-launcher/README.md) — **Android source only**, no compiled/installed APK.
- [Non-executing AI-generated tool admission prototype](../../tools/agent_tool_admission_gate.py) — digest, external review map and read-only scope checks, not a real isolation runtime.

Scanned digital-product pages cover only chapter fragments 8–10, not "7" complete offerings. The five ODYN reports are related version snapshots, with (2)/(3) equal after text normalization. Their assertions of previously implemented modules were not independently verified in ODYN-AI.

## 2026-10-08 — batch 16: DGM, GGUF Agent Studio and IBM Cloud

- [10 PDF source hashes, 556 pages, duplicate and verification notes](../KNOWLEDGE-INGESTION-2026-10-08-BATCH-16.md).
- [Detailed source-derived knowledge / evidence boundaries](2026-10-08-dgm-gguf-ibm-evaluation.md).
- [Project evolution decisions](../PROJECT-EVOLUTION-2026-10-08-BATCH-16.md).
- [P126 IBM Cloud Cognitive Game Backend Reference](../../projekty/126-ibm-cloud-cognitive-game-backend-reference/README.md).
- [Non-executing DGM promotion gate](../../tools/dgm_cycle_gate.py) and [offline IBM conflict/state tests](../../tools/test_ibm_game_state_reference.py).

GGUF input is a 372-page discussion/document, not a packaged running application. DGM (1)/(2) have identical normalized text, not independent implementation evidence. IBM cloud claims not verified in a live account.
