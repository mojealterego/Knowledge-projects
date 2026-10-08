# Batch 19 — agent marketplaces, research MCP, code modeling and two-person game design

**Source ledger:** [19 input URLs and uncertainty](../WEB-SOURCE-INGESTION-2026-10-09-BATCH-19.md).
This report separates vendor documentation, public GitHub product listings, evidence limits, integration architecture and actual source-code changes.

## A. GitHub Agent Apps — governance instead of automatic installation
[GitHub official agent-app docs](https://docs.github.com/en/copilot/concepts/agents/agent-apps) says agent apps are partner-built GitHub agents, run with Copilot cloud agent, and remain public preview. As of the review, GitHub Marketplace exposes:
- **SonarQube Agent**: inspect quality gates, surface branch/PR issues, propose and commit fixes to a working branch. [Listing](https://github.com/marketplace/sonarqube-agent).
- **Endor Labs AgentHQ Plugin**: public dependency CVE/risks via Developer Edition, with separate enterprise features and licensing. [Listing](https://github.com/marketplace/endor-labs-agenthq-plugin).
- **Bright Security Agent**: local/isolated dynamic security scanning, documented trial/concurrency/time restrictions and autonomous remediation modes. A free trial is not an ongoing free production subscription. [Listing](https://github.com/marketplace/bright-security-agent).
- **Miro Agent App**: architecture before/after PR diagrams, design-to-code loop, Miro account/board permissions and code effect possibilities. [Listing](https://github.com/marketplace/miro-agent-app).
- **LaunchDarkly agent**: create flags, AI Config and rollout rules; mutations require owner-granted external account scope, rollbacks and postconditions. [Listing](https://github.com/marketplace/launchdarkly-agent).
- **Octopus Deploy Intelligence Agent**: release/deploy/runbook operations; most actions are *effectful*, so CI logs plus human approval needed. [Listing](https://github.com/marketplace/octopus-deploy-intelligence-agent).
- **Packfiles Agent**: GitHub migrations, backlog waves and diagnostics requiring a Packfiles Warp migration environment; not a general free repo scanner. [Listing](https://github.com/marketplace/packfiles-agent).
- **InstructVault Action**: YAML/JSON prompt versioning, deterministic lint/render/tests, PR workflow; candidate for a **controlled opt-in** prompt CI, not blindly installed on protected branches. [Listing](https://github.com/marketplace/actions/instructvault).
- **Miro GitHub connector**: supplied Marketplace URL could not be verified directly; [Miro GitHub App](https://github.com/apps/miro) describes a separate GitHub↔Miro integration. Verify ID/repo authorizations before linking.

P72 cross-framework assurance, P100 IDE, P108 security and P121 infrastructure already own this domain. New `tools/marketplace_agent_review_gate.py` is an **offline plan authorization checker** only: it rejects missing independently supplied grants, different repo/listing, high-risk capability, missing effect approval, secret export and nonisolated scans. It does **not** discover installed apps, evaluate code vulnerabilities, install a GitHub App, start scanners or enforce GitHub IAM. Actual GitHub App installation/OAuth needs the user's explicit action.

## B. MCP registry — domain-specialized evidence and model-first development

**SAKH**: [registry description](https://github.com/mcp/uk.sadiqoon/sakh) describes a remote seven-tool read-only Arabic/Persian corpus of Ayatollah Khamenei lectures/rulings/writings using hybrid dense+sparse retrieval, reranking and citation/knowledge graph evidence. It is a **bounded, publisher-curated corpus**; content is not a neutral universal normative/legal ground truth. Owner P19/P29: `CorpusSource{publisher,origin,language,rights,scope,document_id,page,confidence,independent_corrob}`.

**Structura**: [vendor technical write-up](https://blog.structura.tools/posts/mcp-server-for-coding-agents/) describes domain modeling, model validation and deterministic generation via MCP. [Independent listing description](https://glama.ai/mcp/servers/metadevpro/structura-mcp/tree) gives OAuth2.1 HTTP integration as a provider claim. Do **not** confuse with the unrelated GitHub `RavinMaddHatter/Structura` for Minecraft Bedrock .mcstructure. Owner P87/P28: model intermediate representation, trace generated file to reviewed model revision, deterministic regeneration and artifact diffs.

**Apricot**: [metadevpro public discovery repository](https://github.com/metadevpro/apricot-mcp) describes SysML2 project read/write operations, while general model editing/validation is still restricted to its in-editor assistant according to the README. Actual `tools/list` requires authenticated connection and can differ. P87/P28: capability negotiation and no assumed `model_validate` tool without runtime inspection.

**Traveler.md**: [official MCP docs](https://docs.traveler.md/mcp) and [tools](https://docs.traveler.md/mcp/tools) specify a travel preferences+trip memory with OAuth-scoped read/create/update, **no availability or bookings**. P29/P114/P72: typed profile and trip provenance, retention, deletion and user-granted scope. The supplied GitHub registry pages for Structura, Apricot and Traveler were not fully fetched; related public provider documentation is the basis for these design observations.

**Integration pipeline**
```
provider claim → public source + provenance
→ signed-in tool discovery (not performed)
→ exact read/write scope negotiation
→ least-privilege + explicit approval for side effects
→ typed result + source evidence → CI/readback
```
MCP server listing text alone is not an installed, signed or secure tool.

## C. Adult two-player games — P122 CHEMIA, no duplicate genesis

**Privé** [site](https://privegame.com/pl): two adults answer separately, and only mutually positive topics appear. This suggests a constrained privacy pattern, **not** proof of cryptographic privacy, GDPR compliance or nationally representative survey findings. Source claims first round free and further one-time paid tiers, but commercial terms remain changeable.

**LovePlay**: [Polish homepage](https://loveplay.io/pl/) lists browser-based two-player formats and often one-device turn-taking, and says pairing a second remote phone is **not** the core mode. [Publisher blog](https://loveplay.io/pl/blog/najlepsze-gry-seksualne-dla-par-online) claims linked accounts/remote two-phone sync for ten games. These are **contradictory provider statements**; no running game account was tested. Feature parity, anonymity, freemium plans and audience research are unverified.

**Modern Love**: [editorial/commercial overview](https://modernlove.pl/gry-erotyczne-online) surveys browser games, visual novels and adult game categories with vendor claims about anonymity/benefit. It is background competitive positioning, not verified clinical or market evidence.

Existing [CHEMIA P122](../../projekty/122-chemia-consent-aware-intimate-two-player-game/README.md) has 520-card target inventory, intensity and HEAT state, skip/no-pressure core rules. It already owns consent-based adult gameplay; creating P127 would fragment the same product.

**Actually implemented:** `projekty/122-chemia-consent-aware-intimate-two-player-game/consent_intersection.py` computes the intersection of two ephemeral opt-in topic-ID sets, subtracts either partner's exclusions, denies mismatched session/inactive/adulthood-not-self-attested and returns **only mutually accepted IDs**. No hidden unilateral answers, no analytics, backend session store, real age verification, end-to-end encryption, server access or cross-device pairing. Independent consent for every real-world action is still necessary: `MATCH ≠ CONSENT`.

**Release gates:** meaningful age-assurance plan, coercion/threat-model, shared-device privacy, authenticated pairing with session expiration, nonce replay defense, secure token storage, per-partner revocation, no sensitive answer logs/telemetry, GDPR legal review, accessible skip/stop and independent mobile UI tests.

## D. Gemini Drops / Codex community

[Gemini Drops](https://gemini.google/gemini-drops/) is a vendor-maintained, localized feature and release showcase; P17/P21 may record dated model releases/availability only after authoritative changelog/provider capability tests. A marketing showcase does not enable a model in the user's plan.

[Codex community on Luma](https://luma.com/codex-community) provides workshop/hackathon/meetup discovery. Registration and events are subject to future date, location, capacity and conditions; no participation, ticket or funding secured. P100/P47 can treat as a time-bounded research and verification source.

## Evidence & roll-out scope

**Repository changes:** source ledger, canonical project specs, two offline Python modules + tests and indexes. **No numbered project:** all scopes are owned by P19/P29/P72/P87/P100/P108/P121/P122 etc.

No GitHub/Miro/Copilot app installations, MCP OAuth links, SAKH corpus export, intimate answer collection, gameplay accounts, payments, adult-content media production, deployed multiplayer room, live SonarQube/Bright scans or runbooks in this batch. Tests are local/CI policy checks, **not a live provider workflow**.
