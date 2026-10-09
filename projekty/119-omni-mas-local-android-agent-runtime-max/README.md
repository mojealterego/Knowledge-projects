# Project 119 — OmniMAS Local Android Agent Runtime MAX

## Status
SOURCE INGESTED → ARCHITECTURE BASELINE → KOTLIN IMPLEMENTATION EVIDENCE → GITHUB RELEASE-AUDIT → NETWORK/DEVOPS/SECURITY CORPUS INTEGRATION — 2026-09-12

## Mission
Build a local-first Android multi-agent runtime in which a Planner, Grounding layer, Executor and Supervisor cooperate over device UI state, with local LLM inference and explicit verification.

## Source implementation witness
The supplied README describes `Planner → Grounding → Executor → Supervisor`, with optional local Ollama at `127.0.0.1:11434`. ADB/Termux is explicitly outside the production core and may be used only for debugging.

Declared modules are AccessibilityService for UI perception/action, NotificationListenerService for authorized local notifications, Planner, Grounding, Executor, Supervisor and task-local Memory.

The source explicitly rejects undocumented Samsung intents, fictitious ADB Burst Mode, private system-setting manipulation and embedding API keys in the application.

## Canonical architecture

```text
USER INTENT
    ↓
PLANNER / LOCAL LLM
    ↓
GROUNDER
    ↓
STRUCTURED UI STATE
    ↓
EXECUTOR
    ↓
ACCESSIBILITY ACTION
    ↓
AUTHORITATIVE READBACK
    ↓
SUPERVISOR
    ├── VERIFY
    └── REPLAN
         ↓
      TASK MEMORY
```

## 1. Planner

Planner converts an operator task into bounded executable intent. Local Ollama is the source-described LLM path. Networked providers are not implied by this artifact.

The supplied Kotlin implementation confirms bounded planning and action decisions in `LocalLlmClient`. Missions are constrained to bounded Android steps and explicitly disallow financial operations, data deletion, password handling and irreversible communications without separate confirmation. The decision contract returns `CLICK`, `TYPE`, `SCROLL`, `BACK`, `HOME`, `DONE`.

## 2. Grounding

Grounding converts current Android accessibility/UI observation into a semantic representation suitable for planning. Raw UI events remain observations; they are not authoritative proof of final application state.

## 3. Executor

Executor owns bounded UI actions. The supplied implementation confirms clicks, text entry, navigation and scrolling through Android accessibility APIs.

## 4. Supervisor

The current implementation does not contain a standalone `Supervisor` class. Supervision is embodied by the mission loop: capture fresh UI state, decide, execute, wait, inspect `expectedNow`, then continue or replan.

## 5. Network boundary

The 2026-09-12 networking corpus adds a strict transport/capability distinction:

```text
CONNECTIVITY
   ↓
TRANSPORT
   ↓
TLS / SESSION
   ↓
APPLICATION AUTH
   ↓
CAPABILITY POLICY
   ↓
ACTION
   ↓
READBACK
```

TCP reliability, WebSocket connectivity, TLS, VPN or an open port do not authorize an Android action. Network reachability is an observation, not authority.

```text
REACHABLE ≠ AUTHORIZED
TLS ≠ AUTHORIZATION
VPN ≠ ZERO TRUST
OPEN PORT ≠ TRUSTED SERVICE
NAT ≠ SECURITY CONTROL
```

## 6. DevOps / release boundary

P119 inherits the delivery-chain model from the 2026-09-12 DevOps corpus:

`commit → CI/test → security analysis → artifact → digest/provenance → deployment strategy → device/runtime readback → promotion/rollback`.

Blue-green/canary/rolling deployment are risk-control strategies, not correctness proofs. Immutable infrastructure reduces uncontrolled mutation. Monitoring, logging, alerting and SLI/SLO/SLA evidence belong to the operational verification layer. Chaos engineering must remain explicitly bounded and authorized.

## 7. Git boundary

Git operations are treated as consequential mutations. Commit, push, reset, revert, rebase, branch deletion and deployment-triggering changes require explicit policy and postcondition verification. Git history, reflog and bisect are evidence/recovery capabilities, not authorization mechanisms.

## 8. Database / persistence boundary

Where P119 uses a backend service or local persistence layer, indexing decisions must be workload-specific. Query-plan/latency evidence, selectivity, write amplification, index bloat and migration behavior should be measured before performance claims are promoted.

A port, API endpoint or database connection proves connectivity only; it does not establish data correctness or authorization.

## 9. Zero Trust / layered security

The supplied security corpus reinforces:

- identity and device posture must be verified;
- least privilege applies to application/data/network access;
- segmentation limits blast radius;
- encryption protects data in transit but does not establish trust;
- secrets require dedicated secure handling;
- input validation and parameterized queries defend application boundaries;
- logs/metrics/traces support detection and audit but are not automatically authoritative success evidence.

## 10. Verification program

1. Accessibility-service enable/disable tests.
2. UI-tree grounding determinism tests.
3. Node identity/TOCTOU tests across changing UI snapshots.
4. Click/type/gesture action readback tests.
5. Supervisor/replan tests after failed postconditions.
6. Notification authorization and redaction tests.
7. Ollama connectivity/error/recovery tests.
8. Offline operation tests.
9. Android-version compatibility tests.
10. Foreground/background lifecycle tests.
11. Capability-policy enforcement tests.
12. Prompt-injection resistance for hostile UI text.
13. Task-memory isolation tests.
14. End-to-end `intent → plan → ground → act → readback → verify` replay.
15. CodeQL/SAST CI validation for supported Kotlin/Java paths.
16. Reproducible build and dependency-lock validation.
17. APK/AAB digest and provenance/attestation verification.
18. Deployment/device test with post-deployment readback.
19. Mobile CI/CD capability permission and version/provenance validation.
20. GitHub-agent-to-mobile authorization boundary test, if such integration is enabled.
21. Network failure/timeout/retry tests with bounded retry budgets.
22. TLS/session/authentication separation tests.
23. Port/service observation tests ensuring port metadata is never treated as service identity proof.
24. Persistence/index performance tests under representative workload.
25. Rollback/readback tests after failed deployment or runtime health checks.

## Definition of Done

P119 advances beyond architecture baseline when the runtime builds reproducibly, local LLM connectivity is observable, accessibility actions are bounded by policy, every consequential action has authoritative readback, supervision/replanning is deterministic under test, notification data is permission-scoped, node identity remains valid across action execution, release artifacts carry verifiable digests/provenance, network failures are bounded and observable, persistence/index changes are measured under representative workload, and P117/P100 integration passes end-to-end verification.

## Evidence classification

- supplied Android README/Gradle/manifest/XML/Kotlin: **OBSERVED / SOURCE-DERIVED IMPLEMENTATION EVIDENCE**;
- 2026-09-12 database/DevOps/Git/network/security images: **SOURCE-DERIVED / CONTEXT**;
- architecture and controls above: **SYNTHESIS / IMPLEMENTATION TARGET**;
- CI execution, CodeQL results, reproducible release, attestations, deployment success and local performance benchmarks: **NOT VERIFIED BY THIS CORPUS**.

## Dependencies

P40, P54, P57, P59, P61, P72, P100, P114, P117.

---

## Knowledge evolution — batch 15 / 2026-10-08: P125 HOME launcher as optional unprivileged entrypoint

Android AI-launcher report (10 pages) suggests on-device AI and contextual application recommendations, but a default HOME role alone does not grant Android Accessibility, notification history, package-wide visibility, location or OS shell privileges.

New [P125 HOME launcher](../125-sovereign-contextual-android-launcher/README.md) is a *separate product*: Kotlin source MVP lists only launchable apps and opens a specific activity on human click, without network or sensitive permissions. P119 remains the local planner/grounding/executor/supervisor runtime; it cannot silently piggyback privileged actions through the launcher. Any future local-LLM adapter must be opt-in, offline or user-approved provider, narrow capability scope, rollback and independently verified Android postcondition. No APK was compiled or installed from this material.

---

## 2026-10-09 — batch 20: ODYN / Hermes / Nous ecosystem

**Android platform correction (2026-10-09):** ODYN-AI's actual default branch is `codex/termux-five-goals` with a native Android MobileFork package `com.mobilefork.hermesagent`, code for GGUF/LiteRT-LM and tests. The **official NousResearch Termux APT** is a separate arm64 installer, and its own docs currently state **"Termux is currently broken"**. Thus neither the older MobileFork `website/docs/getting-started/termux.md` guide nor an unrelated `uv` Windows antivirus issue proves Android command-line install works. `HermesTermuxReleaseEvidence` requires official status freshly checked, exact arm64/API >=24 prefix, signed APT key fingerprint, explicit owner authorization and rollback; no automatic `curl | bash` override or trust of self-declared "installed". [ODYN-AI upstream-safety PR #17](https://github.com/mojealterego/ODYN-AI/pull/17) is the concrete code/test/docs delta; not evidence a phone was connected or native APK broken. Privacy/permissions for the native app remain P119/P105 concerns.

## Batch 24 — 2026-10-09 PDF source evolution

Komunikacja.pdf provides a broad conceptual catalogue of IrDA, BLE, NFC, UWB, Wi-Fi Direct, Li-Fi and messaging layers, but no tested Android implementation. P119 can model capability checks by Android API level/device, explicit user permissions, paired endpoint ownership, battery/network budgets and actual postcondition. A protocol description never grants privileged scanning, geolocation or silent phone control. tools/communication_link_policy.py is an offline planning guard; no radio or device connection was made.
