# Portfolio evolution — GitHub MCP Registry pages 1–7 / batch 23

| Project | Source-derived new decision | Evidence |
|---|---|---|
| **P47** portfolio registry | machine-readable 210-card snapshot; 7×30 vs 394 advertised, two missing extracted titles kept null, dedup and source-only statuses | ACTUAL_INDEX+CODE |
| **P72** agent assurance | listing != authorized `tools/list`, tool scopes/prompt-injection isolation, metadata-only gate; deny install and external writes | CODE+ARCHITECTURE |
| **P100** NeXus developer shell | Context7, Serena, Chrome DevTools, Playwright, Sourcegraph, Kotlin Library Sources and Figma: stage adapters with exact permission manifests | ARCHITECTURE |
| **P29** data research | Markitdown, Chroma, Hugging Face, Semantic Scholar, SearXNG, Scholar Sidekick: source-rights, provenance and cited cross-modal retrieval | ARCHITECTURE |
| **P114** cognitive memory | Basic Memory, PMB AI, memo, Selvedge, XMemo and ContextStream: per-tenant memory, revocation, TTL, independence of evidence | ARCHITECTURE |
| **P108** security workbench | SonarQube, Snyk, Sonatype, Codacy, StackHawk, CrowdStrike and Black Duck classified as candidate diagnostics, not passing scans | ARCHITECTURE |
| **P121** SRE | Netdata, Sentry, Dynatrace, Logfire, Terraform, Octopus, cloud providers; protect provisioning, account access and budget | ARCHITECTURE |
| **P33** app delivery | Playwright/Cypress/mabl QA, Figma and diagram tools, Vercel/Hostinger, product templates | ARCHITECTURE |
| **P56** commercialization | Stripe, Zapier, DSers, Avalara, Atom and retail listings: no purchases, checkout, trade or paid API without explicit approval | ARCHITECTURE |
| **P45** content commerce | WP Agent, Webflow, Wix and WordPress account plugin boundary; no site access assumed | ARCHITECTURE |
| **P57** multimodal | Markitdown/Imagesorcery/OCR/video/search extractions as untrusted sources requiring consent | ARCHITECTURE |
| **P86** game factory | Unity MCP and gamedev.pl as vendor-labeled candidate toolchains, no game created | ARCHITECTURE |
| **P59** agent executor | shell/remote desktop/phone/publish/write capabilities must pass real host-enforced scopes, not metadata text | ARCHITECTURE |

**New numbered project: 0.** Relevant source scopes already owned. **Code:** `tools/mcp_registry_catalog.py` and `tools/test_mcp_registry_catalog.py` (22 tests) + source JSON. No remote server code or packages installed, no CI claims until Actions completes.
