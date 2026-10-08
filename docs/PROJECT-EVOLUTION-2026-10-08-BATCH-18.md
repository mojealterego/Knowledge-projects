# Project evolution — web sources batch 18 (2026-10-08)

| Project | Actual canonical delta | Change class |
|---|---|---|
| **P14** Gemini adaptive agent | embedding/multimodal task boundaries and provider evidence | architecture |
| **P17** model router | Gemma 4 local vs hosted agent cost/privacy/capability routing | architecture |
| **P21** Google developer stack | Google platform capability registry, account vs public resources, cost/IAM control, multimodal gateway | architecture + 2 executable policy gates |
| **P29** research fabric | Gemini Embedding 2 input limits and multimodal provenance RAG | architecture |
| **P33** application foundry | 101 technical blueprints filtered through testable business and app outcomes | architecture |
| **P37** local model runtime | Gemma 4 E2B/E4B/26B MoE/31B model qualification, RAM/driver/quantization tests | architecture |
| **P47** project registry | 34 URLs (33 unique), duplicate Cloud Translation link, public-vs-private access state | provenance |
| **P57** multimodal interface | Google APIs video/speech/image/translation and cross-modal embedding data contract | architecture |
| **P75** Polish-language engine | translation/NLP cross-language ambiguity and source-span evaluation | architecture |
| **P100** NeXus IDE | Gemini CLI, Antigravity 2.0 IDE/SDK and Code Assist tool/approval boundary | architecture |
| **P105** Android agent | Googlebook large-screen/keyboard/trackpad UI tests and isolated automation | architecture |
| **P106/P108** defensive security | Pegasus-samples README as non-executable threat intel; no binary retrieval | architecture |
| **P114** cognitive knowledge | multimodal indexes bitemporal evidence, provenance and cache invalidation | architecture |
| **P115** agent orchestrator | Antigravity/Gemini CLI integration candidates with independent receipts | architecture |
| **P121** infrastructure | region/IAM/budget/Compute/Storage lifecycle and anti-accidental-spend | architecture |
| **P125** Android HOME launcher | Googlebook-style adaptive acceptance matrix; source scaffold unchanged | architecture |

**New numbered projects:** **0**. These are additions to established Google AI, security, Android, multimodal research, cloud and agentic IDE owners, not distinct product lines.

## Executable code
- `tools/multimodal_embedding_intake_gate.py`: offline, no API requests, checks pinned model name, input limits and private-source approval; has **20** unittest methods.
- `tools/cloud_operation_budget_gate.py`: offline, no API requests, checks externally approved resource scope, operations, spend caps, payment/side-effect blocks; has **19** unittest methods.
- Existing GitHub Actions runs all `tools/test_*.py`; PR CI result must be read before merge.
- Provider runtime calls, model downloads, Google authentication, cloud resource creation, payments and device tests are **NOT EXECUTED**.

## Evidentiary restrictions
Google official product page claims are capabilities/advertised limitations as of observed page date. User-specific dashboards and unpublished project boards were not read. Never describe API keys, billed usage, user apps, WordPress plugins, malware samples or project board reward status as obtained from inaccessible URLs. No new project genesis without source-backed distinct ownership.
