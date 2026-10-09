# 33 — Agentic App Builder & Delivery Control Plane MAX

## Cel projektu

Zbudować aplikacyjny „assembly line” łączący natural-language intent, generowanie aplikacji, AI-assisted editing, szablony, build/test, signing, publikację, analitykę i billing — ale pod kontrolą jednej warstwy governance.

Dostarczone materiały OpenAI opisują trzy istotne warstwy: Skills jako wersjonowane pakiety instrukcji i plików, MCP jako warstwę zewnętrznych capabilities oraz MCP Apps jako opcjonalny UI. Skills są traktowane jako potencjalnie uprzywilejowany kod/instrukcje i wymagają przeglądu bezpieczeństwa. fileciteturn28file0L14-L24 fileciteturn28file0L540-L563

## Product architecture

```text
NATURAL-LANGUAGE INTENT
          ↓
REQUIREMENTS / SPEC
          ↓
APP MANIFEST
          ↓
AGENTIC PLANNER
          ↓
GENERATOR / TEMPLATE / SKILL
          ↓
BUILD
          ↓
TEST / SECURITY / POLICY
          ↓
SIGN
          ↓
STAGED RELEASE
          ↓
ANALYTICS / FEEDBACK
          ↓
VERSIONED REVISION
```

## Application Manifest

Każdy projekt otrzymuje jedną jawnie wersjonowaną reprezentację:

```yaml
app_id:
name:
version:
platforms:
ui_contract:
data_contract:
capabilities:
skills:
connectors:
secrets_policy:
build_profile:
signing_profile:
release_channel:
rollback_version:
telemetry_policy:
commercial_plan:
execution_profiles:
  - deterministic
  - local
  - edge
  - cloud
privacy_policy:
data_egress_policy:
```

Manifest jest źródłem prawdy dla delivery; UI builder jest jego klientem.

## Builder UX doctrine

Wewnętrzna inspekcja dostarczonych pakietów AppBuilder/AppStudio została wykorzystana jako materiał do odtworzenia wzorców UX dla buildera: osobny lifecycle projektu, konfiguracja, build status, templates, AI playground, analityka, subskrypcja, onboarding, publikacja i signing/key management.

Nie kopiujemy implementacji binarnej. Wyciągamy z niej wzorzec architektoniczny:

```text
EDIT
CONFIGURE
PREVIEW
BUILD
VERIFY
SIGN
PUBLISH
OBSERVE
```

Każdy etap jest osobnym stanem i może zostać wznowiony.

## AI Playground

Tryb eksperymentalny jest izolowany od produkcji:

`DRAFT → GENERATED → REVIEW → TEST → PROMOTE`

AI-generated code nie jest automatycznie kodem produkcyjnym tylko dlatego, że build przeszedł.

## Skills as executable product capabilities

OpenAI Skills są wersjonowane; `default_version` wskazuje wersję domyślną, a referencja może wskazywać określony numer wersji. fileciteturn28file0L462-L489

Control Plane wymusza:

- skill provenance;
- version pinning dla release builds;
- developer-only skill installation;
- review skryptów i instrukcji;
- brak arbitralnego doboru Skills przez końcowego użytkownika;
- explicit approval dla działań wysokiego wpływu.

## MCP capability boundary

Remote MCP i connectors są źródłami capabilities, nie autorytetem. OpenAI opisuje listowanie narzędzi, filtrowanie `allowed_tools` i osobne mechanizmy approval. fileciteturn25file0L453-L489 fileciteturn25file0L491-L514

W projekcie:

```text
DISCOVER
 ↓
FILTER
 ↓
AUTHORIZE
 ↓
CALL
 ↓
VALIDATE OUTPUT
 ↓
AUDIT
```

## MCP Apps UI

UI jest opcjonalne i nie może być warunkiem poprawności workflow. OpenAI zaleca oddzielenie narzędzi danych od narzędzi renderujących, aby model mógł najpierw przetworzyć dane, a dopiero później wybrać prezentację. fileciteturn25file1L1026-L1054 fileciteturn25file1L1160-L1184

Wersja Control Plane stosuje:

```text
DATA TOOLS
  ↓ structuredContent
MODEL DECISION
  ↓
RENDER TOOL
  ↓
MCP APP
```

## State architecture

Rozdzielone są:

- authoritative business data;
- ephemeral UI state;
- durable cross-session state.

Dokumentacja MCP Apps zaleca utrzymywanie danych biznesowych po stronie serwera oraz przechowywanie trwałego stanu w kontrolowanym backendzie. fileciteturn25file1L1375-L1415 fileciteturn25file1L1493-L1509

## CSP and supply-chain security

Każdy widget posiada minimalny allowlist dla `connectDomains`, `resourceDomains` i — tylko gdy niezbędne — `frameDomains`. Dokumentacja wskazuje także, że polityka jest sprawdzana podczas review. fileciteturn25file1L1684-L1697

Control Plane dodaje:

- dependency lock;
- SBOM;
- provenance builda;
- hash artefaktów;
- signed release metadata;
- rollback;
- staged rollout;
- automatic quarantine po regresji.

## Sovereign execution profiles

Na podstawie Project 37 builder nie zakłada jednego backendu inferencyjnego. Aplikacja może otrzymać kilka profili wykonawczych:

```text
DETERMINISTIC
    ↓ fallback
LOCAL SERVER
    ↓ fallback/escalation
DESKTOP EDGE
    ↓ fallback/escalation
MOBILE EDGE
    ↓ policy-controlled escalation
CLOUD
```

Dzięki temu prywatność, offline mode, latency i koszt stają się parametrami aplikacji, a nie przypadkową właściwością dostawcy modelu. Warstwa edge musi jednak przejść tę samą walidację i policy enforcement co cloud.

## Provider-neutral AI contract

Builder powinien generować adapter zamiast wiązać aplikację z jednym dostawcą:

`capabilities → session → generate → stream → structured_output → tool_call → usage → health → cancel`

Dostarczony materiał o architekturze długowiecznej pokazuje rozdzielenie logiki biznesowej od konkretnego modelu przez warstwę Genkit oraz lokalną inferencję przez Ollama. fileciteturn77file2L129-L153

## Local-first policy

Manifest deklaruje klasę prywatności i dopuszczalny egress danych. Control Plane wybiera najniższy poziom wykonawczy spełniający wymagania.

```text
PRIVATE
 ↓
NO CLOUD EGRESS
 ↓
LOCAL / EDGE ONLY
```

Cloud escalation jest dozwolone wyłącznie, gdy:

- policy dopuszcza transfer;
- wymagane capability nie istnieje lokalnie;
- jakość lokalna nie przechodzi verification gate;
- użytkownik/polityka dopuszcza konsekwencję działania.

## Mobile and browser resilience

Edge APIs są traktowane jako niestabilne zależności. Wrapper odpowiada za capability detection, lifecycle sesji, streaming, timeouty, brak modelu, cold start i graceful degradation. Materiał źródłowy proponuje dokładnie taki wrapper dla `window.ai`. fileciteturn77file4L233-L280

## Monetization integration

Builder jest sprzężony z Project 31, ale billing pozostaje oddzielony od generatora kodu. Plan komercyjny decyduje o limitach i funkcjach, nie o bezpieczeństwie.

Dla workflow z płatnością obowiązuje zasada: serwer jest źródłem prawdy dla ceny i statusu zamówienia, a operacje płatnicze muszą być idempotentne. fileciteturn25file1L1745-L1766

## Release gates

```text
G0 — SPEC VALID
G1 — CAPABILITIES AUTHORIZED
G2 — BUILD REPRODUCIBLE
G3 — TESTS PASS
G4 — SECURITY REVIEW PASS
G5 — SIGNING PASS
G6 — HUMAN / POLICY APPROVAL
G7 — STAGED DEPLOYMENT
G8 — OBSERVED STABILITY
G9 — EDGE/LOCAL FALLBACK VERIFIED
G10 — DATA-EGRESS POLICY VERIFIED
```

Brak przejścia przez gate blokuje promocję.

## Integracja

Project 33 integruje Projects 07, 13, 16, 20, 24, 25, 26, 28, 31, 35, 36 i 37.

## Definition of Done

- inspectable app manifest;
- versioned Skills;
- governed MCP capabilities;
- resumable build lifecycle;
- reproducible release artifacts;
- signing and rollback;
- MCP Apps data/render separation;
- CSP allowlists;
- provider-neutral execution contract;
- local/edge/cloud routing;
- explicit data-egress policy;
- deterministic fallback;
- telemetry/evals;
- commercial limits isolated from security policy.

---

## Knowledge evolution — batch 15 / 2026-10-08: OmniStack app-builder comparison into accountable contracts

Source `Analiza Repozytoriów i Projekt Aplikacji.PDF` (34 pages) describes Convex Chef, Wasp MAGE, Open Lovable, Open Design, CodinIT.dev, December, Dyad, Bolt.diy and a proposed OmniStack AI with agent swarms, WebContainers/sandboxing, full-stack schema/DB design, test-first workflows and "Shadow QA".

**P33 ownership:** source-to-app specification → typed UI + API contracts → tests first → generator → independent security and design QA → staged build/sign/publish → verified outcome/rollback. A simulated test log in a PDF does not prove an actual test run, and upstream tool versions/licensing are unverified. Isolation must be real (not a prompt), secrets off clients and external deploy require explicit authorization. `AppBuildEvidence`: source SHA/requirements, test receipt, code diff, dependency/license lock, security scan, artifact SHA, independent reviewer, rollback. Reuse P100 developer UI and P115 orchestrator rather than creating duplicate "OmniStack" project.

---

## 2026-10-08 — batch 18: public web source evolution

Google Cloud 101 gen-AI technical blueprints (2025) are **illustrative architecture patterns**, not 101 ready applications. Add BlueprintQualification with user problem, canonical source and license, required vs optional cloud stack, privacy/consent, vendor quotes and predictable charges, measurable acceptance tests, actual build/deploy receipt and rollback. Gemini CLI/Antigravity optional build adapters require independent tool authorization and CI. Zed/Railway project boards showed shell/title but no inspectable bounty/task records; no paid WordPress plugin purchase or bounty claim.

---

## 2026-10-09 — batch 21: Zed Guild, Railway bounties, WordPress premium

**Railway Template Bounties — actionable qualification instead of invented task status.** Source `github.com/orgs/railwayapp/projects/2` renders only a board shell. The official `railwayapp/templates` repository confirms a template bounty scheme, and Railway Station provides task-specific examples, but two verified example rewards ($150 each for NodeBB and GPT OSS) are **historical SOLVED tasks**, not current offers. `RailwayTemplateAcceptance`: current open task URL+timestamp, licensing/public source repo, environment/secret boundaries, correct service graph, ordered startup and health checks, persistent volumes, validated domain/network paths, cost cap, independent deploy/test receipt and actual acceptance. No Railway app deploy or new project creation; P33 remains application-delivery owner. [Evidence ledger](../docs/WEB-SOURCE-INGESTION-2026-10-09-BATCH-21.md).

---

## 2026-10-09 — batch 23: seven-page GitHub MCP Registry discovery

Seven-page GitHub MCP scan surfaces **Playwright, Chrome DevTools, Cypress Cloud, mabl, Wopee, Argus Testing** (QA/browser), **Figma, Miro, draw.io, Lucid, Mobbin, Anima** (UX/design references), **Vercel, Webflow, Wix, Lovable, gamedev.pl** (potential app builders). Proposed build adapter contract: platform + exact official publisher and license, baseline app spec, reproducible source, permission envelope, actual CI/build, preview/screenshot evidence, rollback, cost, approval, quality benchmark. Descriptions saying “autonomous testing” or “free” are provider statements; no app was built, installed or published through the catalog. Avoid duplicate app-generator projects; P33 remains canonical owner.
