# Google / GitHub source synthesis — batch 18 (2026-10-08)

**Primary source manifest:** [batch 18 URL registry](../WEB-SOURCE-INGESTION-2026-10-08-BATCH-18.md). This synthesis distinguishes public documentation, feature hypotheses, policy design and executable offline code. Exact API compatibility and commercial entitlements remain unverified without a real authorized account/runtime.

## 1. Multimodal RAG adapter for existing Knowledge-projects

[Google Developers Blog, 2026-04-30](https://developers.googleblog.com/en/building-with-gemini-embedding-2/) describes `gemini-embedding-2` mapping text, images, video, audio and documents to a common semantic space; [official Gemini API model card](https://ai.google.dev/gemini-api/docs/models/gemini-embedding-2) lists 128–3072 embedding dimensions with 768/1536/3072 recommended. The article's per-call examples state up to **8192 text tokens, 6 images, 120 sec video, 180 sec audio and 6 PDF pages**.

**P29** owns corpus metadata/external-data governance, **P57** multimodal understanding, **P114** valid-time/recorded-time evidence, **P21** Google provider adapter, **P17** cost/quality selection. Proposed pipeline:
```text
user-authorized file/source → SHA-256 and rights/PII classification
→ modality-aware chunking respecting documented Gemini Embedding 2 limits
→ explicit remote-processing approval and per-provider cost budget
→ model-version/dimension/task-specific embeddings (if service actually connected)
→ provenance-preserving retrieval + rerank + temporal contradiction checks
→ citation-spans + independently validated answer
```

**Executable part:** `tools/multimodal_embedding_intake_gate.py` is a purely local, non-network eligibility check. It **does not create embeddings**, claim to redact PII, calculate billable tokens or independently establish a user's consent. Exact token counts/durations/pages must come from trusted pre-processing; external sensitive-source approvals must be supplied by a separate policy system. Do not mix embedding spaces from model revisions/dimensions or fuse ungrounded multimodal results.

**Acceptance:** model pin and timestamp, representation unit tests, 6-page / 120-sec / 180-sec rejection fixtures, Polish queries with known reference answers, Recall@k and citation-span correctness, latency, retention, user data export/deletion and opt-in. Commercial claims of specific increases in recall are product/customer case studies, not baseline for Knowledge-projects.

## 2. Local Gemma 4 family and hybrid inference

[Google's 2026-04-02 launch](https://blog.google/innovation-and-ai/technology/developers-tools/gemma-4/) announces E2B, E4B, 26B MoE and 31B Dense with Apache-2.0. **P37/P17** should have a `LocalModelCandidate` record:
```yaml
model_family: gemma-4
variant: E2B|E4B|26B-MoE|31B-Dense
weight_repository: null
weight_sha256: null
license_reviewed: false
tokenizer_and_prompt_format_verified: false
quantization: null
runtime_and_cpu_gpu_npu_support: null
ram_vram_and_context_budget: null
device_model_and_temperature: null
p50_p95_latency_and_energy: null
privacy_safety_and_tool_eval: pending
```

Don't assert that E4B or E2B runs at a usable rate on any particular smartphone, or that Gemma 4 is identical to Gemini models. Evaluate GGUF conversion/version and license terms against the *actual chosen weights*. Do not download model weights based on the title of a blog page.

## 3. Google Agent Platform and spend-safe federation

[Google Cloud Agent Platform](https://cloud.google.com/products/gemini-enterprise-agent-platform) advertises Agent Studio, Model Garden, lifecycle governance/evaluations and paid compute/storage/inference. The linked private console path with a Google account selector and project is not authentication or approval. Do not print API keys, OAuth tokens, secret project metadata or billing screenshots.

P21 provider registry should distinguish `PUBLIC_DOCS, PRIVATE_CONFIG, BILLABLE_TEST, RESOURCE_MUTATION, PAYMENT`. Each non-public capability demands separately verified Google IAM scope; each charge-generating/mutating operation also requires an owner-approved maximum budget and independent approval record. The actual code `tools/cloud_operation_budget_gate.py` validates **plans** against externally supplied policy approvals, not real IAM, receipts or charges. No payment/enable API/storage bucket/VM/agent deployment done.

Related APIs have different media, quota, latency, language and compliance boundaries: [Cloud Translation](https://cloud.google.com/translate), [Vision AI](https://cloud.google.com/vision), [Speech-to-Text](https://cloud.google.com/speech-to-text), [Natural Language](https://cloud.google.com/natural-language), [Video Intelligence](https://cloud.google.com/video-intelligence), [Compute Engine](https://cloud.google.com/products/compute), [Cloud Storage](https://cloud.google.com/storage). Design typed `InputAsset→Task→ProviderCapability→ResponseEvidence` with allowlist, freshness, cost/region and data-class boundaries, rather than one unrestricted "Google API".

## 4. Developer execution: Gemini CLI, Antigravity, Code Assist, Android

[Gemini CLI GitHub](https://github.com/google-gemini/gemini-cli) is Apache-2.0 and documents file, shell, search and MCP-style tools, with official [Gemini CLI docs](https://geminicli.com/). [Antigravity](https://antigravity.google/) describes 2.0 local parallel agents, IDE, CLI and SDK. [Code Assist](https://codeassist.google/) serves IDE-oriented code assistance. Map to P100 IDE and P115 agent orchestration, **not a new duplicate studio project**.

Required policies: per-agent ephemeral workspace, explicit input/output schemas, purpose-limited trusted tool grants, signed code diff, source scope, no default write to `main`, CI/test evidence and independent readback. Prompt instructions and extensions alone aren't an OS sandbox. No CLI installed or external account linked in this batch.

[Android Developers 2026-09-22 "Googlebook" announcement](https://android-developers.googleblog.com/2026/09/adaptive-development-scale-app-googlebook.html) and [agent tooling docs](https://developer.android.com/tools/agents?hl=pl) prioritize window size classes, resizeable desktop windows, pointer, keyboard, multi-instance and accessible responsive content. P125 HOME launcher must be tested on phone, tablet, foldable and desktop-class Android. Add explicit test matrix: compact/medium/expanded width, rotation, split/freeform window, keyboard-only app navigation, TalkBack and safe launcher fallback. **No Googlebook availability, certification, Play listing or APK build verified.**

## 5. 101 technical blueprints are design prompts, not deployments

[Google Cloud's 2025 article](https://cloud.google.com/blog/products/ai-machine-learning/real-world-gen-ai-use-cases-with-technical-blueprints/) supplies **101 illustrative architecture patterns** (RAG/catalog deduplication/retail, enterprise search, media, observability, etc.). Ownership: P33 app delivery and P29 data/research, P21 cloud selection, P121 infrastructure. A blueprint is a starting design requiring region/price/security/licensing/source validation, not proof of production feasibility, profit or free access.

A source-derived `BlueprintDecisionCard` should capture `customer_problem, data_rights, authoritative_sources, minimal_stack, optional_local_substitutes, cost_ceiling, protected_data, external_actions, baseline_quality, acceptance_test, human_owner`. Do not create 101 nearly empty "projects".

## 6. External security and opportunity pages

[Pegasus-samples GitHub README](https://github.com/9aylas/Pegasus-samples) describes spyware samples; avoid downloading or executing potentially malicious binaries. Defensive P106/P108 protocols can capture repo URL, immutable revision, trusted third-party technical analysis and evidence-preservation requirements, without reproducing malware behavior.

Zed Guild project #74 and Railway Template Bounties project #2 render only shell/title when unauthenticated; a title does not establish a currently open reward or available issue. `bug-bash.github.com` was unavailable. These are **follow-up research targets**, not verified funding or jobs.

User WordPress plugin browse path explicitly selects **paid** plugin listings; no plugins purchased/installed. AI Studio keys, Workspace checkout and Cloud billing/multimodal Studio paths require separate authenticated authorization and are **not data sources** for public project changes. Google Notebook and AI Studio Apps cannot be assumed to expose the user's content from a link alone.

## 7. Release evidence and minimum next work

**Implemented in repo:** two static non-network gates + unit tests; canonical project documentation and source ledgers. **Not implemented:** a Gemini Embedding 2 client, cloud auth, paid model call, P125 Android desktop-ready UI, new Gemma 4 GGUF runtime, actual downloaded/opened malicious sample, Google Console workspace interaction.

Next priority after CI: (1) privacy-preserving local ingestion fixtures; (2) authorized provider adapter for a non-sensitive sample when IAM/usage limit is independently confirmed; (3) Android emulator/Googlebook responsiveness build check; (4) optional source-verified user-selected cloud provisioning **only with explicit approval**.
