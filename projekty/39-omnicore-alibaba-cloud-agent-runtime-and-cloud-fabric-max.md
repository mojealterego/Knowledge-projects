# 39 — OmniCore Alibaba Cloud Agent Runtime & Cloud Fabric MAX

## Cel projektu

Zbudować vendor-aware, ale provider-neutral **cloud execution fabric** wykorzystujący najmocniejsze wzorce z Alibaba Cloud: Qwen/Model Studio, MCP, AgentBay, ACK, GPU scheduling, RAM, OSS, SLS, ECS, Function Compute i Container Registry — bez uzależniania warstwy kontroli OmniCore od jednego dostawcy.

Alibaba Cloud pokazuje kompletny pionowy stos: modele i API, agent/workflow, sandbox runtime, storage, compute, Kubernetes/GPU, IAM, observability i security. Strona główna eksponuje m.in. ECS, Model Studio, Database i Security jako główne warstwy platformy. citeturn0view0

## Fundamental architecture

```text
OMNICORE CONTROL PLANE
        ↓
MODEL-NEUTRAL EXECUTION CONTRACT
        ↓
┌─────────────────────────────────────────────┐
│ Model Studio / Qwen / 3rd-party models     │
│ MCP / Responses API / Web / Code tools     │
│ AgentBay sandbox + Browser/Computer Use    │
│ ACK Kubernetes + GPU / heterogeneous       │
│ ECS / Function Compute                     │
│ OSS / databases                            │
│ SLS / audit / observability                │
│ RAM / STS / security                       │
└─────────────────────────────────────────────┘
        ↓
VERIFICATION / POLICY / PROVENANCE
```

## 1. Model Studio as model gateway

Model Studio provides Qwen and third-party models through native Qwen APIs and OpenAI-compatible APIs, with multimodal text/image/audio/video support. It exposes OpenAI-compatible Chat Completions and Responses interfaces, Anthropic-compatible Messages and native DashScope APIs. citeturn0search1turn0search8turn0search13

OmniCore therefore treats Model Studio as an **adapter**, not the cognitive core:

```yaml
ModelProviderAdapter:
  provider: alibaba_model_studio
  capabilities:
    text
    vision
    audio
    video
    tool_calling
    structured_output
    streaming
  endpoint_profile:
  model_id:
  region:
  quota:
  cost_profile:
  evaluation_profile:
```

The portfolio invariant remains:

`MODEL != PROVIDER != EXECUTION SUBSTRATE`.

## 2. MCP as capability boundary

Alibaba Cloud Model Studio supports MCP through the Responses API using SSE MCP servers, with a documented maximum of 10 MCP servers per request and support across current Qwen families and selected open-source models. citeturn0search0

OmniCore does not pass arbitrary MCP servers directly to the model. It inserts the existing Capability Broker:

```text
USER INTENT
 ↓
POLICY
 ↓
CAPABILITY DISCOVERY
 ↓
SERVER TRUST / PROVENANCE
 ↓
ALLOWLIST
 ↓
MCP SESSION
 ↓
OUTPUT VALIDATION
 ↓
AUDIT
```

This extends Projects 26, 30 and 31 with a concrete cloud implementation pattern.

## 3. AgentBay as disposable agent execution substrate

AgentBay is positioned by Alibaba Cloud as AI-agent cloud infrastructure rather than a generic VM. It provides standard agent runtimes, pre-integrated MCP services, isolated user state persistence and real-time client/cloud interaction. citeturn1search22

Its MCP layer can create sandboxes, expose runtime URLs, capture screenshots and release resources; available environments include BrowserUse, Linux/Windows ComputerUse, MobileUse and CodeSpace. citeturn2search5

Architecture:

```text
TASK
 ↓
CREATE EPHEMERAL SANDBOX
 ↓
MOUNT MINIMAL STATE
 ↓
EXECUTE CAPABILITY-SCOPED TASK
 ↓
COLLECT ARTIFACTS
 ↓
VERIFY
 ↓
PERSIST APPROVED ARTIFACT ONLY
 ↓
KILL SANDBOX
```

The sandbox is **execution isolation**, not automatic authorization. The agent still requires policy checks before every consequential capability.

## 4. Persistent browser state as a security-sensitive capability

AgentBay Browser persistence can retain cookies, cache, local storage, session storage, browser preferences and extensions across sessions. citeturn2search0

OmniCore classifies this as `high-sensitivity persistent state`:

```yaml
PersistentBrowserState:
  owner:
  purpose:
  domain_allowlist:
  expiry:
  encryption:
  export_policy: deny
  human_approval:
  audit_id:
```

Persistent browser state must never be silently exposed to a model or copied between tenants.

## 5. Browser and Computer Use

AgentBay supports cloud-hosted browser workflows through SDK, MCP or ASP and computer-use workflows where agents interact with desktop applications that lack APIs. citeturn2search9turn2search12

OmniCore treats screen, DOM, OCR, files and application state as **untrusted multimodal inputs**. Computer Use is therefore routed through the same Capability Broker as API tools.

```text
SCREEN / DOM / FILE
 ↓
UNTRUSTED INPUT CHANNEL
 ↓
INTENT INTERPRETATION
 ↓
CAPABILITY CHECK
 ↓
ACTION PREVIEW
 ↓
EXECUTE
 ↓
OBSERVE RESULT
 ↓
VERIFY
```

## 6. ACK as heterogeneous AI compute fabric

Alibaba Cloud ACK manages heterogeneous resources including GPUs, ASICs and eRDMA. It supports GPU scheduling, autoscaling, GPU sharing and computing-power fencing. citeturn2search13

ACK GPU scheduling follows standard Kubernetes resource requests and explicitly warns against bypassing that mechanism by directly requesting GPUs through node-level container commands. citeturn2search3

This becomes an OmniCore substrate abstraction:

```yaml
ComputeResource:
  kind: GPU|CPU|ASIC|eRDMA|EDGE
  capacity:
  memory:
  isolation:
  scheduler:
  tenant:
  workload_class:
  cost_profile:
  energy_profile:
```

Shared GPU/cGPU can partition GPU memory and compute power between workloads, reducing underutilization for inference. citeturn2search8turn2search18

## 7. Edge-to-cloud continuum

ACK Edge can incorporate on-premises GPU resources into an edge node pool. citeturn2search24

This directly strengthens Project 37:

```text
T0 DETERMINISTIC
 ↓
T1 LOCAL SERVER
 ↓
T2 DESKTOP / BROWSER EDGE
 ↓
T3 MOBILE / ON-PREMISE EDGE
 ↓
T4 ACK / CLOUD GPU
 ↓
T5 SPECIALIZED CLOUD MODEL
```

Routing is based on quality, latency, privacy, energy, cost, data gravity and capability — not provider prestige.

## 8. RAM / STS as cloud Capability Broker substrate

Alibaba Cloud RAM follows deny-by-default semantics; explicit `Deny` overrides `Allow`. Policies expose `Effect`, `Action`, `Resource`, `Condition` and, for resource-based policies, `Principal`. citeturn1search3turn1search5turn1search12

RAM roles provide temporary credentials through STS instead of long-lived credentials. citeturn1search21

This maps almost directly onto OmniCore Capability Grants:

```yaml
CapabilityGrant:
  subject:
  action:
  resource:
  conditions:
  expiry:
  purpose:
  approval:
  audit_id:
```

Resource-group authorization can isolate projects, environments or teams without exposing the entire account. citeturn1search6

## 9. OSS as Evidence/Object Fabric

OSS provides object storage with multiple storage classes from Standard through IA, Archive, Cold Archive and Deep Cold Archive. It supports version IDs and lifecycle rules. citeturn1search2turn1search11turn1search14

OSS also supports server-side and client-side encryption, TLS, integrity verification, content scanning and sandboxing mechanisms. citeturn1search8turn1search17

OmniCore mapping:

```text
RAW INPUT
 ↓
IMMUTABLE/SNAPSHOT OBJECT
 ↓
CONTENT HASH + METADATA
 ↓
EVIDENCE ID
 ↓
VERSIONED DERIVATIVES
 ↓
LIFECYCLE POLICY
```

This can serve Projects 30 and 32 as an evidence artifact substrate, provided retention, jurisdiction and access policies are explicitly enforced.

## 10. SLS as Agent Observability Fabric

Alibaba Cloud Simple Log Service unifies logs, metrics, traces and events. Its stores include Logstore, Metricstore and Eventstore; traces can represent processing information for individual requests. citeturn2search1turn2search2

SLS Alerting provides alert monitoring, noise reduction, incident management and notification dispatching, with SQL/PromQL analysis and correlated monitoring. citeturn2search10

OmniCore observability schema:

```yaml
AgentEvent:
  trace_id:
  span_id:
  agent_id:
  session_id:
  model:
  capability:
  policy_decision:
  input_provenance:
  output_provenance:
  latency:
  tokens:
  cost:
  error:
  security_event:
  artifact_id:
```

This joins Projects 26/27/31/32 into one telemetry fabric.

## 11. Function Compute / serverless execution

Function Compute separates control-plane authorization from data-plane execution and supports RAM-based authorization, encrypted metadata and isolated code/layer caches. citeturn0search9

Its data plane uses TLS 1.2+ and supports VPC-restricted invocation modes. citeturn0search2

OmniCore use:

```text
EVENT
 ↓
POLICY CHECK
 ↓
FUNCTION
 ↓
SHORT-LIVED EFFECT
 ↓
RESULT VALIDATION
 ↓
AUDIT
```

Serverless functions are preferred for bounded, stateless, reversible actions rather than unrestricted agent autonomy.

## 12. Container supply-chain gate

Alibaba Cloud Container Registry can automatically scan pushed images and, with security blocking policies, prevent high-risk images from continuing through CI/CD. citeturn1search4

Project 28 integration:

```text
GENERATE
 ↓
BUILD
 ↓
SBOM
 ↓
IMAGE SCAN
 ↓
POLICY GATE
 ↓
SIGN
 ↓
STAGED DEPLOYMENT
 ↓
OBSERVE
 ↓
PROMOTE / ROLLBACK
```

An AI-generated container never receives production trust merely because it builds successfully.

## 13. Security model

Alibaba Cloud ECS documents hardware memory encryption, trusted computing/vTPM and confidential computing with remote attestation on supported instances. citeturn0search4

Project 39 therefore adds an optional trust tier:

```text
STANDARD CLOUD
 ↓
HARDENED INSTANCE
 ↓
TRUSTED BOOT / vTPM
 ↓
TEE / CONFIDENTIAL COMPUTING
 ↓
REMOTE ATTESTATION
```

This does not eliminate the need for application-level authorization, provenance and verification.

## 14. Model Studio RAG / workflow pattern

Model Studio knowledge bases implement RAG by retrieving relevant chunks and passing them to downstream model nodes. Workflow applications compose LLMs, APIs and Function Compute into ordered steps. citeturn1search0turn1search10turn1search18

OmniCore uses this as an implementation option for the existing state-machine architecture, while keeping its own evidence and policy layer authoritative.

## 15. Cloud Fabric invariants

1. Cloud provider is an execution substrate, not the policy authority.
2. Model selection is independent of authorization.
3. MCP servers require trust/provenance evaluation.
4. Sandbox isolation does not equal authorization.
5. Persistent browser state is high-sensitivity state.
6. RAM/STS capabilities should be short-lived and purpose-bound.
7. Object storage is evidence infrastructure only when provenance and retention are explicit.
8. GPU scheduling must remain scheduler-mediated and tenant-aware.
9. Generated containers require supply-chain verification before promotion.
10. Observability must cover model, capability, policy, artifact and security events.
11. Local/edge/cloud routing is a measurable optimization problem.
12. No cloud-native feature may silently bypass OmniCore policy.

## 16. Evaluation suite

Benchmark the fabric across:

- model migration without application rewrite;
- MCP server allow/deny decisions;
- sandbox escape attempts;
- persistent-state leakage;
- cross-tenant isolation;
- RAM policy conflicts;
- temporary credential expiry;
- GPU scheduling fairness;
- container vulnerability blocking;
- evidence immutability/versioning;
- observability completeness;
- edge-to-cloud failover;
- cloud outage recovery;
- cost/latency/quality tradeoffs.

Core metrics:

```text
policy_bypass_rate
unauthorized_capability_rate
cross_tenant_leakage_rate
artifact_provenance_coverage
credential_exposure_rate
sandbox_recovery_rate
image_block_precision
trace_completeness
cloud_escalation_rate
cost_per_successful_task
p95_task_latency
```

## Definition of Done

- Model Studio adapter;
- provider-neutral Responses/MCP adapter;
- AgentBay sandbox adapter;
- browser/computer-use capability boundary;
- ACK heterogeneous compute adapter;
- RAM/STS capability mapping;
- OSS evidence/object adapter;
- SLS telemetry adapter;
- Function Compute bounded-action adapter;
- ACR supply-chain gate;
- optional confidential-compute attestation path;
- unified policy/provenance/evaluation plane.

## Status

**Architecture / implementation blueprint.** Alibaba Cloud-specific features are treated as concrete implementation capabilities, while the OmniCore control plane remains provider-neutral. Current service/model names and limits must be revalidated against the live regional documentation before production deployment.

---

## 2026-10-09 — batch 24: PDF source evolution

Official Alibaba AI Catalyst page (checked 2026-10-09) advertises **up to USD 120,000 cloud credits** and **up to 2 billion Model Studio tokens**, technical office hours and about 4–5 business days for review. Applicant eligibility includes AI core technology, an unlisted company established within ten years and an accessible public website. Credits are not unrestricted: operator lists domain, CDN, Marketplace and prepaid exclusions. The uploaded 8-page application PDF is a **draft**, not submission or credit allocation. Record `ProgramClaim` states `advertised_maximum, eligibility_verified, submitted, awarded, redeemed` separately; no real user account accessed. [Offline program claim validator](../tools/program_claim_gate.py). See [10-PDF source ledger](../docs/UPLOADED-PDF-INGESTION-2026-10-09-BATCH-24.md).
