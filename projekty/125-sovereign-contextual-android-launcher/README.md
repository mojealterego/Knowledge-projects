# P125 — Sovereign Contextual Android Launcher

**Status:** SOURCE_REVIEWED → ANDROID MVP SOURCE SCAFFOLD / NOT COMPILED OR INSTALLED
**Platform:** Android, Polish-first, privacy-by-default. No cloud account needed.

## New product boundary
Source `AI Launchery Android_ Ranking 2025-2026.PDF` (10 pages) describes contextual home screens, algorithmic app-grouping, UsageStatsManager, sensors and emerging local LLM/NPU use. Existing P105 handles authorized mobile **UI actuation**, P119 handles on-device **multi-agent runtime**, and P08 is a conceptual personal OS. None is a shipping **HOME launcher UI that a user can select as the default Android home**. P125 owns that particular home-screen product, user consent and Android lifecycle; P105/P119 remain optional adapters, not automatic privileges.

## Executable source scaffold
`android/` is a minimal native Kotlin Android application:
- HOME intent filter makes the app eligible as a home-screen choice;
- explicit package-query intent (MAIN + LAUNCHER) observes installed launchable applications only;
- scrollable, alphabetical buttons open **only applications deliberately tapped by the user**;
- no Usage Access, Accessibility, Notification Access, location, mic, contacts or network permission requests;
- no AI or prediction is claimed implemented.
- text is Polish, screen is reversible using the system's default-apps settings.

The scaffold is **source code only**. No Gradle/Android SDK compilation, emulator install, OEM Samsung/Android 16 testing, APK signing, or Play policy certification has been run. GitHub's stdlib policy tests verify declared source invariants, not real Android behavior.

## Versioned architecture
```text
SYSTEM HOME SELECTION BY USER
    → EXPLICIT LAUNCHER APP QUERIES
    → LOCAL, REVERSIBLE CATEGORY/RANKING POLICY
    → TAP TO OPEN (NO HIDDEN AUTO ACTION)
    → READBACK/STATUS → USER OVERRIDE/DEFAULT HOME FALLBACK

FUTURE OPT-IN LOCAL MODEL (separate capability gate)
    → proposed app ordering, never unapproved app actuation
    → privacy / energy / latency baseline
```

## Planned phases
- **MVP:** compile, install and verify exported HOME activity, package queries, tap-to-open and safe fallback on Android emulator.
- **Privacy:** manual favorite/pin categories, no behavioral tracking. Local opt-in patterns require a visible permission/opt-out state.
- **Optional local LLM:** ability to recommend app groups using only user-approved context; cannot secretly read notifications or contacts.
- **Optional P105/P119 integration:** only verified, user-triggered task actions behind independent permission broker.
- **Release:** accessibility (TalkBack/keyboard, contrast, reduced motion), locale tests, OEM battery/lifecycle and permission audits, secure build/signing/provenance.

## Verification
`python -m unittest discover -s tools -p 'test_*.py' -v` from repo root tests static policy assertions; Android/Gradle build and human-device validation are still required.

[Batch-15 evidence](../../docs/KNOWLEDGE-INGESTION-2026-10-08-BATCH-15.md) · [Source synthesis](../../docs/knowledge-base/2026-10-08-corpus-mobile-builders-security-odyn.md).

---

## 2026-10-08 — batch 18: public web source evolution

Android Developers Googlebook Sept 22 2026 adaptive apps article contributes an explicit **pending** emulator/device acceptance matrix: compact/medium/expanded window width, freeform drag-resize, split-screen, dual instances, keyboard/trackpad navigation, TalkBack and focus order, rotation/process recreation, safe HOME fallback, touch targets and contrast. Use actual window classes rather than assumed physical screen dimensions; no Googlebook Play badge or acceptance is claimed. Existing Kotlin source remains **not built or installed**; these are test requirements, not adaptive Compose code implementation.

---

## 2026-10-09 — batch 22: Googlebook responsive launcher implemented

[Source](https://android-developers.googleblog.com/2026/09/adaptive-development-scale-app-googlebook.html) — official Android Developers article, **2026-09-22**, revisited from batch 18 (same article, mobile `?m=1` URL). The earlier batch only added *planned* adaptive QA. This batch updates actual `android/app/src/main/java/org/mojealterego/sovereignlauncher/MainActivity.kt`, `AndroidManifest.xml` and `strings.xml`:

- The existing Kotlin **classic View** implementation now observes the **available window width via `onSizeChanged`** and density conversion, not fixed physical screen dimensions. At `>=840dp`, it reflows the content into a horizontal scrollable app list + supporting information pane; below that it returns to a single-column list. The 840dp cut-off is an **MVP heuristic**, not a Google Play certification threshold or broad Compose window-class implementation.
- During a window-width change the same button list/panes are relaid out rather than rebuilding or silently launching apps. App launch remains inside a per-app `setOnClickListener` with explicit component. Selected app label is restored from `savedInstanceState`.
- Buttons have `48dp` minimum height, keyboard/D-pad focus and explicit Polish accessibility labels. Strings were centralized in `res/values/strings.xml`. Manifest requests `android:resizeableActivity="true"`; **zero Android sensitive permissions**.
- Static assertions in [Googlebook source contract tests](../../tools/test_googlebook_adaptive_launcher.py) and the prior [launcher safety tests](../../tools/test_sovereign_launcher_policy.py) run in Python CI. [Manual/adaptive device QA matrix](android/ADAPTIVE-QA.md) enumerates 21 scenarios.

**Important:** this is meaningful **actual Kotlin source change**, not an APK build or working Googlebook certification. The source article recommends Compose Navigation 3 `ListDetailSceneStrategy`, `SupportingPaneSceneStrategy`, desktop hover/right-click, multi-instance, drag-and-drop, `HandoffActivityData`, widgets and desktop emulator; these are **source-derived potential next features, not implemented in this Views scaffold**. Android SDK compilation, emulator/manual rendering, keyboard accessibility and real OEM device verification remain outstanding. No system HOME default is changed on any phone by committing this code.

[Evidence ledger](../../docs/WEB-SOURCE-INGESTION-2026-10-09-BATCH-22.md) · [Project evolution](../../docs/PROJECT-EVOLUTION-2026-10-09-BATCH-22.md).
