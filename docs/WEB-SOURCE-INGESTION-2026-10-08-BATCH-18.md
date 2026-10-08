# Public web/source intake — batch 18 (2026-10-08)

**User request:** analyze supplied web URLs, evolve existing Knowledge-projects, create a new project only with a genuinely independent product boundary, commit verified changes. **Baseline:** `main@9291122c8155045879cabf7572f5e8482934f448`.

## Acquisition statistics / non-assumptions
- 34 input URLs; **33 distinct URLs by substance** because the Cloud Translation URL was supplied twice (with the same `authuser=2` query).
- Public Google product documentation, Gemini CLI repo/website, Gemma 4 and Embedding 2 articles and the Google Cloud 101-blueprint case-study article were inspectable.
- Google account/console, API key, billing/checkout, user WordPress installation pages, notebook/apps portals and private project boards are **not authenticated by sharing a URL**. They were **not** accessed as the user's signed-in account; no content read from private workspace, no keys extracted, services enabled or charges initiated.
- `Pegasus-samples` is described by its own README as an Android/iOS spyware sample archive. We read descriptive repository metadata only; **no sample binaries, payloads or ZIP were downloaded, executed or published**.
- `bug-bash.github.com` was unreachable during public fetch. Zed Guild's GitHub project #74 and Railway Template Bounties project #2 rendered only a public page shell/title without inspectable board items; **no bounties or tasks were invented**.

## Source-to-owner intake registry

| # | Normalized source | Access at review | Knowledge owner / action |
|---:|---|---|---|
| 1 | https://github.com/9aylas/Pegasus-samples | public README only; risky binary archive not fetched | P106/P108: static threat-intelligence provenance and no execution |
| 2 | https://bug-bash.github.com/ | unreachable | P47: `UNAVAILABLE`, no bug-bounty claim |
| 3 | https://github.com/orgs/zed-industries/projects/74 | public board shell only | P100: watch upstream issue process, no item status claims |
| 4 | https://github.com/orgs/railwayapp/projects/2 | public title "Template Bounties", items unseen | P33/P56: verify actual bounties before planning |
| 5 | https://wordpress.com/plugins/browse/paid/mojealteregopl.wordpress.com | site/account-specific, not fetched | P45: WordPress paid plugin marketplace, no activation/purchase |
| 6 | https://android-developers.googleblog.com/2026/09/adaptive-development-scale-app-googlebook.html | public Sept 22 2026 blog found by canonical search | P125/P105: resizable desktop layout, keyboard/pointer, window-size tests |
| 7 | https://console.cloud.google.com/agent-platform/studio/multimodal | authenticated console (project query suppressed) | P21/P14: private project state **not inspected** |
| 8 | https://cloud.google.com/blog/products/ai-machine-learning/real-world-gen-ai-use-cases-with-technical-blueprints | public canonical alternate to supplied `/blog/u/2/` URL | P21/P33/P29: illustrative patterns, not deployed customer systems |
| 9 | https://cloud.google.com/learn/what-is-artificial-intelligence | public general explainer, nonimplementation source | P21 |
| 10 | https://aistudio.google.com/api-keys | Google sign-in required | P21: no key access or creation |
| 11 | https://notebook.google/ | site loads minimal public shell | P29/P114: user notebooks not accessible |
| 12 | https://codeassist.google/ | public product page | P100: IDE integration candidate |
| 13 | https://workspace.google.com/checkout/payment | payment/authenticated; not used | P56: no transaction |
| 14 | https://cloud.google.com/discover/what-are-ai-agents | public conceptual page, direct fetch unreliable | P115 |
| 15 | https://cloud.google.com/products/gemini-enterprise-agent-platform | public documented platform | P21/P72/P121: governance, tool/data policy and cost controls |
| 16 | https://console.cloud.google.com/agent-platform/agent-garden | authenticated console | P21: no browsing user's account/model garden inventory |
| 17 | https://antigravity.google/ | public Antigravity 2.0 IDE/CLI/SDK page | P100/P115: capability registry, approval, sandbox checks |
| 18 | https://geminicli.com/ | public CLI docs | P100/P115: local terminal agent workflow |
| 19 | https://github.com/google-gemini/gemini-cli | public repo README Apache-2.0 | P100/P115: upstream evaluation, no plugin installed |
| 20 | https://cloud.google.com/translate | public Translation AI | P75/P57: glossary, confidence/human review |
| 21 | https://cloud.google.com/translate | **DUPLICATE #20** | no double counting |
| 22 | https://cloud.google.com/vision | public Vision AI | P57/P29: OCR/object extraction, provenance |
| 23 | https://cloud.google.com/speech-to-text | public STT API | P57: transcript timestamps/confidence/speaker/privacy |
| 24 | https://cloud.google.com/natural-language | public NLP API | P75/P57: morphology/entities/limitations |
| 25 | https://cloud.google.com/video-intelligence | public Video Intelligence | P57: video annotations/temporal evidence |
| 26 | https://cloud.google.com/products/compute | public Compute Engine | P121/P37: VM/IAM/GPU-cost evidence, no VM created |
| 27 | https://cloud.google.com/storage | public Cloud Storage | P121/P29: source object hashes and lifecycle policy, no bucket created |
| 28 | https://developer.android.com/tools/agents?hl=pl | public Android tools/agents docs | P125/P105/P100: adaptive UI/CLI, review and device QA |
| 29 | https://developers.googleblog.com/en/building-with-gemini-embedding-2/ | official dated April 30 2026 technical article | P29/P57/P114: multimodal RAG, limits and eval |
| 30 | https://blog.google/innovation-and-ai/technology/developers-tools/gemma-4/ | official Apr 2 2026 article | P37/P17: local/open model evaluation, don't assume S24 compatibility |
| 31 | https://aistudio.google.com/apps | signed-in app portal, not inspected | P33: user apps not accessible |
| 32 | https://cloud.google.com/solutions#industry-solutions | public cloud solutions directory | P21/P33 |
| 33 | https://docs.cloud.google.com/docs | public docs portal (direct review unavailable) | P21 |
| 34 | https://me.developers.google.com/communities | account/community page with no accessible personal content | P25: no enrollment claimed |

**Source statuses:** `PUBLIC_READ`, `PUBLIC_SHELL_ONLY`, `SEARCH_RESOLVED_PUBLIC`, `AUTH_REQUIRED`, `UNAVAILABLE`. The separate count of exact successfully read pages is not a measure of fully evaluated provider APIs; product features and access must be individually qualified before deployment.

## Verified source-specific learning

1. **Gemini Embedding 2**: Google Developers Blog (2026-04-30) states a shared text/image/audio/video/PDF semantic space and at most 8,192 text tokens, 6 images, 120 sec video, 180 sec audio, 6 PDF pages in one request. Official model ID: `gemini-embedding-2`; dimension support 128–3072, recommended 768/1536/3072 (official model documentation). Google claims use-case improvements, not universal recall gains. This is the basis of the **non-network** `tools/multimodal_embedding_intake_gate.py`.
2. **Gemma 4**: Google blog (2026-04-02) lists E2B, E4B, 26B MoE and 31B Dense and an Apache-2.0 license. Whether a given quantization actually fits/on-device runs on an Android handset is a separate RAM/NPU/kernel/backend test. No model downloaded.
3. **Googlebook desktop-class Android**: 2026-09-22 Android Developers article recommends window-size-class adaptive layouts, freeform resizing, mouse/trackpad/keyboard input, multi-instance tests and accessibility. Source describes marketing/badging programs but does **not** prove P125 qualifies for an app badge or program.
4. **Gemini Enterprise Agent Platform**: describes Agent Studio, Model Garden, governance, evaluation and chargeable training/inference/storage. Sharing a console URL with `authuser` or `project` does **not** grant IAM authority. Implemented non-network `tools/cloud_operation_budget_gate.py` checks externally granted scope/actions/cost bounds before *proposing* paid changes.
5. **Google Cloud 101 blueprints**: these are **illustrative implementation patterns** (e.g. retrieval, observability, catalog normalization, document processing), not ready-to-use production apps or proof of costs, regulatory compliance or delivered revenue.
6. **Google APIs** (Vision/STT/Translation/Natural Language/Video/Compute/Storage): distinct per-provider IO, privacy/retention, costs, region, supported languages/media and changeable quotas; no implied consolidated credential or free unlimited quotas.
7. **Gemini CLI / Antigravity**: publicly documented agent software, shell/code editing, MCP-style tools and project workflows. Source code and product screenshots are not evidence a private user's tools or Google account has been configured.

## Deliverables and limits

- Existing portfolio updated: P14/P17/P21/P29/P33/P37/P47/P57/P75/P100/P105/P106/P108/P114/P115/P121/P125 (scope by owner).
- New conceptual project: **0** — all capabilities fall within already assigned project responsibilities.
- Two non-network Python safety/provenance policy gates + unit tests committed; CI run must be read back before declaring success.
- No exploitation/malware download, account keys, billing, Google Cloud deployment, model inference, user WordPress plugin purchase or private board/task modification.
